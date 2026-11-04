/* SPDX-License-Identifier: MIT
 * Exercise the actual Toolchest footer with foreign XEmbed clients. */
#define main toolbox_program_main
#include "tbmain.c"
#undef main
#include <X11/extensions/XTest.h>
#include <assert.h>

static Display *client;
static Window root, owner;
static Atom info, selection;

static void drain(void) {
    XSync(client, False);
    for (int i = 0; i < 30; ++i) {
        XSync(XtDisplay(wshell), False);
        while (XtAppPending(app_context)) XtAppProcessEvent(app_context, XtIMAll);
        usleep(3000);
    }
    XSync(client, False);
}
static Window parent(Window w) {
    Window r, p, *children = NULL;
    unsigned n;
    assert(XQueryTree(client, w, &r, &p, &children, &n));
    if (children) XFree(children);
    return p;
}
static void mapped(Window w, int visible) {
    unsigned long values[] = {0, visible};
    XChangeProperty(client, w, info, info, 32, PropModeReplace,
        (unsigned char *)values, 2);
}
static Window add_icon(int argb) {
    Window w;
    if (argb) {
        XVisualInfo visual;
        XSetWindowAttributes attrs = {0};
        assert(XMatchVisualInfo(client, 0, 32, TrueColor, &visual));
        attrs.colormap = XCreateColormap(client, root, visual.visual, AllocNone);
        attrs.background_pixel = 0; attrs.border_pixel = 0;
        w = XCreateWindow(client, root, 0, 0, 16, 16, 0, 32, InputOutput,
            visual.visual, CWColormap | CWBackPixel | CWBorderPixel, &attrs);
    } else w = XCreateSimpleWindow(client, root, 0, 0, 16, 16, 0, 0, 0x40a080);
    XSelectInput(client, w, StructureNotifyMask | ButtonPressMask);
    mapped(w, 1);
    XEvent event = {0};
    event.xclient.type = ClientMessage; event.xclient.window = owner;
    event.xclient.message_type = XInternAtom(client, "_NET_SYSTEM_TRAY_OPCODE", False);
    event.xclient.format = 32; event.xclient.data.l[2] = w;
    XSendEvent(client, owner, False, 0, &event);
    drain();
    return w;
}
static int height(Widget w) {
    Dimension h;
    XtVaGetValues(w, XmNheight, &h, NULL);
    return h;
}
static void grid(Window *icons, int count) {
    XWindowAttributes first, attrs, host;
    XGetWindowAttributes(client, parent(icons[0]), &first);
    XGetWindowAttributes(client, parent(parent(icons[0])), &host);
    for (int i = 0; i < count; ++i) {
        int row_count = count - (i / 4)*4;
        if (row_count > 4) row_count = 4;
        XGetWindowAttributes(client, parent(icons[i]), &attrs);
        assert(attrs.x == (host.width - (row_count*24 + (row_count-1)*4))/2 + (i % 4)*28);
        assert(attrs.y == first.y + (i / 4)*28);
        XGetWindowAttributes(client, icons[i], &attrs);
        assert(attrs.width == 24 && attrs.height == 24 && attrs.map_state == IsViewable);
    }
}
int main(int argc, char **argv) {
    Window icons[9], host, ancestor, child;
    XWindowAttributes attrs;
    int x, y, clock_x, clock_y;
    XtSetLanguageProc(NULL, NULL, NULL);
    wshell = XtVaAppInitialize(&app_context, "XmToolbox", xrdb_options,
        XtNumber(xrdb_options), &argc, argv, fallback_res,
        XmNallowShellResize, True, NULL);
    XtGetApplicationResources(wshell, &app_res, xrdb_resources,
        XtNumber(xrdb_resources), NULL, 0);
    wmain = XmVaCreateManagedRowColumn(wshell, "main",
        XmNorientation, app_res.horizontal ? XmHORIZONTAL : XmVERTICAL, NULL);
    create_utility_widgets(wmain);
    XtRealizeWidget(wshell);
    if (app_res.system_tray) TrayStart();
    client = XOpenDisplay(NULL); assert(client);
    root = DefaultRootWindow(client);
    info = XInternAtom(client, "_XEMBED_INFO", False);
    selection = XInternAtom(client, "_NET_SYSTEM_TRAY_S0", False);
    owner = XGetSelectionOwner(client, selection);
    drain();
    if (!app_res.system_tray) {
        assert(owner == None);
        puts("PASS: disabled Toolchest tray leaves selection unowned");
        return 0;
    }
    assert(owner != None);
    int empty_height = height(wshell);
    icons[0] = add_icon(0);
    grid(icons, 1);
    XGetWindowAttributes(client, parent(parent(icons[0])), &attrs);
    assert(attrs.height == 30);
    icons[1] = add_icon(1);
    grid(icons, 2);
    host = parent(parent(icons[0]));
    XGetWindowAttributes(client, host, &attrs);
    assert(attrs.height == 30);
    assert(attrs.width < 160);
    for (int i = 2; i < 4; ++i) icons[i] = add_icon(0);
    grid(icons, 4);
    host = parent(parent(icons[0]));
    ancestor = host;
    while (ancestor != root && ancestor != XtWindow(wshell)) ancestor = parent(ancestor);
    assert(ancestor == XtWindow(wshell));
    XTranslateCoordinates(client, host, root, 0, 0, &x, &y, &child);
    if (app_res.show_date_time) {
        XTranslateCoordinates(client, XtWindow(wdtframe), root, 0, 0, &clock_x, &clock_y, &child);
        assert(y >= clock_y + height(wdtframe));
    }
    int single_height = height(wshell);
    icons[4] = add_icon(0); grid(icons, 5);
    assert(height(wshell) == single_height + 28);
    for (int i = 5; i < 9; ++i) icons[i] = add_icon(i == 8);
    grid(icons, 9);
    assert(height(wshell) == single_height + 56);
    /* Native right clicks reach the embedded application's window. */
    while (XPending(client)) { XEvent e; XNextEvent(client, &e); }
    XTranslateCoordinates(client, icons[4], root, 5, 5, &x, &y, &child);
    XTestFakeMotionEvent(client, -1, x, y, CurrentTime);
    XTestFakeButtonEvent(client, 3, True, CurrentTime);
    XTestFakeButtonEvent(client, 3, False, CurrentTime);
    drain();
    int clicks = 0;
    while (XPending(client)) {
        XEvent e; XNextEvent(client, &e);
        if (e.type == ButtonPress && e.xbutton.window == icons[4] && e.xbutton.button == 3) ++clicks;
    }
    assert(clicks == 1);
    mapped(icons[0], 0); drain(); grid(icons + 1, 8);
    assert(height(wshell) == single_height + 28);
    mapped(icons[0], 1); drain(); grid(icons, 9);
    for (int i = 8; i >= 4; --i) XDestroyWindow(client, icons[i]);
    drain(); grid(icons, 4);
    assert(height(wshell) == single_height);
    for (int i = 0; i < 4; ++i) mapped(icons[i], 0);
    drain();
    assert(height(wshell) == empty_height);
    for (int i = 0; i < 4; ++i) mapped(icons[i], 1);
    drain(); grid(icons, 4);
    /* Losing the tray selection must never close the Toolchest. */
    Window other = XCreateSimpleWindow(client, root, 0, 0, 1, 1, 0, 0, 0);
    XSetSelectionOwner(client, selection, other, CurrentTime);
    drain();
    assert(XGetSelectionOwner(client, selection) == other);
    assert(!XtAppGetExitFlag(app_context));
    XGetWindowAttributes(client, XtWindow(wshell), &attrs);
    assert(attrs.map_state == IsViewable);
    for (int i = 0; i < 4; ++i) assert(parent(icons[i]) == root);
    puts("PASS: Toolchest ancestry, clock placement, 4/5/9 icons, ARGB, clicks, hide/remap, removal and selection loss");
    XCloseDisplay(client);
    XtDestroyWidget(wshell);
    return 0;
}
