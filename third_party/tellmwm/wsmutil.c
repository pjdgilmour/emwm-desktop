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
 * $Id: wsmutil.c,v 1.5 2026/03/15 13:53:40 alx Exp $
 */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <X11/Xlib.h>
#include <X11/Xatom.h>
#include <X11/Xutil.h>
#include <Xm/MwmUtil.h>
#include "wsmutil.h"

#define MAX_WORKSPACES 256 /* this is not enforced by MWM */

#define REQ_BACKDROP "BACKDROP"
/* BACKDROP <workspace id> <foreground> <background> <image name> */

#define REQ_WRKSPACE "WRKSPACE"
/* WRKSPACE <workspace id> */

#define REQ_ADDWKSPC "ADDWKSPC"
/* ADDWKSPC <title> */

#define REQ_DELWKSPC "DELWKSPC"
/* DELWKSPC <id> */

#define REQ_NAMWKSPC "NAMWKSPC"
/* NAMWKSPC <id> <title> */

/* Returns the MWM workspace window on success, None on error. */
Window WsmGetWorkspaceWindow(Display *dpy, Window root)
{
	static Atom xa_mwm_info = None;
	Atom ret_type;
	int ret_format;
	unsigned long ret_items;
	unsigned long left_items;
	PropMotifWmInfo *mwm_info;
	Window mwm_window = None;
	
	if(xa_mwm_info == None)
		xa_mwm_info = XInternAtom(dpy, _XA_MOTIF_WM_INFO, True);

	if(xa_mwm_info == None) return None;	
	
	if(XGetWindowProperty(dpy, root, xa_mwm_info, 0,
		PROP_MOTIF_WM_INFO_ELEMENTS, False,
		xa_mwm_info, &ret_type, &ret_format,
		&ret_items, &left_items,
		(unsigned char**)&mwm_info) != Success)	return None;
	
	if(ret_type == xa_mwm_info) mwm_window = mwm_info->wmWindow;

	XFree(mwm_info);

	return mwm_window;
}

/* Retrieves the list of MWM workspace Atoms and their count.
 * Returns a newly allocated array of Atoms, or NULL on failure.
 * Deallocate the array with free() when  done using. */
Atom* WsmGetWorkspaceList(Display *dpy, Window mwm_wswnd, unsigned int *nret)
{
	static Atom xa_ws_list = None;
	Atom ret_type;
	int ret_format;
	unsigned long ret_items;
	unsigned long left_items;
	unsigned char *data;
	Atom *ws_list = NULL;
	
	if(xa_ws_list == None)
		xa_ws_list = XInternAtom(dpy,  _XA_MWM_WORKSPACE_LIST, True);

	if(xa_ws_list == None) return False;
		
	if(XGetWindowProperty(dpy, mwm_wswnd, xa_ws_list, 0,
		MAX_WORKSPACES, False, XA_ATOM, &ret_type, &ret_format,
		&ret_items, &left_items, &data) != Success) return False;

	if(ret_type == XA_ATOM) {
		ws_list = malloc(sizeof(Atom) * ret_items);
		if(ws_list) memcpy(ws_list, data, sizeof(Atom) * ret_items);
		XFree(data);
		
		*nret = (unsigned int)ret_items;
	}
	return ws_list;
}

/* Retrieves MWM workspace info. Returns True on success.
 * Use WsmFreeWorkspaceInfo to deallocate internal fields when done using. */
