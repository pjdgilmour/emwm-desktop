// SPDX-License-Identifier: GPL-3.0-only
#include "renderer.h"
#include <X11/Xutil.h>
#include <X11/extensions/Xcomposite.h>
#include <X11/extensions/Xfixes.h>
#include <X11/extensions/shape.h>
#include <cstring>
#include <iostream>
#include <stdexcept>

Image::~Image() {
    if (bound && release) release(dpy,glpixmap,GLX_FRONT_LEFT_EXT);
    if (texture) glDeleteTextures(1,&texture);
    if (glpixmap) glXDestroyPixmap(dpy,glpixmap);
    if (picture) XRenderFreePicture(dpy,picture);
    if (pixmap) XFreePixmap(dpy,pixmap);
}

static bool extension(const char *list, const char *name) {
    if (!list) return false;
    size_t n=std::strlen(name);
    for (const char *p=list; (p=std::strstr(p,name)); p+=n)
        if ((p==list || p[-1]==' ') && (p[n]==' ' || p[n]=='\0')) return true;
    return false;
}

bool Renderer::initGL(bool vsync) {
    int major=0,minor=0;
    if (!glXQueryVersion(dpy,&major,&minor) || major<1 || (major==1 && minor<3)) return false;
    const char *ext=glXQueryExtensionsString(dpy,screen);
    if (!extension(ext,"GLX_EXT_texture_from_pixmap")) return false;
    bind=reinterpret_cast<PFNGLXBINDTEXIMAGEEXTPROC>(glXGetProcAddressARB(reinterpret_cast<const GLubyte*>("glXBindTexImageEXT")));
    release=reinterpret_cast<PFNGLXRELEASETEXIMAGEEXTPROC>(glXGetProcAddressARB(reinterpret_cast<const GLubyte*>("glXReleaseTexImageEXT")));
    if (!bind || !release) return false;
    int attrs[]={GLX_X_RENDERABLE,True,GLX_DRAWABLE_TYPE,GLX_WINDOW_BIT,
        GLX_RENDER_TYPE,GLX_RGBA_BIT,GLX_DOUBLEBUFFER,True,GLX_RED_SIZE,8,
        GLX_GREEN_SIZE,8,GLX_BLUE_SIZE,8,None};
    int n=0;
    auto configs=glXChooseFBConfig(dpy,screen,attrs,&n);
    if (!configs || !n) { if(configs) XFree(configs); return false; }
    XVisualInfo *vi=nullptr;
    GLXFBConfig config=nullptr;
    for (int i=0;i<n;i++) {
        auto v=glXGetVisualFromFBConfig(dpy,configs[i]);
        if (v && v->depth==DefaultDepth(dpy,screen)) { vi=v;config=configs[i];break; }
        if (v) XFree(v);
    }
    if (!vi) { XFree(configs);return false; }
    context=glXCreateNewContext(dpy,config,GLX_RGBA_TYPE,nullptr,True);
    if (!context) { XFree(vi);XFree(configs);return false; }
    colormap=XCreateColormap(dpy,overlay,vi->visual,AllocNone);
    XSetWindowAttributes wa{};
    wa.colormap=colormap;wa.border_pixel=0;wa.override_redirect=True;
    output=XCreateWindow(dpy,overlay,0,0,1,1,0,vi->depth,InputOutput,vi->visual,
                         CWColormap|CWBorderPixel|CWOverrideRedirect,&wa);
    glwindow=glXCreateWindow(dpy,config,output,nullptr);
    XFree(vi);XFree(configs);
    if (!glXMakeContextCurrent(dpy,glwindow,glwindow,context)) return false;
    gl=true;
    auto region=XFixesCreateRegion(dpy,nullptr,0);
    XFixesSetWindowShapeRegion(dpy,output,ShapeInput,0,0,region);
    XFixesDestroyRegion(dpy,region);
    XMapWindow(dpy,output);
    glEnable(GL_BLEND);glBlendFunc(GL_ONE,GL_ONE_MINUS_SRC_ALPHA);
    glDisable(GL_DEPTH_TEST);
    glTexEnvi(GL_TEXTURE_ENV,GL_TEXTURE_ENV_MODE,GL_MODULATE);
    bool sync=false;
    if (extension(ext,"GLX_EXT_swap_control")) {
        auto swap=reinterpret_cast<PFNGLXSWAPINTERVALEXTPROC>(glXGetProcAddressARB(reinterpret_cast<const GLubyte*>("glXSwapIntervalEXT")));
        if (swap) { swap(dpy,glwindow,vsync?1:0);sync=vsync; }
    } else if (extension(ext,"GLX_MESA_swap_control")) {
        auto swap=reinterpret_cast<PFNGLXSWAPINTERVALMESAPROC>(glXGetProcAddressARB(reinterpret_cast<const GLubyte*>("glXSwapIntervalMESA")));
        if (swap) sync=(swap(vsync?1:0)==0 && vsync);
    }
    std::cerr << "emwm-compositor: backend=glx renderer=" << glGetString(GL_RENDERER)
              << " direct=" << glXIsDirect(dpy,context) << " vsync=" << sync << '\n';
    return true;
}

