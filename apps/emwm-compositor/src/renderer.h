// SPDX-License-Identifier: GPL-3.0-only
#pragma once
#include "effects.h"
#include <X11/Xlib.h>
#include <X11/extensions/Xrender.h>
#include <GL/glx.h>
#include <GL/glxext.h>
#include <memory>
#include <string>
#include <vector>

struct Image {
    Display *dpy=nullptr;
    Pixmap pixmap=0;
    Picture picture=0;
    GLXPixmap glpixmap=0;
    GLuint texture=0;
    PFNGLXRELEASETEXIMAGEEXTPROC release=nullptr;
    bool bound=false, inverted=true;
    int width=0, height=0;
    ~Image();
};
using ImagePtr = std::shared_ptr<Image>;

class Renderer {
    Display *dpy;
    int screen;
    Window overlay, output=0;
    Colormap colormap=0;
    int width=0, height=0;
    bool gl=false;
    GLXContext context=nullptr;
    GLXWindow glwindow=0;
    PFNGLXBINDTEXIMAGEEXTPROC bind=nullptr;
    PFNGLXRELEASETEXIMAGEEXTPROC release=nullptr;
    Pixmap buffer=0;
    Picture target=0, destination=0;
    ImagePtr shadow;
    bool initGL(bool vsync);
    GLXFBConfig pixmapConfig(Visual *visual, int depth, bool &inverted);
public:
    Renderer(Display *d, int s, Window o): dpy(d),screen(s),overlay(o) {}
    ~Renderer();
    bool init(const std::string &backend, bool vsync);
    bool accelerated() const { return gl; }
    Window outputWindow() const { return output; }
    void resize(int w,int h);
    ImagePtr image(Pixmap pixmap, Visual *visual, int depth, int w, int h);
    ImagePtr capture(Window window, const XWindowAttributes &attrs);
    void refresh(const ImagePtr &image);
    void begin();
    void draw(const ImagePtr &image, Rect dest, double opacity=1,
              const std::vector<XRectangle> *shape=nullptr);
    void shadowAt(Rect r, double opacity, int radius, int offset);
    void line(const Quad &q, double opacity);
    void present();
};