Bool WsmGetWorkspaceInfo(Display *dpy, Window mwm_wswnd,
	Atom xa_ws_id, struct WsmWorkspaceInfo *ir)
{
	Atom xa_ws_info;
	char *ws_id_name;
	char *ws_info_name;
	size_t len;
	XTextProperty xtp;
	Bool result = False;

	ws_id_name = XGetAtomName(dpy, xa_ws_id);
	len = strlen(ws_id_name) + strlen(_XA_MWM_WORKSPACE_INFO) + 3;
	ws_info_name = malloc(len + 1);
	if(!ws_info_name) {
		XFree(ws_id_name);
		return False;
	}

	sprintf(ws_info_name, "%s_%s", _XA_MWM_WORKSPACE_INFO, ws_id_name);
	xa_ws_info = XInternAtom(dpy, ws_info_name, True);
	free(ws_info_name);
	XFree(ws_id_name);
	
	if(xa_ws_info == None) return False;
	
	xtp.value = NULL;
	XGetTextProperty(dpy, mwm_wswnd, &xtp, xa_ws_info);
	
	if(xtp.value) {
		char **list = NULL;
		int n = 0;

		XmbTextPropertyToTextList(dpy, &xtp, &list, &n);
		if(list) {
			if(n >= 7) {
				char *bd_xa_name = NULL;
				Atom xa_backdrop = (Atom)strtol(list[4], NULL, 0);
				if(xa_backdrop) bd_xa_name = XGetAtomName(dpy, xa_backdrop);
				int nbackdrop = (int)strtol(list[5], NULL, 0);
				
				#ifdef WSMUTIL_DEBUG
				printf("Title:\t%s\nPixel Set:\t%s\nBackground:\t%s\n"
					"Foreground:\t%s\nBD File Name:\t%s\nWindow:\t%s\n",
					list[0], list[1], list[2], list[3],
					bd_xa_name ? bd_xa_name : "None",
					nbackdrop ? list[6] : "None");
				#endif
				
				ir->id = xa_ws_id;
				ir->title = strdup(list[0]);
				ir->colorset = (int)strtol(list[1], NULL, 0);
				ir->bg_pixel = strtoul(list[2], NULL, 0);
				ir->fg_pixel = strtoul(list[3], NULL, 0);
				ir->bd_image = bd_xa_name ? strdup(bd_xa_name) : NULL;
				if(nbackdrop)
					ir->bd_window = (Window)strtol(list[6], NULL, 0);
				else
					ir->bd_window = None;

				if(bd_xa_name) XFree(bd_xa_name);
				
				result = True;
			}
			XFreeStringList(list);
		}
		XFree(xtp.value);
	}

	return result;
}

/* Returns the active workspace atom */
Atom WsmGetActiveWorkspace(Display *dpy, Window wsm_wnd)
{
	static Atom xa_currentws = None;
	Atom ret_type;
	int ret_format;
	unsigned long ret_items;
	unsigned long left_items;
	Atom *data;
	Atom ws_id = None;
	
	if(xa_currentws == None) {
		xa_currentws = XInternAtom(dpy, _XA_MWM_WORKSPACE_CURRENT, True);
		if(!xa_currentws) return None;
	}

	if(XGetWindowProperty(dpy, wsm_wnd,
		xa_currentws, 0, 1, False, XA_ATOM, &ret_type,
		&ret_format, &ret_items, &left_items,
		(unsigned char**)&data) != Success) return False;

	if(ret_type == XA_ATOM) ws_id = *data;

	XFree(data);
	
	return ws_id;
}


/* Retrieves the list and count of workspace IDs occupied by client.
 * Returns a newly allocated array of Atoms, NULL on failure.
 * Deallocate the array with free() when  done using. */
Atom* WsmGetPresence(Display *dpy, Window client_window, unsigned int *nret)
{
	static Atom xa_presence = None;
	Atom ret_type;
	int ret_format;
	unsigned long nids;
	unsigned long left;
	Atom *data;
	Atom *ws_ids = NULL;

	if(xa_presence == None) {
		xa_presence = XInternAtom(dpy, _XA_MWM_WORKSPACE_PRESENCE, True);
		if(!xa_presence) return NULL;
	}


	if(XGetWindowProperty(dpy, client_window,
		xa_presence, 0, MAX_WORKSPACES, False, xa_presence,
		&ret_type, &ret_format, &nids, &left,
		(unsigned char **)&data) != Success) return False;

	if( !nids || (ret_type != xa_presence) || (ret_format != 32) ) {
		XFree(data);
		return NULL;
	}
	
	ws_ids = malloc(sizeof(Atom) * nids);
	if(ws_ids) {
		memcpy(ws_ids, data, sizeof(Atom) * nids);
		*nret = nids;
	}

	XFree(data);

    return ws_ids;
}

/* Returns True if client is in workspace ws_id */
Bool WsmIsInWorkspace(Display *dpy, Atom ws_id, Window client)
{
	Atom *ws_list;
	unsigned int nids;
	Bool result = False;
	
	if( (ws_list = WsmGetPresence(dpy, client, &nids)) ) {
		unsigned int i;
		
		for( i = 0; i < nids; i++) {
			if(ws_list[i] == ws_id) {
				result = True;
				break;
			}
		}
		free(ws_list);
	}
	return result;
}

