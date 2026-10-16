/*
 * This software is provided with no  warranty whatsoever. You may use, modify
 * and redistribute it as long as you give a credit to the original author and
 * provide a link to the original source.
 *
 * https://fastestcode.org/emwm.html
 * alx@fastestcode.org
 *
 * $Id: tellmwm.c,v 1.7 2026/07/21 16:01:48 alx Exp $
 */
 
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <X11/Xlib.h>
#include <X11/Xatom.h>
#include "wsmutil.h"

#define APP_NAME "tellmwm"
#define APP_VER	1
#define APP_REV 2

Window get_wsm_window(void);
Bool set_backdrop(char**, int);
Bool name_workspace(char**, int);
Bool del_workspace(char**, int);
Bool add_workspace(char**, int);
Bool set_active(char**, int);
int main(int argc, char **argv);
Bool print_info(void);
void print_usage();
void print_version_info();

static Display *dpy;
static Window wroot;

Window get_wsm_window(void)
{
	Window wsm_wnd = WsmGetWorkspaceWindow(dpy, wroot);
	if(!wsm_wnd)
		fputs("Failed to retrieve workspace manager info!\n", stderr);

	return wsm_wnd;
}

Bool set_backdrop(char **argv, int argc)
{
	Window wsm_wnd = get_wsm_window();
	struct WsmWorkspaceInfo wsi;
	Bool result;
	Atom ws_id;
	Colormap cmap;
	XColor bg_color;
	XColor fg_color;
	XColor tmp;
	char *file_name = NULL;
	int i;
	
	if(!wsm_wnd) return False;
	cmap = DefaultColormap(dpy, DefaultScreen(dpy));
	
	if(argc < 2) {
		fputs("Command requires a workspace ID "
			"and color and/or image parameters\n", stderr);
		return False;
	}
	
	if(!(ws_id = XInternAtom(dpy, argv[0], True)) ) {
		fputs("Invalid workspace ID specified!\n", stderr);
		return False;
	}

	if(!WsmGetWorkspaceInfo(dpy, wsm_wnd, ws_id, &wsi)) {
		fputs("Failed to retrieve workspace info!\n", stderr);
		return False;
	}
	
	/* Store current colors, in case the user hasn't specified any */
	fg_color.pixel = wsi.fg_pixel;
	bg_color.pixel = wsi.bg_pixel;
		
	for(i = 1; i < argc; i++) {
		if(!strcmp(argv[i], "-f")) {
			i++;
			
			if(i == argc) {
				fputs("Missing parameter for -f\n", stderr);
				return False;
			}
			if(!XAllocNamedColor(dpy, cmap, argv[i], &fg_color, &tmp)) {
				fprintf( stderr,
					"Cannot allocate named color \'%s\'\n", argv[i]);
			}
		} else if(!strcmp(argv[i], "-b")) {
			i++;
			
			if(i == argc) {
				fputs("Missing parameter for -b\n", stderr);
				return False;
			}
			if(!XAllocNamedColor(dpy, cmap, argv[i], &bg_color, &tmp)) {
				fprintf(stderr, "Cannot allocate named color \'%s\'\n", argv[i]);
			}
		} else if(!file_name) {
			file_name = argv[i];
		} else {
			fprintf(stderr, "Ambiguous parameter \'%s\'\n", argv[i]);
			return False;
		}		
	}
	
	/* Keep current image if none specified */
	if(!file_name) file_name = wsi.bd_image;
	
	result = WsmChangeBackdrop(dpy, wsm_wnd,
		ws_id, bg_color.pixel, fg_color.pixel, file_name);

	if(!result) fputs("Failed to send WSM command\n", stderr);

	return result;
}


Bool name_workspace(char **argv, int argc)
{
	Bool result;
	Window wsm_wnd = get_wsm_window();
	Atom ws_id;
	char *title = NULL;
	
	if(!wsm_wnd) return False;
	
	if((argc < 2) || !(ws_id = XInternAtom(dpy, argv[0], True)) ) {
		if(argc > 1)
			fputs("Invalid workspace ID specified!\n", stderr);
		else
			fputs("Command requires workspace ID "
				"and name arguments\n", stderr);
		return False;
	}
	title = argv[1];
	
	result = WsmNameWorkspace(dpy, wsm_wnd, ws_id, title);
	if(!result) fputs("Failed to send WSM command\n", stderr);

	return result;
}

Bool del_workspace(char **argv, int argc)
{
	Window wsm_wnd = get_wsm_window();
	Bool result;
	Atom ws_id;
	
	if(!wsm_wnd) return False;
	
	
	if(!argc || !(ws_id = XInternAtom(dpy, argv[0], True)) ) {
		if(argc)
			fputs("Invalid workspace ID specified!\n", stderr);
		else
			fputs("Command requires a workspace ID\n", stderr);
		return False;
	}
	
	result = WsmDeleteWorkspace(dpy, wsm_wnd, ws_id);
	if(!result) fputs("Failed to send WSM command\n", stderr);

	return result;
}


