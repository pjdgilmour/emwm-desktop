/* SPDX-License-Identifier: GPL-3.0-only */
#include <X11/Xlib.h>
#include <X11/Xatom.h>
#include <X11/Xutil.h>
#include <X11/extensions/shape.h>
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
static void animation(Window owner,int kind,Window f) {
    for(int i=0;i<100;i++) {
        if(value(owner,"_EMWM_LAST_ANIMATION",0,XA_CARDINAL)==(unsigned)kind &&
           value(owner,"_EMWM_LAST_ANIMATION",1,XA_CARDINAL)==f) return;
        usleep(20000);
    }
    fail("missing compositor state-transition event");
}
static int mapped(Window w) { XWindowAttributes a;return XGetWindowAttributes(d,w,&a)&&a.map_state==IsViewable; }
static unsigned long pixel(int x,int y) {
    XImage *im=XGetImage(d,root,x,y,1,1,AllPlanes,ZPixmap);
    if(!im) fail("cannot capture composed pixel");
    unsigned long color=XGetPixel(im,0,0)&0xffffff;XDestroyImage(im);return color;
}
static int green_pixels(void) {
    XImage *im=XGetImage(d,root,0,0,1024,768,AllPlanes,ZPixmap);
    if(!im) fail("cannot inspect animation");
    int count=0;
    for(int y=0;y<768;y+=5) for(int x=0;x<1024;x+=5) {
        unsigned long c=XGetPixel(im,x,y);int r=(c>>16)&255,g=(c>>8)&255,b=c&255;
        if(g>r+60 && g>b+60) ++count;
    }
    XDestroyImage(im);return count;
}
int main(int argc,char **argv) {
    d=XOpenDisplay(NULL);if(!d) fail("no display");root=DefaultRootWindow(d);
    if(argc==3 && !strcmp(argv[1],"--workspace")) {
        send(root,"_NET_CURRENT_DESKTOP",atoi(argv[2]),CurrentTime,0);XSync(d,False);XCloseDisplay(d);return 0;
    }
    if(argc==4 && !strcmp(argv[1],"--pixel")) {
        printf("%06lx\n",pixel(atoi(argv[2]),atoi(argv[3])));XCloseDisplay(d);return 0;
    }
    char selection[64];snprintf(selection,sizeof selection,"_NET_WM_CM_S%d",DefaultScreen(d));
    Window owner=XGetSelectionOwner(d,atom(selection));if(!owner) fail("no compositor");
    Window desktop=XCreateSimpleWindow(d,root,0,0,1024,768,0,0,0x6688aa);
    Atom type=atom("_NET_WM_WINDOW_TYPE_DESKTOP");
    XChangeProperty(d,desktop,atom("_NET_WM_WINDOW_TYPE"),XA_ATOM,32,PropModeReplace,(unsigned char*)&type,1);
    XStoreName(d,desktop,"Synthetic desktop surface");XMapWindow(d,desktop);XFlush(d);
    Window df=frame(desktop);usleep(200000);
    XWindowAttributes a;XGetWindowAttributes(d,df,&a);
    if(a.x!=0 || a.y!=0 || a.width!=1024 || a.height!=768) {
        fprintf(stderr,"desktop frame: %d,%d %dx%d border=%d\n",a.x,a.y,a.width,a.height,a.border_width);
        fail("desktop has decorations or wrong geometry");
    }
    Window w=XCreateSimpleWindow(d,root,200,180,320,240,0,0,0xff0000);
    XStoreName(d,w,"Native compositor integration");
    XSizeHints hints={0};hints.flags=USPosition;hints.x=200;hints.y=180;XSetWMNormalHints(d,w,&hints);
    XMapWindow(d,w);XFlush(d);Window f=frame(w);usleep(400000);
    int x,y;Window child;XTranslateCoordinates(d,w,root,0,0,&x,&y,&child);
    if(pixel(x+100,y+100)!=0xff0000) fail("window not rendered above desktop");
    XSetWindowBackground(d,w,0x00ff00);XClearWindow(d,w);XFlush(d);usleep(150000);
    if(pixel(x+100,y+100)!=0x00ff00) fail("damage did not refresh content");
    XSetWindowAttributes oa={0};oa.override_redirect=True;oa.background_pixel=0xff0000;
    Window popup=XCreateWindow(d,root,700,100,100,100,0,CopyFromParent,InputOutput,CopyFromParent,
                              CWOverrideRedirect|CWBackPixel,&oa);
    XRectangle shaped={0,0,45,100};
    XShapeCombineRectangles(d,popup,ShapeBounding,0,0,&shaped,1,ShapeSet,Unsorted);
    XMapWindow(d,popup);XFlush(d);usleep(120000);
    if(pixel(710,120)!=0xff0000 || pixel(780,120)!=0x6688aa) fail("Shape clipping incorrect");
    XDestroyWindow(d,popup);XFlush(d);usleep(50000);
    XGetWindowAttributes(d,f,&a);
    unsigned long outside=pixel(a.x+a.width+5,a.y+60);
    if(outside==0x6688aa || outside==0) fail("soft shadow was not composited");
    // A desktop raising itself must stay below the ordinary client.
    XRaiseWindow(d,desktop);XFlush(d);usleep(100000);
    if(pixel(x+100,y+100)!=0x00ff00) fail("desktop covered an application");
    XIconifyWindow(d,w,DefaultScreen(d));XFlush(d);animation(owner,1,f);usleep(60000);
    if(green_pixels()<30) fail("minimize animation lost the window contents");
    usleep(300000);
    if(mapped(w)) fail("minimize left client mapped");
    XMapWindow(d,w);XFlush(d);animation(owner,2,f);usleep(350000);
    if(!mapped(w)) fail("restore left client hidden");
    send(w,"_NET_WM_STATE",1,atom("_NET_WM_STATE_MAXIMIZED_VERT"),atom("_NET_WM_STATE_MAXIMIZED_HORZ"));
    animation(owner,3,f);usleep(350000);XGetWindowAttributes(d,f,&a);
    if(a.width<900 || a.height<700) fail("maximize geometry incorrect");
    send(w,"_NET_WM_STATE",0,atom("_NET_WM_STATE_MAXIMIZED_VERT"),atom("_NET_WM_STATE_MAXIMIZED_HORZ"));
    animation(owner,4,f);usleep(350000);
    XGetWindowAttributes(d,f,&a);if(a.width>500) fail("unmaximize geometry incorrect");
    send(root,"_NET_CURRENT_DESKTOP",1,CurrentTime,0);usleep(200000);
    if(!mapped(desktop) || mapped(w)) fail("desktop/workspace presence incorrect");
    send(root,"_NET_CURRENT_DESKTOP",0,CurrentTime,0);usleep(200000);
    if(!mapped(w)) fail("workspace restore failed");
    for(int i=0;i<20;i++) { XResizeWindow(d,w,200+i*10,160+i*5);XFlush(d);usleep(15000); }
    XDestroyWindow(d,w);XDestroyWindow(d,desktop);XSync(d,False);XCloseDisplay(d);
    puts("PASS: pixels, damage, shadows, desktop stacking, minimize/restore/maximize and workspaces");
    return 0;
}
