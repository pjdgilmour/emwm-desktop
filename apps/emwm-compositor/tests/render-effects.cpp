// SPDX-License-Identifier: GPL-3.0-only
#include "renderer.h"
#include <X11/Xutil.h>
#include <cstdlib>
#include <iostream>
#include <string>
static int errors=0;
static int error(Display *d,XErrorEvent *e) {
    char message[160];XGetErrorText(d,e->error_code,message,sizeof message);
    std::cerr<<message<<'\n';++errors;return 0;
}
static void check(bool ok,const char *message) { if(!ok) { std::cerr<<"FAIL: "<<message<<'\n';std::exit(1); } }
int main(int argc,char **argv) {
    check(argc==2,"specify glx or xrender");
    std::string backend=argv[1];auto d=XOpenDisplay(nullptr);check(d,"X display");XSetErrorHandler(error);
    int screen=DefaultScreen(d);
    Window root=RootWindow(d,screen),window=XCreateSimpleWindow(d,root,0,0,1024,768,0,0,0);
    XMapWindow(d,window);XSync(d,False);
    {
        Renderer renderer(d,screen,window);check(renderer.init(backend,false),"renderer init");
        check(renderer.accelerated()==(backend=="glx"),"requested backend really active");
        // Let GLX realize the resized drawable before sampling rendered frames.
        renderer.begin();renderer.present();XSync(d,False);
        Pixmap pix=XCreatePixmap(d,window,240,160,DefaultDepth(d,screen));
        auto gc=XCreateGC(d,pix,0,nullptr);
        unsigned long colors[]={0xff0000,0x00ff00,0x0000ff,0xffff00};
        for(int i=0;i<4;i++) { XSetForeground(d,gc,colors[i]);XFillRectangle(d,pix,gc,(i%2)*120,(i/2)*80,120,80); }
        XFreeGC(d,gc);
        auto im=renderer.image(pix,DefaultVisual(d,screen),DefaultDepth(d,screen),240,160);
        auto pixel=[&](int x,int y) {
            auto shot=XGetImage(d,root,x,y,1,1,AllPlanes,ZPixmap);check(shot,"read composed pixels");
            auto value=XGetPixel(shot,0,0)&0xffffff;XDestroyImage(shot);return value;
        };
        auto draw=[&](Effect effect,double time,const std::vector<XRectangle> *shape=nullptr) {
            renderer.begin();renderer.drawTransformed(im,outline({200,180,240,160},effect,time),1,shape);
            renderer.present();if(renderer.accelerated()) { glFinish();glXWaitGL(); }XSync(d,False);
        };
        draw(Effect::Zoom,.25);
        auto background=pixel(100,100);
        check(pixel(260,220)==0xff0000 && pixel(380,220)==0x00ff00,"zoom top quadrants");
        check(pixel(260,300)==0x0000ff && pixel(380,300)==0xffff00,"zoom bottom quadrants");
        draw(Effect::Twist,.25);
        check(pixel(360,200)==0xff0000 && pixel(280,200)==0x0000ff,"twist rotates window pixels 90 degrees");
        check(pixel(360,320)==0x00ff00 && pixel(280,320)==0xffff00,"twist rotates bottom quadrants");
        check(pixel(210,190)==background,"twist does not leave an upright zoom image");
        draw(Effect::Flip,.5);
        check(pixel(260,220)==0x0000ff && pixel(380,220)==0xffff00,"flip inverts top and bottom pixels");
        check(pixel(260,300)==0xff0000 && pixel(380,300)==0x00ff00,"flip retains texture when inverted");
        draw(Effect::Flip,.25);
        check(pixel(320,260)==background,"edge-on flip vanishes without singular matrix artifacts");
        draw(Effect::Flip,.75);
        check(pixel(320,260)==background,"second edge-on flip is safe");
        std::vector<XRectangle> shape{{0,0,120,160}};
        draw(Effect::Twist,.25,&shape);
        check(pixel(360,200)==0xff0000 && pixel(280,200)==0x0000ff,"rotated Shape visible half");
        check(pixel(360,320)==background && pixel(280,320)==background,"rotated Shape clips missing half");
        for(auto effect:{Effect::Twist,Effect::Flip}) for(double t:{0.,1.}) {
            draw(effect,t);check(pixel(260,220)==0xff0000 && pixel(380,300)==0xffff00,"transition endpoints restore orientation");
        }
        check(errors==0,"no X11 rendering errors");
    }
    XDestroyWindow(d,window);XSync(d,False);XCloseDisplay(d);
    check(errors==0,"clean renderer shutdown");
    std::cout<<"PASS: "<<backend<<" distinct zoom/twist/flip pixels, Shape, edge-on frames and endpoints\n";
}
