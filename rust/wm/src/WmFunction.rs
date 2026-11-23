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
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed_0;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_1;
use crate::abi::C2RustUnnamed_WmArgb as C2RustUnnamed;
use crate::abi::WmWorkspaceData_WmBackdrop as WmWorkspaceData;
use crate::abi::*;
extern "C" {
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn XCreateWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: *mut Visual,
        _: ::core::ffi::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> Window;
    fn XBell(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XClearArea(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XCloseDisplay(_: *mut Display) -> ::core::ffi::c_int;
    fn XConfigureWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_uint,
        _: *mut XWindowChanges,
    ) -> ::core::ffi::c_int;
    fn XDestroyWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XFlush(_: *mut Display) -> ::core::ffi::c_int;
    fn XFree(_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn XKillClient(_: *mut Display, _: XID) -> ::core::ffi::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XQueryPointer(
        _: *mut Display,
        _: Window,
        _: *mut Window,
        _: *mut Window,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
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
    fn XSetInputFocus(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: Time,
    ) -> ::core::ffi::c_int;
    fn XSync(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XUnmapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XWarpPointer(
        _: *mut Display,
        _: Window,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut _XmStrings: [::core::ffi::c_char; 0];
    fn XmStringCreate(text: *mut ::core::ffi::c_char, charset: XmStringCharSet) -> XmString;
    fn XmStringCompare(a: XmString, b: XmString) -> Boolean;
    fn XmStringFree(string: XmString);
    static mut wmGD: WmGlobalData;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn putenv(__string: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn execl(
        __path: *const ::core::ffi::c_char,
        __arg: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn execvp(
        __file: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn execlp(
        __file: *const ::core::ffi::c_char,
        __arg: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn setsid() -> __pid_t;
    fn vfork() -> ::core::ffi::c_int;
    fn SetColormapFocus(pSD: *mut WmScreenData, pCD: *mut ClientData);
    fn ProcessColormapList(pSD: *mut WmScreenData, pCD: *mut ClientData);
    fn Warning(message: *mut ::core::ffi::c_char);
    fn GetTimestamp() -> Time;
    fn ConfirmAction(pSD: *mut WmScreenData, nbr: ::core::ffi::c_int);
    fn PackRootIcons();
    fn PackIconBox(
        pIBD: *mut IconBoxData,
        packVert: Boolean,
        packHorz: Boolean,
        passedInWidth: ::core::ffi::c_int,
        passedInHeight: ::core::ffi::c_int,
    );
    fn FindLastTransientTreeFocus(
        pCD: *mut ClientData,
        pcdNoFocus: *mut ClientData,
    ) -> *mut ClientData;
    fn FocusNextWindow(type_0: ::core::ffi::c_ulong, focusTime: Time) -> Boolean;
    fn FocusPrevWindow(type_0: ::core::ffi::c_ulong, focusTime: Time) -> Boolean;
    fn MakeMenu(
        pSD: *mut WmScreenData,
        menuName: String_0,
        initialContext: Context,
        accelContext: Context,
        moreMenuItems: *mut MenuItem,
        fSystemMenu: Boolean,
    ) -> *mut MenuSpec;
    fn PostMenu(
        menuSpec: *mut MenuSpec,
        pCD: *mut ClientData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        button: ::core::ffi::c_uint,
        newContext: Context,
        flags: ::core::ffi::c_long,
        passedInEvent: *mut XEvent,
    );
    fn RestoreDefaultSignalHandlers();
    fn ShowPresenceBox(pClient: *mut ClientData, wsContext: Context);
    fn ChangeToWorkspace(pNewWS: *mut WmWorkspaceData);
    fn ClientInWorkspace(pWS: *mut WmWorkspaceData, pCD: *mut ClientData) -> Boolean;
    fn SetClientWsIndex(pCD: *mut ClientData);
    fn SetMwmInfo(propWindow: Window, flags: ::core::ffi::c_long, wmWindow: Window);
    fn SendClientMsg(
        window: Window,
        type_0: ::core::ffi::c_long,
        data0: ::core::ffi::c_long,
        time: Time,
        pData: *mut ::core::ffi::c_long,
        dataLen: ::core::ffi::c_int,
    );
    fn AddWmTimer(
        timerType: ::core::ffi::c_uint,
        timerInterval: ::core::ffi::c_ulong,
        pCD: *mut ClientData,
    ) -> Boolean;
    fn GetString(linePP: *mut *mut ::core::ffi::c_uchar) -> *mut ::core::ffi::c_uchar;
    fn HandleClientFrameMove(pcd: *mut ClientData, pev: *mut XEvent);
    fn HandleClientFrameResize(pcd: *mut ClientData, pev: *mut XEvent);
    fn StartClientMove(pcd: *mut ClientData, pev: *mut XEvent) -> Boolean;
    fn StartClientResize(pcd: *mut ClientData, pev: *mut XEvent);
    fn HandleMarqueeSelect(pSD: *mut WmScreenData, pev: *mut XEvent);
    fn StartMarqueeSelect(pSD: *mut WmScreenData, pev: *mut XEvent);
    fn CalculateGravityOffset(
        pCD: *mut ClientData,
        xoff: *mut ::core::ffi::c_int,
        yoff: *mut ::core::ffi::c_int,
    );
    fn BumpPrimaryToBottom(pcdLeader: *mut ClientData) -> Boolean;
    fn BumpPrimaryToTop(pcdLeader: *mut ClientData) -> Boolean;
    fn CheckIfClientObscuredByAny(pcd: *mut ClientData) -> Boolean;
    fn CheckIfClientObscuringAny(pcd: *mut ClientData) -> Boolean;
    fn FindClientNameMatch(
        pEntry: *mut ClientListEntry,
        toNext: Boolean,
        clientName: String_0,
        types: ::core::ffi::c_ulong,
    ) -> *mut ClientListEntry;
    fn FindTransientFocus(pcd: *mut ClientData) -> *mut ClientData;
    fn FindTransientTreeLeader(pcd: *mut ClientData) -> *mut ClientData;
    fn LeaderOnTop(pcdLeader: *mut ClientData) -> Boolean;
    fn MoveEntryInList(
        pWS: *mut WmWorkspaceData,
        pEntry: *mut ClientListEntry,
        onTop: Boolean,
        pStackEntry: *mut ClientListEntry,
    );
    fn NormalizeTransientTreeStacking(pcdLeader: *mut ClientData) -> Boolean;
    fn PutTransientBelowSiblings(pcd: *mut ClientData) -> Boolean;
    fn PutTransientOnTop(pcd: *mut ClientData) -> Boolean;
    fn RestackTransients(pcd: *mut ClientData);
    fn StackWindow(
        pWS: *mut WmWorkspaceData,
        pEntry: *mut ClientListEntry,
        onTop: Boolean,
        pStackEntry: *mut ClientListEntry,
    );
    fn SetClientStateWithEventMask(
        pCD: *mut ClientData,
        newState: ::core::ffi::c_int,
        setTime: Time,
        event_mask: ::core::ffi::c_uint,
    );
    fn ResignFromSM();
    fn ConfigureEwmhFullScreen(_: *mut ClientData, _: Boolean);
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CopyFromParent: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const PointerRoot: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
pub const CurrentTime: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ButtonReleaseMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const KeyPress: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const KeyRelease: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ButtonPress: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ButtonRelease: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const RevertToParent: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const InputOutput: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CWX: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const CWY: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const CWBorderWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const CWSibling: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const CWStackMode: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const Above: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Below: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MWM_FUNC_RESIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const MWM_FUNC_MOVE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const MWM_FUNC_MINIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const MWM_FUNC_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const MWM_FUNC_CLOSE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const MWM_DECOR_MENU: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const MWM_INFO_STARTUP_STANDARD: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const MWM_INFO_STARTUP_CUSTOM: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WM_ERROR_EXIT_VALUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NoButton: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ALWAYS_SET_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const CLIENT_AREA_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const WORKSPACE_IF_NULL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const POST_AT_XY: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const POST_TRAVERSAL_ON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const DEFAULT_BEHAVIOR_ACTION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CUSTOM_BEHAVIOR_ACTION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RESTART_ACTION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const QUIT_MWM_ACTION: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const WSM_FUNCTION_OCCUPY_WS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int;
pub const WM_SHOW_FB_BEHAVIOR: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const WM_SHOW_FB_RESTART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const WM_SHOW_FB_QUIT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const F_CONTEXT_ROOT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const F_CONTEXT_ICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const F_CONTEXT_NORMAL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const F_CONTEXT_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const ICONBOX_PART_IICON: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const ICONBOX_PART_WICON: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const F_SUBCONTEXT_IB_IICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_IICON;
pub const F_SUBCONTEXT_IB_WICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_WICON;
pub const F_GROUP_WINDOW: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const F_GROUP_ICON: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const F_GROUP_DEFAULT: ::core::ffi::c_long = F_GROUP_WINDOW | F_GROUP_ICON;
pub const F_GROUP_TRANSIENT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const F_GROUP_ALL: ::core::ffi::c_long = F_GROUP_DEFAULT | F_GROUP_TRANSIENT;
pub const TIMER_QUIT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAXIMIZED_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ICON_REPARENTED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int;
pub const CLIENT_TERMINATING: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 10 as ::core::ffi::c_int;
pub const ICON_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CONFIRM_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 12 as ::core::ffi::c_int;
pub const CLIENT_WM_CLIENTS: ::core::ffi::c_long = ICON_BOX | CONFIRM_BOX;
pub const PROTOCOL_WM_SAVE_YOURSELF: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const PROTOCOL_WM_DELETE_WINDOW: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const PROTOCOL_WM_TAKE_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const CMAP_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STACK_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STACK_WITHIN_FAMILY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STACK_FREE_FAMILY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
static mut dirtyStackEntry: *mut ClientData = ::core::ptr::null::<ClientData>() as *mut ClientData;
static mut dirtyLeader: *mut ClientData = ::core::ptr::null::<ClientData>() as *mut ClientData;
#[no_mangle]
pub unsafe extern "C" fn F_Beep(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    XBell(wmGD.display, 0 as ::core::ffi::c_int);
    return 1 as Boolean;
}
unsafe extern "C" fn ForceLowerWindow(mut pcd: *mut ClientData) -> Boolean {
    let mut changes = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    let mut restack = False as Boolean;
    let mut stackWindow: Window = 0;
    let mut pSD = (*(*wmGD.pActiveSD).pActiveWS).pSD;
    let mut mask: ::core::ffi::c_uint = 0;
    let mut pCLE = ::core::ptr::null_mut::<ClientListEntry>();
    pCLE = (*pSD).lastClient;
    stackWindow = None_0 as Window;
    mask = CWStackMode as ::core::ffi::c_uint;
    while !pCLE.is_null() {
        if (*pCLE).pCD != pcd
            && ClientInWorkspace(
                (*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData,
                (*pCLE).pCD as *mut ClientData,
            ) as ::core::ffi::c_int
                != 0
        {
            if (*pCLE).type_0 == MINIMIZED_STATE && (*(*pCLE).pCD).clientState == MINIMIZED_STATE {
                stackWindow = (*(*(*pCLE).pCD)
                    .pWsList
                    .offset((*(*pCLE).pCD).currentWsc as isize))
                .iconFrameWin;
            } else if (*pCLE).type_0 == NORMAL_STATE
                && ((*(*pCLE).pCD).clientState == NORMAL_STATE
                    || (*(*pCLE).pCD).clientState == MAXIMIZED_STATE)
            {
                stackWindow = (*(*pCLE).pCD).clientFrameWin;
            }
            if stackWindow != None_0 as Window {
                mask |= CWSibling as ::core::ffi::c_uint;
                changes.sibling = stackWindow;
                break;
            }
        }
        if stackWindow == None_0 as Window {
            pCLE = (*pCLE).prevSibling as *mut ClientListEntry;
        }
    }
    changes.stack_mode = Below;
    if mask != 0 {
        XConfigureWindow(wmGD.display, (*pcd).clientFrameWin, mask, &raw mut changes);
    }
    return restack;
}
#[no_mangle]
pub unsafe extern "C" fn F_Lower(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut pNextEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut pStackEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut string = args;
    let mut flags = STACK_NORMAL;
    let mut pWS = (*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData;
    if !string.is_null() {
        if *string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32 {
            string = string.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as String_0;
            string = GetString(&raw mut string as *mut *mut ::core::ffi::c_uchar) as String_0;
            pStackEntry = ::core::ptr::null_mut::<ClientListEntry>();
            pNextEntry = (*wmGD.pActiveSD).lastClient;
            while !pNextEntry.is_null() && {
                pEntry = FindClientNameMatch(
                    pNextEntry,
                    False as Boolean,
                    string,
                    F_GROUP_ALL as ::core::ffi::c_ulong,
                );
                !pEntry.is_null()
            } {
                pNextEntry = (*pEntry).prevSibling as *mut ClientListEntry;
                if ClientInWorkspace(pWS, (*pEntry).pCD as *mut ClientData) != 0 {
                    Do_Lower((*pEntry).pCD as *mut ClientData, pStackEntry, STACK_NORMAL);
                    pStackEntry = pEntry;
                }
            }
        } else if *string != 0 {
            let mut slen: ::core::ffi::c_uint = 0;
            let mut len: ::core::ffi::c_uint = 0;
            let mut index: ::core::ffi::c_uint = 0;
            slen = strlen(args as *const ::core::ffi::c_char).wrapping_sub(2 as size_t)
                as ::core::ffi::c_uint;
            index = 0 as ::core::ffi::c_uint;
            while index < slen {
                string = GetString(&raw mut string as *mut *mut ::core::ffi::c_uchar) as String_0;
                if string.is_null() {
                    break;
                }
                len = strlen(string as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
                if strcmp(
                    string as *const ::core::ffi::c_char,
                    b"within\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    flags |= STACK_WITHIN_FAMILY;
                } else if strcmp(
                    string as *const ::core::ffi::c_char,
                    b"freeFamily\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    flags |= STACK_FREE_FAMILY;
                }
                index = index.wrapping_add(len);
                string = args.offset(index.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                    as *mut ::core::ffi::c_char as String_0;
            }
            if ClientInWorkspace(pWS, pCD) != 0 {
                Do_Lower(pCD, NULL_0 as *mut ClientListEntry, flags);
            }
        }
    } else if !pCD.is_null() && ClientInWorkspace(pWS, pCD) as ::core::ffi::c_int != 0 {
        Do_Lower(pCD, NULL_0 as *mut ClientListEntry, STACK_NORMAL);
    }
    if !pCD.is_null() && !event.is_null() && (*event).type_0 == ButtonPress {
        (*pCD).focusAutoRaiseDisablePending = True as Boolean;
    }
    wmGD.passButtonsCheck = False as Boolean;
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn Do_Lower(
    mut pCD: *mut ClientData,
    mut pStackEntry: *mut ClientListEntry,
    mut flags: ::core::ffi::c_int,
) {
    let mut restackTransients: Boolean = 0;
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut pWS = (*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData;
    let mut bLeaderRestacked: Boolean = 0;
    if ClientInWorkspace(pWS, pCD) as ::core::ffi::c_int != 0
        && (pStackEntry.is_null()
            || ClientInWorkspace(pWS, (*pStackEntry).pCD as *mut ClientData) as ::core::ffi::c_int
                != 0)
    {
        SetClientWsIndex(pCD);
        if !pStackEntry.is_null() {
            SetClientWsIndex((*pStackEntry).pCD as *mut ClientData);
        }
    } else {
        return;
    }
    pcdLeader = if !(*pCD).transientLeader.is_null() {
        FindTransientTreeLeader(pCD)
    } else {
        pCD
    };
    if (*pcdLeader).clientState == MINIMIZED_STATE
        && (*(*pcdLeader)
            .pWsList
            .offset((*pcdLeader).currentWsc as isize))
        .pIconBox
        .is_null()
    {
        if !dirtyStackEntry.is_null() {
            if !(*dirtyStackEntry).transientChildren.is_null()
                || !(*dirtyStackEntry).transientLeader.is_null()
            {
                RestackTransients(dirtyStackEntry);
            }
            dirtyStackEntry = ::core::ptr::null_mut::<ClientData>();
            dirtyLeader = ::core::ptr::null_mut::<ClientData>();
        }
        if !pStackEntry.is_null() {
            if (*pStackEntry).prevSibling != &raw mut (*pcdLeader).iconEntry {
                StackWindow(
                    pWS,
                    &raw mut (*pcdLeader).iconEntry,
                    True as Boolean,
                    pStackEntry,
                );
                MoveEntryInList(
                    pWS,
                    &raw mut (*pcdLeader).iconEntry,
                    True as Boolean,
                    pStackEntry,
                );
            }
        } else if (*wmGD.pActiveSD).lastClient != &raw mut (*pcdLeader).iconEntry {
            StackWindow(
                pWS,
                &raw mut (*pcdLeader).iconEntry,
                False as Boolean,
                NULL_0 as *mut ClientListEntry,
            );
            MoveEntryInList(
                pWS,
                &raw mut (*pcdLeader).iconEntry,
                False as Boolean,
                NULL_0 as *mut ClientListEntry,
            );
        }
    } else {
        bLeaderRestacked = False as Boolean;
        if !(*pcdLeader).transientChildren.is_null()
            && (*pcdLeader).secondariesOnTop == 0
            && wmGD.bSuspendSecondaryRestack == 0
        {
            if pCD == pcdLeader {
                bLeaderRestacked = BumpPrimaryToBottom(pcdLeader);
            } else if !(*pCD).transientChildren.is_null() {
                bLeaderRestacked = BumpPrimaryToBottom(pCD);
            } else if !(*pCD).transientLeader.is_null() {
                let mut pcdLdr = ::core::ptr::null_mut::<ClientData>();
                pcdLdr = (*pCD).transientLeader as *mut ClientData;
                while !pcdLdr.is_null() {
                    bLeaderRestacked = (bLeaderRestacked as ::core::ffi::c_int
                        | BumpPrimaryToBottom(pcdLdr) as ::core::ffi::c_int)
                        as Boolean;
                    pcdLdr = (*pcdLdr).transientLeader as *mut ClientData;
                }
            }
        }
        restackTransients = False as Boolean;
        if !(*pCD).transientLeader.is_null() {
            if flags & STACK_FREE_FAMILY != 0 {
                if !dirtyStackEntry.is_null() && dirtyStackEntry != pCD {
                    RestackTransients(dirtyStackEntry);
                }
                dirtyStackEntry = pCD;
                dirtyLeader = pcdLeader;
                ForceLowerWindow(pCD);
                return;
            }
            restackTransients = PutTransientBelowSiblings(pCD);
        }
        if !dirtyStackEntry.is_null() {
            if dirtyLeader == pcdLeader {
                restackTransients = True as Boolean;
            } else {
                RestackTransients(dirtyStackEntry);
            }
            dirtyStackEntry = ::core::ptr::null_mut::<ClientData>();
        }
        if !pStackEntry.is_null() {
            if (*pStackEntry).prevSibling != &raw mut (*pcdLeader).clientEntry
                && !(wmGD.systemModalActive as ::core::ffi::c_int != 0
                    && pcdLeader == wmGD.systemModalClient)
            {
                StackWindow(
                    pWS,
                    &raw mut (*pcdLeader).clientEntry,
                    True as Boolean,
                    pStackEntry,
                );
                MoveEntryInList(
                    pWS,
                    &raw mut (*pcdLeader).clientEntry,
                    True as Boolean,
                    pStackEntry,
                );
            } else if restackTransients as ::core::ffi::c_int != 0
                || bLeaderRestacked as ::core::ffi::c_int != 0
            {
                RestackTransients(pCD);
            }
        } else if (*(*pWS).pSD).lastClient != &raw mut (*pcdLeader).clientEntry
            && !(wmGD.systemModalActive as ::core::ffi::c_int != 0
                && pcdLeader == wmGD.systemModalClient)
            && flags & STACK_WITHIN_FAMILY == 0
        {
            StackWindow(
                pWS,
                &raw mut (*pcdLeader).clientEntry,
                False as Boolean,
                NULL_0 as *mut ClientListEntry,
            );
            MoveEntryInList(
                pWS,
                &raw mut (*pcdLeader).clientEntry,
                False as Boolean,
                NULL_0 as *mut ClientListEntry,
            );
        } else if restackTransients as ::core::ffi::c_int != 0
            || bLeaderRestacked as ::core::ffi::c_int != 0
        {
            RestackTransients(pCD);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn F_Circle_Down(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut types: ::core::ffi::c_ulong = 0;
    let mut windowType: ::core::ffi::c_ulong = 0;
    let mut pNextEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    types = args as ::core::ffi::c_ulong;
    pNextEntry = (*wmGD.pActiveSD).clientList;
    while !pNextEntry.is_null() {
        pcdNext = (*pNextEntry).pCD as *mut ClientData;
        if (*pNextEntry).type_0 == NORMAL_STATE && (*pcdNext).clientState != MINIMIZED_STATE
            || (*pNextEntry).type_0 == MINIMIZED_STATE && (*pcdNext).clientState == MINIMIZED_STATE
        {
            if (*pcdNext).clientState == MINIMIZED_STATE {
                windowType = F_GROUP_ICON as ::core::ffi::c_ulong;
            } else {
                windowType = F_GROUP_WINDOW as ::core::ffi::c_ulong;
                if !(*pcdNext).transientLeader.is_null() || !(*pcdNext).transientChildren.is_null()
                {
                    windowType |= F_GROUP_TRANSIENT as ::core::ffi::c_ulong;
                }
            }
            if types & windowType != 0 {
                if CheckIfClientObscuringAny(pcdNext) != 0 {
                    wmGD.bSuspendSecondaryRestack = True as Boolean;
                    F_Lower(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        pcdNext,
                        NULL_0 as *mut XEvent,
                    );
                    wmGD.bSuspendSecondaryRestack = False as Boolean;
                    break;
                }
            }
        }
        pNextEntry = (*pNextEntry).nextSibling as *mut ClientListEntry;
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Circle_Up(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut types: ::core::ffi::c_ulong = 0;
    let mut windowType: ::core::ffi::c_ulong = 0;
    let mut pNextEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    types = args as ::core::ffi::c_ulong;
    pNextEntry = (*wmGD.pActiveSD).lastClient;
    while !pNextEntry.is_null() {
        pcdNext = (*pNextEntry).pCD as *mut ClientData;
        if (*pNextEntry).type_0 == NORMAL_STATE && (*pcdNext).clientState != MINIMIZED_STATE
            || (*pNextEntry).type_0 == MINIMIZED_STATE && (*pcdNext).clientState == MINIMIZED_STATE
        {
            if (*pcdNext).clientState == MINIMIZED_STATE {
                windowType = F_GROUP_ICON as ::core::ffi::c_ulong;
            } else {
                windowType = F_GROUP_WINDOW as ::core::ffi::c_ulong;
                if !(*pcdNext).transientLeader.is_null() || !(*pcdNext).transientChildren.is_null()
                {
                    windowType |= F_GROUP_TRANSIENT as ::core::ffi::c_ulong;
                }
            }
            if types & windowType != 0 {
                if CheckIfClientObscuredByAny(pcdNext) != 0 {
                    wmGD.bSuspendSecondaryRestack = True as Boolean;
                    F_Raise(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        pcdNext,
                        NULL_0 as *mut XEvent,
                    );
                    wmGD.bSuspendSecondaryRestack = False as Boolean;
                    break;
                }
            }
        }
        pNextEntry = (*pNextEntry).prevSibling as *mut ClientListEntry;
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Focus_Color(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if wmGD.colormapFocusPolicy == CMAP_FOCUS_EXPLICIT {
        if !pCD.is_null() {
            if (*pCD).clientState == MINIMIZED_STATE {
                pCD = ::core::ptr::null_mut::<ClientData>();
            }
        }
        SetColormapFocus(wmGD.pActiveSD, pCD);
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Exec(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pid: ::core::ffi::c_int = 0;
    let mut shell = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut shellname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*wmGD.pActiveSD).displayString.is_null() {
        putenv((*wmGD.pActiveSD).displayString as *mut ::core::ffi::c_char);
    }
    pid = vfork();
    if pid == 0 as ::core::ffi::c_int {
        setsid();
        RestoreDefaultSignalHandlers();
        shell = getenv(b"WMSHELL\0" as *const u8 as *const ::core::ffi::c_char);
        if !shell.is_null() || {
            shell = getenv(b"SHELL\0" as *const u8 as *const ::core::ffi::c_char);
            !shell.is_null()
        } {
            shellname = strrchr(shell, '/' as i32);
            if shellname.is_null() {
                shellname = shell;
                execlp(
                    shell,
                    shellname,
                    b"-c\0" as *const u8 as *const ::core::ffi::c_char,
                    args,
                    NULL_0,
                );
            } else {
                shellname = shellname.offset(1);
                execl(
                    shell,
                    shellname,
                    b"-c\0" as *const u8 as *const ::core::ffi::c_char,
                    args,
                    NULL_0,
                );
            }
        }
        execl(
            b"/bin/sh\0" as *const u8 as *const ::core::ffi::c_char,
            b"sh\0" as *const u8 as *const ::core::ffi::c_char,
            b"-c\0" as *const u8 as *const ::core::ffi::c_char,
            args,
            NULL_0,
        );
        _exit(127 as ::core::ffi::c_int);
    } else if pid == -(1 as ::core::ffi::c_int) {
        return 1 as Boolean;
    }
    if !(*wmGD.pActiveSD).displayString.is_null() && !wmGD.displayString.is_null() {
        putenv(wmGD.displayString as *mut ::core::ffi::c_char);
    }
    return True as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Quit_Mwm(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if wmGD.showFeedback as ::core::ffi::c_long & WM_SHOW_FB_QUIT != 0 {
        ConfirmAction(wmGD.pActiveSD, QUIT_MWM_ACTION);
    } else {
        Do_Quit_Mwm(False as Boolean);
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn Do_Quit_Mwm(mut diedOnRestart: Boolean) {
    let mut scr: ::core::ffi::c_int = 0;
    let mut pNextEntry = ::core::ptr::null_mut::<ClientListEntry>();
    if !wmGD.display.is_null() {
        XSetInputFocus(
            wmGD.display,
            PointerRoot as Window,
            PointerRoot as ::core::ffi::c_int,
            CurrentTime as Time,
        );
        scr = 0 as ::core::ffi::c_int;
        while scr < wmGD.numScreens {
            if (*wmGD.Screens.offset(scr as isize)).managed != 0 {
                pNextEntry = (*wmGD.Screens.offset(scr as isize)).lastClient;
                while !pNextEntry.is_null() {
                    if (*pNextEntry).type_0 == NORMAL_STATE {
                        if (*(*pNextEntry).pCD).clientFlags & CLIENT_WM_CLIENTS == 0 {
                            ReBorderClient((*pNextEntry).pCD as *mut ClientData, diedOnRestart);
                        }
                    }
                    pNextEntry = (*pNextEntry).prevSibling as *mut ClientListEntry;
                }
            }
            scr += 1;
        }
        ResignFromSM();
        XSync(wmGD.display, False);
        XCloseDisplay(wmGD.display);
    }
    if diedOnRestart != 0 {
        exit(WM_ERROR_EXIT_VALUE);
    } else {
        exit(0 as ::core::ffi::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ReBorderClient(mut pCD: *mut ClientData, mut reMapClient: Boolean) {
    let mut x = 0 as ::core::ffi::c_int;
    let mut y = 0 as ::core::ffi::c_int;
    let mut xoff: ::core::ffi::c_int = 0;
    let mut yoff: ::core::ffi::c_int = 0;
    let mut windowChanges = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    while !pCD.is_null() {
        if (*pCD).iconWindow != 0 && (*pCD).clientFlags & ICON_REPARENTED != 0 && reMapClient == 0 {
            XUnmapWindow(wmGD.display, (*pCD).iconWindow);
            XReparentWindow(
                wmGD.display,
                (*pCD).iconWindow,
                (*(*pCD).pSD).rootWindow,
                (*(*pCD).pWsList).iconX,
                (*(*pCD).pWsList).iconY,
            );
        }
        if reMapClient == 0 {
            if (*pCD).maxConfig != 0 {
                x = (*pCD).maxX;
                y = (*pCD).maxY;
            } else {
                CalculateGravityOffset(pCD, &raw mut xoff, &raw mut yoff);
                x = (*pCD).clientX - xoff;
                y = (*pCD).clientY - yoff;
            }
            XUnmapWindow(wmGD.display, (*pCD).clientFrameWin);
            XReparentWindow(wmGD.display, (*pCD).client, (*(*pCD).pSD).rootWindow, x, y);
        } else {
            XMapWindow(wmGD.display, (*pCD).client);
        }
        if !(*pCD).transientChildren.is_null() {
            ReBorderClient((*pCD).transientChildren as *mut ClientData, reMapClient);
        }
        if reMapClient == 0 {
            windowChanges.x = x;
            windowChanges.y = y;
            windowChanges.border_width = (*pCD).xBorderWidth;
            XConfigureWindow(
                wmGD.display,
                (*pCD).client,
                (CWBorderWidth | CWX | CWY) as ::core::ffi::c_uint,
                &raw mut windowChanges,
            );
        }
        if !(*pCD).transientLeader.is_null() {
            pCD = (*pCD).transientSiblings as *mut ClientData;
        } else {
            pCD = ::core::ptr::null_mut::<ClientData>();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_Focus_Key(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut focusFlags = args as ::core::ffi::c_long;
    if !pCD.is_null() && wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
        Do_Focus_Key(
            pCD,
            GetFunctionTimestamp(event as *mut XButtonEvent),
            focusFlags | ALWAYS_SET_FOCUS,
        );
    }
    return 1 as Boolean;
}
unsafe extern "C" fn FindSomeReasonableClient() -> Window {
    let mut pcdNoFocus = ::core::ptr::null_mut::<ClientData>();
    let mut pNextEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut pcdLastFocus = NULL_0 as *mut ClientData;
    let mut pcdFocus = ::core::ptr::null_mut::<ClientData>();
    let mut focusWindow = NULL_0 as Window;
    pNextEntry = (*wmGD.pActiveSD).clientList;
    while !pNextEntry.is_null() {
        pCD = (*pNextEntry).pCD as *mut ClientData;
        if wmGD.systemModalActive == 0 || wmGD.systemModalClient == pCD {
            if (*pNextEntry).type_0 != MINIMIZED_STATE
                && (*pCD).clientState != MINIMIZED_STATE
                && ClientInWorkspace((*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData, pCD)
                    as ::core::ffi::c_int
                    != 0
                && pCD != pcdNoFocus
            {
                if !(*pCD).transientChildren.is_null() {
                    pcdFocus = FindLastTransientTreeFocus(pCD, pcdNoFocus);
                } else {
                    pcdFocus = pCD;
                }
                if !pcdFocus.is_null()
                    && (pcdLastFocus.is_null()
                        || (*pcdFocus).focusPriority > (*pcdLastFocus).focusPriority)
                {
                    pcdLastFocus = pcdFocus;
                }
            }
        }
        pNextEntry = (*pNextEntry).nextSibling as *mut ClientListEntry;
    }
    if !pcdLastFocus.is_null()
        && ClientInWorkspace(
            (*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData,
            pcdLastFocus,
        ) as ::core::ffi::c_int
            != 0
    {
        focusWindow = (*pcdLastFocus).client;
    }
    if focusWindow == NULL_0 as Window {
        pNextEntry = (*wmGD.pActiveSD).clientList;
        while !pNextEntry.is_null() {
            pCD = (*pNextEntry).pCD as *mut ClientData;
            if ClientInWorkspace((*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData, pCD) != 0 {
                if (*pNextEntry).type_0 == MINIMIZED_STATE || (*pCD).clientState == MINIMIZED_STATE
                {
                    focusWindow = (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin;
                    break;
                }
            }
            pNextEntry = (*pNextEntry).nextSibling as *mut ClientListEntry;
        }
    }
    return focusWindow;
}
#[no_mangle]
pub unsafe extern "C" fn Do_Focus_Key(
    mut pCD: *mut ClientData,
    mut focusTime: Time,
    mut flags: ::core::ffi::c_long,
) {
    let mut pcdFocus = ::core::ptr::null_mut::<ClientData>();
    let mut focusWindow: Window = 0;
    wmGD.replayEnterEvent = False;
    pcdFocus = pCD;
    if !pCD.is_null()
        && ClientInWorkspace((*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData, pCD)
            as ::core::ffi::c_int
            != 0
    {
        if (*pCD).clientState == MINIMIZED_STATE {
            focusWindow = (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin;
        } else if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
            if (*pCD).primaryModalCount != 0 || (*pCD).fullModalCount != 0 {
                let mut pcdFocusLeader = ::core::ptr::null_mut::<ClientData>();
                let mut currFocusLeader = ::core::ptr::null_mut::<ClientData>();
                if !wmGD.keyboardFocus.is_null() {
                    currFocusLeader = (*wmGD.keyboardFocus).transientLeader as *mut ClientData;
                } else {
                    currFocusLeader = NULL_0 as *mut ClientData;
                }
                pcdFocusLeader = pCD;
                while !(*pcdFocusLeader).transientLeader.is_null()
                    && pcdFocusLeader != currFocusLeader
                {
                    pcdFocusLeader = (*pcdFocusLeader).transientLeader as *mut ClientData;
                }
                if pcdFocusLeader == currFocusLeader {
                    pcdFocus = wmGD.keyboardFocus;
                    flags = 0 as ::core::ffi::c_long;
                } else {
                    pcdFocus = FindTransientFocus(pcdFocusLeader);
                }
            }
            if !pcdFocus.is_null() {
                focusWindow = (*pcdFocus).client;
            } else {
                focusWindow = if !wmGD.keyboardFocus.is_null() {
                    (*wmGD.keyboardFocus).client
                } else {
                    (*wmGD.pActiveSD).wmWorkspaceWin
                };
            }
        } else if (*pCD).primaryModalCount != 0 || (*pCD).fullModalCount != 0 {
            pcdFocus = ::core::ptr::null_mut::<ClientData>();
            focusWindow = (*wmGD.pActiveSD).wmWorkspaceWin;
            wmGD.replayEnterEvent = True;
        } else {
            focusWindow = (*pcdFocus).client;
        }
    } else if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER {
        focusWindow = PointerRoot as Window;
    } else if WORKSPACE_IF_NULL & flags != 0 {
        focusWindow = (*wmGD.pActiveSD).wmWorkspaceWin;
    } else {
        focusWindow = FindSomeReasonableClient();
        if focusWindow == NULL_0 as Window {
            focusWindow = (*wmGD.pActiveSD).wmWorkspaceWin;
        }
    }
    if pcdFocus != wmGD.keyboardFocus || flags & ALWAYS_SET_FOCUS != 0 {
        if !pcdFocus.is_null() {
            if flags & CLIENT_AREA_FOCUS != 0
                && (*pcdFocus).protocolFlags & PROTOCOL_WM_TAKE_FOCUS != 0
                && (*pcdFocus).inputFocusModel == 0
                && pcdFocus == pCD
                && wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT
                && !((*pcdFocus).clientState == MINIMIZED_STATE)
            {
                SendClientMsg(
                    (*pcdFocus).client,
                    wmGD.xa_WM_PROTOCOLS as ::core::ffi::c_long,
                    wmGD.xa_WM_TAKE_FOCUS as ::core::ffi::c_long,
                    focusTime,
                    ::core::ptr::null_mut::<::core::ffi::c_long>(),
                    0 as ::core::ffi::c_int,
                );
            } else {
                if (*pcdFocus).protocolFlags & PROTOCOL_WM_TAKE_FOCUS != 0
                    && !((*pcdFocus).clientState == MINIMIZED_STATE)
                {
                    SendClientMsg(
                        (*pcdFocus).client,
                        wmGD.xa_WM_PROTOCOLS as ::core::ffi::c_long,
                        wmGD.xa_WM_TAKE_FOCUS as ::core::ffi::c_long,
                        focusTime,
                        ::core::ptr::null_mut::<::core::ffi::c_long>(),
                        0 as ::core::ffi::c_int,
                    );
                }
                if wmGD.enforceKeyFocus as ::core::ffi::c_int != 0
                    || flags & ALWAYS_SET_FOCUS != 0
                    || flags & CLIENT_AREA_FOCUS == 0
                    || (*pcdFocus).inputFocusModel != 0
                    || (*pcdFocus).clientState == MINIMIZED_STATE
                {
                    if flags & CLIENT_AREA_FOCUS == 0
                        && (*pcdFocus).inputFocusModel == 0
                        && !((*pcdFocus).clientState == MINIMIZED_STATE)
                    {
                        XSetInputFocus(
                            wmGD.display,
                            (*pcdFocus).clientBaseWin,
                            PointerRoot as ::core::ffi::c_int,
                            focusTime,
                        );
                    } else if flags & CLIENT_AREA_FOCUS == 0
                        && (*pcdFocus).protocolFlags & PROTOCOL_WM_TAKE_FOCUS == 0
                        && (*pcdFocus).inputFocusModel != 0
                    {
                        XSetInputFocus(
                            wmGD.display,
                            focusWindow,
                            PointerRoot as ::core::ffi::c_int,
                            focusTime,
                        );
                    } else {
                        XSetInputFocus(wmGD.display, focusWindow, RevertToParent, focusTime);
                    }
                } else {
                    pcdFocus = wmGD.nextKeyboardFocus;
                }
            }
        } else {
            XSetInputFocus(
                wmGD.display,
                focusWindow,
                PointerRoot as ::core::ffi::c_int,
                focusTime,
            );
        }
        wmGD.nextKeyboardFocus = pcdFocus;
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_Goto_Workspace(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pSD = wmGD.pActiveSD;
    let mut iwsx: ::core::ffi::c_int = 0;
    let mut xms = ::core::ptr::null_mut::<__XmStringRec>();
    xms = XmStringCreate(
        args as *mut ::core::ffi::c_char,
        (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(12578 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
    );
    iwsx = 0 as ::core::ffi::c_int;
    while iwsx < (*pSD).numWorkspaces {
        if strcmp(
            (*(*pSD).pWS.offset(iwsx as isize)).name as *const ::core::ffi::c_char,
            args as *const ::core::ffi::c_char,
        ) == 0
            || XmStringCompare(xms, (*(*pSD).pWS.offset(iwsx as isize)).title) as ::core::ffi::c_int
                != 0
        {
            break;
        }
        iwsx += 1;
    }
    XmStringFree(xms);
    if iwsx >= (*pSD).numWorkspaces {
        Warning(
            b"Invalid workspace name specified for f.goto_workspace\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        ChangeToWorkspace((*pSD).pWS.offset(iwsx as isize) as *mut WmWorkspaceData);
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Next_Key(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
        FocusNextWindow(
            args as ::core::ffi::c_ulong,
            GetFunctionTimestamp(event as *mut XButtonEvent),
        );
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Prev_Cmap(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if pCD.is_null() {
        pCD = (*wmGD.pActiveSD).colormapFocus as *mut ClientData;
    }
    if !pCD.is_null()
        && (*pCD).clientCmapCount > 0 as ::core::ffi::c_int
        && ((*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE)
    {
        (*pCD).clientCmapIndex -= 1;
        if (*pCD).clientCmapIndex < 0 as ::core::ffi::c_int {
            (*pCD).clientCmapIndex = (*pCD).clientCmapCount - 1 as ::core::ffi::c_int;
        }
        (*pCD).clientColormap = *(*pCD)
            .clientCmapList
            .offset((*pCD).clientCmapIndex as isize);
        if (*wmGD.pActiveSD).colormapFocus == pCD {
            (*pCD).clientCmapFlagsInitialized = 0 as ::core::ffi::c_int;
            ProcessColormapList(wmGD.pActiveSD, pCD);
        }
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Prev_Key(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
        FocusPrevWindow(
            args as ::core::ffi::c_ulong,
            GetFunctionTimestamp(event as *mut XButtonEvent),
        );
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Pass_Key(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if wmGD.passKeysActive != 0 {
        wmGD.passKeysActive = False as Boolean;
        wmGD.passKeysKeySpec = ::core::ptr::null_mut::<KeySpec>();
    } else {
        wmGD.passKeysActive = True as Boolean;
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Maximize(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if !pCD.is_null() && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MAXIMIZE != 0 {
        SetClientStateWithEventMask(
            pCD,
            MAXIMIZED_STATE,
            GetFunctionTimestamp(event as *mut XButtonEvent),
            GetEventInverseMask(event),
        );
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Menu(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut menuSpec = ::core::ptr::null_mut::<MenuSpec>();
    let mut menuContext: Context = 0;
    let mut button: ::core::ffi::c_uint = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut flags = POST_AT_XY;
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    if !event.is_null() && ((*event).type_0 == ButtonPress || (*event).type_0 == ButtonRelease) {
        button = (*event).xbutton.button;
        x = (*event).xbutton.x_root;
        y = (*event).xbutton.y_root;
        if (*event).type_0 == ButtonRelease {
            flags |= POST_TRAVERSAL_ON;
        } else if wmGD.rootButtonClick as ::core::ffi::c_int != 0 && (*event).type_0 == ButtonPress
        {
            if wmGD.bReplayedButton != 0 {
                return 0 as Boolean;
            }
            wmGD.checkHotspot = True as Boolean;
            wmGD.hotspotRectangle.x =
                (x - wmGD.moveThreshold / 2 as ::core::ffi::c_int) as ::core::ffi::c_short;
            wmGD.hotspotRectangle.y =
                (y - wmGD.moveThreshold / 2 as ::core::ffi::c_int) as ::core::ffi::c_short;
            wmGD.hotspotRectangle.width = wmGD.moveThreshold as ::core::ffi::c_ushort;
            wmGD.hotspotRectangle.height = wmGD.moveThreshold as ::core::ffi::c_ushort;
        }
    } else if !event.is_null() && ((*event).type_0 == KeyPress || (*event).type_0 == KeyRelease) {
        button = NoButton as ::core::ffi::c_uint;
        x = (*event).xkey.x_root;
        y = (*event).xkey.y_root;
    } else {
        return 0 as Boolean;
    }
    if !pCD.is_null() {
        if (*pCD).clientState == NORMAL_STATE {
            menuContext = F_CONTEXT_NORMAL as Context;
        } else if (*pCD).clientState == MAXIMIZED_STATE {
            menuContext = F_CONTEXT_MAXIMIZE as Context;
        } else {
            menuContext = F_CONTEXT_ICON as Context;
        }
        if !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
            .pIconBox
            .is_null()
            && (*event).xany.window
                == (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin
        {
            if (*pCD).clientState == MINIMIZED_STATE {
                menuContext = F_SUBCONTEXT_IB_IICON as Context;
            } else {
                menuContext = F_SUBCONTEXT_IB_WICON as Context;
            }
        }
    } else {
        menuContext = F_CONTEXT_ROOT as Context;
    }
    pSD = if !pCD.is_null() {
        (*pCD).pSD
    } else {
        wmGD.pActiveSD
    };
    menuSpec = MakeMenu(
        pSD,
        args,
        menuContext,
        menuContext,
        NULL_0 as *mut MenuItem,
        FALSE as Boolean,
    );
    if !menuSpec.is_null() {
        PostMenu(menuSpec, pCD, x, y, button, menuContext, flags, event);
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Minimize(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    if !pCD.is_null() {
        pcdLeader = if !(*pCD).transientLeader.is_null() {
            FindTransientTreeLeader(pCD)
        } else {
            pCD
        };
        if (*pcdLeader).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0 {
            SetClientStateWithEventMask(
                pCD,
                MINIMIZED_STATE,
                GetFunctionTimestamp(event as *mut XButtonEvent),
                GetEventInverseMask(event),
            );
        }
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Move(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if !pCD.is_null() && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MOVE != 0 {
        StartClientMove(pCD, event);
        HandleClientFrameMove(pCD, event);
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Next_Cmap(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if pCD.is_null() {
        pCD = (*wmGD.pActiveSD).colormapFocus as *mut ClientData;
    }
    if !pCD.is_null()
        && (*pCD).clientCmapCount > 0 as ::core::ffi::c_int
        && ((*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE)
    {
        (*pCD).clientCmapIndex += 1;
        if (*pCD).clientCmapIndex >= (*pCD).clientCmapCount {
            (*pCD).clientCmapIndex = 0 as ::core::ffi::c_int;
        }
        (*pCD).clientColormap = *(*pCD)
            .clientCmapList
            .offset((*pCD).clientCmapIndex as isize);
        if (*wmGD.pActiveSD).colormapFocus == pCD {
            (*pCD).clientCmapFlagsInitialized = 0 as ::core::ffi::c_int;
            ProcessColormapList(wmGD.pActiveSD, pCD);
        }
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Nop(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Normalize(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if !pCD.is_null() {
        SetClientStateWithEventMask(
            pCD,
            NORMAL_STATE,
            GetFunctionTimestamp(event as *mut XButtonEvent),
            GetEventInverseMask(event),
        );
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Normalize_And_Raise(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if !pCD.is_null() {
        if (*pCD).clientState == MINIMIZED_STATE {
            SetClientStateWithEventMask(
                pCD,
                NORMAL_STATE,
                if !event.is_null() {
                    GetFunctionTimestamp(event as *mut XButtonEvent)
                } else {
                    GetTimestamp()
                },
                GetEventInverseMask(event),
            );
        } else {
            SetClientStateWithEventMask(
                pCD,
                NORMAL_STATE,
                GetFunctionTimestamp(event as *mut XButtonEvent),
                GetEventInverseMask(event),
            );
            wmGD.bSuspendSecondaryRestack = True as Boolean;
            F_Raise(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                pCD,
                NULL_0 as *mut XEvent,
            );
            wmGD.bSuspendSecondaryRestack = False as Boolean;
            if wmGD.raiseKeyFocus != 0 {
                F_Focus_Key(
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    pCD,
                    if !event.is_null() {
                        event
                    } else {
                        NULL_0 as *mut XEvent
                    },
                );
            }
        }
        wmGD.clickData.clickPending = False as Boolean;
        wmGD.clickData.doubleClickPending = False as Boolean;
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Restore(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut newState: ::core::ffi::c_int = 0;
    if !pCD.is_null() {
        if (*pCD).fullScreen as ::core::ffi::c_int != 0 && (*pCD).clientState != MINIMIZED_STATE {
            ConfigureEwmhFullScreen(pCD, False as Boolean);
        } else if (*pCD).clientState == MAXIMIZED_STATE {
            SetClientStateWithEventMask(
                pCD,
                NORMAL_STATE,
                GetFunctionTimestamp(event as *mut XButtonEvent),
                GetEventInverseMask(event),
            );
        } else {
            if (*pCD).maxConfig != 0 {
                newState = MAXIMIZED_STATE;
            } else {
                newState = NORMAL_STATE;
            }
            SetClientStateWithEventMask(
                pCD,
                newState,
                GetFunctionTimestamp(event as *mut XButtonEvent),
                GetEventInverseMask(event),
            );
        }
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Restore_And_Raise(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut newState: ::core::ffi::c_int = 0;
    if !pCD.is_null() {
        if (*pCD).clientState == MINIMIZED_STATE {
            if (*pCD).maxConfig != 0 {
                newState = MAXIMIZED_STATE;
            } else {
                newState = NORMAL_STATE;
            }
            SetClientStateWithEventMask(
                pCD,
                newState,
                if !event.is_null() {
                    GetFunctionTimestamp(event as *mut XButtonEvent)
                } else {
                    GetTimestamp()
                },
                GetEventInverseMask(event),
            );
        } else {
            F_Restore(::core::ptr::null_mut::<::core::ffi::c_char>(), pCD, event);
            wmGD.bSuspendSecondaryRestack = True as Boolean;
            F_Raise(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                pCD,
                NULL_0 as *mut XEvent,
            );
            wmGD.bSuspendSecondaryRestack = False as Boolean;
            if wmGD.raiseKeyFocus != 0 {
                F_Focus_Key(
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    pCD,
                    if !event.is_null() {
                        event
                    } else {
                        NULL_0 as *mut XEvent
                    },
                );
            }
        }
        wmGD.clickData.clickPending = False as Boolean;
        wmGD.clickData.doubleClickPending = False as Boolean;
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Pack_Icons(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pIBD = ::core::ptr::null_mut::<IconBoxData>();
    if (*wmGD.pActiveSD).useIconBox != 0 {
        pIBD = (*(*wmGD.pActiveSD).pActiveWS).pIconBox;
        if !pCD.is_null() {
            while pCD != (*pIBD).pCD_iconBox {
                if !(*pIBD).pNextIconBox.is_null() {
                    pIBD = (*pIBD).pNextIconBox as *mut IconBoxData;
                } else {
                    pIBD = ::core::ptr::null_mut::<IconBoxData>();
                    break;
                }
            }
        }
        if !pIBD.is_null() {
            PackIconBox(
                pIBD,
                False as Boolean,
                False as Boolean,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        } else {
            PackRootIcons();
        }
    } else {
        PackRootIcons();
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Post_SMenu(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut menuContext: Context = 0;
    if !event.is_null() && !pCD.is_null() && !(*pCD).systemMenuSpec.is_null() {
        if (*pCD).fullScreen as ::core::ffi::c_int != 0 && (*pCD).clientState != MINIMIZED_STATE {
            menuContext = F_CONTEXT_MAXIMIZE as Context;
        } else if (*pCD).clientState == NORMAL_STATE {
            menuContext = F_CONTEXT_NORMAL as Context;
        } else if (*pCD).clientState == MAXIMIZED_STATE {
            menuContext = F_CONTEXT_MAXIMIZE as Context;
        } else {
            menuContext = F_CONTEXT_ICON as Context;
        }
        if !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
            .pIconBox
            .is_null()
            && (*event).xany.window
                == (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin
        {
            if (*pCD).clientState == MINIMIZED_STATE {
                menuContext = F_SUBCONTEXT_IB_IICON as Context;
            } else {
                menuContext = F_SUBCONTEXT_IB_WICON as Context;
            }
        }
        if (*event).type_0 == KeyPress || (*event).type_0 == KeyRelease {
            if (*pCD).clientState == MINIMIZED_STATE
                || menuContext == (F_SUBCONTEXT_IB_IICON | F_SUBCONTEXT_IB_WICON) as Context
            {
                if wmGD.iconClick != 0 {
                    wmGD.checkHotspot = True as Boolean;
                }
            } else if wmGD.systemButtonClick as ::core::ffi::c_int != 0
                && (*pCD).decor & MWM_DECOR_MENU != 0
            {
                wmGD.checkHotspot = True as Boolean;
            }
            PostMenu(
                (*pCD).systemMenuSpec,
                pCD,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NoButton as ::core::ffi::c_uint,
                menuContext,
                0 as ::core::ffi::c_long,
                event,
            );
        } else if (*event).type_0 == ButtonPress {
            if wmGD.rootButtonClick as ::core::ffi::c_int != 0 && wmGD.checkHotspot == 0 {
                wmGD.checkHotspot = True as Boolean;
                wmGD.hotspotRectangle.x = ((*event).xbutton.x_root
                    - wmGD.moveThreshold / 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_short;
                wmGD.hotspotRectangle.y = ((*event).xbutton.y_root
                    - wmGD.moveThreshold / 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_short;
                wmGD.hotspotRectangle.width = wmGD.moveThreshold as ::core::ffi::c_ushort;
                wmGD.hotspotRectangle.height = wmGD.moveThreshold as ::core::ffi::c_ushort;
            }
            PostMenu(
                (*pCD).systemMenuSpec,
                pCD,
                (*event).xbutton.x_root,
                (*event).xbutton.y_root,
                (*event).xbutton.button,
                menuContext,
                POST_AT_XY,
                event,
            );
        } else if (*event).type_0 == ButtonRelease {
            PostMenu(
                (*pCD).systemMenuSpec,
                pCD,
                (*event).xbutton.x_root,
                (*event).xbutton.y_root,
                (*event).xbutton.button,
                menuContext,
                POST_AT_XY | POST_TRAVERSAL_ON,
                event,
            );
        }
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Kill(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if !pCD.is_null() && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0 {
        let mut do_delete_window = ((*pCD).protocolFlags & PROTOCOL_WM_DELETE_WINDOW) as Boolean;
        let mut do_save_yourself = ((*pCD).protocolFlags & PROTOCOL_WM_SAVE_YOURSELF) as Boolean;
        if do_delete_window == 0 && do_save_yourself == 0 {
            XKillClient(wmGD.display, (*pCD).client as XID);
        } else if do_delete_window != 0 {
            SendClientMsg(
                (*pCD).client,
                wmGD.xa_WM_PROTOCOLS as ::core::ffi::c_long,
                wmGD.xa_WM_DELETE_WINDOW as ::core::ffi::c_long,
                GetFunctionTimestamp(event as *mut XButtonEvent),
                ::core::ptr::null_mut::<::core::ffi::c_long>(),
                0 as ::core::ffi::c_int,
            );
        } else if do_save_yourself != 0 {
            if AddWmTimer(
                TIMER_QUIT as ::core::ffi::c_uint,
                wmGD.quitTimeout as ::core::ffi::c_ulong,
                pCD,
            ) != 0
            {
                SendClientMsg(
                    (*pCD).client,
                    wmGD.xa_WM_PROTOCOLS as ::core::ffi::c_long,
                    wmGD.xa_WM_SAVE_YOURSELF as ::core::ffi::c_long,
                    CurrentTime as Time,
                    ::core::ptr::null_mut::<::core::ffi::c_long>(),
                    0 as ::core::ffi::c_int,
                );
                (*pCD).clientFlags |= CLIENT_TERMINATING;
            } else {
                XKillClient(wmGD.display, (*pCD).client as XID);
            }
        }
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Marquee_Selection(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if pCD.is_null() {
        StartMarqueeSelect(wmGD.pActiveSD, event);
        HandleMarqueeSelect(wmGD.pActiveSD, event);
    }
    return 0 as Boolean;
}
unsafe extern "C" fn RefreshByClearing(mut win: Window) {
    let mut status: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut root: Window = 0;
    let mut parent: Window = 0;
    let mut nchildren: ::core::ffi::c_uint = 0;
    let mut winChildren = ::core::ptr::null_mut::<Window>();
    XClearArea(
        wmGD.display,
        win,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        True,
    );
    status = XQueryTree(
        wmGD.display,
        win,
        &raw mut root,
        &raw mut parent,
        &raw mut winChildren,
        &raw mut nchildren,
    );
    if status != 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint) < nchildren {
            RefreshByClearing(*winChildren.offset(i as isize));
            i += 1;
        }
        if nchildren > 0 as ::core::ffi::c_uint {
            XFree(winChildren as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_Refresh(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut win: Window = 0;
    if wmGD.refreshByClearing != 0 {
        RefreshByClearing((*wmGD.pActiveSD).rootWindow);
    } else {
        win = XCreateWindow(
            wmGD.display,
            (*wmGD.pActiveSD).rootWindow,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*wmGD.pActiveSD).screen as isize))
            .width as ::core::ffi::c_uint,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*wmGD.pActiveSD).screen as isize))
            .height as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_int,
            InputOutput as ::core::ffi::c_uint,
            ::core::ptr::null_mut::<Visual>(),
            0 as ::core::ffi::c_ulong,
            NULL_0 as *mut XSetWindowAttributes,
        );
        XMapWindow(wmGD.display, win);
        XDestroyWindow(wmGD.display, win);
    }
    XFlush(wmGD.display);
    return True as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Resize(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if !pCD.is_null()
        && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_RESIZE != 0
        && ((*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE)
    {
        StartClientResize(pCD, event);
        HandleClientFrameResize(pCD, event);
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Restart(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if wmGD.showFeedback as ::core::ffi::c_long & WM_SHOW_FB_RESTART != 0 {
        ConfirmAction(wmGD.pActiveSD, RESTART_ACTION);
    } else {
        RestartWm(MWM_INFO_STARTUP_CUSTOM);
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn Do_Restart(mut dummy: Boolean) {
    RestartWm(MWM_INFO_STARTUP_CUSTOM);
}
#[no_mangle]
pub unsafe extern "C" fn RestartWm(mut startupFlags: ::core::ffi::c_long) {
    let mut pNextEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut scr: ::core::ffi::c_int = 0;
    scr = 0 as ::core::ffi::c_int;
    while scr < wmGD.numScreens {
        if (*wmGD.Screens.offset(scr as isize)).managed != 0 {
            SetMwmInfo(
                (*wmGD.Screens.offset(scr as isize)).rootWindow,
                startupFlags,
                0 as Window,
            );
            pNextEntry = (*wmGD.Screens.offset(scr as isize)).lastClient;
            while !pNextEntry.is_null() {
                if (*pNextEntry).type_0 == NORMAL_STATE {
                    if (*(*pNextEntry).pCD).clientFlags & CLIENT_WM_CLIENTS != 0 {
                        if (*(*pNextEntry).pCD).clientState != MINIMIZED_STATE {
                            XUnmapWindow(wmGD.display, (*(*pNextEntry).pCD).clientFrameWin);
                        }
                    } else {
                        DeFrameClient((*pNextEntry).pCD as *mut ClientData);
                    }
                }
                pNextEntry = (*pNextEntry).prevSibling as *mut ClientListEntry;
            }
        }
        scr += 1;
    }
    ResignFromSM();
    XSetInputFocus(
        wmGD.display,
        PointerRoot as Window,
        PointerRoot as ::core::ffi::c_int,
        CurrentTime as Time,
    );
    XSync(wmGD.display, False);
    execvp(*wmGD.argv, wmGD.argv as *const *mut ::core::ffi::c_char);
    Warning(
        b"The window manager restart failed. The window manager program could not \tbe found or could not be executed.\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    Do_Quit_Mwm(True as Boolean);
}
#[no_mangle]
pub unsafe extern "C" fn DeFrameClient(mut pCD: *mut ClientData) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut xoff: ::core::ffi::c_int = 0;
    let mut yoff: ::core::ffi::c_int = 0;
    let mut windowChanges = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    while !pCD.is_null() {
        if (*pCD).clientState != MINIMIZED_STATE {
            XUnmapWindow(wmGD.display, (*pCD).clientFrameWin);
        }
        if (*pCD).iconWindow != 0 && (*pCD).clientFlags & ICON_REPARENTED != 0 {
            XUnmapWindow(wmGD.display, (*pCD).iconWindow);
            XRemoveFromSaveSet(wmGD.display, (*pCD).iconWindow);
            XReparentWindow(
                wmGD.display,
                (*pCD).iconWindow,
                (*(*pCD).pSD).rootWindow,
                (*(*pCD).pWsList).iconX,
                (*(*pCD).pWsList).iconY,
            );
        }
        if (*pCD).maxConfig != 0 {
            x = (*pCD).maxX;
            y = (*pCD).maxY;
        } else {
            CalculateGravityOffset(pCD, &raw mut xoff, &raw mut yoff);
            x = (*pCD).clientX - xoff;
            y = (*pCD).clientY - yoff;
        }
        XUnmapWindow(wmGD.display, (*pCD).client);
        XRemoveFromSaveSet(wmGD.display, (*pCD).client);
        XReparentWindow(wmGD.display, (*pCD).client, (*(*pCD).pSD).rootWindow, x, y);
        if !(*pCD).transientChildren.is_null() {
            DeFrameClient((*pCD).transientChildren as *mut ClientData);
        }
        windowChanges.x = x;
        windowChanges.y = y;
        windowChanges.border_width = (*pCD).xBorderWidth;
        XConfigureWindow(
            wmGD.display,
            (*pCD).client,
            (CWBorderWidth | CWX | CWY) as ::core::ffi::c_uint,
            &raw mut windowChanges,
        );
        if !(*pCD).transientLeader.is_null() {
            pCD = (*pCD).transientSiblings as *mut ClientData;
        } else {
            pCD = ::core::ptr::null_mut::<ClientData>();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_Send_Msg(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut i: ::core::ffi::c_int = 0;
    if !pCD.is_null() && (*pCD).mwmMessagesCount != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < (*pCD).mwmMessagesCount {
            if *(*pCD).mwmMessages.offset(i as isize) == args as ::core::ffi::c_long {
                SendClientMsg(
                    (*pCD).client,
                    wmGD.xa_MWM_MESSAGES as ::core::ffi::c_long,
                    args as ::core::ffi::c_long,
                    CurrentTime as Time,
                    ::core::ptr::null_mut::<::core::ffi::c_long>(),
                    0 as ::core::ffi::c_int,
                );
                return 1 as Boolean;
            }
            i += 1;
        }
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Separator(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    return 1 as Boolean;
}
unsafe extern "C" fn ForceRaiseWindow(mut pcd: *mut ClientData) -> Boolean {
    let mut changes = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    let mut restack = False as Boolean;
    changes.stack_mode = Above;
    XConfigureWindow(
        wmGD.display,
        (*pcd).clientFrameWin,
        CWStackMode as ::core::ffi::c_uint,
        &raw mut changes,
    );
    return restack;
}
#[no_mangle]
pub unsafe extern "C" fn F_Raise(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut pNextEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut pStackEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut string = args;
    let mut flags = STACK_NORMAL;
    let mut pWS = (*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData;
    if !string.is_null() {
        if *string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32 {
            string = string.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as String_0;
            string = GetString(&raw mut string as *mut *mut ::core::ffi::c_uchar) as String_0;
            pStackEntry = ::core::ptr::null_mut::<ClientListEntry>();
            pNextEntry = (*wmGD.pActiveSD).clientList;
            while !pNextEntry.is_null() && {
                pEntry = FindClientNameMatch(
                    pNextEntry,
                    True as Boolean,
                    string,
                    F_GROUP_ALL as ::core::ffi::c_ulong,
                );
                !pEntry.is_null()
            } {
                pNextEntry = (*pEntry).nextSibling as *mut ClientListEntry;
                if ClientInWorkspace(pWS, (*pEntry).pCD as *mut ClientData) != 0 {
                    Do_Raise((*pEntry).pCD as *mut ClientData, pStackEntry, STACK_NORMAL);
                    pStackEntry = pEntry;
                }
            }
        } else if *string != 0 {
            let mut slen: ::core::ffi::c_uint = 0;
            let mut len: ::core::ffi::c_uint = 0;
            let mut index: ::core::ffi::c_uint = 0;
            slen = strlen(args as *const ::core::ffi::c_char).wrapping_sub(2 as size_t)
                as ::core::ffi::c_uint;
            index = 0 as ::core::ffi::c_uint;
            while index < slen {
                string = GetString(&raw mut string as *mut *mut ::core::ffi::c_uchar) as String_0;
                if string.is_null() {
                    break;
                }
                len = strlen(string as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
                if strcmp(
                    string as *const ::core::ffi::c_char,
                    b"within\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    flags |= STACK_WITHIN_FAMILY;
                } else if strcmp(
                    string as *const ::core::ffi::c_char,
                    b"freeFamily\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    flags |= STACK_FREE_FAMILY;
                }
                index = index.wrapping_add(len);
                string = args.offset(index.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                    as *mut ::core::ffi::c_char as String_0;
            }
            if ClientInWorkspace(pWS, pCD) != 0 {
                Do_Raise(pCD, NULL_0 as *mut ClientListEntry, flags);
            }
        }
    } else if !pCD.is_null() {
        if ClientInWorkspace(pWS, pCD) != 0 {
            Do_Raise(pCD, NULL_0 as *mut ClientListEntry, STACK_NORMAL);
        }
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn Do_Raise(
    mut pCD: *mut ClientData,
    mut pStackEntry: *mut ClientListEntry,
    mut flags: ::core::ffi::c_int,
) {
    let mut restackTransients: Boolean = 0;
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut pWS = (*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData;
    let mut bLeaderRestacked: Boolean = 0;
    if ClientInWorkspace(pWS, pCD) as ::core::ffi::c_int != 0
        && (pStackEntry.is_null()
            || ClientInWorkspace(pWS, (*pStackEntry).pCD as *mut ClientData) as ::core::ffi::c_int
                != 0)
    {
        SetClientWsIndex(pCD);
        if !pStackEntry.is_null() {
            SetClientWsIndex((*pStackEntry).pCD as *mut ClientData);
        }
    } else {
        return;
    }
    pcdLeader = if !(*pCD).transientLeader.is_null() {
        FindTransientTreeLeader(pCD)
    } else {
        pCD
    };
    if !(wmGD.systemModalActive as ::core::ffi::c_int != 0 && pcdLeader != wmGD.systemModalClient) {
        if (*pcdLeader).clientState == MINIMIZED_STATE
            && (*(*pcdLeader)
                .pWsList
                .offset((*pcdLeader).currentWsc as isize))
            .pIconBox
            .is_null()
        {
            if !dirtyStackEntry.is_null() {
                if !(*dirtyStackEntry).transientChildren.is_null()
                    || !(*dirtyStackEntry).transientLeader.is_null()
                {
                    RestackTransients(dirtyStackEntry);
                }
                dirtyStackEntry = ::core::ptr::null_mut::<ClientData>();
                dirtyLeader = ::core::ptr::null_mut::<ClientData>();
            }
            if !pStackEntry.is_null() {
                if (*pStackEntry).nextSibling != &raw mut (*pcdLeader).iconEntry {
                    StackWindow(
                        pWS,
                        &raw mut (*pcdLeader).iconEntry,
                        False as Boolean,
                        pStackEntry,
                    );
                    MoveEntryInList(
                        pWS,
                        &raw mut (*pcdLeader).iconEntry,
                        False as Boolean,
                        pStackEntry,
                    );
                }
            } else if (*wmGD.pActiveSD).clientList != &raw mut (*pcdLeader).iconEntry {
                StackWindow(
                    pWS,
                    &raw mut (*pcdLeader).iconEntry,
                    True as Boolean,
                    NULL_0 as *mut ClientListEntry,
                );
                MoveEntryInList(
                    pWS,
                    &raw mut (*pcdLeader).iconEntry,
                    True as Boolean,
                    NULL_0 as *mut ClientListEntry,
                );
            }
        } else {
            bLeaderRestacked = False as Boolean;
            if !(*pcdLeader).transientChildren.is_null()
                && (*pCD).secondariesOnTop == 0
                && wmGD.bSuspendSecondaryRestack == 0
                && !((*pCD).primaryModalCount != 0 || (*pCD).fullModalCount != 0)
            {
                if pCD != pcdLeader {
                    bLeaderRestacked = NormalizeTransientTreeStacking(pcdLeader);
                    if !(*pCD).transientChildren.is_null() {
                        bLeaderRestacked = (bLeaderRestacked as ::core::ffi::c_int
                            | BumpPrimaryToTop(pCD) as ::core::ffi::c_int)
                            as Boolean;
                    }
                } else {
                    bLeaderRestacked = BumpPrimaryToTop(pcdLeader);
                }
            }
            restackTransients = False as Boolean;
            if !dirtyStackEntry.is_null() {
                if !(*dirtyStackEntry).transientChildren.is_null()
                    || !(*dirtyStackEntry).transientLeader.is_null()
                {
                    RestackTransients(dirtyStackEntry);
                }
                dirtyStackEntry = ::core::ptr::null_mut::<ClientData>();
                dirtyLeader = ::core::ptr::null_mut::<ClientData>();
            }
            if (*pCD).transientLeader.is_null() || flags & STACK_WITHIN_FAMILY == 0 {
                if !pStackEntry.is_null() {
                    if (*pStackEntry).nextSibling != &raw mut (*pcdLeader).clientEntry {
                        StackWindow(
                            pWS,
                            &raw mut (*pcdLeader).clientEntry,
                            False as Boolean,
                            pStackEntry,
                        );
                        MoveEntryInList(
                            pWS,
                            &raw mut (*pcdLeader).clientEntry,
                            False as Boolean,
                            pStackEntry,
                        );
                    }
                } else if (*wmGD.pActiveSD).clientList != &raw mut (*pcdLeader).clientEntry {
                    StackWindow(
                        pWS,
                        &raw mut (*pcdLeader).clientEntry,
                        True as Boolean,
                        NULL_0 as *mut ClientListEntry,
                    );
                    MoveEntryInList(
                        pWS,
                        &raw mut (*pcdLeader).clientEntry,
                        True as Boolean,
                        NULL_0 as *mut ClientListEntry,
                    );
                }
            }
            if flags & STACK_FREE_FAMILY != 0
                && (!(*pCD).transientLeader.is_null() || !(*pCD).transientChildren.is_null())
            {
                dirtyStackEntry = pCD;
                dirtyLeader = pcdLeader;
                restackTransients = ForceRaiseWindow(pCD);
            } else {
                restackTransients = PutTransientOnTop(pCD);
            }
            if restackTransients as ::core::ffi::c_int != 0
                || bLeaderRestacked as ::core::ffi::c_int != 0
            {
                RestackTransients(pCD);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_Raise_Lower(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    if !pCD.is_null() {
        pcdLeader = if !(*pCD).transientLeader.is_null() {
            FindTransientTreeLeader(pCD)
        } else {
            pCD
        };
        if CheckIfClientObscuredByAny(pcdLeader) != 0 {
            F_Raise(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                pCD,
                NULL_0 as *mut XEvent,
            );
        } else if CheckIfClientObscuringAny(pcdLeader) as ::core::ffi::c_int != 0
            && !(wmGD.systemModalActive as ::core::ffi::c_int != 0
                && pcdLeader == wmGD.systemModalClient)
        {
            F_Lower(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                pcdLeader,
                NULL_0 as *mut XEvent,
            );
            if (*pcdLeader).secondariesOnTop as ::core::ffi::c_int == False
                && !(*pCD).transientLeader.is_null()
                && !((*pcdLeader).primaryModalCount != 0 || (*pcdLeader).fullModalCount != 0)
            {
                BumpPrimaryToTop(pcdLeader);
                RestackTransients(pcdLeader);
            }
        } else if (*pcdLeader).secondariesOnTop as ::core::ffi::c_int == False
            && !(*pcdLeader).transientChildren.is_null()
            && wmGD.systemModalActive == 0
            && !((*pcdLeader).primaryModalCount != 0 || (*pcdLeader).fullModalCount != 0)
        {
            if LeaderOnTop(pcdLeader) != 0 {
                BumpPrimaryToBottom(pcdLeader);
                RestackTransients(pcdLeader);
            } else {
                F_Raise(
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    pCD,
                    NULL_0 as *mut XEvent,
                );
                BumpPrimaryToTop(pcdLeader);
                RestackTransients(pcdLeader);
            }
        }
    }
    return True as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Refresh_Win(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut win: Window = 0;
    let mut w: ::core::ffi::c_uint = 0;
    let mut h: ::core::ffi::c_uint = 0;
    if !pCD.is_null()
        && ((*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE)
    {
        if (*pCD).clientState == NORMAL_STATE {
            w = (*pCD).clientWidth as ::core::ffi::c_uint;
            h = (*pCD).clientHeight as ::core::ffi::c_uint;
        } else {
            w = (*pCD).maxWidth as ::core::ffi::c_uint;
            h = (*pCD).maxHeight as ::core::ffi::c_uint;
        }
        if wmGD.refreshByClearing != 0 {
            RefreshByClearing((*pCD).clientFrameWin);
        } else {
            win = XCreateWindow(
                wmGD.display,
                (*pCD).clientBaseWin,
                (*pCD).matteWidth,
                (*pCD).matteWidth,
                w,
                h,
                0 as ::core::ffi::c_uint,
                0 as ::core::ffi::c_int,
                InputOutput as ::core::ffi::c_uint,
                ::core::ptr::null_mut::<Visual>(),
                0 as ::core::ffi::c_ulong,
                NULL_0 as *mut XSetWindowAttributes,
            );
            XMapWindow(wmGD.display, win);
            XDestroyWindow(wmGD.display, win);
        }
        XFlush(wmGD.display);
    }
    return True as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Set_Behavior(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    if wmGD.showFeedback as ::core::ffi::c_long & WM_SHOW_FB_BEHAVIOR != 0 {
        ConfirmAction(
            wmGD.pActiveSD,
            if wmGD.useStandardBehavior as ::core::ffi::c_int != 0 {
                CUSTOM_BEHAVIOR_ACTION
            } else {
                DEFAULT_BEHAVIOR_ACTION
            },
        );
    } else {
        RestartWm(if wmGD.useStandardBehavior as ::core::ffi::c_int != 0 {
            MWM_INFO_STARTUP_CUSTOM
        } else {
            MWM_INFO_STARTUP_STANDARD
        });
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn Do_Set_Behavior(mut dummy: Boolean) {
    RestartWm(if wmGD.useStandardBehavior as ::core::ffi::c_int != 0 {
        MWM_INFO_STARTUP_CUSTOM
    } else {
        MWM_INFO_STARTUP_STANDARD
    });
}
#[no_mangle]
pub unsafe extern "C" fn F_Set_Context(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    wmGD.requestContextWin = args as Window;
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Title(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Screen(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut dumwin: Window = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut dumint: ::core::ffi::c_int = 0;
    let mut dummask: ::core::ffi::c_uint = 0;
    let mut newscr = ::core::ptr::null_mut::<WmScreenData>();
    let mut scr: ::core::ffi::c_int = 0;
    let mut inc: ::core::ffi::c_int = 0;
    static mut PreviousScreen: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut pch: [::core::ffi::c_char; 80] = [0; 80];
    if PreviousScreen == -(1 as ::core::ffi::c_int) {
        PreviousScreen = (*(wmGD.display as _XPrivDisplay)).default_screen;
    }
    if strcmp(
        args as *const ::core::ffi::c_char,
        b"next\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        scr = (*wmGD.pActiveSD).screen + 1 as ::core::ffi::c_int;
        inc = 1 as ::core::ffi::c_int;
    } else if strcmp(
        args as *const ::core::ffi::c_char,
        b"prev\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        scr = (*wmGD.pActiveSD).screen - 1 as ::core::ffi::c_int;
        inc = -(1 as ::core::ffi::c_int);
    } else if strcmp(
        args as *const ::core::ffi::c_char,
        b"back\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        scr = PreviousScreen;
        inc = 0 as ::core::ffi::c_int;
    } else {
        scr = atoi(args as *const ::core::ffi::c_char);
        inc = 0 as ::core::ffi::c_int;
    }
    while newscr.is_null() {
        if scr < 0 as ::core::ffi::c_int {
            scr = wmGD.numScreens - 1 as ::core::ffi::c_int;
        } else if scr >= wmGD.numScreens {
            scr = 0 as ::core::ffi::c_int;
        }
        newscr = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
        if !((*wmGD.Screens.offset(scr as isize)).managed == 0) {
            continue;
        }
        if inc != 0 {
            scr += inc;
        } else {
            sprintf(
                &raw mut pch as *mut ::core::ffi::c_char,
                b"Unable to warp to unmanaged screen %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                scr,
            );
            Warning(
                (&raw mut pch as *mut ::core::ffi::c_char).offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
            );
            XBell(wmGD.display, 0 as ::core::ffi::c_int);
            return 0 as Boolean;
        }
    }
    if (*wmGD.pActiveSD).screen == scr {
        return 0 as Boolean;
    }
    PreviousScreen = (*wmGD.pActiveSD).screen;
    XQueryPointer(
        wmGD.display,
        (*wmGD.pActiveSD).rootWindow,
        &raw mut dumwin,
        &raw mut dumwin,
        &raw mut x,
        &raw mut y,
        &raw mut dumint,
        &raw mut dumint,
        &raw mut dummask,
    );
    XWarpPointer(
        wmGD.display,
        None_0 as Window,
        (*newscr).rootWindow,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        x,
        y,
    );
    if !newscr.is_null() && wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
        wmGD.pActiveSD = newscr;
        wmGD.queryScreen = False as Boolean;
        Do_Focus_Key(
            ::core::ptr::null_mut::<ClientData>(),
            GetFunctionTimestamp(event as *mut XButtonEvent),
            ALWAYS_SET_FOCUS,
        );
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn GetFunctionTimestamp(mut pEvent: *mut XButtonEvent) -> Time {
    let mut time: Time = 0;
    if !pEvent.is_null()
        && ((*pEvent).type_0 == ButtonPress
            || (*pEvent).type_0 == ButtonRelease
            || ((*pEvent).type_0 == KeyPress || (*pEvent).type_0 == KeyRelease))
    {
        time = (*pEvent).time;
    } else {
        time = GetTimestamp();
    }
    return time;
}
unsafe extern "C" fn GetEventInverseMask(mut event: *mut XEvent) -> ::core::ffi::c_uint {
    if event.is_null() {
        return 0 as ::core::ffi::c_uint;
    }
    if ButtonPress == (*event).type_0 {
        return ButtonReleaseMask as ::core::ffi::c_uint;
    } else {
        return 0 as ::core::ffi::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ClearDirtyStackEntry(mut pCD: *mut ClientData) {
    if pCD == dirtyStackEntry {
        dirtyStackEntry = ::core::ptr::null_mut::<ClientData>();
        dirtyLeader = ::core::ptr::null_mut::<ClientData>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_Next_Workspace(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pSD = wmGD.pActiveSD;
    let mut iwsx: ::core::ffi::c_int = 0;
    iwsx = 0 as ::core::ffi::c_int;
    while iwsx < (*pSD).numWorkspaces {
        if (*(*pSD).pWS.offset(iwsx as isize)).id == (*(*pSD).pActiveWS).id {
            iwsx += 1;
            break;
        } else {
            iwsx += 1;
        }
    }
    if iwsx >= (*pSD).numWorkspaces {
        iwsx = 0 as ::core::ffi::c_int;
    }
    ChangeToWorkspace((*pSD).pWS.offset(iwsx as isize) as *mut WmWorkspaceData);
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Prev_Workspace(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pSD = wmGD.pActiveSD;
    let mut iwsx: ::core::ffi::c_int = 0;
    iwsx = 0 as ::core::ffi::c_int;
    while iwsx < (*pSD).numWorkspaces {
        if (*(*pSD).pWS.offset(iwsx as isize)).id == (*(*pSD).pActiveWS).id {
            iwsx -= 1;
            break;
        } else {
            iwsx += 1;
        }
    }
    if iwsx < 0 as ::core::ffi::c_int {
        iwsx = (*pSD).numWorkspaces - 1 as ::core::ffi::c_int;
    }
    ChangeToWorkspace((*pSD).pWS.offset(iwsx as isize) as *mut WmWorkspaceData);
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Workspace_Presence(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut wsContext = NULL_0 as Context;
    if !pCD.is_null() && (*pCD).wsmFunctions & WSM_FUNCTION_OCCUPY_WS != 0 {
        if (*pCD).clientState == NORMAL_STATE {
            wsContext = F_CONTEXT_NORMAL as Context;
        } else if (*pCD).clientState == MAXIMIZED_STATE {
            wsContext = F_CONTEXT_MAXIMIZE as Context;
        } else {
            wsContext = F_CONTEXT_ICON as Context;
        }
        ShowPresenceBox(pCD, wsContext);
    }
    return 0 as Boolean;
}
