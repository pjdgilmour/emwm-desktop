/* SPDX-License-Identifier: GPL-3.0-only
 * Private-Xvfb fixture: emulate GDK monitor topology, including RandR signals.
 * No X11 requests are intercepted; rendering, file operations and DnD are real.
 */
#include <gtk/gtk.h>
#include <stdio.h>
#include <stdlib.h>
static int count = 2, primary = 1;
static GdkScreen *watched;
static gboolean poll_topology(gpointer unused)
{
    int n = count, p = primary;
    const char *path = getenv("EMWM_TEST_TOPOLOGY");
    FILE *file = path ? fopen(path, "r") : NULL;
    if (file) { if (fscanf(file, "%d %d", &n, &p) != 2) { n=count; p=primary; } fclose(file); }
    if (n != count || p != primary) {
        count = n; primary = p;
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
{ return g_strdup(monitor ? "TEST-B" : "TEST-A"); }
void gdk_screen_get_monitor_geometry(GdkScreen *screen, gint monitor, GdkRectangle *rect)
{ *rect=(GdkRectangle){monitor*800, 0, 800, 600}; }
void gdk_screen_get_monitor_workarea(GdkScreen *screen, gint monitor, GdkRectangle *rect)
{ gdk_screen_get_monitor_geometry(screen, monitor, rect); }
