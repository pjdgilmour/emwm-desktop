/*
 * This software is provided with no  warranty whatsoever. You may use, modify
 * and redistribute it as long as you give a credit to the original author and
 * provide a link to the original source.
 *
 * https://fastestcode.org/emwm.html
 * alx@fastestcode.org
 *
 * EMWM workspace manager utility functions
 *
 * $Id: wsmutil.h,v 1.4 2026/03/15 13:53:40 alx Exp $
 */

#ifndef WSM_UTIL_H
#define WSM_UTIL_H

#define _XA_MWM_WORKSPACE_LIST     "_MWM_WORKSPACE_LIST"
#define _XA_MWM_WORKSPACE_INFO     "_MWM_WORKSPACE_INFO"
#define _XA_MWM_WORKSPACE_CURRENT  "_MWM_WORKSPACE_CURRENT"
#define _XA_MWM_WORKSPACE_PRESENCE "_MWM_WORKSPACE_PRESENCE"
#define _XA_MWM_WM_REQUEST         "_MWM_WM_REQUEST"

struct WsmWorkspaceInfo {
	Atom id;
	char *title;
	int colorset;
	unsigned long bg_pixel;
	unsigned long fg_pixel;
	char *bd_image;
	Window bd_window;
};

/* Returns the MWM workspace window on success, None on error. */
Window WsmGetWorkspaceWindow(Display*, Window root_window);

/* Retrieves the list of MWM workspace Atoms and their count.
 * Returns a newly allocated array of Atoms, or NULL on failure.
 * Deallocate the array with free() when  done using. */
Atom* WsmGetWorkspaceList(Display*, Window mwm_wswnd, unsigned int *nret);

/* Retrieves MWM workspace info. Returns True on success.
 * Use WsmFreeWorkspaceInfo to deallocate internal fields when done using. */
Bool WsmGetWorkspaceInfo(Display *dpy, Window mwm_wswnd,
	Atom xa_ws_id, struct WsmWorkspaceInfo*);

/* Frees internal fields of the WsmWorkspaceInfo structure
 * previously allocaded by a call to WsmGetWorkspaceInfo. */
void WsmFreeWorkspaceInfo(struct WsmWorkspaceInfo*);

/* Returns currently active workspace ID atom. */
Atom WsmGetActiveWorkspace(Display *dpy, Window mwm_swnd);

/* Returns True if client is in workspace specified (among others). */
Bool WsmIsInWorkspace(Display *dpy, Atom ws_id, Window client);

/* Retrieves the list and count of workspace IDs occupied by client.
 * Returns a newly allocated array of Atoms, NULL on failure.
 * Deallocate the array with free() when  done using. */
Atom* WsmGetPresence(Display *dpy, Window client, unsigned int *nret);

/* Sends a request to the window manager */
Bool WsmSendRequest(Display *dpy, Window mwm_wswnd, const char *request);

/* Sends a Change Backdrop request to the window manager. */
Bool WsmChangeBackdrop(Display *dpy, Window mwm_wswnd,
	Atom workspace_id, unsigned long background, unsigned long foreground,
	const char *file_name);

/* Sends a Change Workspace request to the window manager. */
Bool WsmChangeWorkspace(Display *dpy, Window mwm_wswnd, Atom xa_wsid);

/* Sends a Name Workspace request to the window manager */
Bool WsmNameWorkspace(Display *dpy,
	Window mwm_wswnd, Atom xa_wsid, const char *title);

/* Sends an Add Workspace request to the window manager.
 * The title argument is optional and can be NULL. */
Bool WsmAddWorkspace(Display *dpy, Window mwm_wswnd,  const char *title);

/* Sends a Delete Workspace request to the window manager. */
Bool WsmDeleteWorkspace(Display *dpy, Window mwm_wswnd, Atom xa_wsid);


#endif /* WSM_UTIL_H */