GLXFBConfig Renderer::pixmapConfig(Visual *visual,int depth,bool &inverted) {
    auto key=std::make_pair(XVisualIDFromVisual(visual),depth);
    auto cached=pixmapConfigs.find(key);
    if(cached!=pixmapConfigs.end()) { inverted=cached->second.second;return cached->second.first; }
    int n=0;auto configs=glXGetFBConfigs(dpy,screen,&n);
    GLXFBConfig found=nullptr;
    for (int i=0;i<n;i++) {
        int drawable=0,bindable=0,targets=0,inv=0;
        // Pixmaps have a depth and pixel layout, not a VisualID. In particular,
        // NVIDIA's first ARGB X visual need not itself have a bindable FBConfig.
        // Accept an equivalent layout instead of requiring the same visual ID.
        auto vi=glXGetVisualFromFBConfig(dpy,configs[i]);
        bool compatible=vi && vi->depth==depth && vi->c_class==TrueColor &&
            vi->red_mask==visual->red_mask && vi->green_mask==visual->green_mask &&
            vi->blue_mask==visual->blue_mask;
        if (vi) XFree(vi);
        if (!compatible) continue;
        glXGetFBConfigAttrib(dpy,configs[i],GLX_DRAWABLE_TYPE,&drawable);
        glXGetFBConfigAttrib(dpy,configs[i],depth==32?GLX_BIND_TO_TEXTURE_RGBA_EXT:GLX_BIND_TO_TEXTURE_RGB_EXT,&bindable);
        glXGetFBConfigAttrib(dpy,configs[i],GLX_BIND_TO_TEXTURE_TARGETS_EXT,&targets);
        if (!(drawable&GLX_PIXMAP_BIT) || !bindable || !(targets&GLX_TEXTURE_2D_BIT_EXT)) continue;
        glXGetFBConfigAttrib(dpy,configs[i],GLX_Y_INVERTED_EXT,&inv);
        inverted=inv;found=configs[i];break;
    }
    if (configs) XFree(configs);
    pixmapConfigs.emplace(key,std::make_pair(found,inverted));
    return found;
}

bool Renderer::init(const std::string &backend,bool vsync) {
    if (backend!="xrender" && !initGL(vsync)) {
        if (backend=="glx") return false;
        std::cerr << "emwm-compositor: GLX unavailable; using XRender\n";
        if (glwindow) { glXDestroyWindow(dpy,glwindow);glwindow=0; }
        if (output) { XDestroyWindow(dpy,output);output=0; }
        if (context) { glXDestroyContext(dpy,context);context=nullptr; }
    }
    if (!gl) {
        XRenderPictureAttributes pa{};pa.subwindow_mode=IncludeInferiors;
        destination=XRenderCreatePicture(dpy,overlay,
            XRenderFindVisualFormat(dpy,DefaultVisual(dpy,screen)),CPSubwindowMode,&pa);
        std::cerr << "emwm-compositor: backend=xrender\n";
    }
    resize(DisplayWidth(dpy,screen),DisplayHeight(dpy,screen));
    // Reusable nine-patch Gaussian shadow. CPU generation happens once.
    XVisualInfo vi{};
    if (!XMatchVisualInfo(dpy,screen,32,TrueColor,&vi)) return true;
    constexpr int size=65;
    auto pix=XCreatePixmap(dpy,overlay,size,size,32);
    XImage *xi=XCreateImage(dpy,vi.visual,32,ZPixmap,0,nullptr,size,size,32,0);
    if (!xi) { XFreePixmap(dpy,pix); return true; }
    xi->data=static_cast<char*>(calloc(size,xi->bytes_per_line));
    if (!xi->data) { XDestroyImage(xi);XFreePixmap(dpy,pix);return true; }
    for(int y=0;y<size;y++) for(int x=0;x<size;x++) {
        auto axis=[](double p) { return .5*(std::erf((p-16)/7.)-std::erf((p-49)/7.)); };
        unsigned long a=std::lround(255*axis(x)*axis(y));
        XPutPixel(xi,x,y,a<<24);
    }
    auto gc=XCreateGC(dpy,pix,0,nullptr);
    XPutImage(dpy,pix,gc,xi,0,0,0,0,size,size);
    XFreeGC(dpy,gc);XDestroyImage(xi);
    shadow=image(pix,vi.visual,32,size,size);
    return true;
}

