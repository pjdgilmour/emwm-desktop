/* Exercise the pinned C unit and the integrated Rust unit through WmXinerama.h.
 * Only X server replies are scripted; both implementations query them normally. */
#include "WmGlobal.h"
#include "WmXinerama.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

WmGlobalData wmGD;
static WmScreenData screen_data;
static ClientData focused;

static const XineramaScreenInfo layout[] = {
    {7, 0, 0, 100, 80}, {12, 100, 20, 60, 40}, {19, 75, 10, 60, 30}
};
static const XineramaScreenInfo replacement[] = {{27, 0, 0, 80, 90}};
static int present = 1, active = 1, answer = 1, changed, same_screen = 1;
static int pointer_x = 101, pointer_y = 42;

Bool XQueryExtension(Display *d, const char *name, int *op, int *event, int *error)
{
    (void)d;
    *op = *event = *error = 0;
    return !strcmp(name, "XINERAMA") && present;
}

Bool XineramaIsActive(Display *d) { (void)d; return active; }

XineramaScreenInfo *XineramaQueryScreens(Display *d, int *count)
{
    (void)d;
    const XineramaScreenInfo *src = changed ? replacement : layout;
    *count = !answer ? 0 : changed ? 1 : 3;
    if (!answer) return NULL;
    size_t size = (size_t)*count * sizeof(*src);
    XineramaScreenInfo *p = malloc(size);
    if (!p) abort();
    return memcpy(p, src, size);
}

Bool XQueryPointer(Display *d, Window w, Window *root, Window *child,
                  int *x, int *y, int *child_x, int *child_y, unsigned int *mask)
{
    (void)d;
    *root = w;
    *child = *mask = 0;
    *x = *child_x = pointer_x;
    *y = *child_y = pointer_y;
    return same_screen;
}

int XFree(void *p) { free(p); return 1; }
void Warning(char *message) { puts(message); }

static void dump(const char *label, int ok, const XineramaScreenInfo *s)
{
    printf("%s %d %d %d %d %d %d\n", label, ok, s->screen_number, s->x_org,
           s->y_org, s->width, s->height);
}

static void all_queries(void)
{
    static const int points[][2] = {
        {-4, -2}, {99, 79}, {100, 79}, {100, 19}, {100, 20}, {159, 59},
        {160, 59}, {99, 15}, {101, 16}, {135, 16}, {2147483647, 42}
    };
    XineramaScreenInfo s = {-1, -1, -1, -1, -1};
    int count = -1;
    int ok = GetXineramaScreenCount(&count);
    printf("count %d %d\n", ok, count);
    for (unsigned int i = 0; i < sizeof(points)/sizeof(*points); i++)
        dump("point", GetXineramaScreenFromLocation(points[i][0], points[i][1], &s), &s);
    for (int i = 0; i <= count; i++)
        dump("index", GetXineramaScreenInfo(i, &s), &s);
    for (int index = -1; index <= count; index++) {
        wmGD.primaryXineramaScreen = index;
        dump("primary", GetPrimaryXineramaScreen(&s), &s);
    }
    wmGD.keyboardFocus = NULL;
    same_screen = 1;
    dump("pointer", GetXineramaScreenFromPointer(&s), &s);
    dump("active-pointer", GetActiveXineramaScreen(&s), &s);
    same_screen = 0;
    dump("pointer-other-root", GetXineramaScreenFromPointer(&s), &s);
    wmGD.keyboardFocus = &focused;
    focused.clientX = 2;
    focused.clientY = 2;
    dump("focused", GetActiveXineramaScreen(&s), &s);
    focused.clientX = 999;
    dump("focused-offscreen", GetActiveXineramaScreen(&s), &s);
}

int main(int argc, char **argv)
{
    const char *scenario = argc == 2 ? argv[1] : "";
    wmGD.pActiveSD = &screen_data;
    if (!strcmp(scenario, "absent")) present = 0;
    if (!strcmp(scenario, "disabled")) active = 0;
    if (!strcmp(scenario, "failed-query")) answer = 0;
    SetupXinerama();
    all_queries();
    if (!strcmp(scenario, "change")) {
        changed = 1;
        UpdateXineramaInfo();
        all_queries();
        active = 0;
        UpdateXineramaInfo();
        all_queries();
        active = 1;
        UpdateXineramaInfo();
        all_queries();
    }
    return 0;
}
