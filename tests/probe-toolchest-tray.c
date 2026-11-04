/* SPDX-License-Identifier: MIT
 * Black-box check of the installed Toolchest under the real window manager. */
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <signal.h>
#include <stdlib.h>

static Display *dpy;
static Window root;
static Window parent(Window w) {
    Window r, p, *children = NULL;
    unsigned n;
    assert(XQueryTree(dpy, w, &r, &p, &children, &n));
    if (children) XFree(children);
    return p;
}
int main(int argc, char **argv) {
    dpy = XOpenDisplay(NULL); assert(dpy);
    root = DefaultRootWindow(dpy);
    Atom selection = XInternAtom(dpy, "_NET_SYSTEM_TRAY_S0", False);
    Atom opcode = XInternAtom(dpy, "_NET_SYSTEM_TRAY_OPCODE", False);
    Atom info = XInternAtom(dpy, "_XEMBED_INFO", False);
    Window owner = None, icons[9];
    for (int i = 0; i < 100 && !owner; ++i) {
        owner = XGetSelectionOwner(dpy, selection);
        if (!owner) usleep(20000);
    }
    assert(owner != None);
    for (int i = 0; i < 9; ++i) {
        unsigned long values[] = {0, 1};
        icons[i] = XCreateSimpleWindow(dpy, root, 0, 0, 16, 16, 0, 0, 0x40a080);
        XChangeProperty(dpy, icons[i], info, info, 32, PropModeReplace,
            (unsigned char *)values, 2);
        XEvent e = {0};
        e.xclient.type = ClientMessage; e.xclient.window = owner;
        e.xclient.message_type = opcode; e.xclient.format = 32;
        e.xclient.data.l[2] = icons[i];
        XSendEvent(dpy, owner, False, 0, &e);
        if (i == 0) {
            XFlush(dpy);
            for (int retry = 0; retry < 100 && parent(icons[0]) == root; ++retry) usleep(20000);
            XWindowAttributes host;
            XGetWindowAttributes(dpy, parent(parent(icons[0])), &host);
            assert(host.height == 30);
        }
    }
    XFlush(dpy);
    for (int i = 0; i < 100 && parent(icons[8]) == root; ++i) usleep(20000);
    usleep(200000);
    Window host = parent(parent(icons[0])), top = host;
    int found = 0;
    while (top != root) {
        XClassHint hint = {0};
        if (XGetClassHint(dpy, top, &hint)) {
            found = hint.res_class && !strcmp(hint.res_class, "XmToolbox");
            if (hint.res_name) XFree(hint.res_name);
            if (hint.res_class) XFree(hint.res_class);
            if (found) break;
        }
        top = parent(top);
    }
    assert(found);
    XWindowAttributes first, attrs, host_attrs;
    XGetWindowAttributes(dpy, parent(icons[0]), &first);
    XGetWindowAttributes(dpy, host, &host_attrs);
    for (int i = 0; i < 9; ++i) {
        assert(parent(parent(icons[i])) == host);
        XGetWindowAttributes(dpy, parent(icons[i]), &attrs);
        int row_count = 9 - (i / 4)*4;
        if (row_count > 4) row_count = 4;
        assert(attrs.x == (host_attrs.width - (row_count*24 + (row_count-1)*4))/2 + (i % 4)*28);
        assert(attrs.y == first.y + (i / 4)*28);
        XGetWindowAttributes(dpy, icons[i], &attrs);
        assert(attrs.width == 24 && attrs.height == 24 && attrs.map_state == IsViewable);
    }
    XGetWindowAttributes(dpy, host, &attrs);
    assert(attrs.height == 3*24 + 2*4 + 6);
    if (argc == 3) {
        /* Real WM + foreign clients: gracefully stopping the Toolchest must
         * release every icon before the WM destroys its frame. */
        pid_t previous = (pid_t)strtol(argv[1], NULL, 10);
        assert(previous > 1 && kill(previous, SIGTERM) == 0);
        for (int i = 0; i < 100 && XGetSelectionOwner(dpy, selection) == owner; ++i)
            usleep(20000);
        assert(XGetSelectionOwner(dpy, selection) == None);
        for (int i = 0; i < 9; ++i) assert(parent(icons[i]) == root);
        pid_t replacement = fork(); assert(replacement >= 0);
        if (!replacement) { execl(argv[2], argv[2], NULL); _exit(127); }
        owner = None;
        for (int i = 0; i < 100 && !owner; ++i) {
            owner = XGetSelectionOwner(dpy, selection);
            if (!owner) usleep(20000);
        }
        assert(owner != None);
        for (int i = 0; i < 9; ++i) {
            XEvent e = {0};
            e.xclient.type = ClientMessage; e.xclient.window = owner;
            e.xclient.message_type = opcode; e.xclient.format = 32;
            e.xclient.data.l[2] = icons[i];
            XSendEvent(dpy, owner, False, 0, &e);
        }
        XFlush(dpy);
        for (int i = 0; i < 100 && parent(icons[8]) == root; ++i) usleep(20000);
        for (int i = 0; i < 9; ++i) {
            assert(parent(icons[i]) != root);
            XGetWindowAttributes(dpy, icons[i], &attrs);
            assert(attrs.map_state == IsViewable);
        }
        puts("PASS: all foreign icons survive Toolchest shutdown and dock again after restart");
    }
    for (int i = 0; i < 9; ++i) XDestroyWindow(dpy, icons[i]);
    XCloseDisplay(dpy);
    puts("PASS: packaged Toolchest owns tray and displays nine icons in three rows");
    return 0;
}