Renderer::~Renderer() {
    shadow.reset();
    if (target) XRenderFreePicture(dpy,target);
    if (buffer) XFreePixmap(dpy,buffer);
    if (destination) XRenderFreePicture(dpy,destination);
    if (context) {
        glXMakeContextCurrent(dpy,None,None,nullptr);
        if (glwindow) glXDestroyWindow(dpy,glwindow);
        glXDestroyContext(dpy,context);
    }
    if (output) XDestroyWindow(dpy,output);
    if (colormap) XFreeColormap(dpy,colormap);
}

void Renderer::resize(int w,int h) {
    width=std::max(1,w);height=std::max(1,h);
    if (gl) {
        XResizeWindow(dpy,output,width,height);
        glViewport(0,0,width,height);
        glMatrixMode(GL_PROJECTION);glLoadIdentity();glOrtho(0,width,height,0,-1,1);
        glMatrixMode(GL_MODELVIEW);glLoadIdentity();
    } else {
        if(target) XRenderFreePicture(dpy,target);
        if(buffer) XFreePixmap(dpy,buffer);
        buffer=XCreatePixmap(dpy,overlay,width,height,DefaultDepth(dpy,screen));
        target=XRenderCreatePicture(dpy,buffer,XRenderFindVisualFormat(dpy,DefaultVisual(dpy,screen)),0,nullptr);
    }
}

ImagePtr Renderer::image(Pixmap pixmap,Visual *visual,int depth,int w,int h) {
    auto im=std::make_shared<Image>();
    im->dpy=dpy;im->pixmap=pixmap;im->width=w;im->height=h;
    if (gl) {
        auto config=pixmapConfig(visual,depth,im->inverted);
        if (!config) throw std::runtime_error("No GLX texture configuration for window visual");
        int attrs[]={GLX_TEXTURE_TARGET_EXT,GLX_TEXTURE_2D_EXT,GLX_TEXTURE_FORMAT_EXT,
            depth==32?GLX_TEXTURE_FORMAT_RGBA_EXT:GLX_TEXTURE_FORMAT_RGB_EXT,None};
        im->glpixmap=glXCreatePixmap(dpy,config,pixmap,attrs);
        im->release=release;
        glGenTextures(1,&im->texture);glBindTexture(GL_TEXTURE_2D,im->texture);
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MIN_FILTER,GL_LINEAR);
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MAG_FILTER,GL_LINEAR);
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_WRAP_S,GL_CLAMP_TO_EDGE);
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_WRAP_T,GL_CLAMP_TO_EDGE);
        refresh(im);
    } else {
        XRenderPictureAttributes pa{};pa.subwindow_mode=IncludeInferiors;
        im->picture=XRenderCreatePicture(dpy,pixmap,XRenderFindVisualFormat(dpy,visual),CPSubwindowMode,&pa);
        XRenderSetPictureFilter(dpy,im->picture,FilterBilinear,nullptr,0);
    }
    return im;
}

ImagePtr Renderer::capture(Window window,const XWindowAttributes &a) {
    return image(XCompositeNameWindowPixmap(dpy,window),a.visual,a.depth,
                 a.width+2*a.border_width,a.height+2*a.border_width);
}

void Renderer::refresh(const ImagePtr &im) {
    if (gl && im && im->texture) {
        glBindTexture(GL_TEXTURE_2D,im->texture);
        if(im->bound) release(dpy,im->glpixmap,GLX_FRONT_LEFT_EXT);
        bind(dpy,im->glpixmap,GLX_FRONT_LEFT_EXT,nullptr);im->bound=true;
    }
}

