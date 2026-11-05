/* SPDX-License-Identifier: GPL-3.0-only
 * Private-Xvfb fixture: emulate GDK monitor topology, including RandR signals.
 * No X11 requests are intercepted; rendering, file operations and DnD are real.
 */
#include <gtk/gtk.h>
#include <stdio.h>
#include <stdlib.h>
static int count = 2, primary = 1, first = 0, origin = 0;
static GdkScreen *watched;
static gboolean poll_topology(gpointer unused)
{
    int n = count, p = primary, f = 0, x = 0;
    const char *path = getenv("EMWM_TEST_TOPOLOGY");
    FILE *file = path ? fopen(path, "r") : NULL;
    if (file) { if (fscanf(file, "%d %d %d %d", &n, &p, &f, &x) < 2) { n=count; p=primary; } fclose(file); }
    if (n != count || p != primary || f != first || x != origin) {
        count = n; primary = p; first = f; origin = x;
        g_signal_emit_by_name(watched, "monitors-changed");
    }
    return G_SOURCE_CONTINUE;
}
gint gdk_screen_get_n_monitors(GdkScreen *screen)
{
    if (!watched) { watched=screen; g_timeout_add(100, poll_topology, NULL); }
    return count;
}
gint gdk_screen_get_primary_monitor(GdkScreen *screen) { return primary; }
gchar *gdk_screen_get_monitor_plug_name(GdkScreen *screen, gint monitor)
{ return g_strdup((monitor + first) ? "TEST-B" : "TEST-A"); }
void gdk_screen_get_monitor_geometry(GdkScreen *screen, gint monitor, GdkRectangle *rect)
{ *rect=(GdkRectangle){origin+monitor*800, 0, 800, 600}; }
void gdk_screen_get_monitor_workarea(GdkScreen *screen, gint monitor, GdkRectangle *rect)
{ gdk_screen_get_monitor_geometry(screen, monitor, rect);
  if (count == 1 && first) rect->width = MAX(1, 800 - origin); }
gint gdk_screen_get_width(GdkScreen *screen) { return count * 800; }