/* Frees internal fields of WsmWorkspaceInfo  */
void WsmFreeWorkspaceInfo(struct WsmWorkspaceInfo *wsi)
{
	free(wsi->title);
	if(wsi->bd_image) free(wsi->bd_image);
}

/* Send a request to the window manager */
Bool WsmSendRequest(Display *dpy, Window mwm_wswnd,	const char *req)
{
	static Atom xa_wmrequest = None;

	if(xa_wmrequest == None) {
		xa_wmrequest = XInternAtom(dpy, _XA_MWM_WM_REQUEST, True);
		if(!xa_wmrequest) return False;
	}
	#ifdef WSMUTIL_DEBUG
	printf("Request: \'%s\'\n", req);
	#endif
	/* append a 0 terminated string */
	XChangeProperty(dpy, mwm_wswnd, xa_wmrequest, XA_STRING,
		8, PropModeAppend, (unsigned char*)req, strlen(req) + 1);

	return True;
}

/* Sends a Change Backdrop request to the window manager */
Bool WsmChangeBackdrop(Display *dpy, Window mwm_wswnd,
	Atom workspace, unsigned long background,
	unsigned long foreground, const char *file_name)
{
	Bool result;
	size_t data_len;
	char *data;

	data_len = snprintf(NULL, 0, "%s 0x%ld 0x%lx 0x%lx \"%s\"",
		REQ_BACKDROP, workspace, foreground, background, file_name);

	data = malloc(data_len + 1);
	if(!data) return False;
	
	sprintf(data, "%s 0x%lx 0x%lx 0x%lx \"%s\"",
		REQ_BACKDROP, workspace,
		foreground, background, file_name);

	result = WsmSendRequest(dpy, mwm_wswnd, data);
	
	free(data);
	
	return result;
}

/* Sends a Change Workspace request to the window manager */
Bool WsmChangeWorkspace(Display *dpy, Window mwm_wswnd, Atom xa_wsid)
{
	Bool result;
	size_t data_len;
	char *data;

	data_len = snprintf(NULL, 0, "%s 0x%lx", REQ_WRKSPACE, xa_wsid);

	data = malloc(data_len + 1);
	if(!data) return False;
	
	sprintf(data, "%s 0x%lx", REQ_WRKSPACE, xa_wsid);

	result = WsmSendRequest(dpy, mwm_wswnd, data);
	
	free(data);
	
	return result;	
}

/* Sends a Name Workspace request to the window manager */
Bool WsmNameWorkspace(Display *dpy,
	Window mwm_wswnd, Atom xa_wsid, const char *title)
{
	Bool result;
	size_t data_len;
	char *data;

	data_len = snprintf(NULL, 0,
		"%s 0x%lx \"%s\"", REQ_NAMWKSPC, xa_wsid, title);

	data = malloc(data_len + 1);
	if(!data) return False;
	
	sprintf(data, "%s 0x%lx \"%s\"", REQ_NAMWKSPC, xa_wsid, title);

	result = WsmSendRequest(dpy, mwm_wswnd, data);
	
	free(data);
	
	return result;	
}

/* Sends an Add Workspace request to the window manager */
Bool WsmAddWorkspace(Display *dpy, Window mwm_wswnd,  const char *title)
{
	Bool result;
	size_t data_len;
	char *data;

	if(title) {
		data_len = snprintf(NULL, 0,"%s \"%s\"", REQ_ADDWKSPC, title);
		data = malloc(data_len + 1);
		if(!data) return False;

		sprintf(data,"%s \"%s\"", REQ_ADDWKSPC, title);
	} else {
		data = strdup(REQ_ADDWKSPC);
	}

	result = WsmSendRequest(dpy, mwm_wswnd, data);
	
	free(data);
	
	return result;	
}


/* Sends a Delete Workspace request to the window manager */
Bool WsmDeleteWorkspace(Display *dpy, Window mwm_wswnd, Atom xa_wsid)
{
	Bool result;
	size_t data_len;
	char *data;

	data_len = snprintf(NULL, 0, "%s 0x%lx", REQ_DELWKSPC, xa_wsid);

	data = malloc(data_len + 1);
	if(!data) return False;
	
	sprintf(data, "%s 0x%lx", REQ_DELWKSPC, xa_wsid);

	result = WsmSendRequest(dpy, mwm_wswnd, data);
	
	free(data);
	
	return result;	
}
