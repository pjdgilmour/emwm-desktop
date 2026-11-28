// SPDX-License-Identifier: GPL-3.0-only
// Exercise the real keyboard, wheel and composited framebuffer on a private X server.
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/Xatom.h>
#include <X11/keysym.h>
#include <X11/extensions/XTest.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static Display *display;
static Window root;

static unsigned long window_property(Window window, const char *name) {
    Atom type;
    int format;
    unsigned long n, left, result = ~0UL;
    unsigned char *data = NULL;
    XGetWindowProperty(display, window, XInternAtom(display, name, False), 0, 2, False,
                       AnyPropertyType, &type, &format, &n, &left, &data);
    if (format == 32 && n) result = *(unsigned long *)data;
    if (data) XFree(data);
    return result;
}

static unsigned long property(const char *name) { return window_property(root, name); }

static unsigned long pixel(int x, int y) {
    XImage *image = XGetImage(display, root, x, y, 1, 1, AllPlanes, ZPixmap);
    unsigned long value = XGetPixel(image, 0, 0) & 0xffffff;
    XDestroyImage(image);
    return value;
}

#define WAIT(condition) do { \
    int i = 0; \
    while (!(condition)) { \
        if (++i == 120) { fprintf(stderr, "Failed at line %d: %s\n", __LINE__, #condition); exit(1); } \
        usleep(25000); \
    } \
} while (0)

static void key(KeySym sym, int down) {
    XTestFakeKeyEvent(display, XKeysymToKeycode(display, sym), down, 0);
    XSync(display, False);
}
static void tap(KeySym sym) { key(sym, True); key(sym, False); }
static void button(int number) {
    XTestFakeButtonEvent(display, number, True, 0);
    XTestFakeButtonEvent(display, number, False, 0);
    XSync(display, False);
}
static Window application(const char *title, unsigned long color) {
    Window w = XCreateSimpleWindow(display, root, 40, 50, 260, 160, 0, 0, color);
    XStoreName(display, w, title);
    XMapWindow(display, w);
    XSync(display, False);
    WAIT(pixel(120, 130) == color);
    return w;
}

int main(int argc, char **argv) {
    display = XOpenDisplay(NULL);
    if (!display) return 1;
    root = DefaultRootWindow(display);
    const unsigned long overview = 0x161c23, red = 0xcc3311, blue = 0x2266cc;
    WAIT(property("_NET_NUMBER_OF_DESKTOPS") == 4);
    Window first = application("Primeira janela", red);
    XTestFakeMotionEvent(display, -1, 120, 130, 0);
    key(XK_Super_L, True);
    button(5);
    WAIT(property("_NET_CURRENT_DESKTOP") == 1);
    key(XK_Super_L, False);
    if (argc == 2 && strcmp(argv[1], "scroll-only") == 0) {
        key(XK_Super_R, True);
        button(4);
        button(4);
        key(XK_Super_R, False);
        WAIT(property("_NET_CURRENT_DESKTOP") == 3);
        XCloseDisplay(display);
        puts("PASS: direct workspace scrolling without a compositor");
        return 0;
    }
    Window second = application("Segunda janela", blue);
    tap(XK_Super_L);
    WAIT(pixel(1, 1) == overview);
    // Both an inactive workspace and the current one have distinct thumbnails.
    WAIT(pixel(100, 72) == red && pixel(612, 72) == blue);
    key(XK_Super_R, True);
    button(4);
    WAIT(property("_NET_CURRENT_DESKTOP") == 0);
    key(XK_Super_R, False);
    WAIT(pixel(1, 1) == overview);
    tap(XK_Escape);
    WAIT(pixel(120, 130) == red);
    key(XK_Super_L, True);
    tap(XK_Tab);
    key(XK_Super_L, False);
    WAIT(pixel(1, 1) == overview);
    // Both windows are exposed, without overlap, regardless of stacking order.
    unsigned long left, right;
    WAIT(((left=pixel(256,350))==red || left==blue) &&
         ((right=pixel(768,350))==red || right==blue) && left != right);
    XTestFakeMotionEvent(display, -1, left==blue ? 256 : 768, 350, 0);
    button(1);
    WAIT(property("_NET_CURRENT_DESKTOP") == 1);
    WAIT(property("_NET_ACTIVE_WINDOW") == second);
    WAIT(pixel(120, 130) == blue);
    XIconifyWindow(display, second, DefaultScreen(display));
    XFlush(display);
    WAIT(window_property(second, "WM_STATE") == IconicState);
    key(XK_Super_L, True);
    tap(XK_Tab);
    key(XK_Super_L, False);
    WAIT(pixel(1, 1) == overview);
    WAIT(((left=pixel(256,350))==red || left==blue) &&
         ((right=pixel(768,350))==red || right==blue) && left != right);
    XTestFakeMotionEvent(display, -1, left==blue ? 256 : 768, 350, 0);
    button(1);
    WAIT(window_property(second, "WM_STATE") == NormalState);
    WAIT(property("_NET_ACTIVE_WINDOW") == second);
    WAIT(pixel(120, 130) == blue);

    // Navigate and confirm immediately after the overview appears.
    tap(XK_Super_R);
    WAIT(pixel(1, 1) == overview);
    tap(XK_Right);
    tap(XK_Return);
    WAIT(property("_NET_CURRENT_DESKTOP") == 2);
    tap(XK_Super_R);
    WAIT(pixel(1, 1) == overview);
    tap(XK_Super_R);
    WAIT(pixel(1, 1) != overview);
    key(XK_Super_R, True);
    button(4);
    key(XK_Super_R, False);
    WAIT(property("_NET_CURRENT_DESKTOP") == 1);
    button(5);
    XSync(display, False);
    usleep(80000);
    if (property("_NET_CURRENT_DESKTOP") != 1) return 1;

    // Other modal clients (including the session locker) can take input.
    tap(XK_Super_L);
    WAIT(pixel(1, 1) == overview);
    if (XGrabKeyboard(display, second, False, GrabModeAsync, GrabModeAsync, CurrentTime) != GrabSuccess) return 1;
    WAIT(pixel(1, 1) != overview);
    tap(XK_Super_L);
    usleep(80000);
    if (pixel(1, 1) == overview) return 1;
    XUngrabKeyboard(display, CurrentTime);
    // A Super chord must not be mistaken for a lone tap.
    key(XK_Super_L, True);
    tap(XK_Left);
    key(XK_Super_L, False);
    usleep(350000);
    if (pixel(1, 1) == overview) return 1;
    XDestroyWindow(display, first);
    XDestroyWindow(display, second);
    XCloseDisplay(display);
    puts("PASS: workspace thumbnails, scroll, Exposé selection and Super chords");
}
