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
    fn XConfigureWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_uint,
        _: *mut XWindowChanges,
    ) -> ::core::ffi::c_int;
    fn XGrabPointer(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: Window,
        _: Cursor,
        _: Time,
    ) -> ::core::ffi::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XMaskEvent(_: *mut Display, _: ::core::ffi::c_long, _: *mut XEvent) -> ::core::ffi::c_int;
    fn XMoveWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XReparentWindow(
        _: *mut Display,
        _: Window,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XResizeWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XUngrabPointer(_: *mut Display, _: Time) -> ::core::ffi::c_int;
    fn XUnmapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    static mut wmGD: WmGlobalData;
    fn SendConfigureNotify(pCD: *mut ClientData);
    fn FrameExposureProc(pcd: *mut ClientData);
    fn RegenerateClientFrame(pcd: *mut ClientData);
    fn Do_Focus_Key(pCD: *mut ClientData, focusTime: Time, flags: ::core::ffi::c_long);
    fn F_Raise(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn HideActiveIconText(pSD: *mut WmScreenData);
    fn IconExposureProc(pcd: *mut ClientData, clearFirst: Boolean);
    fn GetNextIconPlace(pIPD: *mut IconPlacementData) -> ::core::ffi::c_int;
    fn CvtIconPlaceToPosition(
        pIPD: *mut IconPlacementData,
        place: ::core::ffi::c_int,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
    );
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
    fn ShowClientIconState(pCD: *mut ClientData, newState: ::core::ffi::c_int);
    fn AutoResetKeyFocus(pcdNoFocus: *mut ClientData, focusTime: Time);
    fn ClearFocusIndication(pCD: *mut ClientData, refresh: Boolean);
    fn SetKeyboardFocus(pCD: *mut ClientData, focusFlags: ::core::ffi::c_long);
    fn UnManageWindow(pCD: *mut ClientData);
    fn SetWMState(window: Window, state: ::core::ffi::c_int, icon: Window);
    fn FindTransientFocus(pcd: *mut ClientData) -> *mut ClientData;
    fn FindTransientTreeLeader(pcd: *mut ClientData) -> *mut ClientData;
    fn MoveEntryInList(
        pWS: *mut WmWorkspaceData,
        pEntry: *mut ClientListEntry,
        onTop: Boolean,
        pStackEntry: *mut ClientListEntry,
    );
    fn GetWorkspaceData(pSD: *mut WmScreenData, wsID: WorkspaceID) -> *mut WmWorkspaceData;
    fn SetClientWsIndex(pCD: *mut ClientData);
    fn UpdateEwmhClientState(_: *mut ClientData);
    fn NotifyCompositorTransition(
        client: *mut ClientData,
        kind: ::core::ffi::c_int,
        phase: ::core::ffi::c_int,
    );
    static mut firstTime: ::core::ffi::c_int;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CurrentTime: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const GrabModeAsync: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CWSibling: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const CWStackMode: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const Below: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NormalState: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IconicState: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ALWAYS_SET_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const WORKSPACE_IF_NULL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const WITHDRAWN_STATE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2;
pub const MAXIMIZED_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const UNSEEN_STATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ICON_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NO_ICON_PLACE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C" fn SetClientState(
    mut pCD: *mut ClientData,
    mut newState: ::core::ffi::c_int,
    mut setTime: Time,
) {
    SetClientStateWithEventMask(
        pCD,
        newState,
        setTime,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SetClientStateWithEventMask(
    mut pCD: *mut ClientData,
    mut newState: ::core::ffi::c_int,
    mut setTime: Time,
    mut event_mask: ::core::ffi::c_uint,
) {
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut currentState: ::core::ffi::c_int = 0;
    let mut pSD = (*pCD).pSD;
    let mut notShowing = (newState & UNSEEN_STATE) as Boolean;
    let mut transition = 0 as ::core::ffi::c_int;
    currentState = (*pCD).clientState;
    if currentState == newState {
        return;
    }
    pcdLeader = if !(*pCD).transientLeader.is_null() {
        FindTransientTreeLeader(pCD)
    } else {
        pCD
    };
    SetClientWsIndex(pCD);
    if !(*pCD).transientLeader.is_null() {
        if (*pcdLeader).clientState == MINIMIZED_STATE && newState != WITHDRAWN_STATE {
            newState = MINIMIZED_STATE;
            if notShowing != 0 {
                newState |= UNSEEN_STATE;
            }
        } else if newState == MINIMIZED_STATE && (*pcdLeader).clientState != MINIMIZED_STATE {
            if currentState == WITHDRAWN_STATE {
                newState = NORMAL_STATE;
                if notShowing != 0 {
                    newState |= UNSEEN_STATE;
                }
            } else {
                newState = currentState;
                if notShowing != 0 {
                    newState |= UNSEEN_STATE;
                }
            }
        }
        if newState == currentState {
            return;
        }
    }
    if currentState & UNSEEN_STATE == 0
        && newState & UNSEEN_STATE == 0
        && currentState != WITHDRAWN_STATE
        && newState != WITHDRAWN_STATE
    {
        if newState == MINIMIZED_STATE {
            transition = 1 as ::core::ffi::c_int;
        } else if currentState == MINIMIZED_STATE {
            transition = 2 as ::core::ffi::c_int;
        } else if newState == MAXIMIZED_STATE {
            transition = 3 as ::core::ffi::c_int;
        } else if currentState == MAXIMIZED_STATE && newState == NORMAL_STATE {
            transition = 4 as ::core::ffi::c_int;
        }
    }
    NotifyCompositorTransition(pCD, transition, 0 as ::core::ffi::c_int);
    match newState {
        8 => {
            UnManageWindow(pCD);
        }
        NORMAL_STATE | MAXIMIZED_STATE => {
            SetupWindowStateWithEventMask(pCD, newState, setTime, event_mask);
            XMapWindow(wmGD.display, (*pCD).client);
            XMapWindow(wmGD.display, (*pCD).clientFrameWin);
        }
        MINIMIZED_STATE => {
            let mut clientHasFocus: Boolean = 0;
            if (*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE {
                if wmGD.keyboardFocus == pCD
                    || !(*pCD).transientChildren.is_null()
                        && !wmGD.keyboardFocus.is_null()
                        && pCD == FindTransientTreeLeader(wmGD.keyboardFocus)
                {
                    clientHasFocus = True as Boolean;
                } else {
                    clientHasFocus = False as Boolean;
                }
                if clientHasFocus as ::core::ffi::c_int != 0
                    || (wmGD.nextKeyboardFocus == pCD
                        || !(*pCD).transientChildren.is_null()
                            && !wmGD.keyboardFocus.is_null()
                            && pCD == FindTransientTreeLeader(wmGD.nextKeyboardFocus))
                {
                    if wmGD.autoKeyFocus as ::core::ffi::c_int != 0
                        && wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT
                    {
                        AutoResetKeyFocus(pcdLeader, setTime);
                    } else {
                        Do_Focus_Key(
                            ::core::ptr::null_mut::<ClientData>(),
                            setTime,
                            ALWAYS_SET_FOCUS | WORKSPACE_IF_NULL,
                        );
                    }
                    if clientHasFocus != 0 {
                        SetKeyboardFocus(
                            ::core::ptr::null_mut::<ClientData>(),
                            0 as ::core::ffi::c_long,
                        );
                    }
                }
                UnmapClients(pCD, event_mask);
            }
            if (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin != 0 {
                if (*pCD).clientState & UNSEEN_STATE != 0 {
                    if (*pCD).iconWindow != 0 {
                        XMapWindow(wmGD.display, (*pCD).iconWindow);
                    }
                    XMapWindow(
                        wmGD.display,
                        (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
                    );
                }
                ShowAllIconsForMinimizedClient(pCD);
            }
            SetClientWMState(pCD, IconicState, MINIMIZED_STATE);
            if (*pSD).useIconBox as ::core::ffi::c_int != 0
                && !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
                    .pIconBox
                    .is_null()
            {
                if (*pCD).clientFlags & ICON_BOX != 0 && (*wmGD.pActiveSD).activeIconTextWin != 0 {
                    HideActiveIconText(NULL as *mut WmScreenData);
                    (*pSD).activeLabelParent = (*wmGD.pActiveSD).rootWindow;
                    XReparentWindow(
                        wmGD.display,
                        (*wmGD.pActiveSD).activeIconTextWin,
                        (*wmGD.pActiveSD).rootWindow,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                }
                if (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin != 0 {
                    IconExposureProc(pCD, True as Boolean);
                }
            }
        }
        9 | 11 | 10 => {
            if wmGD.keyboardFocus == pCD {
                Do_Focus_Key(
                    NULL as *mut ClientData,
                    CurrentTime as Time,
                    ALWAYS_SET_FOCUS,
                );
                SetKeyboardFocus(
                    ::core::ptr::null_mut::<ClientData>(),
                    0 as ::core::ffi::c_long,
                );
            }
            if (*pCD).clientState & UNSEEN_STATE == 0
                && ((*pCD).clientState & !UNSEEN_STATE == NORMAL_STATE
                    || (*pCD).clientState & !UNSEEN_STATE == MAXIMIZED_STATE)
            {
                UnmapClients(pcdLeader, event_mask);
            }
            if (*pCD).clientFrameWin != 0 {
                if (*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
                    .pIconBox
                    .is_null()
                {
                    if (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin != 0 {
                        XUnmapWindow(
                            wmGD.display,
                            (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
                        );
                    }
                    if (*pCD).iconWindow != 0 {
                        XUnmapWindow(wmGD.display, (*pCD).iconWindow);
                    }
                }
            }
            match newState & !UNSEEN_STATE {
                MINIMIZED_STATE => {
                    SetClientWMState(pCD, IconicState, newState);
                }
                NORMAL_STATE | MAXIMIZED_STATE | _ => {
                    SetClientWMState(pCD, NormalState, newState);
                }
            }
        }
        _ => {}
    }
    UpdateEwmhClientState(pCD);
    NotifyCompositorTransition(pCD, transition, 1 as ::core::ffi::c_int);
}
unsafe extern "C" fn SetupWindowStateWithEventMask(
    mut pCD: *mut ClientData,
    mut newState: ::core::ffi::c_int,
    mut setTime: Time,
    mut event_mask: ::core::ffi::c_uint,
) {
    let mut currentState: ::core::ffi::c_int = 0;
    let mut wsI: ::core::ffi::c_int = 0;
    let mut iplace: ::core::ffi::c_int = 0;
    let mut pSD = (*pCD).pSD;
    currentState = (*pCD).clientState;
    if newState == NORMAL_STATE {
        if (*pCD).maxConfig as ::core::ffi::c_int == True {
            ConfigureNewState(pCD);
        }
    } else if (*pCD).maxConfig as ::core::ffi::c_int == False {
        ConfigureNewState(pCD);
    }
    if currentState == MINIMIZED_STATE {
        let mut clearIconFocus: Boolean = 0;
        if wmGD.keyboardFocus == pCD || wmGD.nextKeyboardFocus == pCD {
            Do_Focus_Key(
                ::core::ptr::null_mut::<ClientData>(),
                setTime,
                ALWAYS_SET_FOCUS | WORKSPACE_IF_NULL,
            );
        }
        if wmGD.keyboardFocus == pCD {
            clearIconFocus = True as Boolean;
        } else {
            clearIconFocus = False as Boolean;
        }
        if (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin != 0 {
            if (*pSD).useIconBox as ::core::ffi::c_int != 0
                && !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
                    .pIconBox
                    .is_null()
                && (*pCD).clientFlags & ICON_BOX == 0
            {
                ShowClientIconState(pCD, newState);
            } else {
                let mut doGrab = False as Boolean;
                if event_mask != 0 {
                    doGrab = (Success
                        == XGrabPointer(
                            wmGD.display,
                            (*(*(wmGD.display as _XPrivDisplay)).screens.offset(
                                (*(wmGD.display as _XPrivDisplay)).default_screen as isize,
                            ))
                            .root,
                            False,
                            event_mask,
                            GrabModeAsync,
                            GrabModeAsync,
                            None_0 as Window,
                            None_0 as Cursor,
                            CurrentTime as Time,
                        )) as ::core::ffi::c_int as Boolean;
                }
                XUnmapWindow(
                    wmGD.display,
                    (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
                );
                if (*pCD).iconWindow != 0 {
                    XUnmapWindow(wmGD.display, (*pCD).iconWindow);
                }
                if event_mask != 0 && doGrab as ::core::ffi::c_int != 0 {
                    let mut event = _XEvent { type_0: 0 };
                    XMaskEvent(
                        wmGD.display,
                        event_mask as ::core::ffi::c_long,
                        &raw mut event,
                    );
                    XUngrabPointer(wmGD.display, CurrentTime as Time);
                }
                if wmGD.iconAutoPlace != 0 {
                    wsI = 0 as ::core::ffi::c_int;
                    while wsI < (*pCD).numInhabited {
                        iplace = (*(*pCD).pWsList.offset(wsI as isize)).iconPlace;
                        if iplace != NO_ICON_PLACE {
                            let ref mut fresh3 = (*(*(*(*pCD).pWsList.offset(wsI as isize))
                                .IPData)
                                .placeList
                                .offset(iplace as isize))
                            .pCD;
                            *fresh3 = ::core::ptr::null_mut::<_ClientData>();
                        }
                        wsI += 1;
                    }
                }
            }
            if clearIconFocus != 0 {
                ClearFocusIndication(pCD, False as Boolean);
                wmGD.keyboardFocus = ::core::ptr::null_mut::<ClientData>();
            }
        }
    }
    if currentState != NORMAL_STATE && currentState != MAXIMIZED_STATE {
        if currentState == MINIMIZED_STATE {
            (*pCD).clientState = newState;
            wmGD.bSuspendSecondaryRestack = True as Boolean;
            F_Raise(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                pCD,
                ::core::ptr::null_mut::<XEvent>(),
            );
            wmGD.bSuspendSecondaryRestack = False as Boolean;
        }
        if (*pCD).clientFlags & ICON_BOX == 0
            || (*pCD).clientFlags & ICON_BOX != 0 && firstTime == 0
        {
            MapClientWindows(pCD);
        }
        SetClientWMState(pCD, NormalState, NORMAL_STATE);
    }
    (*pCD).clientState = newState;
    if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT
        && currentState == MINIMIZED_STATE
        && wmGD.deiconifyKeyFocus as ::core::ffi::c_int != 0
    {
        let mut pcdFocus = ::core::ptr::null_mut::<ClientData>();
        pcdFocus = FindTransientFocus(pCD);
        if !pcdFocus.is_null() {
            Do_Focus_Key(pcdFocus, setTime, ALWAYS_SET_FOCUS);
        }
    }
    if (*pSD).useIconBox as ::core::ffi::c_int != 0
        && !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
            .pIconBox
            .is_null()
        && (*pCD).clientFlags & ICON_BOX == 0
        && (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin != 0
    {
        IconExposureProc(pCD, True as Boolean);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ConfigureNewState(mut pcd: *mut ClientData) {
    if (*pcd).maxConfig != 0 {
        (*pcd).maxConfig = FALSE as Boolean;
        RegenerateClientFrame(pcd);
        XResizeWindow(
            wmGD.display,
            (*pcd).client,
            (*pcd).clientWidth as ::core::ffi::c_uint,
            (*pcd).clientHeight as ::core::ffi::c_uint,
        );
    } else {
        XResizeWindow(
            wmGD.display,
            (*pcd).client,
            (*pcd).maxWidth as ::core::ffi::c_uint,
            (*pcd).maxHeight as ::core::ffi::c_uint,
        );
        (*pcd).maxConfig = TRUE as Boolean;
        RegenerateClientFrame(pcd);
    }
    SendConfigureNotify(pcd);
    if (*pcd).clientWidth == (*pcd).maxWidth && (*pcd).clientHeight == (*pcd).maxHeight {
        FrameExposureProc(pcd);
    }
}
unsafe extern "C" fn UnmapClients(mut pCD: *mut ClientData, mut event_mask: ::core::ffi::c_uint) {
    let mut pNext = ::core::ptr::null_mut::<ClientData>();
    let mut doGrab = False as Boolean;
    pNext = (*pCD).transientChildren as *mut ClientData;
    while !pNext.is_null() {
        if !(*pNext).transientChildren.is_null() {
            UnmapClients(pNext, 0 as ::core::ffi::c_int as ::core::ffi::c_uint);
        }
        XUnmapWindow(wmGD.display, (*pNext).clientFrameWin);
        XUnmapWindow(wmGD.display, (*pNext).client);
        (*pNext).wmUnmapCount += 1;
        pNext = (*pNext).transientSiblings as *mut ClientData;
    }
    if event_mask != 0 {
        doGrab = (Success
            == XGrabPointer(
                wmGD.display,
                (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
                .root,
                False,
                event_mask,
                GrabModeAsync,
                GrabModeAsync,
                None_0 as Window,
                None_0 as Cursor,
                CurrentTime as Time,
            )) as ::core::ffi::c_int as Boolean;
    }
    XUnmapWindow(wmGD.display, (*pCD).clientFrameWin);
    XUnmapWindow(wmGD.display, (*pCD).client);
    if event_mask != 0 && doGrab as ::core::ffi::c_int != 0 {
        let mut event = _XEvent { type_0: 0 };
        XMaskEvent(
            wmGD.display,
            event_mask as ::core::ffi::c_long,
            &raw mut event,
        );
        XUngrabPointer(wmGD.display, CurrentTime as Time);
    }
    (*pCD).wmUnmapCount += 1;
}
#[no_mangle]
pub unsafe extern "C" fn SetClientWMState(
    mut pCD: *mut ClientData,
    mut wmState: ::core::ffi::c_int,
    mut mwmState: ::core::ffi::c_int,
) {
    let mut pNext = ::core::ptr::null_mut::<ClientData>();
    let mut bToUnseen: Boolean = 0;
    bToUnseen =
        (mwmState & UNSEEN_STATE != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as Boolean;
    mwmState &= !UNSEEN_STATE;
    SetClientWsIndex(pCD);
    pNext = (*pCD).transientChildren as *mut ClientData;
    while !pNext.is_null() {
        if !(*pNext).transientChildren.is_null() {
            SetClientWMState(pNext, wmState, mwmState);
        }
        SetClientWsIndex(pNext);
        SetWMState(
            (*pNext).client,
            wmState,
            (*(*pNext).pWsList.offset((*pNext).currentWsc as isize)).iconFrameWin,
        );
        if (*pNext).maxConfig as ::core::ffi::c_int != 0 && mwmState == NORMAL_STATE {
            (*pNext).clientState = MAXIMIZED_STATE;
        } else if (*pNext).maxConfig == 0 && mwmState == MAXIMIZED_STATE {
            (*pNext).clientState = NORMAL_STATE;
        } else {
            (*pNext).clientState = mwmState;
        }
        if bToUnseen != 0 {
            (*pNext).clientState |= UNSEEN_STATE;
        }
        pNext = (*pNext).transientSiblings as *mut ClientData;
    }
    SetWMState(
        (*pCD).client,
        wmState,
        (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
    );
    (*pCD).clientState = mwmState;
    if bToUnseen != 0 {
        (*pCD).clientState |= UNSEEN_STATE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MapClientWindows(mut pCD: *mut ClientData) {
    let mut pNext = ::core::ptr::null_mut::<ClientData>();
    pNext = (*pCD).transientChildren as *mut ClientData;
    while !pNext.is_null() {
        if !(*pNext).transientChildren.is_null() {
            MapClientWindows(pNext);
        }
        XMapWindow(wmGD.display, (*pNext).client);
        XMapWindow(wmGD.display, (*pNext).clientFrameWin);
        pNext = (*pNext).transientSiblings as *mut ClientData;
    }
    XMapWindow(wmGD.display, (*pCD).client);
    XMapWindow(wmGD.display, (*pCD).clientFrameWin);
}
#[no_mangle]
pub unsafe extern "C" fn ShowIconForMinimizedClient(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
) {
    let mut pSD = (*pCD).pSD;
    if wmGD.iconAutoPlace as ::core::ffi::c_int != 0
        && (*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
            .pIconBox
            .is_null()
    {
        let mut ipd = (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).IPData;
        if ipd.is_null()
            || (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconPlace == NO_ICON_PLACE
            || !(*(*ipd)
                .placeList
                .offset((*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconPlace as isize))
            .pCD
            .is_null()
                && (*(*ipd).placeList.offset(
                    (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconPlace as isize,
                ))
                .pCD != pCD
        {
            if ipd.is_null() {
                ipd = PositionToPlacementData(pWS, (*pCD).clientX, (*pCD).clientY);
            }
            let ref mut fresh0 = (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconPlace;
            *fresh0 = GetNextIconPlace(ipd);
            if *fresh0 == NO_ICON_PLACE {
                (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconPlace =
                    CvtIconPositionToPlace(ipd, (*pCD).clientX, (*pCD).clientY);
            }
            CvtIconPlaceToPosition(
                ipd,
                (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconPlace,
                &raw mut (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconX,
                &raw mut (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconY,
            );
        }
        let ref mut fresh1 = (*(*ipd)
            .placeList
            .offset((*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconPlace as isize))
        .pCD;
        *fresh1 = pCD as *mut _ClientData;
        let ref mut fresh2 = (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).IPData;
        *fresh2 = ipd;
    }
    if pWS == (*pSD).pActiveWS
        && (*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
            .pIconBox
            .is_null()
    {
        XMoveWindow(
            wmGD.display,
            (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
            (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconX,
            (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconY,
        );
    }
    if (*pCD).iconWindow != 0 {
        XMapWindow(wmGD.display, (*pCD).iconWindow);
    }
    if (*pSD).useIconBox as ::core::ffi::c_int != 0
        && !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
            .pIconBox
            .is_null()
    {
        ShowClientIconState(pCD, MINIMIZED_STATE);
    } else {
        let mut windowChanges = XWindowChanges {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            border_width: 0,
            sibling: 0,
            stack_mode: 0,
        };
        if wmGD.lowerOnIconify != 0 {
            if &raw mut (*pCD).iconEntry != (*pSD).lastClient && !(*pSD).lastClient.is_null() {
                if (*(*pSD).lastClient).type_0 == MINIMIZED_STATE {
                    windowChanges.sibling = (*(*(*(*pSD).lastClient).pCD)
                        .pWsList
                        .offset((*(*(*pSD).lastClient).pCD).currentWsc as isize))
                    .iconFrameWin;
                } else {
                    windowChanges.sibling = (*(*(*pSD).lastClient).pCD).clientFrameWin;
                }
                windowChanges.stack_mode = Below;
                XConfigureWindow(
                    wmGD.display,
                    (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
                    (CWSibling | CWStackMode) as ::core::ffi::c_uint,
                    &raw mut windowChanges,
                );
                MoveEntryInList(
                    pWS,
                    &raw mut (*pCD).iconEntry,
                    False as Boolean,
                    ::core::ptr::null_mut::<ClientListEntry>(),
                );
            }
        } else {
            windowChanges.sibling = (*pCD).clientFrameWin;
            windowChanges.stack_mode = Below;
            XConfigureWindow(
                wmGD.display,
                (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
                (CWSibling | CWStackMode) as ::core::ffi::c_uint,
                &raw mut windowChanges,
            );
            MoveEntryInList(
                pWS,
                &raw mut (*pCD).iconEntry,
                False as Boolean,
                &raw mut (*pCD).clientEntry,
            );
        }
        if pWS == (*pSD).pActiveWS {
            XMapWindow(
                wmGD.display,
                (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ShowAllIconsForMinimizedClient(mut pCD: *mut ClientData) {
    let mut saveWsc = (*pCD).currentWsc;
    let mut tmpWsc: ::core::ffi::c_int = 0;
    let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
    tmpWsc = 0 as ::core::ffi::c_int;
    while tmpWsc < (*pCD).numInhabited {
        (*pCD).currentWsc = tmpWsc;
        pWS = GetWorkspaceData((*pCD).pSD, (*(*pCD).pWsList.offset(tmpWsc as isize)).wsID);
        ShowIconForMinimizedClient(pWS, pCD);
        tmpWsc += 1;
    }
    (*pCD).currentWsc = saveWsc;
}
