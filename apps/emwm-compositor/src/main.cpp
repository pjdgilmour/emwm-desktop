// SPDX-License-Identifier: GPL-3.0-only
#include "renderer.h"
#include <X11/Xatom.h>
#include <X11/Xutil.h>
#include <X11/extensions/Xcomposite.h>
#include <X11/extensions/Xdamage.h>
#include <X11/extensions/Xfixes.h>
#include <X11/extensions/shape.h>
#include <X11/extensions/Xrandr.h>
#include <algorithm>
#include <chrono>
#include <csignal>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <poll.h>
#include <sstream>
#include <stdexcept>
#include <unistd.h>

namespace fs=std::filesystem;
using Clock=std::chrono::steady_clock;
static double now() { return std::chrono::duration<double>(Clock::now().time_since_epoch()).count(); }
static volatile sig_atomic_t stopping=0,reloading=0;
static void signalHandler(int s) { if(s==SIGHUP || s==SIGUSR1) reloading=1;else stopping=1; }
static int lastError=0;
static int damageError=0;
static bool trap=false;
static int errorHandler(Display *d,XErrorEvent *e) {
    lastError=e->error_code;
    // Damage objects are destroyed by the server with their drawable; queued
    // notifications and cleanup requests can legitimately race that destruction.
    if (!trap && e->error_code!=damageError && e->error_code!=BadWindow && e->error_code!=BadPixmap && e->error_code!=BadDrawable) {
        char text[160];XGetErrorText(d,e->error_code,text,sizeof text);
        std::cerr << "emwm-compositor: X11 " << text << " request=" << int(e->request_code)
                  << '.' << int(e->minor_code) << " resource=" << e->resourceid << '\n';
    }
    return 0;
}
static void beginTrap(Display *d) { XSync(d,False);lastError=0;trap=true; }
static bool endTrap(Display *d) { XSync(d,False);trap=false;return lastError==0; }

struct Settings {
    std::string backend="auto";
    Effect effect=Effect::Zoom;
    int duration=220,fade=120,radius=16,offset=6,fps=60;
    double shadowOpacity=.45;
    bool shadows=true,vsync=true;
};
static std::string trim(std::string s) {
    auto a=s.find_first_not_of(" \t\r\n"),b=s.find_last_not_of(" \t\r\n");
    return a==std::string::npos?"":s.substr(a,b-a+1);
}
static Settings readSettings(const fs::path &path) {
    Settings s;std::ifstream f(path);std::string line;
    while(std::getline(f,line)) {
        line=trim(line);if(line.empty() || line[0]=='#' || line[0]=='[') continue;
        auto p=line.find('=');if(p==std::string::npos) continue;
        auto key=trim(line.substr(0,p)),value=trim(line.substr(p+1));
        try {
            if(key=="backend" && (value=="auto" || value=="glx" || value=="xrender")) s.backend=value;
            else if(key=="effect") {
                if(value=="none") s.effect=Effect::Disabled;
                else if(value=="zoom") s.effect=Effect::Zoom;
                else if(value=="twist") s.effect=Effect::Twist;
                else if(value=="flip") s.effect=Effect::Flip;
            } else if(key=="duration_ms") s.duration=std::clamp(std::stoi(value),50,2000);
            else if(key=="fade_ms") s.fade=std::clamp(std::stoi(value),0,1000);
            else if(key=="shadow_radius") s.radius=std::clamp(std::stoi(value),1,64);
            else if(key=="shadow_offset") s.offset=std::clamp(std::stoi(value),-32,64);
            else if(key=="shadow_opacity") { double n=std::stod(value);if(std::isfinite(n)) s.shadowOpacity=std::clamp(n,0.,1.); }
            else if(key=="shadows") s.shadows=value=="true" || value=="1";
            else if(key=="vsync") s.vsync=value=="true" || value=="1";
            else if(key=="max_fps") s.fps=std::clamp(std::stoi(value),15,240);
        } catch(const std::exception &) { std::cerr << "emwm-compositor: ignoring invalid setting " << key << '\n'; }
    }
    return s;
}

