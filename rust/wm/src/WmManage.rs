/*
 * Motif
 *
 * Copyright (c) 1987-2012, The Open Group. All rights reserved.
 * Copyright (c) 2018-2026, alx@fastestcode.org
 *
 * These libraries and programs are free software; you can
 * redistribute them and/or modify them under the terms of the GNU
 * Lesser General Public License as published by the Free Software
 * Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * These libraries and programs are distributed in the hope that
 * they will be useful, but WITHOUT ANY WARRANTY; without even the
 * implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 * PURPOSE. See the GNU Lesser General Public License for more
 * details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with these librararies and programs; if not, write
 * to the Free Software Foundation, Inc., 51 Franklin Street, Fifth
 * Floor, Boston, MA 02110-1301 USA
*/
use crate::abi::_ArgbFrame_WmArgb as _ArgbFrame;
use crate::abi::_ButtonSpec_WmArgb as _ButtonSpec;
use crate::abi::_ClickData_WmArgb as _ClickData;
use crate::abi::_ClientData_WmArgb as _ClientData;
use crate::abi::_ClientListEntry_WmArgb as _ClientListEntry;
use crate::abi::_IconBoxData_WmArgb as _IconBoxData;
use crate::abi::_IconInfo_WmArgb as _IconInfo;
use crate::abi::_IconPlacementData_WmArgb as _IconPlacementData;
use crate::abi::_KeySpec_WmArgb as _KeySpec;
use crate::abi::_MenuButton_WmArgb as _MenuButton;
use crate::abi::_MenuItem_WmArgb as _MenuItem;
use crate::abi::_MenuSpec_WmArgb as _MenuSpec;
use crate::abi::_ObjectPart_WmEvent as _ObjectPart;
use crate::abi::_ObjectRec_WmEvent as _ObjectRec;
use crate::abi::_WmGlobalData_WmArgb as _WmGlobalData;
use crate::abi::_WmScreenData_WmArgb as _WmScreenData;
use crate::abi::_WmTimer_WmArgb as _WmTimer;
use crate::abi::_WmWorkspaceData_WmArgb as _WmWorkspaceData;
use crate::abi::_WsClientData_WmArgb as _WsClientData;
use crate::abi::_WsPresenceData_WmArgb as _WsPresenceData;
use crate::abi::ButtonSpec_WmArgb as ButtonSpec;
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_0;
use crate::abi::ClickData_WmArgb as ClickData;
use crate::abi::ClientData_WmArgb as ClientData;
use crate::abi::ClientListEntry_WmArgb as ClientListEntry;
use crate::abi::IconBoxData_WmArgb as IconBoxData;
use crate::abi::IconInfo_WmArgb as IconInfo;
use crate::abi::IconPlacementData_WmArgb as IconPlacementData;
use crate::abi::KeySpec_WmArgb as KeySpec;
use crate::abi::MenuButton_WmArgb as MenuButton;
use crate::abi::MenuItem_WmArgb as MenuItem;
use crate::abi::MenuSpec_WmArgb as MenuSpec;
use crate::abi::ObjectPart_WmEvent as ObjectPart;
use crate::abi::Object_WmEvent as Object;
use crate::abi::WmFunction_WmArgb as WmFunction;
use crate::abi::WmGlobalData_WmArgb as WmGlobalData;
use crate::abi::WmScreenData_WmArgb as WmScreenData;
use crate::abi::WmTimer_WmArgb as WmTimer;
use crate::abi::WmWorkspaceData_WmManage as WmWorkspaceData;
use crate::abi::WsClientData_WmManage as WsClientData;
use crate::abi::WsPresenceData_WmArgb as WsPresenceData;
use crate::abi::*;
extern "C" {
    fn XCheckTypedWindowEvent(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: *mut XEvent,
    ) -> ::core::ffi::c_int;
    fn XConfigureWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_uint,
        _: *mut XWindowChanges,
    ) -> ::core::ffi::c_int;
    fn XDestroyWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XFlush(_: *mut Display) -> ::core::ffi::c_int;
    fn XFree(_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> ::core::ffi::c_int;
    fn XGrabButton(
        _: *mut Display,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: Window,
        _: Cursor,
    ) -> ::core::ffi::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XPutBackEvent(_: *mut Display, _: *mut XEvent) -> ::core::ffi::c_int;
    fn XQueryTree(
        _: *mut Display,
        _: Window,
        _: *mut Window,
        _: *mut Window,
        _: *mut *mut Window,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XRemoveFromSaveSet(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XReparentWindow(
        _: *mut Display,
        _: Window,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XSync(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XUnmapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XDeleteContext(_: *mut Display, _: XID, _: XContext) -> ::core::ffi::c_int;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
    fn XGetWMHints(_: *mut Display, _: Window) -> *mut XWMHints;
    fn XSaveContext(
        _: *mut Display,
        _: XID,
        _: XContext,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn XtManageChild(_: Widget);
    fn XtUnmanageChild(_: Widget);
    fn XtWindowOfObject(_: Widget) -> Window;
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    fn XmStringFree(string: XmString);
    static mut wmGD: WmGlobalData;
    fn RestackDesktopSurfaces(screen: *mut WmScreenData);
    fn PublishCompositorFrame(client: *mut ClientData);
    fn FrameWindow(pcd: *mut ClientData) -> Boolean;
    fn SetupCButtonBindings(window: Window, buttonSpecs: *mut ButtonSpec);
    fn SetColormapFocus(pSD: *mut WmScreenData, pCD: *mut ClientData);
    fn AddColormapWindowReference(pCD: *mut ClientData, window: Window);
    fn RemoveColormapWindowReference(pCD: *mut ClientData, window: Window);
    fn Warning(message: *mut ::core::ffi::c_char);
    fn GetTimestamp() -> Time;
    fn SetupKeyBindings(
        keySpecs: *mut KeySpec,
        grabWindow: Window,
        keyboardMode: ::core::ffi::c_int,
        context: ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn Do_Focus_Key(pCD: *mut ClientData, focusTime: Time, flags: ::core::ffi::c_long);
    fn ClearDirtyStackEntry(pCD: *mut ClientData);
    fn FreeRList(prl: *mut RList);
    fn MakeIcon(pWS: *mut WmWorkspaceData, pcd: *mut ClientData) -> Boolean;
    fn DeleteIconFromBox(pIBD: *mut IconBoxData, pCD: *mut ClientData);
    fn AutoResetKeyFocus(pcdNoFocus: *mut ClientData, focusTime: Time);
    fn DoExplicitSelectGrab(window: Window);
    fn RepairFocus();
    fn SetKeyboardFocus(pCD: *mut ClientData, focusFlags: ::core::ffi::c_long);
    fn FreeArgbFrame(pcd: *mut ClientData);
    fn FreeCustomMenuSpec(menuSpec: *mut MenuSpec);
    fn GetWMState(window: Window) -> *mut PropWMState;
    fn SetWMState(window: Window, state: ::core::ffi::c_int, icon: Window);
    fn DiscardInitialPropertyList(pCD: *mut ClientData);
    fn SendConfigureNotify(pCD: *mut ClientData);
    fn SendClientOffsetMessage(pCD: *mut ClientData);
    fn DeleteClientWmTimers(pCD: *mut ClientData);
    fn GetClientInfo(
        pSD: *mut WmScreenData,
        clientWindow: Window,
        manageFlags: ::core::ffi::c_long,
    ) -> *mut ClientData;
    fn CalculateGravityOffset(
        pCD: *mut ClientData,
        xoff: *mut ::core::ffi::c_int,
        yoff: *mut ::core::ffi::c_int,
    );
    fn InitClientPlacement(pCD: *mut ClientData, manageFlags: ::core::ffi::c_long) -> Boolean;
    fn WmGetWindowAttributes(window: Window) -> Boolean;
    fn AddClientToList(pWS: *mut WmWorkspaceData, pCD: *mut ClientData, onTop: Boolean);
    fn DeleteClientFromList(pWS: *mut WmWorkspaceData, pCD: *mut ClientData);
    fn DeleteTransient(pCD: *mut ClientData);
    fn FindTransientTreeLeader(pcd: *mut ClientData) -> *mut ClientData;
    fn StackWindow(
        pWS: *mut WmWorkspaceData,
        pEntry: *mut ClientListEntry,
        onTop: Boolean,
        pStackEntry: *mut ClientListEntry,
    );
    fn SetClientState(pCD: *mut ClientData, newState: ::core::ffi::c_int, setTime: Time);
    fn HidePresenceBox(pSD: *mut WmScreenData, userDismissed: Boolean);
    fn UpdateWorkspacePresenceProperty(pCD: *mut ClientData);
    fn TakeClientOutOfWorkspace(pWS: *mut WmWorkspaceData, pCD: *mut ClientData);
    fn GetWorkspaceData(pSD: *mut WmScreenData, wsID: WorkspaceID) -> *mut WmWorkspaceData;
    fn InWindowList(w: Window, wl: *mut Window, num: ::core::ffi::c_int) -> Boolean;
    fn ClientInWorkspace(pWS: *mut WmWorkspaceData, pCD: *mut ClientData) -> Boolean;
    fn SetClientWsIndex(pCD: *mut ClientData);
    fn F_AddToAllWorkspaces(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn ExitWM(exitCode: ::core::ffi::c_int) -> !;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const AnyButton: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ButtonPressMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ButtonReleaseMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const ButtonMotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 13 as ::core::ffi::c_int;
pub const DestroyNotify: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const AnyModifier: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int;
pub const GrabModeSync: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GrabModeAsync: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CWBorderWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const IsUnmapped: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const IconWindowHint: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const NormalState: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IconicState: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_FUNC_MINIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const MWM_INPUT_PRIMARY_APPLICATION_MODAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_INPUT_SYSTEM_MODAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MWM_INPUT_FULL_APPLICATION_MODAL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XmUNSPECIFIED_PIXMAP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WM_ERROR_EXIT_VALUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MANAGEW_WM_STARTUP: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const MANAGEW_WM_RESTART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const MANAGEW_ICON_BOX: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const MANAGEW_CONFIRM_BOX: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const MANAGEW_WM_RESTART_ICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const MANAGEW_WM_CLIENTS: ::core::ffi::c_long = MANAGEW_ICON_BOX | MANAGEW_CONFIRM_BOX;
pub const ALWAYS_SET_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const WORKSPACE_IF_NULL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const WSM_FUNCTION_OCCUPY_WS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int;
pub const F_CONTEXT_ROOT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const F_CONTEXT_ICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const F_CONTEXT_NORMAL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const F_CONTEXT_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const F_CONTEXT_WINDOW: ::core::ffi::c_long = F_CONTEXT_NORMAL | F_CONTEXT_MAXIMIZE;
pub const F_CONTEXT_ALL: ::core::ffi::c_long = F_CONTEXT_ROOT | F_CONTEXT_ICON | F_CONTEXT_WINDOW;
pub const WITHDRAWN_STATE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAXIMIZED_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const UNSEEN_STATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CLIENT_HINTS_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const CLIENT_REPARENTED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const CLIENT_CONTEXT_SAVED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const CLIENT_IN_SAVE_SET: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const WM_INITIALIZATION: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const CLIENT_DESTROYED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
pub const ICON_REPARENTED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int;
pub const ICON_IN_SAVE_SET: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const ICON_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CONFIRM_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 12 as ::core::ffi::c_int;
pub const CLIENT_WM_CLIENTS: ::core::ffi::c_long = ICON_BOX | CONFIRM_BOX;
pub const ICON_HINTS_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const PROTOCOL_WM_TAKE_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const PROTOCOL_MWM_OFFSET: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NO_ICON_PLACE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const WithdrawnSTATE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn AdoptInitialClients(mut pSD: *mut WmScreenData) {
    let mut root: Window = 0;
    let mut parent: Window = 0;
    let mut clients = ::core::ptr::null_mut::<Window>();
    let mut nAncillaries: ::core::ffi::c_int = 0;
    let mut iAnc: ::core::ffi::c_int = 0;
    let mut pAncillaryWindows = ::core::ptr::null_mut::<Window>();
    let mut pWin1 = ::core::ptr::null_mut::<Window>();
    let mut pWS0 = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut nclients: ::core::ffi::c_uint = 0;
    let mut pcd = ::core::ptr::null_mut::<ClientData>();
    let mut wmStateProp = ::core::ptr::null_mut::<PropWMState>();
    let mut manageOnRestart: Boolean = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut manageFlags: ::core::ffi::c_long = 0;
    nAncillaries = 2 as ::core::ffi::c_int + (*pSD).numWorkspaces;
    pAncillaryWindows = XtMalloc(
        (::core::mem::size_of::<Window>() as usize).wrapping_mul(nAncillaries as usize) as Cardinal,
    ) as *mut Window;
    if pAncillaryWindows.is_null() {
        Warning(
            b"Insufficient memory to adopt initial clients\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        ExitWM(WM_ERROR_EXIT_VALUE);
    }
    pWS0 = (*pSD).pWS as *mut WmWorkspaceData;
    pWin1 = pAncillaryWindows;
    iAnc = 0 as ::core::ffi::c_int;
    while iAnc < (*pSD).numWorkspaces {
        *pWin1 = XtWindowOfObject((*pWS0).workspaceTopLevelW);
        pWin1 = pWin1.offset(1);
        pWS0 = pWS0.offset(1);
        iAnc += 1;
    }
    let fresh0 = pWin1;
    pWin1 = pWin1.offset(1);
    *fresh0 = XtWindowOfObject((*pSD).screenTopLevelW);
    *pWin1 = (*pSD).activeIconTextWin;
    if XQueryTree(
        wmGD.display,
        (*pSD).rootWindow,
        &raw mut root,
        &raw mut parent,
        &raw mut clients,
        &raw mut nclients,
    ) != 0
    {
        let mut tmphint = ::core::ptr::null_mut::<XWMHints>();
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint) < nclients {
            if *clients.offset(i as isize) != 0 {
                tmphint = XGetWMHints(wmGD.display, *clients.offset(i as isize));
                if !tmphint.is_null() {
                    if (*tmphint).flags & IconWindowHint != 0 {
                        j = 0 as ::core::ffi::c_int;
                        while (j as ::core::ffi::c_uint) < nclients {
                            if *clients.offset(j as isize) == (*tmphint).icon_window {
                                *clients.offset(j as isize) = None_0 as Window;
                                break;
                            } else {
                                j += 1;
                            }
                        }
                    }
                    XFree(tmphint as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
                }
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint) < nclients {
            if !(InWindowList(
                *clients.offset(i as isize),
                pAncillaryWindows as *mut Window,
                nAncillaries,
            ) != 0)
            {
                if !(XFindContext(
                    wmGD.display,
                    *clients.offset(i as isize) as XID,
                    wmGD.windowContextType,
                    &raw mut pcd as *mut XPointer,
                ) == 0)
                {
                    if !(WmGetWindowAttributes(*clients.offset(i as isize)) == 0) {
                        manageFlags = MANAGEW_WM_STARTUP;
                        manageOnRestart = True as Boolean;
                        if wmGD.wmRestarted != 0 {
                            manageFlags |= MANAGEW_WM_RESTART;
                            wmStateProp = GetWMState(*clients.offset(i as isize));
                            if !wmStateProp.is_null() {
                                if (*wmStateProp).state == IconicState as ::core::ffi::c_ulong {
                                    manageFlags |= MANAGEW_WM_RESTART_ICON;
                                } else if (*wmStateProp).state
                                    != NormalState as ::core::ffi::c_ulong
                                {
                                    manageOnRestart = False as Boolean;
                                }
                                XFree(
                                    wmStateProp as *mut ::core::ffi::c_char
                                        as *mut ::core::ffi::c_void,
                                );
                            } else {
                                manageOnRestart = False as Boolean;
                            }
                        }
                        if wmGD.windowAttributes.override_redirect != True
                            && (wmGD.wmRestarted as ::core::ffi::c_int != 0
                                && manageOnRestart as ::core::ffi::c_int != 0
                                || wmGD.windowAttributes.map_state != IsUnmapped)
                        {
                            ManageWindow(pSD, *clients.offset(i as isize), manageFlags);
                        }
                    }
                }
            }
            i += 1;
        }
        if nclients != 0 {
            XFree(clients as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        }
    }
    if !pAncillaryWindows.is_null() {
        XtFree(pAncillaryWindows as *mut ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ManageWindow(
    mut pSD: *mut WmScreenData,
    mut clientWindow: Window,
    mut manageFlags: ::core::ffi::c_long,
) {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut initialState: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut sendConfigNotify: Boolean = 0;
    let mut pwsi = ::core::ptr::null_mut::<WmWorkspaceData>();
    pCD = GetClientInfo(pSD, clientWindow, manageFlags);
    if pCD.is_null() {
        return;
    }
    if (*pCD).inputMode == MWM_INPUT_SYSTEM_MODAL {
        (*pCD).wsmFunctions |= WSM_FUNCTION_OCCUPY_WS;
        F_AddToAllWorkspaces(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pCD,
            ::core::ptr::null_mut::<XEvent>(),
        );
        (*pCD).wsmFunctions &= !WSM_FUNCTION_OCCUPY_WS;
    }
    if manageFlags & MANAGEW_WM_RESTART != 0 {
        if manageFlags & MANAGEW_WM_RESTART_ICON != 0 {
            (*pCD).clientState = MINIMIZED_STATE;
        } else {
            (*pCD).clientState = NORMAL_STATE;
        }
    }
    sendConfigNotify = InitClientPlacement(pCD, manageFlags);
    if FrameWindow(pCD) == 0 {
        UnManageWindow(pCD);
        return;
    }
    if sendConfigNotify != 0 {
        SendConfigureNotify(pCD);
    }
    if (*pCD).protocolFlags & PROTOCOL_MWM_OFFSET != 0
        && ((*pCD).clientOffset.x as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || (*pCD).clientOffset.y as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
    {
        SendClientOffsetMessage(pCD);
    }
    if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0
        && (*pCD).transientLeader.is_null()
    {
        i = 0 as ::core::ffi::c_int;
        while i < (*pCD).numInhabited {
            pwsi = GetWorkspaceData((*pCD).pSD, (*(*pCD).pWsList.offset(i as isize)).wsID);
            if !pwsi.is_null() {
                if (*(*pCD).pSD).useIconBox as ::core::ffi::c_int != 0
                    && manageFlags & MANAGEW_WM_CLIENTS == 0
                    || i == 0 as ::core::ffi::c_int
                {
                    if MakeIcon(pwsi, pCD) == 0 {
                        UnManageWindow(pCD);
                        return;
                    } else {
                        XSaveContext(
                            wmGD.display,
                            (*(*pCD).pWsList.offset(i as isize)).iconFrameWin as XID,
                            wmGD.windowContextType,
                            pCD as XPointer as *const ::core::ffi::c_char,
                        );
                        if (*pCD).iconWindow != 0
                            && (*(*pCD).pWsList.offset(i as isize)).iconFrameWin != 0
                        {
                            XGrabButton(
                                wmGD.display,
                                AnyButton as ::core::ffi::c_uint,
                                AnyModifier as ::core::ffi::c_uint,
                                (*(*pCD).pWsList.offset(i as isize)).iconFrameWin,
                                True,
                                (ButtonPressMask | ButtonReleaseMask | ButtonMotionMask)
                                    as ::core::ffi::c_uint,
                                GrabModeAsync,
                                GrabModeAsync,
                                None_0 as Window,
                                wmGD.workspaceCursor,
                            );
                        }
                    }
                } else if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0
                    && (*pCD).transientLeader.is_null()
                {
                    if i == 0 as ::core::ffi::c_int && MakeIcon(pwsi, pCD) == 0 {
                        UnManageWindow(pCD);
                        return;
                    } else {
                        (*(*pCD).pWsList.offset(i as isize)).iconFrameWin =
                            (*(*pCD).pWsList.offset(0 as ::core::ffi::c_int as isize)).iconFrameWin;
                    }
                }
            }
            i += 1;
        }
    }
    XSaveContext(
        wmGD.display,
        (*pCD).clientFrameWin as XID,
        wmGD.windowContextType,
        pCD as XPointer as *const ::core::ffi::c_char,
    );
    XSaveContext(
        wmGD.display,
        (*pCD).clientBaseWin as XID,
        wmGD.windowContextType,
        pCD as XPointer as *const ::core::ffi::c_char,
    );
    if (*(*pCD).pSD).decoupleTitleAppearance as ::core::ffi::c_int != 0
        && (*pCD).clientTitleWin != 0
    {
        XSaveContext(
            wmGD.display,
            (*pCD).clientTitleWin as XID,
            wmGD.windowContextType,
            pCD as XPointer as *const ::core::ffi::c_char,
        );
    }
    if (*pCD).clientCmapCount > 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < (*pCD).clientCmapCount {
            if *(*pCD).cmapWindows.offset(i as isize) != (*pCD).client {
                AddColormapWindowReference(pCD, *(*pCD).cmapWindows.offset(i as isize));
            }
            i += 1;
        }
    }
    (*pCD).clientFlags |= CLIENT_CONTEXT_SAVED;
    if !(*(*pCD).pSD).buttonSpecs.is_null() {
        SetupCButtonBindings((*pCD).clientBaseWin, (*(*pCD).pSD).buttonSpecs);
    }
    if !(*pCD).systemMenuSpec.is_null() && !(*(*pCD).systemMenuSpec).accelKeySpecs.is_null() {
        SetupKeyBindings(
            (*(*pCD).systemMenuSpec).accelKeySpecs,
            (*pCD).clientFrameWin,
            GrabModeSync,
            F_CONTEXT_ALL,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*pCD).numInhabited {
            if (*(*pCD).pWsList.offset(i as isize)).pIconBox.is_null()
                && (*(*pCD).pWsList.offset(i as isize)).iconFrameWin != 0
            {
                SetupKeyBindings(
                    (*(*pCD).systemMenuSpec).accelKeySpecs,
                    (*(*pCD).pWsList.offset(i as isize)).iconFrameWin,
                    GrabModeSync,
                    F_CONTEXT_ALL,
                );
            }
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*pCD).numInhabited {
        if (*(*pCD).pWsList.offset(i as isize)).pIconBox.is_null()
            && (*(*pCD).pWsList.offset(i as isize)).iconFrameWin != 0
        {
            static mut iconKeySpec: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            static mut iconAccelSpec: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            if iconKeySpec != 0 as ::core::ffi::c_int && !(*(*pCD).pSD).keySpecs.is_null() {
                iconKeySpec = SetupKeyBindings(
                    (*(*pCD).pSD).keySpecs,
                    (*(*pCD).pWsList.offset(i as isize)).iconFrameWin,
                    GrabModeSync,
                    F_CONTEXT_ICON,
                );
            }
            if iconAccelSpec != 0 as ::core::ffi::c_int && (*(*pCD).pSD).acceleratorMenuCount != 0 {
                let mut n: ::core::ffi::c_int = 0;
                iconAccelSpec = 0 as ::core::ffi::c_int;
                n = 0 as ::core::ffi::c_int;
                while (n as ::core::ffi::c_uint) < (*pSD).acceleratorMenuCount {
                    iconAccelSpec += SetupKeyBindings(
                        (**(*(*pCD).pSD).acceleratorMenuSpecs.offset(n as isize)).accelKeySpecs,
                        (*(*pCD).pWsList.offset(i as isize)).iconFrameWin,
                        GrabModeSync,
                        F_CONTEXT_ICON,
                    );
                    n += 1;
                }
            }
        }
        i += 1;
    }
    if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
        DoExplicitSelectGrab((*pCD).clientBaseWin);
    }
    UpdateWorkspacePresenceProperty(pCD);
    if manageFlags & MANAGEW_WM_CLIENTS == 0 {
        XSync(wmGD.display, False);
        if (*pCD).clientFlags & CLIENT_DESTROYED != 0 {
            UnManageWindow(pCD);
            return;
        }
    }
    initialState = (*pCD).clientState;
    if ClientInWorkspace((*pSD).pActiveWS as *mut WmWorkspaceData, pCD) == 0 {
        initialState |= UNSEEN_STATE;
    }
    (*pCD).clientState = WITHDRAWN_STATE;
    (*pCD).clientFlags &= !WM_INITIALIZATION;
    PublishCompositorFrame(pCD);
    AddClientToList(
        GetWorkspaceData(
            pSD,
            (*(*pCD).pWsList.offset(0 as ::core::ffi::c_int as isize)).wsID,
        ),
        pCD,
        True as Boolean,
    );
    SetClientState(pCD, initialState, GetTimestamp());
    RestackDesktopSurfaces(pSD);
    if (*pCD).desktopSurface == 0
        && wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT
        && ((*pCD).inputMode == MWM_INPUT_SYSTEM_MODAL
            || (wmGD.systemModalActive == 0
                || wmGD.systemModalClient == FindTransientTreeLeader(pCD))
                && (wmGD.startupKeyFocus as ::core::ffi::c_int != 0
                    || !wmGD.keyboardFocus.is_null()
                        && ((*wmGD.keyboardFocus).primaryModalCount != 0
                            || (*wmGD.keyboardFocus).fullModalCount != 0))
                && manageFlags & (MANAGEW_WM_STARTUP | MANAGEW_WM_RESTART | MANAGEW_WM_CLIENTS)
                    == 0
                && (*pCD).clientState != MINIMIZED_STATE
                && (*pCD).clientState & UNSEEN_STATE == 0
                && ((*pCD).inputFocusModel != 0
                    || (*pCD).protocolFlags & PROTOCOL_WM_TAKE_FOCUS != 0))
    {
        Do_Focus_Key(pCD, GetTimestamp(), ALWAYS_SET_FOCUS);
    } else if (*pCD).inputMode == MWM_INPUT_SYSTEM_MODAL
        || !wmGD.keyboardFocus.is_null()
            && ((*wmGD.keyboardFocus).primaryModalCount != 0
                || (*wmGD.keyboardFocus).fullModalCount != 0)
    {
        Do_Focus_Key(NULL as *mut ClientData, GetTimestamp(), ALWAYS_SET_FOCUS);
    }
    DiscardInitialPropertyList(pCD);
}
#[no_mangle]
pub unsafe extern "C" fn UnManageWindow(mut pCD: *mut ClientData) {
    if !(*pCD).transientChildren.is_null() {
        WithdrawTransientChildren(pCD);
    }
    if !(*pCD).transientLeader.is_null() {
        DeleteTransient(pCD);
        if wmGD.replayEnterEvent != 0 {
            XPutBackEvent(wmGD.display, &raw mut wmGD.savedEnterEvent as *mut XEvent);
            wmGD.replayEnterEvent = False;
        }
    }
    WithdrawWindow(pCD);
}
#[no_mangle]
pub unsafe extern "C" fn WithdrawTransientChildren(mut pCD: *mut ClientData) {
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    let mut pcdThis = ::core::ptr::null_mut::<ClientData>();
    pcdNext = (*pCD).transientChildren as *mut ClientData;
    while !pcdNext.is_null() {
        if !(*pcdNext).transientChildren.is_null() {
            WithdrawTransientChildren(pcdNext);
        }
        pcdThis = pcdNext;
        pcdNext = (*pcdThis).transientSiblings as *mut ClientData;
        DeleteTransient(pcdThis);
        WithdrawWindow(pcdThis);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WithdrawWindow(mut pCD: *mut ClientData) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut xwc = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    SetClientWsIndex(pCD);
    if (*pCD).clientFlags & WM_INITIALIZATION == 0 {
        if (*pCD).transientLeader.is_null() {
            DeleteClientFromList((*(*pCD).pSD).pActiveWS as *mut WmWorkspaceData, pCD);
        }
        ResetWithdrawnFocii(pCD);
        if (*pCD).clientState & MINIMIZED_STATE != 0 {
            if wmGD.iconAutoPlace as ::core::ffi::c_int != 0
                && (*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
                    .pIconBox
                    .is_null()
            {
                let mut pWsc = ::core::ptr::null_mut::<WsClientData>();
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < (*pCD).numInhabited {
                    pWsc = (*pCD).pWsList.offset(j as isize) as *mut _WsClientData
                        as *mut WsClientData;
                    if (*pWsc).iconPlace != NO_ICON_PLACE {
                        let ref mut fresh1 = (*(*(*pWsc).IPData)
                            .placeList
                            .offset((*pWsc).iconPlace as isize))
                        .pCD;
                        *fresh1 = ::core::ptr::null_mut::<_ClientData>();
                    }
                    j += 1;
                }
            }
            if (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin != 0 {
                XUnmapWindow(
                    wmGD.display,
                    (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
                );
            }
            XFlush(wmGD.display);
        } else if (*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE {
            XUnmapWindow(wmGD.display, (*pCD).clientFrameWin);
            XFlush(wmGD.display);
        }
    }
    if !(*(*pCD).pSD).presence.shellW.is_null() && (*(*pCD).pSD).presence.pCDforClient == pCD {
        if (*(*pCD).pSD).presence.onScreen != 0 {
            HidePresenceBox((*pCD).pSD, True as Boolean);
        }
        (*(*pCD).pSD).presence.pCDforClient = ::core::ptr::null_mut::<_ClientData>();
    }
    if (*pCD).clientFlags & CLIENT_DESTROYED == 0 {
        let mut eventReturn = _XEvent { type_0: 0 };
        if XCheckTypedWindowEvent(
            wmGD.display,
            (*pCD).clientBaseWin,
            DestroyNotify,
            &raw mut eventReturn,
        ) != 0
        {
            (*pCD).clientFlags |= CLIENT_DESTROYED;
        }
    }
    if (*pCD).clientFlags & CLIENT_REPARENTED != 0 && (*pCD).clientFlags & CLIENT_DESTROYED == 0 {
        SetWMState(
            (*pCD).client,
            WithdrawnSTATE,
            (*(*pCD).pWsList.offset(0 as ::core::ffi::c_int as isize)).iconFrameWin,
        );
        if (*pCD).maxConfig != 0 {
            x = (*pCD).maxX;
            y = (*pCD).maxY;
        } else {
            let mut xoff: ::core::ffi::c_int = 0;
            let mut yoff: ::core::ffi::c_int = 0;
            CalculateGravityOffset(pCD, &raw mut xoff, &raw mut yoff);
            x = (*pCD).clientX - xoff;
            y = (*pCD).clientY - yoff;
        }
        XUnmapWindow(wmGD.display, (*pCD).client);
        XReparentWindow(wmGD.display, (*pCD).client, (*(*pCD).pSD).rootWindow, x, y);
        xwc.border_width = (*pCD).xBorderWidth;
        XConfigureWindow(
            wmGD.display,
            (*pCD).client,
            CWBorderWidth as ::core::ffi::c_uint,
            &raw mut xwc,
        );
        if (*pCD).iconWindow != 0 && (*pCD).clientFlags & ICON_REPARENTED != 0 {
            XUnmapWindow(wmGD.display, (*pCD).iconWindow);
            XReparentWindow(
                wmGD.display,
                (*pCD).iconWindow,
                (*(*pCD).pSD).rootWindow,
                (*(*pCD).pWsList.offset(0 as ::core::ffi::c_int as isize)).iconX,
                (*(*pCD).pWsList.offset(0 as ::core::ffi::c_int as isize)).iconY,
            );
        }
    }
    if (*pCD).clientFlags & CLIENT_IN_SAVE_SET != 0 && (*pCD).clientFlags & CLIENT_DESTROYED == 0 {
        XRemoveFromSaveSet(wmGD.display, (*pCD).client);
        if (*pCD).iconWindow != 0 && (*pCD).clientFlags & ICON_IN_SAVE_SET != 0 {
            XRemoveFromSaveSet(wmGD.display, (*pCD).iconWindow);
        }
    }
    FreeCustomMenuSpec((*pCD).systemMenuSpec);
    if (*pCD).clientFrameWin != 0 {
        FreeClientFrame(pCD);
    }
    if (*pCD).iconPixmap != XmUNSPECIFIED_PIXMAP as Pixmap
        && (*pCD).iconPixmap != None_0 as Pixmap
        && (*pCD).iconPixmap != NULL as Pixmap
    {
        XFreePixmap(wmGD.display, (*pCD).iconPixmap);
    }
    if (*pCD).numInhabited > 0 as ::core::ffi::c_int
        && (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin != 0
    {
        FreeIcon(pCD);
    }
    if !(*pCD).clientProtocols.is_null() {
        XtFree((*pCD).clientProtocols as *mut ::core::ffi::c_char);
    }
    if !(*pCD).mwmMessages.is_null() {
        XtFree((*pCD).mwmMessages as *mut ::core::ffi::c_char);
    }
    DeleteClientWmTimers(pCD);
    DeleteClientContext(pCD);
    i = (*pCD).numInhabited - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut wsData = GetWorkspaceData((*pCD).pSD, (*(*pCD).pWsList.offset(i as isize)).wsID);
        if (*wsData).keyFocus == pCD {
            (*wsData).keyFocus = ::core::ptr::null_mut::<_ClientData>();
        }
        if (*wsData).nextKeyFocus == pCD {
            (*wsData).nextKeyFocus = ::core::ptr::null_mut::<_ClientData>();
        }
        TakeClientOutOfWorkspace(wsData, pCD);
        i -= 1;
    }
    if (*pCD).clientFlags & CLIENT_WM_CLIENTS == 0 {
        if !(*pCD).clientName.is_null() {
            XFree((*pCD).clientName as *mut ::core::ffi::c_void);
        }
        if !(*pCD).clientClass.is_null() {
            XFree((*pCD).clientClass as *mut ::core::ffi::c_void);
        }
    }
    if (*pCD).clientFlags & CLIENT_HINTS_TITLE != 0 && !(*pCD).clientTitle.is_null() {
        XmStringFree((*pCD).clientTitle);
    }
    if (*pCD).iconFlags & ICON_HINTS_TITLE != 0 && !(*pCD).iconTitle.is_null() {
        XmStringFree((*pCD).iconTitle);
    }
    if (*pCD).clientCmapCount > 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < (*pCD).clientCmapCount {
            if *(*pCD).cmapWindows.offset(i as isize) != (*pCD).client {
                RemoveColormapWindowReference(pCD, *(*pCD).cmapWindows.offset(i as isize));
            }
            i += 1;
        }
        XtFree((*pCD).cmapWindows as *mut ::core::ffi::c_char);
        XtFree((*pCD).clientCmapList as *mut ::core::ffi::c_char);
        XtFree((*pCD).clientCmapFlags as *mut ::core::ffi::c_char);
    }
    DiscardInitialPropertyList(pCD);
    if !(*pCD).pWsList.is_null() {
        XtFree((*pCD).pWsList as *mut ::core::ffi::c_char);
    }
    if !(*pCD).pWorkspaceHints.is_null() {
        XtFree((*pCD).pWorkspaceHints as *mut ::core::ffi::c_char);
    }
    if !(*pCD).ewmhClientTitle.is_null() {
        XmStringFree((*pCD).ewmhClientTitle);
    }
    if !(*pCD).ewmhIconTitle.is_null() {
        XmStringFree((*pCD).ewmhIconTitle);
    }
    if (*pCD).ewmhIconPixmap != 0 {
        XFreePixmap(wmGD.display, (*pCD).ewmhIconPixmap);
    }
    if !(*pCD).smClientID.is_null() {
        XFree((*pCD).smClientID as *mut ::core::ffi::c_void);
    }
    if wmGD.menuClient == pCD {
        wmGD.menuClient = ::core::ptr::null_mut::<ClientData>();
    }
    if wmGD.gadgetClient == pCD {
        wmGD.gadgetClient = ::core::ptr::null_mut::<ClientData>();
        wmGD.gadgetDepressed = 0 as ::core::ffi::c_int;
    }
    if wmGD.clickData.pCD == pCD {
        wmGD.clickData.pCD = ::core::ptr::null_mut::<_ClientData>();
    }
    if wmGD.nextKeyboardFocus == pCD {
        wmGD.nextKeyboardFocus = ::core::ptr::null_mut::<ClientData>();
    }
    if wmGD.keyboardFocus == pCD {
        wmGD.keyboardFocus = ::core::ptr::null_mut::<ClientData>();
    }
    ClearDirtyStackEntry(pCD);
    XtFree(pCD as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn DeleteClientContext(mut pCD: *mut ClientData) {
    XDeleteContext(wmGD.display, (*pCD).client as XID, wmGD.windowContextType);
    if (*pCD).clientFlags & CLIENT_CONTEXT_SAVED != 0 {
        XDeleteContext(
            wmGD.display,
            (*pCD).clientFrameWin as XID,
            wmGD.windowContextType,
        );
        XDeleteContext(
            wmGD.display,
            (*pCD).clientBaseWin as XID,
            wmGD.windowContextType,
        );
        if (*(*pCD).pSD).decoupleTitleAppearance != 0 {
            XDeleteContext(
                wmGD.display,
                (*pCD).clientTitleWin as XID,
                wmGD.windowContextType,
            );
        }
        if (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin != 0 {
            let mut k: ::core::ffi::c_int = 0;
            k = 0 as ::core::ffi::c_int;
            while k < (*pCD).numInhabited {
                XDeleteContext(
                    wmGD.display,
                    (*(*pCD).pWsList.offset(k as isize)).iconFrameWin as XID,
                    wmGD.windowContextType,
                );
                k += 1;
            }
        }
        (*pCD).clientFlags &= !CLIENT_CONTEXT_SAVED;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ResetWithdrawnFocii(mut pCD: *mut ClientData) {
    if wmGD.keyboardFocus == pCD
        || wmGD.keyboardFocus.is_null()
            && wmGD.nextKeyboardFocus == pCD
            && wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT
    {
        if wmGD.autoKeyFocus as ::core::ffi::c_int != 0
            && wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT
        {
            if wmGD.nextKeyboardFocus == pCD {
                AutoResetKeyFocus(pCD, GetTimestamp());
            } else {
                Do_Focus_Key(wmGD.nextKeyboardFocus, GetTimestamp(), ALWAYS_SET_FOCUS);
            }
        } else if wmGD.nextKeyboardFocus == wmGD.keyboardFocus {
            Do_Focus_Key(
                NULL as *mut ClientData,
                GetTimestamp(),
                ALWAYS_SET_FOCUS | WORKSPACE_IF_NULL,
            );
        }
        SetKeyboardFocus(NULL as *mut ClientData, 0 as ::core::ffi::c_long);
    }
    if ((*pCD).inputMode == MWM_INPUT_PRIMARY_APPLICATION_MODAL
        || (*pCD).inputMode == MWM_INPUT_FULL_APPLICATION_MODAL)
        && wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER
    {
        RepairFocus();
    }
    if wmGD.nextKeyboardFocus == pCD {
        wmGD.nextKeyboardFocus = ::core::ptr::null_mut::<ClientData>();
    }
    if (*wmGD.pActiveSD).colormapFocus == pCD {
        SetColormapFocus(wmGD.pActiveSD, NULL as *mut ClientData);
    }
}
#[no_mangle]
pub unsafe extern "C" fn FreeClientFrame(mut pCD: *mut ClientData) {
    if !(*pCD).pclientTopShadows.is_null() {
        FreeRList((*pCD).pclientTopShadows);
        (*pCD).pclientTopShadows = ::core::ptr::null_mut::<RList>();
    }
    if !(*pCD).pclientBottomShadows.is_null() {
        FreeRList((*pCD).pclientBottomShadows);
        (*pCD).pclientBottomShadows = ::core::ptr::null_mut::<RList>();
    }
    if !(*pCD).pclientTitleTopShadows.is_null() {
        FreeRList((*pCD).pclientTitleTopShadows);
        (*pCD).pclientTitleTopShadows = ::core::ptr::null_mut::<RList>();
    }
    if !(*pCD).pclientTitleBottomShadows.is_null() {
        FreeRList((*pCD).pclientTitleBottomShadows);
        (*pCD).pclientTitleBottomShadows = ::core::ptr::null_mut::<RList>();
    }
    if !(*pCD).pclientMatteTopShadows.is_null() {
        FreeRList((*pCD).pclientMatteTopShadows);
        (*pCD).pclientMatteTopShadows = ::core::ptr::null_mut::<RList>();
    }
    if !(*pCD).pclientMatteBottomShadows.is_null() {
        FreeRList((*pCD).pclientMatteBottomShadows);
        (*pCD).pclientMatteBottomShadows = ::core::ptr::null_mut::<RList>();
    }
    if !(*pCD).pTitleGadgets.is_null() {
        XtFree((*pCD).pTitleGadgets as *mut ::core::ffi::c_char);
        (*pCD).pTitleGadgets = ::core::ptr::null_mut::<GadgetRectangle>();
        (*pCD).cTitleGadgets = 0 as ::core::ffi::c_int;
    }
    if !(*pCD).pResizeGadgets.is_null() {
        XtFree((*pCD).pResizeGadgets as *mut ::core::ffi::c_char);
        (*pCD).pResizeGadgets = ::core::ptr::null_mut::<GadgetRectangle>();
    }
    FreeArgbFrame(pCD);
    XDestroyWindow(wmGD.display, (*pCD).clientFrameWin);
}
#[no_mangle]
pub unsafe extern "C" fn FreeIcon(mut pCD: *mut ClientData) {
    let mut pWsTmp = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut i: ::core::ffi::c_int = 0;
    if !(*pCD).piconTopShadows.is_null() {
        FreeRList((*pCD).piconTopShadows);
        (*pCD).piconTopShadows = ::core::ptr::null_mut::<RList>();
    }
    if !(*pCD).piconBottomShadows.is_null() {
        FreeRList((*pCD).piconBottomShadows);
        (*pCD).piconBottomShadows = ::core::ptr::null_mut::<RList>();
    }
    if (*(*pCD).pSD).useIconBox as ::core::ffi::c_int != 0
        && !(*(*pCD).pWsList.offset(0 as ::core::ffi::c_int as isize))
            .pIconBox
            .is_null()
    {
        i = 0 as ::core::ffi::c_int;
        while i < (*pCD).numInhabited {
            pWsTmp = GetWorkspaceData((*pCD).pSD, (*(*pCD).pWsList.offset(i as isize)).wsID);
            if !pWsTmp.is_null() {
                DeleteIconFromBox((*pWsTmp).pIconBox, pCD);
            }
            i += 1;
        }
    } else if (*(*pCD).pWsList.offset(0 as ::core::ffi::c_int as isize)).iconFrameWin != 0 {
        XDestroyWindow(
            wmGD.display,
            (*(*pCD).pWsList.offset(0 as ::core::ffi::c_int as isize)).iconFrameWin,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn WithdrawDialog(mut dialogboxW: Widget) {
    let mut i: ::core::ffi::c_int = 0;
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    if XFindContext(
        wmGD.display,
        XtWindowOfObject((*(dialogboxW as Object)).object.parent) as XID,
        wmGD.windowContextType,
        &raw mut pCD as *mut XPointer,
    ) != 0
    {
        return;
    }
    XtUnmanageChild(dialogboxW);
    DeleteClientFromList((*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData, pCD);
    i = (*pCD).numInhabited - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        TakeClientOutOfWorkspace(
            GetWorkspaceData((*pCD).pSD, (*(*pCD).pWsList.offset(i as isize)).wsID),
            pCD,
        );
        i -= 1;
    }
    ResetWithdrawnFocii(pCD);
    XUnmapWindow(wmGD.display, (*pCD).clientFrameWin);
}
#[no_mangle]
pub unsafe extern "C" fn ReManageDialog(mut pSD: *mut WmScreenData, mut dialogboxW: Widget) {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    if XFindContext(
        wmGD.display,
        XtWindowOfObject((*(dialogboxW as Object)).object.parent) as XID,
        wmGD.windowContextType,
        &raw mut pCD as *mut XPointer,
    ) != 0
    {
        return;
    }
    (*pCD).wsmFunctions |= WSM_FUNCTION_OCCUPY_WS;
    F_AddToAllWorkspaces(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pCD,
        ::core::ptr::null_mut::<XEvent>(),
    );
    (*pCD).wsmFunctions &= !WSM_FUNCTION_OCCUPY_WS;
    if !(*pSD).clientList.is_null() {
        StackWindow(
            (*pSD).pActiveWS as *mut WmWorkspaceData,
            &raw mut (*pCD).clientEntry,
            TRUE as Boolean,
            NULL as *mut ClientListEntry,
        );
    }
    AddClientToList(
        (*pSD).pActiveWS as *mut WmWorkspaceData,
        pCD,
        True as Boolean,
    );
    XMapWindow(wmGD.display, (*pCD).clientFrameWin);
    XtManageChild(dialogboxW);
    if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
        Do_Focus_Key(pCD, GetTimestamp(), ALWAYS_SET_FOCUS);
    }
}
