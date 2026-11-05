/* SPDX-License-Identifier: GPL-3.0-only */
#include <X11/Xlib.h>
#include <X11/Xatom.h>
#include <X11/Xutil.h>
#include <X11/extensions/Xrender.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static Display *d;
static Window root;
static Atom atom(const char *name) { return XInternAtom(d,name,False); }
static void fail(const char *message) { fprintf(stderr,"FAIL: %s\n",message);exit(1); }
static unsigned long value(Window window,const char *name,int index,Atom type) {
    Atom actual;int format;unsigned long n,left,result=0;unsigned char *data=NULL;
    if(XGetWindowProperty(d,window,atom(name),0,32,False,type,&actual,&format,&n,&left,&data)==Success
        && actual==type && format==32 && n>(unsigned)index) result=((unsigned long*)data)[index];
    if(data) XFree(data);
    return result;
}
static void send(Window w,const char *name,long a,long b,long c) {
    XEvent event={0};event.xclient.type=ClientMessage;event.xclient.window=w;
    event.xclient.message_type=atom(name);event.xclient.format=32;
    event.xclient.data.l[0]=a;event.xclient.data.l[1]=b;event.xclient.data.l[2]=c;
    XSendEvent(d,root,False,SubstructureRedirectMask|SubstructureNotifyMask,&event);XFlush(d);
}
static Window frame(Window w) {
    Window r,p,*children=NULL;unsigned n;
    for(int i=0;i<100;i++) {
        if(XQueryTree(d,w,&r,&p,&children,&n)) {
            if(children) XFree(children);
            if(p!=root) {
                Window rr,pp,*cc=NULL;unsigned nn;
                while(XQueryTree(d,p,&rr,&pp,&cc,&nn)) { if(cc) XFree(cc);if(pp==root) return p;p=pp; }
            }
        }
        usleep(20000);
    }
    fail("client was not managed");return 0;
}