struct Animation {
    ImagePtr image;
    Rect from,to;
    double start=0;
    int kind=0;
    bool pending=false;
};
struct Surface {
    Window id=0,client=0;
    XWindowAttributes attr{};
    Damage damage=0;
    ImagePtr image;
    Animation animation;
    bool mapped=false,desktop=false,managed=false,shadow=false,dirty=true,shaped=false;
    double opacity=1,appeared=0;
    std::vector<XRectangle> shape;
    Rect geometry() const { return {double(attr.x),double(attr.y),double(attr.width+2*attr.border_width),double(attr.height+2*attr.border_width)}; }
};

class Compositor {
    Display *d;
    int screen,damageEvent=0,shapeEvent=0,randrEvent=0;
    Window root,owner=0,overlay=0;
    Atom selection=0;
    bool redirected=false,dirty=true,stackDirty=true;
    std::map<Window,Surface> surfaces;
    std::vector<Window> order;
    std::unique_ptr<Renderer> renderer;
    Settings settings;
    fs::path config;
    std::string overrideBackend;
    std::map<std::string,Atom> atoms;
    Atom atom(const std::string &s) { auto [it,added]=atoms.emplace(s,0);if(added) it->second=XInternAtom(d,s.c_str(),False);return it->second; }
    std::vector<unsigned long> property(Window w,const char *name,Atom type=XA_CARDINAL) {
        Atom actual;int format;unsigned long n=0,left=0;unsigned char *data=nullptr;
        std::vector<unsigned long> values;
        if (w && XGetWindowProperty(d,w,atom(name),0,64,False,type,&actual,&format,&n,&left,&data)==Success
            && actual==type && format==32 && data) {
            auto p=reinterpret_cast<unsigned long*>(data);values.assign(p,p+n);
        }
        if(data) XFree(data);
        return values;
    }
    void identify(Surface &s) {
        auto client=property(s.id,"_EMWM_FRAME_CLIENT",XA_WINDOW);
        s.client=client.empty()?s.id:client[0];s.managed=!client.empty();
        if(s.client!=s.id) XSelectInput(d,s.client,PropertyChangeMask);
        auto types=property(s.client,"_NET_WM_WINDOW_TYPE",XA_ATOM);
        s.desktop=std::find(types.begin(),types.end(),atom("_NET_WM_WINDOW_TYPE_DESKTOP"))!=types.end();
        bool dock=std::find(types.begin(),types.end(),atom("_NET_WM_WINDOW_TYPE_DOCK"))!=types.end();
        auto opacity=property(s.client,"_NET_WM_WINDOW_OPACITY");
        if(opacity.empty()) opacity=property(s.id,"_NET_WM_WINDOW_OPACITY");
        s.opacity=opacity.empty()?1.:double(opacity[0]&0xffffffffUL)/4294967295.;
        // Exclude desktops, backdrops, icons and ARGB popups with client shadows.
        s.shadow=!s.desktop && !dock && !s.shaped && (s.managed || (s.attr.override_redirect && s.attr.depth!=32
            && s.attr.width<DisplayWidth(d,screen)*.9 && s.attr.height<DisplayHeight(d,screen)*.9));
        auto state=property(s.client,"_NET_WM_STATE",XA_ATOM);
        if(std::find(state.begin(),state.end(),atom("_NET_WM_STATE_FULLSCREEN"))!=state.end()) s.shadow=false;
    }
    void updateShape(Surface &s) {
        int n=0,ordering=0;
        auto rects=XShapeGetRectangles(d,s.id,ShapeBounding,&n,&ordering);
        s.shape.clear();s.shaped=false;
        if(rects) {
            s.shape.assign(rects,rects+n);
            s.shaped=n!=1 || rects[0].x!=0 || rects[0].y!=0 ||
                rects[0].width!=s.attr.width || rects[0].height!=s.attr.height;
            XFree(rects);
        }
    }
    void add(Window w) {
        if(w==owner || w==overlay || w==renderer->outputWindow() || surfaces.count(w)) return;
        Surface s;s.id=w;
        if(!XGetWindowAttributes(d,w,&s.attr) || s.attr.c_class==InputOnly) return;
        XSelectInput(d,w,PropertyChangeMask);
        XShapeSelectInput(d,w,ShapeNotifyMask);
        s.damage=XDamageCreate(d,w,XDamageReportNonEmpty);
        s.mapped=s.attr.map_state==IsViewable;
        updateShape(s);identify(s);
        surfaces.emplace(w,std::move(s));stackDirty=dirty=true;
    }
    bool capture(Surface &s) {
        if(s.image) return true;
        if(!s.mapped) return false;
        beginTrap(d);
        try { s.image=renderer->capture(s.id,s.attr); }
        catch(...) { endTrap(d);throw; }
        if(!endTrap(d)) { s.image.reset();return false; }
        s.dirty=false;return true;
    }
    void stack() {
        Window rr,parent,*children=nullptr;unsigned n=0;
        order.clear();
        if(XQueryTree(d,root,&rr,&parent,&children,&n)) {
            for(unsigned i=0;i<n;i++) { add(children[i]);if(surfaces.count(children[i])) order.push_back(children[i]); }
        }
        if(children) XFree(children);
        stackDirty=false;
    }
    void cancelAnimations() { for(auto &[id,s]:surfaces) { s.animation={};s.appeared=0; } }
    void configureRenderer(bool forceFallback=false) {
        cancelAnimations();for(auto &[id,s]:surfaces) { s.image.reset();s.dirty=true; }
        renderer.reset();
        renderer=std::make_unique<Renderer>(d,screen,overlay);
        bool initialized=false;
        try { initialized=renderer->init(forceFallback?"xrender":settings.backend,settings.vsync); }
        catch(const std::runtime_error &error) {
            if(forceFallback || settings.backend=="xrender") throw;
            std::cerr << "emwm-compositor: initialization failed: " << error.what() << '\n';
        }
        if(!initialized) {
            std::cerr << "emwm-compositor: requested backend failed; restoring XRender\n";
            renderer.reset();renderer=std::make_unique<Renderer>(d,screen,overlay);
            if(!renderer->init("xrender",false)) throw std::runtime_error("Cannot initialize renderer");
        }
        std::string backend=renderer->accelerated()?"glx":"xrender";
        XChangeProperty(d,owner,atom("_EMWM_COMPOSITOR_BACKEND"),XA_STRING,8,PropModeReplace,
                        reinterpret_cast<const unsigned char*>(backend.data()),backend.size());
        stackDirty=dirty=true;
    }
    void reload() {
        auto s=readSettings(config);if(!overrideBackend.empty()) s.backend=overrideBackend;
        bool rebuild=s.backend!=settings.backend || s.vsync!=settings.vsync;
        settings=s;
        if(rebuild) configureRenderer();
        if(settings.effect==Effect::Disabled) cancelAnimations();
        dirty=true;
    }
    Rect iconGeometry(Window w,Rect fallback) {
        XWindowAttributes a{};int x=0,y=0;Window child;
        if(w && XGetWindowAttributes(d,w,&a) && XTranslateCoordinates(d,w,root,0,0,&x,&y,&child))
            return {double(x),double(y),double(a.width),double(a.height)};
        return {fallback.x+fallback.w/2-24.,double(DisplayHeight(d,screen)-56),48,48};
    }
    void animate(const XClientMessageEvent &e) {
        if(settings.effect==Effect::Disabled || e.format!=32) return;
        auto it=surfaces.find(e.data.l[0]);if(it==surfaces.end()) return;
        auto &s=it->second;int kind=e.data.l[1],phase=e.data.l[3];
        if(!s.managed || s.desktop || kind<1 || kind>4) return;
        auto icon=iconGeometry(e.data.l[2],s.geometry());
        if(phase==0) {
            Rect from=s.geometry();
            if(s.animation.kind && !s.animation.pending)
                from=interpolate(s.animation.from,s.animation.to,ease((now()-s.animation.start)*1000/settings.duration));
            s.animation={s.image,kind==2?icon:from,s.geometry(),now(),kind,true};
        } else if(phase==1 && s.animation.kind==kind) {
            XWindowAttributes a;
            if(XGetWindowAttributes(d,s.id,&a)) s.attr=a;
            s.animation.to=kind==1?icon:s.geometry();
            s.animation.start=now();s.animation.pending=false;s.appeared=0;
            dirty=true;
            // Exposed as diagnostics for isolated integration tests and debugging.
            unsigned long values[]={static_cast<unsigned long>(kind),s.id};
            XChangeProperty(d,owner,atom("_EMWM_LAST_ANIMATION"),XA_CARDINAL,32,PropModeReplace,
                            reinterpret_cast<unsigned char*>(values),2);
        }
    }
    void event(XEvent &e) {
        if(e.type==damageEvent+XDamageNotify) {
            auto ev=reinterpret_cast<XDamageNotifyEvent*>(&e);
            auto it=surfaces.find(ev->drawable);
            if(it!=surfaces.end()) { XDamageSubtract(d,it->second.damage,None,None);it->second.dirty=true;dirty=true; }
            return;
        }
        if(e.type==shapeEvent+ShapeNotify) {
            auto ev=reinterpret_cast<XShapeEvent*>(&e);auto it=surfaces.find(ev->window);
            if(it!=surfaces.end()) { updateShape(it->second);identify(it->second);dirty=true; }return;
        }
        if(e.type==randrEvent+RRScreenChangeNotify) {
            XRRUpdateConfiguration(&e);renderer->resize(DisplayWidth(d,screen),DisplayHeight(d,screen));
            cancelAnimations();dirty=true;return;
        }
        switch(e.type) {
        case CreateNotify: if(e.xcreatewindow.parent==root) add(e.xcreatewindow.window);break;
        case ReparentNotify:
            if(e.xreparent.parent==root) add(e.xreparent.window);
            else { auto it=surfaces.find(e.xreparent.window);if(it!=surfaces.end()) { XDamageDestroy(d,it->second.damage);surfaces.erase(it); } }
            stackDirty=dirty=true;break;
        case MapNotify: {
            add(e.xmap.window);auto it=surfaces.find(e.xmap.window);
            if(it==surfaces.end()) break;
            auto &s=it->second;s.mapped=true;s.image.reset();s.dirty=true;
            XGetWindowAttributes(d,s.id,&s.attr);updateShape(s);identify(s);
            if(s.managed && !s.desktop && !s.animation.kind) s.appeared=now();
            stackDirty=dirty=true;break;
        }
        case UnmapNotify: {
            auto it=surfaces.find(e.xunmap.window);
            if(it!=surfaces.end()) { it->second.mapped=false;it->second.image.reset();dirty=true; }break;
        }
        case DestroyNotify: {
            auto it=surfaces.find(e.xdestroywindow.window);
            if(it!=surfaces.end()) { XDamageDestroy(d,it->second.damage);surfaces.erase(it);dirty=stackDirty=true; }break;
        }
        case ConfigureNotify: {
            if(e.xconfigure.window==root) {
                renderer->resize(e.xconfigure.width,e.xconfigure.height);cancelAnimations();dirty=true;break;
            }
            auto it=surfaces.find(e.xconfigure.window);if(it==surfaces.end()) break;
            auto &s=it->second;auto &a=s.attr;
            if(a.width!=e.xconfigure.width || a.height!=e.xconfigure.height || a.border_width!=e.xconfigure.border_width) {
                s.image.reset();s.dirty=true;
            }
            a.x=e.xconfigure.x;a.y=e.xconfigure.y;a.width=e.xconfigure.width;
            a.height=e.xconfigure.height;a.border_width=e.xconfigure.border_width;
            updateShape(s);identify(s);stackDirty=dirty=true;break;
        }
        case CirculateNotify: stackDirty=dirty=true;break;
        case PropertyNotify:
            if(e.xproperty.window==root) {
                if(e.xproperty.atom==atom("_NET_CURRENT_DESKTOP")) cancelAnimations();
                dirty=true;
            } else {
                for(auto &[id,s]:surfaces) if(id==e.xproperty.window || s.client==e.xproperty.window) { identify(s);dirty=true; }
            }
            break;
        case ClientMessage: if(e.xclient.message_type==atom("_EMWM_ANIMATION")) animate(e.xclient);break;
        case SelectionClear: if(e.xselectionclear.selection==selection) stopping=1;break;
        case Expose: dirty=true;break;
        }
    }
    bool paint() {
        if(stackDirty) stack();
        bool active=false;double t=now();renderer->begin();
        for(auto id:order) {
            auto &s=surfaces.at(id);auto &anim=s.animation;
            if(anim.kind && (t-anim.start)*1000>settings.duration+100) anim={};
            if(!s.mapped && !anim.kind) continue;
            if(s.mapped) {
                capture(s);
                if(s.dirty && s.image) { renderer->refresh(s.image);s.dirty=false; }
            }
            Rect rect=s.geometry();double alpha=s.opacity;
            ImagePtr image=s.image;
            double progress=1;
            if(anim.kind && !anim.pending) {
                progress=std::clamp((t-anim.start)*1000/settings.duration,0.,1.);
                if(progress>=1) { anim={};if(!s.mapped) continue; }
                else {
                    active=true;rect=interpolate(anim.from,anim.to,ease(progress));
                    if(anim.kind==1) { image=anim.image;alpha*=1-.7*ease(progress); }
                    else if(!image) image=anim.image;
                }
            }
            if(s.appeared && settings.fade) {
                double p=(t-s.appeared)*1000/settings.fade;
                if(p<1) { alpha*=ease(p);active=true; } else s.appeared=0;
            }
            if(!image) continue;
            if(settings.shadows && s.shadow) renderer->shadowAt(rect,settings.shadowOpacity*alpha,settings.radius,settings.offset);
            renderer->draw(image,rect,alpha,s.shaped?&s.shape:nullptr);
            if(anim.kind && !anim.pending) {
                int count=settings.effect==Effect::Zoom?3:1;
                for(int i=0;i<count;i++) {
                    double p=std::clamp(progress-i*.045,0.,1.);
                    renderer->line(outline(interpolate(anim.from,anim.to,ease(p)),settings.effect,p),.85-i*.2);
                }
            }
        }
        renderer->present();dirty=false;return active;
    }
public:
    Compositor(Display *display,fs::path file,std::string backend):d(display),screen(DefaultScreen(d)),
        root(RootWindow(d,screen)),config(std::move(file)),overrideBackend(std::move(backend)) {}
    ~Compositor() {
        for(auto &[id,s]:surfaces) if(s.damage) XDamageDestroy(d,s.damage);
        surfaces.clear();renderer.reset();
        if(redirected) XCompositeUnredirectSubwindows(d,root,CompositeRedirectManual);
        if(overlay) XCompositeReleaseOverlayWindow(d,root);
        if(owner) {
            if(XGetSelectionOwner(d,selection)==owner) XSetSelectionOwner(d,selection,None,CurrentTime);
            XDestroyWindow(d,owner);
        }
        XSync(d,False);
    }
    void run() {
        int err,ev,major=0,minor=0;
        if(!XCompositeQueryExtension(d,&ev,&err) || !XDamageQueryExtension(d,&damageEvent,&err)
            || !XFixesQueryExtension(d,&ev,&err) || !XRenderQueryExtension(d,&ev,&err)
            || !XShapeQueryExtension(d,&shapeEvent,&err)) throw std::runtime_error("Required X11 extensions unavailable");
        XCompositeQueryVersion(d,&major,&minor);
        XDamageQueryExtension(d,&damageEvent,&damageError);
        if(major==0 && minor<3) throw std::runtime_error("XComposite 0.3 or newer is required");
        if(XRRQueryExtension(d,&randrEvent,&err)) XRRSelectInput(d,root,RRScreenChangeNotifyMask);
        selection=atom("_NET_WM_CM_S"+std::to_string(screen));
        XGrabServer(d);
        if(XGetSelectionOwner(d,selection)!=None) { XUngrabServer(d);throw std::runtime_error("Another compositor already owns this X screen"); }
        owner=XCreateSimpleWindow(d,root,-1,-1,1,1,0,0,0);
        XStoreName(d,owner,"EMWM Compositor");
        XSetSelectionOwner(d,selection,owner,CurrentTime);
        XUngrabServer(d);XSync(d,False);
        if(XGetSelectionOwner(d,selection)!=owner) throw std::runtime_error("Cannot acquire compositor selection");
        XEvent manager{};manager.xclient.type=ClientMessage;manager.xclient.window=root;
        manager.xclient.message_type=atom("MANAGER");manager.xclient.format=32;
        manager.xclient.data.l[0]=CurrentTime;manager.xclient.data.l[1]=selection;manager.xclient.data.l[2]=owner;
        XSendEvent(d,root,False,StructureNotifyMask,&manager);
        beginTrap(d);XCompositeRedirectSubwindows(d,root,CompositeRedirectManual);
        if(!endTrap(d)) throw std::runtime_error("Cannot redirect X11 windows (another compositor may be active)");
        redirected=true;overlay=XCompositeGetOverlayWindow(d,root);
        auto empty=XFixesCreateRegion(d,nullptr,0);
        XFixesSetWindowShapeRegion(d,overlay,ShapeInput,0,0,empty);XFixesDestroyRegion(d,empty);
        XSelectInput(d,root,SubstructureNotifyMask|StructureNotifyMask|PropertyChangeMask);
        settings=readSettings(config);if(!overrideBackend.empty()) settings.backend=overrideBackend;
        configureRenderer();stack();
        double lastPaint=0,lastConfig=0;fs::file_time_type stamp{};
        bool active=false;
        while(!stopping) {
            // Bound event draining so a busy client cannot starve presentation.
            for(int n=0;n<2048 && XPending(d);n++) { XEvent e;XNextEvent(d,&e);event(e); }
            double t=now();
            if(reloading || t-lastConfig>=1) {
                std::error_code ec;auto changed=fs::last_write_time(config,ec);
                if(reloading || (!ec && changed!=stamp)) { reload();stamp=changed;reloading=0; }
                lastConfig=t;
            }
            double interval=1./settings.fps;
            if((dirty || active) && t-lastPaint>=interval) {
                try { active=paint(); }
                catch(const std::runtime_error &error) {
                    if(!renderer->accelerated()) throw;
                    std::cerr << "emwm-compositor: " << error.what() << "; switching to XRender\n";
                    configureRenderer(true);active=false;
                }
                lastPaint=now();
            }
            int timeout=dirty||active?std::max(1,int((interval-(now()-lastPaint))*1000)):500;
            if(!XPending(d)) { pollfd p{ConnectionNumber(d),POLLIN,0};poll(&p,1,timeout); }
        }
    }
};