void Renderer::begin() {
    if (gl) { glDisable(GL_SCISSOR_TEST);glClearColor(.22,.25,.29,1);glClear(GL_COLOR_BUFFER_BIT); }
    else { XRenderColor c{14418,16384,19005,65535};XRenderFillRectangle(dpy,PictOpSrc,target,&c,0,0,width,height); }
}

void Renderer::draw(const ImagePtr &im,Rect r,double opacity,const std::vector<XRectangle> *shape) {
    if (!im || r.w<.1 || r.h<.1 || opacity<=0) return;
    opacity=std::clamp(opacity,0.,1.);
    if (gl) {
        glEnable(GL_TEXTURE_2D);glBindTexture(GL_TEXTURE_2D,im->texture);
        glColor4d(opacity,opacity,opacity,opacity);
        auto quad=[&] {
            double y0=im->inverted?0:1,y1=1-y0;
            glBegin(GL_QUADS);
            glTexCoord2d(0,y0);glVertex2d(r.x,r.y);
            glTexCoord2d(1,y0);glVertex2d(r.x+r.w,r.y);
            glTexCoord2d(1,y1);glVertex2d(r.x+r.w,r.y+r.h);
            glTexCoord2d(0,y1);glVertex2d(r.x,r.y+r.h);
            glEnd();
        };
        if (shape) {
            glEnable(GL_SCISSOR_TEST);
            for (auto s:*shape) {
                int x=std::floor(r.x+s.x*r.w/im->width), y=std::floor(r.y+s.y*r.h/im->height);
                int w=std::ceil(s.width*r.w/im->width), h=std::ceil(s.height*r.h/im->height);
                glScissor(x,height-y-h,w,h);quad();
            }
            glDisable(GL_SCISSOR_TEST);
        } else quad();
        glDisable(GL_TEXTURE_2D);
    } else {
        XTransform transform{{{XDoubleToFixed(im->width/r.w),0,0},
            {0,XDoubleToFixed(im->height/r.h),0},{0,0,XDoubleToFixed(1)}}};
        XRenderSetPictureTransform(dpy,im->picture,&transform);
        if (shape) {
            std::vector<XRectangle> rects;
            for (auto s:*shape) rects.push_back({static_cast<short>(r.x+s.x*r.w/im->width),
                static_cast<short>(r.y+s.y*r.h/im->height),
                static_cast<unsigned short>(std::ceil(s.width*r.w/im->width)),
                static_cast<unsigned short>(std::ceil(s.height*r.h/im->height))});
            XRenderSetPictureClipRectangles(dpy,target,0,0,rects.data(),rects.size());
        }
        Picture mask=None;
        if (opacity<.9999) { XRenderColor c{0,0,0,static_cast<unsigned short>(65535*opacity)};mask=XRenderCreateSolidFill(dpy,&c); }
        XRenderComposite(dpy,PictOpOver,im->picture,mask,target,0,0,0,0,
            std::lround(r.x),std::lround(r.y),std::ceil(r.w),std::ceil(r.h));
        if (mask) XRenderFreePicture(dpy,mask);
        if (shape) { XRenderPictureAttributes pa{};pa.clip_mask=None;XRenderChangePicture(dpy,target,CPClipMask,&pa); }
    }
}