static unsigned long pixel(int x,int y) {
    XImage *im=XGetImage(d,root,x,y,1,1,AllPlanes,ZPixmap);
    if(!im) fail("capture failed");
    unsigned long c=XGetPixel(im,0,0)&0xffffff; XDestroyImage(im);return c;
}
static void configure(const char *mode) {
    char resources[2048];
    snprintf(resources,sizeof(resources),
        "Emwm*titleGradient: %s\n"
        "Emwm*titleGradientActiveStart: #ff0000\n"
        "Emwm*titleGradientActiveEnd: #0000ff\n"
        "Emwm*titleGradientActiveText: #ffffff\n"
        "Emwm*titleGradientInactiveStart: #00ff00\n"
        "Emwm*titleGradientInactiveEnd: #ffff00\n"
        "Emwm*titleGradientInactiveText: #000000\n",mode);
    XChangeProperty(d,root,XA_RESOURCE_MANAGER,XA_STRING,8,PropModeReplace,
                    (unsigned char*)resources,strlen(resources));
    XSync(d,False);usleep(200000);
}
static void check(Window w,Window f,const char *mode,int active) {
    XWindowAttributes a; XGetWindowAttributes(d,f,&a);
    int left=value(w,"_NET_FRAME_EXTENTS",0,XA_CARDINAL);
    int top=value(w,"_NET_FRAME_EXTENTS",2,XA_CARDINAL);
    int width=a.width-2*left,height=top-left;
    if(height<8) fail("title too small");
    for(int i=0;i<2;i++) {
        int x=left+width*(i?0.65:0.25),y=left+height*(i?0.65:0.25);
        double dx=(x-left+.5)/(width-1),dy=(y-left+.5)/(height-1);
        double t=!strcmp(mode,"horizontal")?dx:!strcmp(mode,"vertical")?dy:(dx+dy)/2;
        unsigned long c=pixel(a.x+x,a.y+y);
        int expected[3]={active?255*(1-t):255*t,active?0:255,active?255*t:0};
        int actual[3]={(c>>16)&255,(c>>8)&255,c&255};
        for(int k=0;k<3;k++) if(abs(expected[k]-actual[k])>5) {
            fprintf(stderr,"%s active=%d geometry=%dx%d title=%d,%d sample=%d,%d color=%06lx expected=%d,%d,%d\n",
                    mode,active,a.width,a.height,left,top,x,y,c,expected[0],expected[1],expected[2]);
            fail("gradient pixel mismatch");
        }
    }
}
static void check_text(Window w,Window f,int active) {
    XWindowAttributes a;XGetWindowAttributes(d,f,&a);
    int left=value(w,"_NET_FRAME_EXTENTS",0,XA_CARDINAL);
    int top=value(w,"_NET_FRAME_EXTENTS",2,XA_CARDINAL);
    /* The central title area excludes the bevels and all button glyphs. */
    XImage *im=XGetImage(d,root,a.x+a.width/3,a.y+left+3,
                        a.width/3,top-left-6,AllPlanes,ZPixmap);
    if(!im) fail("text capture failed");
    int found=0;
    for(int y=0;y<im->height;y++) for(int x=0;x<im->width;x++) {
        unsigned long c=XGetPixel(im,x,y)&0xffffff;
        if(c==(active?0xffffff:0x000000)) found++;
    }
    XDestroyImage(im);
    if(found<10) fail("configured title text color missing");
}
int main(int argc,char **argv) {
    d=XOpenDisplay(NULL);if(!d) fail("no display");root=DefaultRootWindow(d);
    Window w;
    if(argc>1 && !strcmp(argv[1],"argb")) {
        XVisualInfo query={.screen=DefaultScreen(d),.depth=32,.class=TrueColor};int n;
        XVisualInfo *infos=XGetVisualInfo(d,VisualScreenMask|VisualDepthMask|VisualClassMask,&query,&n),*chosen=NULL;
        for(int i=0;i<n;i++){XRenderPictFormat *f=XRenderFindVisualFormat(d,infos[i].visual);if(f&&f->direct.alphaMask){chosen=&infos[i];break;}}
        if(!chosen) fail("no ARGB visual");
        XSetWindowAttributes attrs={.background_pixel=0x80008000,.border_pixel=0};
        attrs.colormap=XCreateColormap(d,root,chosen->visual,AllocNone);
        w=XCreateWindow(d,root,100,150,500,180,0,32,InputOutput,chosen->visual,CWBackPixel|CWBorderPixel|CWColormap,&attrs);
        XFree(infos);
    } else w=XCreateSimpleWindow(d,root,100,150,500,180,0,0,0x008000);
    XStoreName(d,w," ");XMapWindow(d,w);XFlush(d);Window f=frame(w);usleep(250000);
    Window other=XCreateSimpleWindow(d,root,900,550,70,80,0,0,0);
    XMapWindow(d,other);XFlush(d);frame(other);
    send(other,"_NET_ACTIVE_WINDOW",1,CurrentTime,0);usleep(200000);
    XWindowAttributes initial;XGetWindowAttributes(d,f,&initial);
    int initialLeft=value(w,"_NET_FRAME_EXTENTS",0,XA_CARDINAL),initialTop=value(w,"_NET_FRAME_EXTENTS",2,XA_CARDINAL);
    unsigned long baseline[2]={pixel(initial.x+150,initial.y+(initialLeft+initialTop)/2),pixel(initial.x+300,initial.y+(initialLeft+initialTop)/2)};
    for(int k=0;k<3;k++) {
        const char *mode=k==0?"horizontal":k==1?"vertical":"diagonal";
        configure(mode);
        send(w,"_NET_ACTIVE_WINDOW",1,CurrentTime,0);usleep(200000);
        check(w,f,mode,1);
        XStoreName(d,w,"short");XSync(d,False);usleep(100000);check(w,f,mode,1);check_text(w,f,1);
        send(other,"_NET_ACTIVE_WINDOW",1,CurrentTime,0);usleep(200000);check_text(w,f,0);
        send(w,"_NET_ACTIVE_WINDOW",1,CurrentTime,0);usleep(200000);
        XStoreName(d,w," ");
        XResizeWindow(d,w,650,200);XSync(d,False);usleep(200000);check(w,f,mode,1);
        send(other,"_NET_ACTIVE_WINDOW",1,CurrentTime,0);usleep(200000);check(w,f,mode,0);
    }
    configure("none");
    XWindowAttributes a;XGetWindowAttributes(d,f,&a);
    int left=value(w,"_NET_FRAME_EXTENTS",0,XA_CARDINAL),top=value(w,"_NET_FRAME_EXTENTS",2,XA_CARDINAL);
    if(pixel(a.x+150,a.y+(left+top)/2)!=baseline[0] || pixel(a.x+300,a.y+(left+top)/2)!=baseline[1]) {
        fprintf(stderr,"disable pixels=%06lx,%06lx baseline=%06lx,%06lx\n",pixel(a.x+150,a.y+(left+top)/2),pixel(a.x+300,a.y+(left+top)/2),baseline[0],baseline[1]);
        fail("disable did not restore original background");
    }
    configure("invalid");
    if(pixel(a.x+150,a.y+(left+top)/2)!=baseline[0] || pixel(a.x+300,a.y+(left+top)/2)!=baseline[1]) fail("invalid mode did not fall back");
    XDestroyWindow(d,w);XDestroyWindow(d,other);XCloseDisplay(d);
    puts("PASS: gradient pixels, text colors, focus, title update, resize, live disable and invalid fallback");
}
