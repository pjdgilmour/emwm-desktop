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
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_0;
use crate::abi::WmWorkspaceData_WmBackdrop as WmWorkspaceData;
use crate::abi::*;
extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn XInternAtom(_: *mut Display, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int) -> Atom;
    fn XFlush(_: *mut Display) -> ::core::ffi::c_int;
    fn XFree(_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn XGetWindowProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_int,
        _: Atom,
        _: *mut Atom,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_ulong,
        _: *mut ::core::ffi::c_ulong,
        _: *mut *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
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
    fn XMoveWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XSaveContext(
        _: *mut Display,
        _: XID,
        _: XContext,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XtDestroyWidget(_: Widget);
    fn XtScreenOfObject(_: Widget) -> *mut Screen;
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtRealloc(_: *mut ::core::ffi::c_char, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    fn XmDestroyPixmap(screen: *mut Screen, pixmap: Pixmap) -> Boolean;
    fn XmStringCreateLocalized(text: String_0) -> XmString;
    fn XmStringCompare(a: XmString, b: XmString) -> Boolean;
    fn XmStringFree(string: XmString);
    static mut wmGD: WmGlobalData;
    fn GetNextIconPlace(pIPD: *mut IconPlacementData) -> ::core::ffi::c_int;
    fn CvtIconPlaceToPosition(
        pIPD: *mut IconPlacementData,
        place: ::core::ffi::c_int,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
    );
    fn FindIconPlace(
        pCD: *mut ClientData,
        pIPD: *mut IconPlacementData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn CvtIconPositionToPlace(
        pIPD: *mut IconPlacementData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn PositionToPlacementData(
        pWS: *mut WmWorkspaceData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    ) -> *mut IconPlacementData;
    fn InitWmWorkspace(pWS: *mut WmWorkspaceData, pSD: *mut WmScreenData);
    fn Warning(message: *mut ::core::ffi::c_char);
    fn GetXineramaScreenCount(count: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ShowPresenceBox(pClient: *mut ClientData, wsContext: Context);
    fn HidePresenceBox(pSD: *mut WmScreenData, userDismissed: Boolean);
    fn UpdatePresenceWorkspaces(pSD: *mut WmScreenData);
    fn F_Focus_Key(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn HideActiveIconText(pSD: *mut WmScreenData);
    fn MakeIcon(pWS: *mut WmWorkspaceData, pcd: *mut ClientData) -> Boolean;
    fn ReparentIconWindow(
        pcd: *mut ClientData,
        xOffset: ::core::ffi::c_int,
        yOffset: ::core::ffi::c_int,
    );
    fn AddIconBoxForWorkspace(pWS: *mut WmWorkspaceData);
    fn DeleteIconFromBox(pIBD: *mut IconBoxData, pCD: *mut ClientData);
    fn ShowClientIconState(pCD: *mut ClientData, newState: ::core::ffi::c_int);
    fn UnmapIconBoxes(pWS: *mut WmWorkspaceData);
    fn DestroyIconBox(pWS: *mut WmWorkspaceData);
    fn SetWorkspacePresence(
        propWindow: Window,
        pWsPresence: *mut Atom,
        cPresence: ::core::ffi::c_ulong,
    );
    fn HasProperty(pCD: *mut ClientData, property: Atom) -> Boolean;
    fn SetWorkspaceListProperty(pSD: *mut WmScreenData);
    fn SetCurrentWorkspaceProperty(pSD: *mut WmScreenData);
    fn SetWorkspaceInfoProperty(pWS: *mut WmWorkspaceData);
    fn DeleteWorkspaceInfoProperty(pWS: *mut WmWorkspaceData);
    fn GetString(linePP: *mut *mut ::core::ffi::c_uchar) -> *mut ::core::ffi::c_uchar;
    fn PlaceIconOnScreen(
        pCD: *mut ClientData,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
    );
    fn FindTransientTreeLeader(pcd: *mut ClientData) -> *mut ClientData;
    fn SetClientState(pCD: *mut ClientData, newState: ::core::ffi::c_int, setTime: Time);
    fn ExitWM(exitCode: ::core::ffi::c_int) -> !;
    fn ChangeBackdrop(pWS: *mut WmWorkspaceData);
    fn SetFocusIndication(pCD: *mut ClientData);
    fn UpdateEwmhWorkspaceProperties(_: *mut WmScreenData);
    fn UpdateEwmhActiveWorkspace(_: *mut WmScreenData, _: WorkspaceID);
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 61] = unsafe {
    ::core::mem::transmute::<[u8; 61], [::core::ffi::c_char; 61]>(
        *b"void PutClientIntoWorkspace(WmWorkspaceData *, ClientData *)\0",
    )
};
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const AnyButton: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CurrentTime: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ButtonPressMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ButtonReleaseMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const ButtonMotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 13 as ::core::ffi::c_int;
pub const AnyModifier: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int;
pub const GrabModeAsync: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_FUNC_MINIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WM_ERROR_EXIT_VALUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MANAGEW_WM_RESTART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const WS_ALLOC_AMOUNT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const WINDOW_ALLOC_AMOUNT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const WSM_FUNCTION_OCCUPY_WS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int;
pub const F_CONTEXT_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_CONTEXT_ICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const IB_MARGIN_HEIGHT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const IB_MARGIN_WIDTH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAX_WORKSPACE_COUNT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const BACKDROP_IMAGE_ALLOCED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const UNSEEN_STATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const WM_INITIALIZATION: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const ICON_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CONFIRM_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 12 as ::core::ffi::c_int;
pub const CLIENT_WM_CLIENTS: ::core::ffi::c_long = ICON_BOX | CONFIRM_BOX;
pub const AMAP_BEHAVIOR_MOVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ICON_IMAGE_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const ICON_ACTIVE_LABEL_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const NO_ICON_PLACE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut numResIDs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut pResIDs: *mut WorkspaceID = ::core::ptr::null::<WorkspaceID>() as *mut WorkspaceID;
#[no_mangle]
pub unsafe extern "C" fn ChangeToWorkspace(mut pNewWS: *mut WmWorkspaceData) {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut setFocus = ::core::ptr::null_mut::<ClientData>();
    let mut keepFocus = False as Boolean;
    let mut i: ::core::ffi::c_int = 0;
    let mut pSD = (*pNewWS).pSD;
    let mut pWsPCD = ::core::ptr::null_mut::<ClientData>();
    let mut wsContext = F_CONTEXT_NONE as Context;
    if pNewWS == (*pSD).pActiveWS {
        return;
    }
    (*(*pSD).pActiveWS).keyFocus = wmGD.keyboardFocus as *mut _ClientData;
    (*(*pSD).pActiveWS).nextKeyFocus = wmGD.nextKeyboardFocus as *mut _ClientData;
    wmGD.nextKeyboardFocus = ::core::ptr::null_mut::<ClientData>();
    (*pSD).pLastWS = (*pSD).pActiveWS as *mut _WmWorkspaceData;
    if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT
        && !wmGD.keyboardFocus.is_null()
        && ClientInWorkspace(pNewWS, wmGD.keyboardFocus) as ::core::ffi::c_int != 0
    {
        keepFocus = True as Boolean;
    }
    if !(*pSD).presence.shellW.is_null()
        && (*pSD).presence.onScreen as ::core::ffi::c_int != 0
        && (*pSD).presence.contextForClient == F_CONTEXT_ICON as Context
    {
        pWsPCD = (*pSD).presence.pCDforClient as *mut ClientData;
        wsContext = (*pSD).presence.contextForClient;
        HidePresenceBox(pSD, False as Boolean);
    }
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < (*(*pSD).pActiveWS).numClients {
        pCD = *(*(*pSD).pActiveWS).ppClients.offset(i as isize) as *mut ClientData;
        if ClientInWorkspace(pNewWS, pCD) == 0 {
            SetClientWsIndex(pCD);
            SetClientState(pCD, (*pCD).clientState | UNSEEN_STATE, CurrentTime as Time);
        }
        i += 1;
    }
    if (*pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART != 0
        && wmGD.activeIconTextDisplayed as ::core::ffi::c_int != 0
    {
        HideActiveIconText(pSD);
    }
    if (*pSD).useIconBox != 0 {
        UnmapIconBoxes((*pSD).pLastWS as *mut WmWorkspaceData);
    }
    (*pSD).pActiveWS = pNewWS as *mut _WmWorkspaceData;
    ChangeBackdrop(pNewWS);
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < (*pNewWS).numClients {
        pCD = *(*pNewWS).ppClients.offset(i as isize) as *mut ClientData;
        SetClientWsIndex(pCD);
        if (*pCD).clientState & UNSEEN_STATE != 0 {
            SetClientState(pCD, (*pCD).clientState & !UNSEEN_STATE, CurrentTime as Time);
        }
        if (*pCD).clientState == MINIMIZED_STATE
            && ((*(*pCD).pSD).useIconBox == 0
                || (*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
                    .pIconBox
                    .is_null())
        {
            XMoveWindow(
                wmGD.display,
                (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
                (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconX,
                (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconY,
            );
        }
        if (*pCD).iconWindow != 0 {
            let mut xOffset: ::core::ffi::c_uint = 0;
            let mut yOffset: ::core::ffi::c_uint = 0;
            if !(*pNewWS).pIconBox.is_null() {
                xOffset = IB_MARGIN_WIDTH as ::core::ffi::c_uint;
                yOffset = IB_MARGIN_HEIGHT as ::core::ffi::c_uint;
            } else {
                xOffset = 0 as ::core::ffi::c_uint;
                yOffset = 0 as ::core::ffi::c_uint;
            }
            if (*(*pCD).pSD).iconDecoration as ::core::ffi::c_long & ICON_IMAGE_PART != 0
                && (*pCD).iconWindow != 0
            {
                ReparentIconWindow(
                    pCD,
                    xOffset as ::core::ffi::c_int,
                    yOffset as ::core::ffi::c_int,
                );
            }
        }
        if pCD == (*pNewWS).keyFocus {
            setFocus = (*pNewWS).keyFocus as *mut ClientData;
        }
        if pCD == (*pNewWS).nextKeyFocus {
            wmGD.nextKeyboardFocus = (*pNewWS).nextKeyFocus as *mut ClientData;
        }
        i += 1;
    }
    if wsContext == F_CONTEXT_ICON as Context
        && ClientInWorkspace((*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData, pWsPCD)
            as ::core::ffi::c_int
            != 0
        || !(*pSD).presence.shellW.is_null()
            && (*pSD).presence.userDismissed == 0
            && ClientInWorkspace(
                (*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData,
                (*pSD).presence.pCDforClient as *mut ClientData,
            ) as ::core::ffi::c_int
                != 0
            && (*pSD).presence.contextForClient == F_CONTEXT_ICON as Context
    {
        ShowPresenceBox(
            (*pSD).presence.pCDforClient as *mut ClientData,
            F_CONTEXT_ICON as Context,
        );
    }
    SetCurrentWorkspaceProperty(pSD);
    UpdateEwmhActiveWorkspace(pSD, (*pNewWS).id);
    if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
        if keepFocus == 0 {
            F_Focus_Key(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                setFocus,
                ::core::ptr::null_mut::<XEvent>(),
            );
        }
        if !wmGD.keyboardFocus.is_null() {
            SetFocusIndication(wmGD.keyboardFocus);
        }
    }
    (*pNewWS).keyFocus = ::core::ptr::null_mut::<_ClientData>();
    (*pNewWS).nextKeyFocus = ::core::ptr::null_mut::<_ClientData>();
}
#[no_mangle]
pub unsafe extern "C" fn ChangeWorkspaceTitle(
    mut pWS: *mut WmWorkspaceData,
    mut pchTitle: *mut ::core::ffi::c_char,
) {
    let mut xmstr = ::core::ptr::null_mut::<__XmStringRec>();
    xmstr = XmStringCreateLocalized(pchTitle as String_0);
    XmStringFree((*pWS).title);
    (*pWS).title = xmstr;
    SetWorkspaceInfoProperty(pWS);
    XFlush(wmGD.display);
    UpdateEwmhWorkspaceProperties((*pWS).pSD);
}
#[no_mangle]
pub unsafe extern "C" fn UpdateWorkspacePresenceProperty(mut pCD: *mut ClientData) {
    let mut pPresence = ::core::ptr::null_mut::<Atom>();
    let mut cPresence = 0 as ::core::ffi::c_ulong;
    let mut i: ::core::ffi::c_ulong = 0;
    if wmGD.useStandardBehavior != 0 {
        return;
    }
    pPresence = XtMalloc(
        ((*pCD).numInhabited as usize).wrapping_mul(::core::mem::size_of::<Atom>() as usize)
            as Cardinal,
    ) as *mut Atom;
    if pPresence.is_null() {
        Warning(
            b"Insufficient memory for workspace presence property\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    if (*pCD).putInAll != 0 {
        *pPresence.offset(0 as ::core::ffi::c_int as isize) = wmGD.xa_ALL_WORKSPACES;
        cPresence = 1 as ::core::ffi::c_ulong;
    } else {
        i = 0 as ::core::ffi::c_ulong;
        while i < (*pCD).numInhabited as ::core::ffi::c_ulong {
            *pPresence.offset(i as isize) = (*(*pCD).pWsList.offset(i as isize)).wsID as Atom;
            i = i.wrapping_add(1);
        }
        cPresence = (*pCD).numInhabited as ::core::ffi::c_ulong;
    }
    SetWorkspacePresence((*pCD).client, pPresence, cPresence);
    XtFree(pPresence as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn AddPersistentWindows(mut pWS: *mut WmWorkspaceData) {
    let mut pSD = (*pWS).pSD;
    let mut pCLE = ::core::ptr::null_mut::<ClientListEntry>();
    pCLE = (*pSD).clientList;
    while !pCLE.is_null() {
        if (*pCLE).type_0 == NORMAL_STATE && (*(*pCLE).pCD).putInAll as ::core::ffi::c_int != 0 {
            AddClientToWorkspaces(
                (*pCLE).pCD as *mut ClientData,
                &raw mut (*pWS).id,
                1 as ::core::ffi::c_uint,
            );
        }
        if pCLE == (*pSD).lastClient {
            break;
        }
        pCLE = (*pCLE).nextSibling as *mut ClientListEntry;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CreateWorkspace(
    mut pSD: *mut WmScreenData,
    mut pchTitle: *mut ::core::ffi::c_uchar,
) -> *mut WmWorkspaceData {
    let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iActiveWS: ::core::ffi::c_int = 0;
    if (*pSD).numWsDataAllocated <= (*pSD).numWorkspaces {
        iActiveWS =
            (*pSD).pActiveWS.offset_from((*pSD).pWS) as ::core::ffi::c_long as ::core::ffi::c_int;
        (*pSD).numWsDataAllocated += WS_ALLOC_AMOUNT;
        (*pSD).pWS = XtRealloc(
            (*pSD).pWS as *mut ::core::ffi::c_char,
            ((*pSD).numWsDataAllocated as usize)
                .wrapping_mul(::core::mem::size_of::<WmWorkspaceData>() as usize)
                as Cardinal,
        ) as *mut WmWorkspaceData as *mut _WmWorkspaceData;
        (*pSD).pActiveWS =
            (*pSD).pWS.offset(iActiveWS as isize) as *mut _WmWorkspaceData as *mut _WmWorkspaceData;
    }
    pWS = (*pSD).pWS.offset((*pSD).numWorkspaces as isize) as *mut _WmWorkspaceData
        as *mut WmWorkspaceData;
    string = GenerateWorkspaceName(pSD, (*pSD).numWorkspaces) as String_0;
    (*pWS).name = (if !string.is_null() {
        strcpy(
            XtMalloc(
                (strlen(string as *const ::core::ffi::c_char) as Cardinal)
                    .wrapping_add(1 as Cardinal),
            ),
            string as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }) as String_0;
    InitWmWorkspace(pWS, pSD);
    if !pchTitle.is_null() {
        if !(*pWS).title.is_null() {
            XmStringFree((*pWS).title);
        }
        (*pWS).title = XmStringCreateLocalized(pchTitle as String_0);
    }
    (*pSD).numWorkspaces += 1;
    SetWorkspaceInfoProperty(pWS);
    SetWorkspaceListProperty(pSD);
    if (*pSD).useIconBox != 0 {
        AddIconBoxForWorkspace(pWS);
    }
    AddPersistentWindows(pWS);
    UpdatePresenceWorkspaces(pSD);
    UpdateEwmhWorkspaceProperties(pSD);
    return pWS;
}
#[no_mangle]
pub unsafe extern "C" fn DeleteWorkspace(mut pWS: *mut WmWorkspaceData) {
    let mut pWSdest = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut iNextWs: ::core::ffi::c_int = 0;
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut pSD = (*pWS).pSD;
    if (*pSD).numWorkspaces > 1 as ::core::ffi::c_int {
        iNextWs = 0 as ::core::ffi::c_int;
        while iNextWs < (*pSD).numWorkspaces {
            if (*(*pSD).pWS.offset(iNextWs as isize)).id == (*pWS).id {
                iNextWs += 1;
                break;
            } else {
                iNextWs += 1;
            }
        }
        if iNextWs >= (*pSD).numWorkspaces {
            iNextWs = 0 as ::core::ffi::c_int;
        }
        if pWS == (*wmGD.pActiveSD).pActiveWS {
            pWSdest = (*pSD).pWS.offset(iNextWs as isize) as *mut _WmWorkspaceData
                as *mut WmWorkspaceData;
        } else {
            pWSdest = (*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData;
        }
        while (*pWS).numClients > 0 as ::core::ffi::c_uint {
            pCD = *(*pWS).ppClients.offset(0 as ::core::ffi::c_int as isize) as *mut ClientData;
            if (*pCD).numInhabited == 1 as ::core::ffi::c_int {
                if (*pCD).clientFlags & (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int == 0
                {
                    AddClientToWorkspaces(pCD, &raw mut (*pWSdest).id, 1 as ::core::ffi::c_uint);
                }
            }
            RemoveClientFromWorkspaces(pCD, &raw mut (*pWS).id, 1 as ::core::ffi::c_uint);
        }
        if pWS == (*wmGD.pActiveSD).pActiveWS {
            ChangeToWorkspace(pWSdest);
        }
        if (*pSD).useIconBox != 0 {
            DestroyIconBox(pWS);
        }
        DeleteWorkspaceInfoProperty(pWS);
        if (*pWS).backdrop.imagePixmap != 0 {
            XmDestroyPixmap(
                XtScreenOfObject((*pWS).workspaceTopLevelW),
                (*pWS).backdrop.imagePixmap,
            ) == 0;
        }
        if (*pWS).backdrop.flags as ::core::ffi::c_long & BACKDROP_IMAGE_ALLOCED != 0
            && !(*pWS).backdrop.image.is_null()
        {
            free((*pWS).backdrop.image as *mut ::core::ffi::c_void);
        }
        if wmGD.iconAutoPlace != 0 {
            let mut nxs: ::core::ffi::c_int = 0;
            let mut i: ::core::ffi::c_int = 0;
            if GetXineramaScreenCount(&raw mut nxs) == 0 {
                nxs = 1 as ::core::ffi::c_int;
            }
            i = 0 as ::core::ffi::c_int;
            while i < nxs {
                if !(*(*pWS).IPData.offset(i as isize)).placeList.is_null() {
                    XtFree(
                        (*(*pWS).IPData.offset(i as isize)).placeList as *mut ::core::ffi::c_char,
                    );
                }
                if !(*(*pWS).IPData.offset(i as isize)).placementRowY.is_null() {
                    XtFree(
                        (*(*pWS).IPData.offset(i as isize)).placementRowY
                            as *mut ::core::ffi::c_char,
                    );
                }
                if !(*(*pWS).IPData.offset(i as isize)).placementColX.is_null() {
                    XtFree(
                        (*(*pWS).IPData.offset(i as isize)).placementColX
                            as *mut ::core::ffi::c_char,
                    );
                }
                i += 1;
            }
        }
        XtFree((*pWS).name);
        XmStringFree((*pWS).title);
        XtFree((*pWS).ppClients as *mut ::core::ffi::c_char);
        if !(*pWS).iconBoxGeometry.is_null() {
            XtFree((*pWS).iconBoxGeometry);
        }
        XtDestroyWidget((*pWS).workspaceTopLevelW);
        if iNextWs > 0 as ::core::ffi::c_int {
            let mut pWSdest_0 = ::core::ptr::null_mut::<WmWorkspaceData>();
            let mut pWSsrc = ::core::ptr::null_mut::<WmWorkspaceData>();
            let mut j: ::core::ffi::c_int = 0;
            pWSdest_0 = pWS;
            pWSsrc = (*pSD).pWS.offset(iNextWs as isize) as *mut _WmWorkspaceData
                as *mut WmWorkspaceData;
            j = iNextWs;
            while j < (*pSD).numWorkspaces {
                memcpy(
                    pWSdest_0 as *mut ::core::ffi::c_void,
                    pWSsrc as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<WmWorkspaceData>() as size_t,
                );
                if (*pSD).pActiveWS == pWSsrc {
                    (*pSD).pActiveWS = pWSdest_0 as *mut _WmWorkspaceData;
                }
                pWSdest_0 = pWSdest_0.offset(1);
                pWSsrc = pWSsrc.offset(1);
                j += 1;
            }
        }
        (*pSD).numWorkspaces -= 1;
        SetWorkspaceListProperty(pSD);
        UpdatePresenceWorkspaces(pSD);
        UpdateEwmhWorkspaceProperties(pSD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetClientWorkspaceInfo(
    mut pCD: *mut ClientData,
    mut manageFlags: ::core::ffi::c_long,
) -> Boolean {
    let mut pIDs = ::core::ptr::null_mut::<Atom>();
    let mut i: ::core::ffi::c_int = 0;
    let mut numIDs = 0 as ::core::ffi::c_uint;
    let mut bAll: Boolean = 0;
    (*pCD).pWsList = XtMalloc(
        ((*(*pCD).pSD).numWorkspaces as usize)
            .wrapping_mul(::core::mem::size_of::<WsClientData>() as usize) as Cardinal,
    ) as *mut WsClientData as *mut _WsClientData;
    if (*pCD).pWsList.is_null() {
        Warning(
            b"Insufficient memory for client data\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as Boolean;
    }
    (*pCD).currentWsc = 0 as ::core::ffi::c_int;
    (*pCD).pWorkspaceHints = ::core::ptr::null_mut::<WorkspaceID>();
    (*pCD).sizeWsList = (*(*pCD).pSD).numWorkspaces;
    (*pCD).numInhabited = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*(*pCD).pSD).numWorkspaces {
        (*(*pCD).pWsList.offset(i as isize)).wsID = None_0 as WorkspaceID;
        (*(*pCD).pWsList.offset(i as isize)).iconPlace = NO_ICON_PLACE;
        let ref mut fresh6 = (*(*pCD).pWsList.offset(i as isize)).IPData;
        *fresh6 = ::core::ptr::null_mut::<IconPlacementData>();
        (*(*pCD).pWsList.offset(i as isize)).iconX = 0 as ::core::ffi::c_int;
        (*(*pCD).pWsList.offset(i as isize)).iconY = 0 as ::core::ffi::c_int;
        (*(*pCD).pWsList.offset(i as isize)).iconFrameWin = None_0 as Window;
        let ref mut fresh7 = (*(*pCD).pWsList.offset(i as isize)).pIconBox;
        *fresh7 = ::core::ptr::null_mut::<IconBoxData>();
        i += 1;
    }
    bAll = False as Boolean;
    (*pCD).putInAll = bAll;
    if (*pCD).desktopSurface != 0 {
        (*pCD).putInAll = True as Boolean;
        i = 0 as ::core::ffi::c_int;
        while i < (*(*pCD).pSD).numWorkspaces {
            PutClientIntoWorkspace(
                (*(*pCD).pSD).pWS.offset(i as isize) as *mut WmWorkspaceData,
                pCD,
            );
            i += 1;
        }
        return True as Boolean;
    }
    if (*pCD).client != 0
        && (!(*pCD).transientLeader.is_null()
            && GetLeaderPresence(pCD, &raw mut pIDs, &raw mut numIDs) as ::core::ffi::c_int != 0
            || manageFlags & MANAGEW_WM_RESTART != 0
                && GetMyOwnPresence(pCD, &raw mut pIDs, &raw mut numIDs) as ::core::ffi::c_int != 0)
        && numIDs != 0
    {
        (*pCD).putInAll = bAll;
        ProcessWorkspaceHintList(pCD, pIDs as *mut WorkspaceID, numIDs);
    }
    if (*pCD).numInhabited == 0 as ::core::ffi::c_int {
        if !(*pCD).occupyWorkspaces.is_null()
            && ConvertNamesToIDs(
                (*pCD).pSD,
                (*pCD).occupyWorkspaces as *mut ::core::ffi::c_uchar,
                &raw mut pIDs,
                &raw mut numIDs,
            ) as ::core::ffi::c_int
                != 0
        {
            ProcessWorkspaceHintList(pCD, pIDs as *mut WorkspaceID, numIDs);
        } else if GetMyOwnPresence(pCD, &raw mut pIDs, &raw mut numIDs) != 0 {
            CheckForPutInAllRequest(pCD, pIDs, numIDs);
            XtFree(pIDs as *mut ::core::ffi::c_char);
            if (*pCD).putInAll != 0 {
                i = 0 as ::core::ffi::c_int;
                while i < (*(*pCD).pSD).numWorkspaces {
                    PutClientIntoWorkspace(
                        (*(*pCD).pSD).pWS.offset(i as isize) as *mut WmWorkspaceData,
                        pCD,
                    );
                    i += 1;
                }
            } else {
                PutClientIntoWorkspace((*(*pCD).pSD).pActiveWS as *mut WmWorkspaceData, pCD);
            }
        } else {
            PutClientIntoWorkspace((*(*pCD).pSD).pActiveWS as *mut WmWorkspaceData, pCD);
        }
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ConvertNamesToIDs(
    mut pSD: *mut WmScreenData,
    mut pchIn: *mut ::core::ffi::c_uchar,
    mut ppAtoms: *mut *mut WorkspaceID,
    mut pNumAtoms: *mut ::core::ffi::c_uint,
) -> Boolean {
    let mut pchLocal = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut pch = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut pchName = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut num = 0 as ::core::ffi::c_int;
    let mut numLocalIDs: ::core::ffi::c_int = 0;
    let mut pLocalIDs = ::core::ptr::null_mut::<WorkspaceID>();
    if pchIn.is_null() {
        return False as Boolean;
    }
    pLocalIDs = XtMalloc(
        (WS_ALLOC_AMOUNT as usize).wrapping_mul(::core::mem::size_of::<WorkspaceID>() as usize)
            as Cardinal,
    ) as *mut WorkspaceID;
    if pLocalIDs.is_null() {
        Warning(
            b"Insufficient Memory (ConvertNamesToIDs)\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        ExitWM(WM_ERROR_EXIT_VALUE);
    }
    numLocalIDs = WS_ALLOC_AMOUNT;
    pchLocal = (if !(pchIn as *mut ::core::ffi::c_char).is_null() {
        strcpy(
            XtMalloc(
                (strlen(pchIn as *mut ::core::ffi::c_char) as Cardinal).wrapping_add(1 as Cardinal),
            ),
            pchIn as *mut ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }) as *mut ::core::ffi::c_uchar;
    pch = pchLocal;
    loop {
        pchName = GetString(&raw mut pch);
        if pchName.is_null() {
            break;
        }
        let mut iwsx: ::core::ffi::c_int = 0;
        let ref mut fresh8 = *pLocalIDs.offset(num as isize);
        *fresh8 = XInternAtom(wmGD.display, pchName as *mut ::core::ffi::c_char, True);
        if *fresh8 != 0 {
            iwsx = 0 as ::core::ffi::c_int;
            while iwsx < (*pSD).numWorkspaces {
                if (*(*pSD).pWS.offset(iwsx as isize)).id == *pLocalIDs.offset(num as isize)
                    || *pLocalIDs.offset(num as isize) == wmGD.xa_ALL_WORKSPACES
                {
                    num += 1;
                }
                iwsx += 1;
            }
        } else {
            let mut xms = ::core::ptr::null_mut::<__XmStringRec>();
            xms = XmStringCreateLocalized(pchName as String_0);
            iwsx = 0 as ::core::ffi::c_int;
            while iwsx < (*pSD).numWorkspaces {
                if XmStringCompare(xms, (*(*pSD).pWS.offset(iwsx as isize)).title) != 0 {
                    break;
                }
                iwsx += 1;
            }
            XmStringFree(xms);
            if iwsx < (*pSD).numWorkspaces {
                *pLocalIDs.offset(num as isize) = (*(*pSD).pWS.offset(iwsx as isize)).id;
                num += 1;
            }
        }
        if num >= numLocalIDs {
            numLocalIDs += WS_ALLOC_AMOUNT;
            pLocalIDs = XtRealloc(
                pLocalIDs as *mut ::core::ffi::c_char,
                (numLocalIDs as usize).wrapping_mul(::core::mem::size_of::<WorkspaceID>() as usize)
                    as Cardinal,
            ) as *mut WorkspaceID;
            if pLocalIDs.is_null() {
                Warning(
                    b"Insufficient Memory (ConvertNamesToIDs)\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                ExitWM(WM_ERROR_EXIT_VALUE);
            }
        }
    }
    XtFree(pchLocal as *mut ::core::ffi::c_char);
    if num != 0 {
        *ppAtoms = pLocalIDs;
        *pNumAtoms = num as ::core::ffi::c_uint;
    } else {
        if !pchIn.is_null() {
            Warning(
                b"occupyWorkspaces set for client, but contains no valid workspace IDs\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        XtFree(pLocalIDs as *mut ::core::ffi::c_char);
    }
    return (if num > 0 as ::core::ffi::c_int {
        True
    } else {
        False
    }) as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn CheckForPutInAllRequest(
    mut pCD: *mut ClientData,
    mut pIDs: *mut Atom,
    mut numIDs: ::core::ffi::c_uint,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < numIDs && (*pCD).putInAll == 0 {
        if *pIDs.offset(i as isize) == wmGD.xa_ALL_WORKSPACES {
            (*pCD).putInAll = True as Boolean;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn PutClientIntoWorkspace(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
) {
    let mut i = (*pCD).numInhabited;
    let mut iAdded: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    if (*pCD).sizeWsList < (*(*pCD).pSD).numWorkspaces {
        iAdded = (*(*pCD).pSD).numWorkspaces - (*pCD).sizeWsList;
        (*pCD).sizeWsList = (*(*pCD).pSD).numWorkspaces;
        (*pCD).pWsList = XtRealloc(
            (*pCD).pWsList as *mut ::core::ffi::c_char,
            ((*(*pCD).pSD).numWorkspaces as usize)
                .wrapping_mul(::core::mem::size_of::<WsClientData>() as usize)
                as Cardinal,
        ) as *mut WsClientData as *mut _WsClientData;
        j = (*pCD).sizeWsList - 1 as ::core::ffi::c_int;
        j = 1 as ::core::ffi::c_int;
        while j <= iAdded {
            k = (*pCD).sizeWsList - j;
            (*(*pCD).pWsList.offset(k as isize)).iconPlace = NO_ICON_PLACE;
            let ref mut fresh0 = (*(*pCD).pWsList.offset(k as isize)).IPData;
            *fresh0 = ::core::ptr::null_mut::<IconPlacementData>();
            (*(*pCD).pWsList.offset(k as isize)).iconX = 0 as ::core::ffi::c_int;
            (*(*pCD).pWsList.offset(k as isize)).iconY = 0 as ::core::ffi::c_int;
            (*(*pCD).pWsList.offset(k as isize)).iconFrameWin = 0 as ::core::ffi::c_int as Window;
            let ref mut fresh1 = (*(*pCD).pWsList.offset(k as isize)).pIconBox;
            *fresh1 = ::core::ptr::null_mut::<IconBoxData>();
            j += 1;
        }
    }
    '_c2rust_label: {
        if i < (*(*pCD).pSD).numWorkspaces {
        } else {
            __assert_fail(
                b"i < pCD->pSD->numWorkspaces\0" as *const u8 as *const ::core::ffi::c_char,
                b"WmWrkspace.c\0" as *const u8 as *const ::core::ffi::c_char,
                1025 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    (*(*pCD).pWsList.offset(i as isize)).wsID = (*pWS).id;
    (*pCD).numInhabited += 1;
    if (*pCD).clientFlags & WM_INITIALIZATION == 0 {
        InsureIconForWorkspace(pWS, pCD);
    }
    AddClientToWsList(pWS, pCD);
}
#[no_mangle]
pub unsafe extern "C" fn TakeClientOutOfWorkspace(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
) {
    let mut ixA: ::core::ffi::c_int = 0;
    let mut Copying = False as Boolean;
    let mut pWsc = ::core::ptr::null_mut::<WsClientData>();
    if !pWS.is_null() && !pCD.is_null() && ClientInWorkspace(pWS, pCD) as ::core::ffi::c_int != 0 {
        if (*pCD).transientLeader.is_null() {
            pWsc = GetWsClientData(pWS, pCD);
            if (*(*pCD).pSD).useIconBox as ::core::ffi::c_int != 0
                && !(*pWsc).pIconBox.is_null()
                && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0
            {
                DeleteIconFromBox((*pWS).pIconBox, pCD);
            } else if wmGD.iconAutoPlace != 0 {
                if (*pWsc).iconPlace != NO_ICON_PLACE
                    && (*(*(*pWsc).IPData)
                        .placeList
                        .offset((*pWsc).iconPlace as isize))
                    .pCD == pCD
                {
                    let ref mut fresh4 = (*(*(*pWsc).IPData)
                        .placeList
                        .offset((*pWsc).iconPlace as isize))
                    .pCD;
                    *fresh4 = ::core::ptr::null_mut::<_ClientData>();
                    (*pWsc).iconPlace = NO_ICON_PLACE;
                    (*pWsc).IPData = ::core::ptr::null_mut::<IconPlacementData>();
                }
            }
        }
        ixA = 0 as ::core::ffi::c_int;
        while ixA < (*pCD).numInhabited {
            if Copying != 0 {
                memcpy(
                    (*pCD)
                        .pWsList
                        .offset((ixA - 1 as ::core::ffi::c_int) as isize)
                        as *mut _WsClientData as *mut ::core::ffi::c_void,
                    (*pCD).pWsList.offset(ixA as isize) as *mut _WsClientData
                        as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<WsClientData>() as size_t,
                );
            } else if (*(*pCD).pWsList.offset(ixA as isize)).wsID == (*pWS).id {
                Copying = True as Boolean;
            }
            ixA += 1;
        }
        (*pCD).numInhabited -= 1;
        RemoveClientFromWsList(pWS, pCD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetWorkspaceData(
    mut pSD: *mut WmScreenData,
    mut wsID: WorkspaceID,
) -> *mut WmWorkspaceData {
    let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pSD).numWorkspaces {
        if (*(*pSD).pWS.offset(i as isize)).id == wsID {
            pWS = (*pSD).pWS.offset(i as isize) as *mut _WmWorkspaceData as *mut WmWorkspaceData;
            break;
        } else {
            i += 1;
        }
    }
    return pWS;
}
#[no_mangle]
pub unsafe extern "C" fn GenerateWorkspaceName(
    mut pSD: *mut WmScreenData,
    mut wsnum: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    static mut nameReturned: [::core::ffi::c_uchar; 20] = [0; 20];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i <= (*pSD).numWorkspaces {
        snprintf(
            &raw mut nameReturned as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            20 as size_t,
            b"ws%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        if DuplicateWorkspaceName(
            pSD,
            &raw mut nameReturned as *mut ::core::ffi::c_uchar,
            wsnum,
        ) == 0
        {
            break;
        }
        i += 1;
    }
    return &raw mut nameReturned as *mut ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn InWindowList(
    mut w: Window,
    mut wl: *mut Window,
    mut num: ::core::ffi::c_int,
) -> Boolean {
    let mut i: ::core::ffi::c_int = 0;
    let mut rval = False as Boolean;
    i = 0 as ::core::ffi::c_int;
    while i < num && rval == 0 {
        if w == *wl.offset(i as isize) {
            rval = True as Boolean;
        }
        i += 1;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn ClientInWorkspace(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
) -> Boolean {
    let mut i: ::core::ffi::c_int = 0;
    let mut rval = False as Boolean;
    i = 0 as ::core::ffi::c_int;
    while i < (*pCD).numInhabited && rval == 0 {
        if (*pWS).id == (*(*pCD).pWsList.offset(i as isize)).wsID {
            rval = True as Boolean;
        }
        i += 1;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn GetWsClientData(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
) -> *mut WsClientData {
    let mut i: ::core::ffi::c_int = 0;
    let mut pWsc = ::core::ptr::null_mut::<WsClientData>();
    i = 0 as ::core::ffi::c_int;
    while i < (*pCD).numInhabited && pWsc.is_null() {
        if (*pWS).id == (*(*pCD).pWsList.offset(i as isize)).wsID {
            pWsc = (*pCD).pWsList.offset(i as isize) as *mut _WsClientData as *mut WsClientData;
        }
        i += 1;
    }
    if pWsc.is_null() {
        pWsc = (*pCD).pWsList.offset(0 as ::core::ffi::c_int as isize) as *mut _WsClientData
            as *mut WsClientData;
    }
    return pWsc;
}
#[no_mangle]
pub unsafe extern "C" fn SetClientWsIndex(mut pCD: *mut ClientData) {
    let mut i: ::core::ffi::c_int = 0;
    let mut pWS = (*(*pCD).pSD).pActiveWS as *mut WmWorkspaceData;
    i = 0 as ::core::ffi::c_int;
    while i < (*pCD).numInhabited {
        if (*pWS).id == (*(*pCD).pWsList.offset(i as isize)).wsID {
            break;
        }
        i += 1;
    }
    if i >= (*pCD).numInhabited {
        i = 0 as ::core::ffi::c_int;
    }
    (*pCD).currentWsc = i;
}
unsafe extern "C" fn InsureUniqueWorkspaceHints(mut pCD: *mut ClientData) {
    let mut next: ::core::ffi::c_int = 0;
    let mut trail: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut pID = ::core::ptr::null_mut::<WorkspaceID>();
    let mut duplicate: Boolean = 0;
    if (*pCD).numWorkspaceHints < 2 as ::core::ffi::c_int {
        return;
    }
    pID = (*pCD).pWorkspaceHints;
    trail = 0 as ::core::ffi::c_int;
    next = 1 as ::core::ffi::c_int;
    while next < (*pCD).numWorkspaceHints {
        duplicate = False as Boolean;
        i = 0 as ::core::ffi::c_int;
        while i < next {
            if *pID.offset(next as isize) == *pID.offset(i as isize) {
                duplicate = True as Boolean;
                break;
            } else {
                i += 1;
            }
        }
        if duplicate != 0 {
            next += 1;
        } else {
            trail += 1;
            if next > trail {
                *pID.offset(trail as isize) = *pID.offset(next as isize);
            }
        }
        next += 1;
    }
    (*pCD).numWorkspaceHints = trail + 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWorkspaceHintList(
    mut pCD: *mut ClientData,
    mut pIDs: *mut WorkspaceID,
    mut numIDs: ::core::ffi::c_uint,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
    if numIDs > 0 as ::core::ffi::c_uint {
        (*pCD).pWorkspaceHints = pIDs;
        (*pCD).numWorkspaceHints = numIDs as ::core::ffi::c_int;
        InsureUniqueWorkspaceHints(pCD);
        numIDs = (*pCD).numWorkspaceHints as ::core::ffi::c_uint;
        if !(*pCD).pWorkspaceHints.is_null() {
            CheckForPutInAllRequest(pCD, pIDs as *mut Atom, numIDs);
            if (*pCD).putInAll != 0 {
                i = 0 as ::core::ffi::c_int;
                while i < (*(*pCD).pSD).numWorkspaces {
                    PutClientIntoWorkspace(
                        (*(*pCD).pSD).pWS.offset(i as isize) as *mut WmWorkspaceData,
                        pCD,
                    );
                    i += 1;
                }
            } else {
                i = 0 as ::core::ffi::c_int;
                while (i as ::core::ffi::c_uint) < numIDs {
                    pWS = GetWorkspaceData((*pCD).pSD, *(*pCD).pWorkspaceHints.offset(i as isize));
                    if !pWS.is_null() {
                        PutClientIntoWorkspace(pWS, pCD);
                    }
                    i += 1;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn RemoveSingleClientFromWorkspaces(
    mut pCD: *mut ClientData,
    mut pIDs: *mut WorkspaceID,
    mut numIDs: ::core::ffi::c_uint,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < numIDs {
        pWS = GetWorkspaceData((*pCD).pSD, *pIDs.offset(i as isize));
        if !pWS.is_null() && ClientInWorkspace(pWS, pCD) as ::core::ffi::c_int != 0 {
            if pWS == (*(*pCD).pSD).pActiveWS
                && (*pCD).transientLeader.is_null()
                && (*pCD).clientState & UNSEEN_STATE == 0
            {
                SetClientState(pCD, (*pCD).clientState | UNSEEN_STATE, CurrentTime as Time);
            }
            TakeClientOutOfWorkspace(pWS, pCD);
            UpdateWorkspacePresenceProperty(pCD);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RemoveSubtreeFromWorkspaces(
    mut pCD: *mut ClientData,
    mut pIDs: *mut WorkspaceID,
    mut numIDs: ::core::ffi::c_uint,
) {
    let mut pNext = ::core::ptr::null_mut::<ClientData>();
    pNext = (*pCD).transientChildren as *mut ClientData;
    while !pNext.is_null() {
        if !(*pNext).transientChildren.is_null() {
            RemoveSubtreeFromWorkspaces(pNext, pIDs, numIDs);
        } else {
            RemoveSingleClientFromWorkspaces(pNext, pIDs, numIDs);
        }
        pNext = (*pNext).transientSiblings as *mut ClientData;
    }
    RemoveSingleClientFromWorkspaces(pCD, pIDs, numIDs);
}
#[no_mangle]
pub unsafe extern "C" fn GetListOfOccupiedWorkspaces(
    mut pCD: *mut ClientData,
    mut numIDs: *mut ::core::ffi::c_int,
) -> *mut WorkspaceID {
    let mut i: ::core::ffi::c_int = 0;
    let mut pLocalIDs = ::core::ptr::null_mut::<WorkspaceID>();
    let mut activeWsID = (*(*(*pCD).pSD).pActiveWS).id;
    *numIDs = 0 as ::core::ffi::c_int;
    pLocalIDs = XtMalloc(
        ((*pCD).numInhabited as usize).wrapping_mul(::core::mem::size_of::<WorkspaceID>() as usize)
            as Cardinal,
    ) as *mut WorkspaceID;
    if pLocalIDs.is_null() {
        Warning(
            b"Insufficient memory\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<WorkspaceID>();
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*pCD).numInhabited {
        if activeWsID != (*(*pCD).pWsList.offset(i as isize)).wsID {
            let fresh9 = *numIDs;
            *numIDs = *numIDs + 1;
            *pLocalIDs.offset(fresh9 as isize) = (*(*pCD).pWsList.offset(i as isize)).wsID;
        }
        i += 1;
    }
    return pLocalIDs;
}
#[no_mangle]
pub unsafe extern "C" fn HonorAbsentMapBehavior(mut pCD: *mut ClientData) {
    let mut inWorkspace = 0 as ::core::ffi::c_int;
    if (*pCD).absentMapBehavior == AMAP_BEHAVIOR_MOVE {
        let mut wsCnt: ::core::ffi::c_int = 0;
        wsCnt = 0 as ::core::ffi::c_int;
        while wsCnt < (*pCD).numInhabited {
            if (*(*pCD).pWsList.offset(wsCnt as isize)).wsID != (*(*(*pCD).pSD).pActiveWS).id {
                RemoveClientFromWorkspaces(
                    pCD,
                    &raw mut (*(*pCD).pWsList.offset(wsCnt as isize)).wsID,
                    1 as ::core::ffi::c_uint,
                );
            } else {
                inWorkspace += 1;
            }
            wsCnt = inWorkspace;
        }
    }
    if inWorkspace == 0 as ::core::ffi::c_int {
        AddClientToWorkspaces(
            pCD,
            &raw mut (*(*wmGD.pActiveSD).pActiveWS).id,
            1 as ::core::ffi::c_uint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn RemoveClientFromWorkspaces(
    mut pCD: *mut ClientData,
    mut pIDs: *mut WorkspaceID,
    mut numIDs: ::core::ffi::c_uint,
) {
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    pcdLeader = if !(*pCD).transientLeader.is_null() {
        FindTransientTreeLeader(pCD)
    } else {
        pCD
    };
    RemoveSubtreeFromWorkspaces(pcdLeader, pIDs, numIDs);
}
#[no_mangle]
pub unsafe extern "C" fn AddSingleClientToWorkspaces(
    mut pCD: *mut ClientData,
    mut pIDs: *mut WorkspaceID,
    mut numIDs: ::core::ffi::c_uint,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < numIDs {
        pWS = GetWorkspaceData((*pCD).pSD, *pIDs.offset(i as isize));
        if !pWS.is_null() && ClientInWorkspace(pWS, pCD) == 0 {
            PutClientIntoWorkspace(pWS, pCD);
            if pWS == (*(*pCD).pSD).pActiveWS
                && (*pCD).transientLeader.is_null()
                && (*pCD).clientState & UNSEEN_STATE != 0
            {
                SetClientState(pCD, (*pCD).clientState & !UNSEEN_STATE, CurrentTime as Time);
            }
            UpdateWorkspacePresenceProperty(pCD);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn AddSubtreeToWorkspaces(
    mut pCD: *mut ClientData,
    mut pIDs: *mut WorkspaceID,
    mut numIDs: ::core::ffi::c_uint,
) {
    let mut pNext = ::core::ptr::null_mut::<ClientData>();
    pNext = (*pCD).transientChildren as *mut ClientData;
    while !pNext.is_null() {
        if !(*pNext).transientChildren.is_null() {
            AddSubtreeToWorkspaces(pNext, pIDs, numIDs);
        } else {
            AddSingleClientToWorkspaces(pNext, pIDs, numIDs);
        }
        pNext = (*pNext).transientSiblings as *mut ClientData;
    }
    AddSingleClientToWorkspaces(pCD, pIDs, numIDs);
}
#[no_mangle]
pub unsafe extern "C" fn AddClientToWorkspaces(
    mut pCD: *mut ClientData,
    mut pIDs: *mut WorkspaceID,
    mut numIDs: ::core::ffi::c_uint,
) {
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    pcdLeader = if !(*pCD).transientLeader.is_null() {
        FindTransientTreeLeader(pCD)
    } else {
        pCD
    };
    AddSubtreeToWorkspaces(pcdLeader, pIDs, numIDs);
}
#[no_mangle]
pub unsafe extern "C" fn AddClientToWsList(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
) {
    if (*pWS).numClients >= (*pWS).sizeClientList {
        if (*pWS).sizeClientList == 0 as ::core::ffi::c_uint {
            (*pWS).ppClients = XtMalloc(
                (WINDOW_ALLOC_AMOUNT as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut ClientData>() as usize)
                    as Cardinal,
            ) as *mut *mut ClientData as *mut *mut _ClientData;
        } else {
            (*pWS).ppClients = XtRealloc(
                (*pWS).ppClients as *mut ::core::ffi::c_char,
                ((*pWS)
                    .sizeClientList
                    .wrapping_add(WINDOW_ALLOC_AMOUNT as ::core::ffi::c_uint)
                    as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut ClientData>() as usize)
                    as Cardinal,
            ) as *mut *mut ClientData as *mut *mut _ClientData;
        }
        if (*pWS).ppClients.is_null() {
            Warning(
                b"Insufficient memory to add window to workspace\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            ExitWM(WM_ERROR_EXIT_VALUE);
        }
        (*pWS).sizeClientList = (*pWS)
            .sizeClientList
            .wrapping_add(WINDOW_ALLOC_AMOUNT as ::core::ffi::c_uint);
    }
    if (*pWS).numClients < (*pWS).sizeClientList {
        let ref mut fresh2 = *(*pWS).ppClients.offset((*pWS).numClients as isize);
        *fresh2 = pCD as *mut _ClientData;
        (*pWS).numClients = (*pWS).numClients.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn RemoveClientFromWsList(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
) {
    let mut src: ::core::ffi::c_int = 0;
    let mut dest: ::core::ffi::c_int = 0;
    dest = 0 as ::core::ffi::c_int;
    while (dest as ::core::ffi::c_uint) < (*pWS).numClients {
        if *(*pWS).ppClients.offset(dest as isize) == pCD {
            break;
        }
        dest += 1;
    }
    src = dest + 1 as ::core::ffi::c_int;
    while (src as ::core::ffi::c_uint) < (*pWS).numClients {
        let ref mut fresh5 = *(*pWS).ppClients.offset(dest as isize);
        *fresh5 = *(*pWS).ppClients.offset(src as isize);
        src += 1;
        dest += 1;
    }
    (*pWS).numClients = (*pWS).numClients.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn F_CreateWorkspace(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pSD = wmGD.pActiveSD;
    if (*pSD).numWorkspaces >= MAX_WORKSPACE_COUNT {
        let mut buffer: [::core::ffi::c_char; 1023] = [0; 1023];
        sprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"Maximum number of workspaces is %d. New workspace was not created.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            MAX_WORKSPACE_COUNT,
        );
        Warning(&raw mut buffer as *mut ::core::ffi::c_char);
    } else {
        CreateWorkspace(wmGD.pActiveSD, args as *mut ::core::ffi::c_uchar);
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_DeleteWorkspace(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pSD = wmGD.pActiveSD;
    let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut i: ::core::ffi::c_int = 0;
    if args.is_null() {
        pWS = (*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData;
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < (*pSD).numWorkspaces {
            if strcmp(
                (*(*pSD).pWS.offset(i as isize)).name as *const ::core::ffi::c_char,
                args as *const ::core::ffi::c_char,
            ) == 0
            {
                pWS =
                    (*pSD).pWS.offset(i as isize) as *mut _WmWorkspaceData as *mut WmWorkspaceData;
                break;
            } else {
                i += 1;
            }
        }
    }
    if !pWS.is_null() {
        DeleteWorkspace(pWS);
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_GotoWorkspace(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut wsID: WorkspaceID = 0;
    let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
    wsID = XInternAtom(wmGD.display, args as *const ::core::ffi::c_char, False) as WorkspaceID;
    pWS = GetWorkspaceData(wmGD.pActiveSD, wsID);
    if !pWS.is_null() {
        ChangeToWorkspace(pWS);
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_AddToAllWorkspaces(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    let mut i: ::core::ffi::c_int = 0;
    if !pCD.is_null() && (*pCD).wsmFunctions & WSM_FUNCTION_OCCUPY_WS != 0 {
        pSD = (*pCD).pSD;
        ReserveIdListSpace((*pSD).numWorkspaces);
        i = 0 as ::core::ffi::c_int;
        while i < (*pSD).numWorkspaces {
            *pResIDs.offset(i as isize) = (*(*pSD).pWS.offset(i as isize)).id;
            i += 1;
        }
        AddClientToWorkspaces(pCD, pResIDs, (*pSD).numWorkspaces as ::core::ffi::c_uint);
        (*pCD).putInAll = True as Boolean;
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Remove(
    mut args: String_0,
    mut pCD: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut rval = False as Boolean;
    if !pCD.is_null()
        && (*pCD).wsmFunctions & WSM_FUNCTION_OCCUPY_WS != 0
        && (*pCD).numInhabited > 1 as ::core::ffi::c_int
    {
        if ClientInWorkspace((*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData, pCD) != 0 {
            RemoveClientFromWorkspaces(
                pCD,
                &raw mut (*(*wmGD.pActiveSD).pActiveWS).id,
                1 as ::core::ffi::c_uint,
            );
            (*pCD).putInAll = False as Boolean;
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn GetCurrentWorkspaceIndex(
    mut pSD: *mut WmScreenData,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pSD).numWorkspaces {
        if (*(*pSD).pWS.offset(i as isize)).id == (*(*pSD).pActiveWS).id {
            break;
        }
        i += 1;
    }
    if i >= (*pSD).numWorkspaces {
        i = 0 as ::core::ffi::c_int;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn InsureIconForWorkspace(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
) {
    let mut pWsc = ::core::ptr::null_mut::<WsClientData>();
    if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0 {
        pWsc = GetWsClientData(pWS, pCD);
        if (*(*pCD).pSD).useIconBox as ::core::ffi::c_int != 0
            && (*pCD).clientFlags & CLIENT_WM_CLIENTS == 0
        {
            if MakeIcon(pWS, pCD) != 0 {
                XSaveContext(
                    wmGD.display,
                    (*pWsc).iconFrameWin as XID,
                    wmGD.windowContextType,
                    pCD as XPointer as *const ::core::ffi::c_char,
                );
                if (*pCD).iconWindow != 0 && (*pWsc).iconFrameWin != 0 {
                    XGrabButton(
                        wmGD.display,
                        AnyButton as ::core::ffi::c_uint,
                        AnyModifier as ::core::ffi::c_uint,
                        (*pWsc).iconFrameWin,
                        True,
                        (ButtonPressMask | ButtonReleaseMask | ButtonMotionMask)
                            as ::core::ffi::c_uint,
                        GrabModeAsync,
                        GrabModeAsync,
                        None_0 as Window,
                        wmGD.workspaceCursor,
                    );
                }
                ShowClientIconState(pCD, (*pCD).clientState & !UNSEEN_STATE);
            }
        } else {
            (*pWsc).iconFrameWin =
                (*(*pCD).pWsList.offset(0 as ::core::ffi::c_int as isize)).iconFrameWin;
            (*pWsc).iconX = (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconX;
            (*pWsc).iconY = (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconY;
            (*pWsc).IPData = PositionToPlacementData(pWS, (*pWsc).iconX, (*pWsc).iconY);
            if (*pCD).clientState & !UNSEEN_STATE != MINIMIZED_STATE {
                (*pWsc).iconPlace = NO_ICON_PLACE;
            } else if wmGD.iconAutoPlace == 0 {
                PlaceIconOnScreen(pCD, &raw mut (*pWsc).iconX, &raw mut (*pWsc).iconY);
            } else {
                (*pWsc).iconPlace =
                    CvtIconPositionToPlace((*pWsc).IPData, (*pWsc).iconX, (*pWsc).iconY);
                if !(*(*(*pWsc).IPData)
                    .placeList
                    .offset((*pWsc).iconPlace as isize))
                .pCD
                .is_null()
                {
                    (*pWsc).iconPlace =
                        FindIconPlace(pCD, (*pWsc).IPData, (*pWsc).iconX, (*pWsc).iconY);
                    if (*pWsc).iconPlace == NO_ICON_PLACE {
                        (*pWsc).iconPlace = GetNextIconPlace((*pWsc).IPData);
                        if (*pWsc).iconPlace == NO_ICON_PLACE {
                            (*pWsc).iconPlace = CvtIconPositionToPlace(
                                (*pWsc).IPData,
                                (*pCD).clientX,
                                (*pCD).clientY,
                            );
                        }
                    }
                }
                CvtIconPlaceToPosition(
                    (*pWsc).IPData,
                    (*pWsc).iconPlace,
                    &raw mut (*pWsc).iconX,
                    &raw mut (*pWsc).iconY,
                );
                if (*(*(*pWsc).IPData)
                    .placeList
                    .offset((*pWsc).iconPlace as isize))
                .pCD
                .is_null()
                {
                    let ref mut fresh3 = (*(*(*pWsc).IPData)
                        .placeList
                        .offset((*pWsc).iconPlace as isize))
                    .pCD;
                    *fresh3 = pCD as *mut _ClientData;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetLeaderPresence(
    mut pCD: *mut ClientData,
    mut ppIDs: *mut *mut WorkspaceID,
    mut pnumIDs: *mut ::core::ffi::c_uint,
) -> Boolean {
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut i: ::core::ffi::c_int = 0;
    let mut rval = False as Boolean;
    let mut pLocalIDs = ::core::ptr::null_mut::<WorkspaceID>();
    pLocalIDs = XtMalloc(
        ((*(*pCD).pSD).numWorkspaces as usize)
            .wrapping_mul(::core::mem::size_of::<WorkspaceID>() as usize) as Cardinal,
    ) as *mut WorkspaceID;
    if pLocalIDs.is_null() {
        Warning(
            b"Insufficient Memory (GetLeaderPresence)\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        ExitWM(WM_ERROR_EXIT_VALUE);
    }
    if !(*pCD).transientLeader.is_null() {
        pcdLeader = FindTransientTreeLeader(pCD);
        i = 0 as ::core::ffi::c_int;
        while i < (*pcdLeader).numInhabited {
            *pLocalIDs.offset(i as isize) = (*(*pcdLeader).pWsList.offset(i as isize)).wsID;
            i += 1;
        }
        *ppIDs = pLocalIDs;
        *pnumIDs = (*pcdLeader).numInhabited as ::core::ffi::c_uint;
        rval = True as Boolean;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn GetMyOwnPresence(
    mut pCD: *mut ClientData,
    mut ppIDs: *mut *mut WorkspaceID,
    mut pnumIDs: *mut ::core::ffi::c_uint,
) -> Boolean {
    if HasProperty(pCD, wmGD.xa_MWM_WORKSPACE_PRESENCE) != 0 {
        let mut IDs = ::core::ptr::null_mut::<WorkspaceID>();
        let mut nIDs = 0 as ::core::ffi::c_ulong;
        let mut actualType: Atom = 0;
        let mut actualFormat: ::core::ffi::c_int = 0;
        let mut leftover = 0 as ::core::ffi::c_ulong;
        let mut status: ::core::ffi::c_int = 0;
        nIDs = (*(*pCD).pSD).numWorkspaces as ::core::ffi::c_ulong;
        status = XGetWindowProperty(
            wmGD.display,
            (*pCD).client,
            wmGD.xa_MWM_WORKSPACE_PRESENCE,
            0 as ::core::ffi::c_long,
            nIDs as ::core::ffi::c_long,
            False,
            wmGD.xa_MWM_WORKSPACE_PRESENCE,
            &raw mut actualType,
            &raw mut actualFormat,
            &raw mut nIDs,
            &raw mut leftover,
            &raw mut IDs as *mut *mut ::core::ffi::c_uchar,
        );
        if status != Success {
            return False as Boolean;
        }
        if actualType != wmGD.xa_MWM_WORKSPACE_PRESENCE
            || actualFormat != 32 as ::core::ffi::c_int
            || leftover != 0
        {
            XFree(IDs as *mut ::core::ffi::c_void);
            return False as Boolean;
        }
        *ppIDs = IDs;
        *pnumIDs = nIDs as ::core::ffi::c_uint;
        return True as Boolean;
    }
    return False as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ReserveIdListSpace(mut numIDs: ::core::ffi::c_int) {
    if numResIDs == 0 as ::core::ffi::c_int {
        pResIDs = XtMalloc(
            (numIDs as usize).wrapping_mul(::core::mem::size_of::<WorkspaceID>() as usize)
                as Cardinal,
        ) as *mut WorkspaceID;
        if !pResIDs.is_null() {
            numResIDs = numIDs;
        }
    } else if numResIDs < numIDs {
        pResIDs = XtRealloc(
            pResIDs as *mut ::core::ffi::c_char,
            (numIDs as usize).wrapping_mul(::core::mem::size_of::<WorkspaceID>() as usize)
                as Cardinal,
        ) as *mut WorkspaceID;
        numResIDs = if !pResIDs.is_null() {
            numIDs
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if pResIDs.is_null() {
        Warning(
            b"Insufficient memory\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        ExitWM(WM_ERROR_EXIT_VALUE);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DuplicateWorkspaceName(
    mut pSD: *mut WmScreenData,
    mut name: *mut ::core::ffi::c_uchar,
    mut num: ::core::ffi::c_int,
) -> Boolean {
    let mut i: ::core::ffi::c_int = 0;
    let mut duplicate = False as Boolean;
    if !pSD.is_null() && !(*pSD).pWS.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < num && duplicate == 0 {
            if strcmp(
                (*(*pSD).pWS.offset(i as isize)).name as *const ::core::ffi::c_char,
                name as *mut ::core::ffi::c_char,
            ) == 0
            {
                duplicate = True as Boolean;
            }
            i += 1;
        }
    }
    return duplicate;
}