void Renderer::drawTransformed(const ImagePtr &im,const Quad &q,double opacity,
                               const std::vector<XRectangle> *shape) {
    if (!im || opacity<=0) return;
    Point a{q[1].x-q[0].x,q[1].y-q[0].y},b{q[3].x-q[0].x,q[3].y-q[0].y};
    double det=a.x*b.y-a.y*b.x;
    // A flip is momentarily edge-on. Avoid a singular inverse and fixed-point
    // overflow in XRender; drawing nothing is the correct edge-on projection.
    if (std::abs(det)<1. || std::abs(det)/std::max(1.,std::hypot(a.x,a.y))<.5) return;
    opacity=std::clamp(opacity,0.,1.);
    auto point=[&](double u,double v) { return Point{q[0].x+u*a.x+v*b.x,q[0].y+u*a.y+v*b.y}; };
    if (gl) {
        glEnable(GL_TEXTURE_2D);glBindTexture(GL_TEXTURE_2D,im->texture);
        glColor4d(opacity,opacity,opacity,opacity);
        auto vertex=[&](double u,double v) {
            auto p=point(u,v);glTexCoord2d(u,im->inverted?v:1-v);glVertex2d(p.x,p.y);
        };
        auto quad=[&](double u,double v,double right,double bottom) {
            glBegin(GL_QUADS);vertex(u,v);vertex(right,v);vertex(right,bottom);vertex(u,bottom);glEnd();
        };
        // Transform each Shape rectangle with its texture coordinates. Scissor
        // rectangles would clip against the unrotated shape instead.
        if (shape) for(auto s:*shape)
            quad(double(s.x)/im->width,double(s.y)/im->height,
                 double(s.x+s.width)/im->width,double(s.y+s.height)/im->height);
        else quad(0,0,1,1);
        glDisable(GL_TEXTURE_2D);
    } else {
        double left=q[0].x,top=q[0].y,right=left,bottom=top;
        for(auto p:q) { left=std::min(left,p.x);top=std::min(top,p.y);right=std::max(right,p.x);bottom=std::max(bottom,p.y); }
        int x=std::clamp(int(std::floor(left)),0,width),y=std::clamp(int(std::floor(top)),0,height);
        int w=std::clamp(int(std::ceil(right)),0,width)-x,h=std::clamp(int(std::ceil(bottom)),0,height)-y;
        if(w<=0 || h<=0) return;
        // XRender maps destination pixels back into the source. The source
        // origin of Composite is (0,0), corresponding to this clipped bbox.
        double xx=im->width*b.y/det,xy=-im->width*b.x/det;
        double yx=-im->height*a.y/det,yy=im->height*a.x/det;
        XTransform inverse{{{XDoubleToFixed(xx),XDoubleToFixed(xy),XDoubleToFixed(xx*(x-q[0].x)+xy*(y-q[0].y))},
                            {XDoubleToFixed(yx),XDoubleToFixed(yy),XDoubleToFixed(yx*(x-q[0].x)+yy*(y-q[0].y))},
                            {0,0,XDoubleToFixed(1)}}};
        XRenderSetPictureTransform(dpy,im->picture,&inverse);
        if(shape) {
            Region region=XCreateRegion();
            for(auto s:*shape) {
                double u=double(s.x)/im->width,v=double(s.y)/im->height;
                double u1=double(s.x+s.width)/im->width,v1=double(s.y+s.height)/im->height;
                Point corners[]={point(u,v),point(u1,v),point(u1,v1),point(u,v1)};
                XPoint points[4];
                for(int i=0;i<4;i++) points[i]={static_cast<short>(std::clamp(std::lround(corners[i].x),-32000L,32000L)),
                                               static_cast<short>(std::clamp(std::lround(corners[i].y),-32000L,32000L))};
                Region part=XPolygonRegion(points,4,WindingRule);
                XUnionRegion(region,part,region);XDestroyRegion(part);
            }
            XRenderSetPictureClipRegion(dpy,target,region);XDestroyRegion(region);
        }
        Picture mask=None;
        if(opacity<.9999) { XRenderColor c{0,0,0,static_cast<unsigned short>(65535*opacity)};mask=XRenderCreateSolidFill(dpy,&c); }
        XRenderComposite(dpy,PictOpOver,im->picture,mask,target,0,0,0,0,x,y,w,h);
        if(mask) XRenderFreePicture(dpy,mask);
        if(shape) { XRenderPictureAttributes pa{};pa.clip_mask=None;XRenderChangePicture(dpy,target,CPClipMask,&pa); }
    }
}

