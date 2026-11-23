/* SPDX-License-Identifier: MIT. Public test hooks in the test-probes build only. */
#include <Xm/Xm.h>
#include <Xm/PushBG.h>
#include <Xm/CascadeBG.h>
#include <X11/Xutil.h>
#include <string.h>
#include <unistd.h>
#include <stdio.h>
void toolbox_probe_init(int, char **, int);
Widget toolbox_probe_shell(void);
XtAppContext toolbox_probe_context(void);
Widget toolbox_probe_menu(void);
Widget toolbox_probe_clock(void);
Boolean toolbox_probe_tray_enabled(void);
Boolean toolbox_probe_clock_enabled(void);
void toolbox_probe_set_config(char *);
void toolbox_probe_applications(Boolean);
Boolean toolbox_probe_construct_menu(void);
#define wshell toolbox_probe_shell()
#define app_context toolbox_probe_context()
#define wmenu toolbox_probe_menu()
#define wdtframe toolbox_probe_clock()