int main(int argc,char **argv) {
    auto home=getenv("HOME"),xdg=getenv("XDG_CONFIG_HOME");
    fs::path config=(xdg&&*xdg?fs::path(xdg):fs::path(home?home:".")/".config")/"emwm-desktop/compositor.conf";
    std::string backend;
    for(int i=1;i<argc;i++) {
        std::string arg=argv[i];
        if(arg=="--help") { std::cout << "emwm-compositor [--backend auto|glx|xrender] [--config FILE]\nReload: SIGHUP/SIGUSR1, or edit compositor.conf.\n";return 0; }
        if((arg=="--backend" || arg=="--config") && i+1<argc) {
            if(arg=="--backend") backend=argv[++i];else config=argv[++i];
        } else { std::cerr << "Invalid argument: " << arg << '\n';return 2; }
    }
    if(!backend.empty() && backend!="auto" && backend!="glx" && backend!="xrender") return 2;
    Display *d=XOpenDisplay(nullptr);if(!d) { std::cerr << "emwm-compositor: cannot open X display\n";return 1; }
    XSetErrorHandler(errorHandler);
    for(int s:{SIGINT,SIGTERM,SIGHUP,SIGUSR1}) std::signal(s,signalHandler);
    int result=0;
    try { Compositor compositor(d,config,backend);compositor.run(); }
    catch(const std::exception &e) { std::cerr << "emwm-compositor: " << e.what() << '\n';result=1; }
    XCloseDisplay(d);return result;
}