Bool add_workspace(char **argv, int argc)
{
	Window wsm_wnd = get_wsm_window();
	Bool result;
	char *title = NULL;
	
	if(!wsm_wnd) return False;
	
	if(argc) title = argv[0];
	
	result = WsmAddWorkspace(dpy, wsm_wnd, title);
	if(!result) fputs("Failed to send WSM command\n", stderr);

	return result;
}


Bool set_active(char **argv, int argc)
{
	Window wsm_wnd = get_wsm_window();
	Bool result;
	Atom ws_id;
	
	if(!wsm_wnd) return False;
	
	if(!argc || !(ws_id = XInternAtom(dpy, argv[0], True)) ) {
		if(argc)
			fputs("Invalid workspace ID specified!\n", stderr);
		else
			fputs("Command requires a workspace ID\n", stderr);
		return False;
	}
	
	result = WsmChangeWorkspace(dpy, wsm_wnd, ws_id);
	if(!result) fputs("Failed to send WSM command\n", stderr);

	return result;
}

Bool print_info(void)
{
	Window wsm_wnd;
	Atom *id_list;
	Atom id_active;
	unsigned int i, nws;
	char *id_name;
		
	wsm_wnd = WsmGetWorkspaceWindow(dpy, wroot);
	if(!wsm_wnd) return False;

	id_list = WsmGetWorkspaceList(dpy, wsm_wnd, &nws);
	if(!id_list) return False;
	
	id_active = WsmGetActiveWorkspace(dpy, wsm_wnd);
	
	for(i = 0; i < nws; i++) {
		struct WsmWorkspaceInfo wsi;
		char *id_name;
		
		if(!WsmGetWorkspaceInfo(dpy, wsm_wnd, id_list[i], &wsi))
			return False;
		
		id_name = XGetAtomName(dpy, wsi.id);
		if(!id_name) return False;
		
		printf("Workspace \"%s\"\n"
			"\tTitle     : %s\n"
			"\tForeground: #%.6lx\n"
			"\tBackground: #%.6lx\n"
			"\tBackdrop  : %s\n\n",
			id_name,
			wsi.title,
			wsi.fg_pixel,
			wsi.bg_pixel,
			wsi.bd_image ? wsi.bd_image : "(none)");
		
		WsmFreeWorkspaceInfo(&wsi);
	}
	id_name = XGetAtomName(dpy, id_active);
	printf("Current Workspace: %s\n", id_name);
	free(id_list);
	
	return True;
}


int main(int argc, char **argv)
{
	Bool result = True;

	dpy = XOpenDisplay(NULL);
	if(!dpy) {
		fputs("Can't open display\n", stderr);
		return EXIT_FAILURE;
	}
	
	wroot = DefaultRootWindow(dpy);

	if(argc < 2) {
		result = print_info();
	} else {
		if(!strcmp(argv[1], "backdrop")) 
			result = set_backdrop(argv + 2, argc - 2);
		else if(!strcmp(argv[1], "setws"))
			result = set_active(argv + 2, argc - 2);
		else if(!strcmp(argv[1], "addws"))
			result = add_workspace(argv + 2, argc - 2);
		else if(!strcmp(argv[1], "namews"))
			result = name_workspace(argv + 2, argc - 2);
		else if(!strcmp(argv[1], "delws"))
			result = del_workspace(argv + 2, argc - 2);
		else if(!strcmp(argv[1], "-h") || !strcmp(argv[1], "help"))
			print_usage();
		else if(!strcmp(argv[1], "-v"))
			print_version_info();
		else {
			fputs("Invalid command specified\n", stderr);
		}
	}
	
	XSync(dpy, True);
	XCloseDisplay(dpy);
	
	return result ? EXIT_SUCCESS : EXIT_FAILURE;
}

void print_usage()
{
	print_version_info();
	
	printf("\nUsage: tellmwm [<command> <arguments>]\n"
		"Commands and arguments (see man 1 tellmwm for more info):\n"
		"\tbackdrop <workspace ID> [-f <color> -b <color>] [<image name> | none]\n"
		"\tsetws    <workspace ID>\n"
		"\taddws    [<title>]\n"
		"\tnamews   <workspace ID> <title>\n"
		"\tdelws    <workspace ID>\n"
		"If no command is specified, current configuration will be printed.\n"
	);
}

void print_version_info()
{
	const char APP_INFO[] = "http://fastestcode.org/emwm.html";
	printf("%s v%d.%d - %s\n", APP_NAME, APP_VER, APP_REV, APP_INFO);
}