void Renderer::shadowAt(Rect r,double opacity,int radius,int offset) {
    if (!shadow || opacity<=0 || radius<=0) return;
    // Keep the shadow outside the frame; ARGB clients must reveal the actual
    // scene underneath, rather than a dark rectangle painted under the client.
    int left=std::clamp(int(std::floor(r.x)),0,width);
    int right=std::clamp(int(std::ceil(r.x+r.w)),0,width);
    int top=std::clamp(int(std::floor(r.y)),0,height);
    int bottom=std::clamp(int(std::ceil(r.y+r.h)),0,height);
    XRectangle clips[]={{0,0,static_cast<unsigned short>(width),static_cast<unsigned short>(top)},
        {0,static_cast<short>(bottom),static_cast<unsigned short>(width),static_cast<unsigned short>(height-bottom)},
        {0,static_cast<short>(top),static_cast<unsigned short>(left),static_cast<unsigned short>(bottom-top)},
        {static_cast<short>(right),static_cast<short>(top),static_cast<unsigned short>(width-right),static_cast<unsigned short>(bottom-top)}};
    if(gl) glEnable(GL_SCISSOR_TEST);
    else XRenderSetPictureClipRectangles(dpy,target,0,0,clips,4);
    // Nine patches retain a fixed blur radius while the middle stretches.
    double edge=std::min<double>(2*radius,std::min(r.w,r.h)/2+radius);
    double xs[]={r.x-radius+offset,r.x-radius+offset+edge,r.x+r.w+radius+offset-edge,r.x+r.w+radius+offset};
    double ys[]={r.y-radius+offset,r.y-radius+offset+edge,r.y+r.h+radius+offset-edge,r.y+r.h+radius+offset};
    double uv[]={0,32./65,33./65,1};
    for(int j=0;j<3;j++) for(int i=0;i<3;i++) {
        double w=xs[i+1]-xs[i],h=ys[j+1]-ys[j];
        if (w<=0 || h<=0) continue;
        if (gl) {
            glEnable(GL_TEXTURE_2D);glBindTexture(GL_TEXTURE_2D,shadow->texture);
            glColor4d(opacity,opacity,opacity,opacity);
            double t0=shadow->inverted?uv[j]:1-uv[j], t1=shadow->inverted?uv[j+1]:1-uv[j+1];
            for(auto clip:clips) {
                if(!clip.width || !clip.height) continue;
                glScissor(clip.x,height-clip.y-clip.height,clip.width,clip.height);
                glBegin(GL_QUADS);
                glTexCoord2d(uv[i],t0);glVertex2d(xs[i],ys[j]);
                glTexCoord2d(uv[i+1],t0);glVertex2d(xs[i+1],ys[j]);
                glTexCoord2d(uv[i+1],t1);glVertex2d(xs[i+1],ys[j+1]);
                glTexCoord2d(uv[i],t1);glVertex2d(xs[i],ys[j+1]);glEnd();
            }
            glDisable(GL_TEXTURE_2D);
        } else {
            XTransform t{{{XDoubleToFixed((uv[i+1]-uv[i])*65/w),0,XDoubleToFixed(uv[i]*65)},
                {0,XDoubleToFixed((uv[j+1]-uv[j])*65/h),XDoubleToFixed(uv[j]*65)}, {0,0,XDoubleToFixed(1)}}};
            XRenderSetPictureTransform(dpy,shadow->picture,&t);
            XRenderColor c{0,0,0,static_cast<unsigned short>(65535*opacity)};
            Picture mask=XRenderCreateSolidFill(dpy,&c);
            XRenderComposite(dpy,PictOpOver,shadow->picture,mask,target,0,0,0,0,
                std::lround(xs[i]),std::lround(ys[j]),std::ceil(w),std::ceil(h));
            XRenderFreePicture(dpy,mask);
        }
    }
    if(gl) glDisable(GL_SCISSOR_TEST);
    else { XRenderPictureAttributes pa{};pa.clip_mask=None;XRenderChangePicture(dpy,target,CPClipMask,&pa); }
}

void Renderer::line(const Quad &q,double opacity) {
    if (gl) {
        glDisable(GL_TEXTURE_2D);glColor4d(.8*opacity,.9*opacity,opacity,opacity);
        glBegin(GL_LINE_LOOP);for(auto p:q) glVertex2d(p.x,p.y);glEnd();
    } else {
        XGCValues v{};v.foreground=0xb8d8f0;
        auto gc=XCreateGC(dpy,buffer,GCForeground,&v);
        XPoint points[5];
        for(int i=0;i<5;i++) points[i]={static_cast<short>(std::clamp(q[i%4].x,-32000.,32000.)),
            static_cast<short>(std::clamp(q[i%4].y,-32000.,32000.))};
        XDrawLines(dpy,buffer,gc,points,5,CoordModeOrigin);XFreeGC(dpy,gc);
    }
}

void Renderer::present() {
    if (gl) glXSwapBuffers(dpy,glwindow);
    else XRenderComposite(dpy,PictOpSrc,target,None,destination,0,0,0,0,0,0,width,height);
    XFlush(dpy);
}
