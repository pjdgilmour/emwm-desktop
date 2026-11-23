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
use crate::abi::*;
extern "C" {
    fn XCheckMaskEvent(
        _: *mut Display,
        _: ::core::ffi::c_long,
        _: *mut XEvent,
    ) -> ::core::ffi::c_int;
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
    fn XSync(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
    static mut wmGD: WmGlobalData;
    fn ShowActiveClientFrame(pcd: *mut ClientData);
    fn ShowInactiveClientFrame(pcd: *mut ClientData);
    fn SetColormapFocus(pSD: *mut WmScreenData, pCD: *mut ClientData);
    fn GetTimestamp() -> Time;
    fn WmGrabButton(
        display: *mut Display,
        button: ::core::ffi::c_uint,
        modifiers: ::core::ffi::c_uint,
        grab_window: Window,
        event_mask: ::core::ffi::c_uint,
        owner_events: ::core::ffi::c_int,
        pointer_mode: ::core::ffi::c_int,
        keyboard_mode: ::core::ffi::c_int,
        confine_to: Window,
        cursor: Cursor,
    );
    fn WmUngrabButton(
        display: *mut Display,
        button: ::core::ffi::c_uint,
        modifiers: ::core::ffi::c_uint,
        grab_window: Window,
    );
    fn F_Lower(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn Do_Focus_Key(pCD: *mut ClientData, focusTime: Time, flags: ::core::ffi::c_long);
    fn Do_Raise(pCD: *mut ClientData, pStackEntry: *mut ClientListEntry, flags: ::core::ffi::c_int);
    fn ShowActiveIcon(pcd: *mut ClientData);
    fn ShowInactiveIcon(pcd: *mut ClientData, refresh: Boolean);
    fn AddWmTimer(
        timerType: ::core::ffi::c_uint,
        timerInterval: ::core::ffi::c_ulong,
        pCD: *mut ClientData,
    ) -> Boolean;
    fn FindTransientTreeLeader(pcd: *mut ClientData) -> *mut ClientData;
    fn PutTransientBelowSiblings(pcd: *mut ClientData) -> Boolean;
    fn RestackTransients(pcd: *mut ClientData);
    fn SetEwmhActiveWindow(_: *mut ClientData);
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CurrentTime: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ButtonReleaseMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const EnterWindowMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const LeaveWindowMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const Button1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GrabModeSync: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SELECT_BUTTON: ::core::ffi::c_int = Button1;
pub const FOCUS_SELECT_BUTTON: ::core::ffi::c_int = SELECT_BUTTON;
pub const ALWAYS_SET_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const REFRESH_LAST_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const SCREEN_SWITCH_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const F_CONTEXT_NORMAL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const F_CONTEXT_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const F_CONTEXT_WINDOW: ::core::ffi::c_long = F_CONTEXT_NORMAL | F_CONTEXT_MAXIMIZE;
pub const FRAME_CLIENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_SUBCONTEXT_W_CLIENT: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << FRAME_CLIENT;
pub const F_GROUP_WINDOW: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const F_GROUP_ICON: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const F_GROUP_TRANSIENT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const TIMER_RAISE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAXIMIZED_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const UNSEEN_STATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CMAP_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CMAP_FOCUS_KEYBOARD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STACK_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut removeSelectGrab: Boolean = True as Boolean;
#[no_mangle]
pub unsafe extern "C" fn InitKeyboardFocus() {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut sameScreen: Boolean = 0;
    let mut focusSet = False as Boolean;
    let mut scr: ::core::ffi::c_int = 0;
    let mut junk: ::core::ffi::c_int = 0;
    let mut junk_win: Window = 0;
    let mut root_returned: Window = 0;
    let mut currentX: ::core::ffi::c_int = 0;
    let mut currentY: ::core::ffi::c_int = 0;
    wmGD.keyboardFocus = ::core::ptr::null_mut::<ClientData>();
    wmGD.nextKeyboardFocus = ::core::ptr::null_mut::<ClientData>();
    scr = 0 as ::core::ffi::c_int;
    while scr < wmGD.numScreens {
        if (*wmGD.Screens.offset(scr as isize)).managed != 0 {
            (*wmGD.Screens.offset(scr as isize)).focusPriority = 0 as ::core::ffi::c_long;
            if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER {
                pCD = GetClientUnderPointer(&raw mut sameScreen);
                if wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER {
                    SetColormapFocus(wmGD.pActiveSD, pCD);
                }
                if !pCD.is_null() {
                    Do_Focus_Key(pCD, GetTimestamp(), ALWAYS_SET_FOCUS);
                    focusSet = True as Boolean;
                }
            } else {
                let mut buttonSpec = ::core::ptr::null_mut::<ButtonSpec>();
                buttonSpec = (*wmGD.Screens.offset(scr as isize)).buttonSpecs;
                while !buttonSpec.is_null() {
                    if (*buttonSpec).button == FOCUS_SELECT_BUTTON as ::core::ffi::c_uint
                        && (*buttonSpec).context & F_CONTEXT_WINDOW as Context != 0
                        && (*buttonSpec).subContext & F_SUBCONTEXT_W_CLIENT as Context != 0
                    {
                        if (*buttonSpec).state == 0 as ::core::ffi::c_uint {
                            removeSelectGrab = False as Boolean;
                        }
                    }
                    buttonSpec = (*buttonSpec).nextButtonSpec as *mut ButtonSpec;
                }
            }
        }
        scr += 1;
    }
    if focusSet == 0 {
        if XQueryPointer(
            wmGD.display,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
            .root,
            &raw mut root_returned,
            &raw mut junk_win,
            &raw mut currentX,
            &raw mut currentY,
            &raw mut junk,
            &raw mut junk,
            &raw mut junk as *mut ::core::ffi::c_uint,
        ) != 0
        {
            scr = 0 as ::core::ffi::c_int;
            while scr < wmGD.numScreens {
                if (*wmGD.Screens.offset(scr as isize)).managed as ::core::ffi::c_int != 0
                    && (*wmGD.Screens.offset(scr as isize)).rootWindow == root_returned
                {
                    wmGD.pActiveSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
                    wmGD.queryScreen = False as Boolean;
                    break;
                } else {
                    scr += 1;
                }
            }
        }
        Do_Focus_Key(
            NULL as *mut ClientData,
            CurrentTime as Time,
            ALWAYS_SET_FOCUS,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetKeyboardFocus(
    mut pCD: *mut ClientData,
    mut focusFlags: ::core::ffi::c_long,
) {
    let mut currentFocus = ::core::ptr::null_mut::<ClientData>();
    if wmGD.keyboardFocus == pCD {
        return;
    }
    currentFocus = wmGD.keyboardFocus;
    (*wmGD.pActiveSD).focusPriority += 1;
    if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
        if !currentFocus.is_null() {
            ResetExplicitSelectHandling(currentFocus);
            wmGD.keyboardFocus = ::core::ptr::null_mut::<ClientData>();
        }
        if !pCD.is_null()
            && ((*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE)
        {
            if removeSelectGrab != 0 {
                WmUngrabButton(
                    wmGD.display,
                    FOCUS_SELECT_BUTTON as ::core::ffi::c_uint,
                    0 as ::core::ffi::c_uint,
                    (*pCD).clientBaseWin,
                );
            }
        }
    }
    wmGD.keyboardFocus = pCD;
    if !pCD.is_null() && (*pCD).focusAutoRaise as ::core::ffi::c_int != 0 {
        if wmGD.autoRaiseDelay != 0 && wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER {
            AddWmTimer(
                TIMER_RAISE as ::core::ffi::c_uint,
                wmGD.autoRaiseDelay as ::core::ffi::c_ulong,
                pCD,
            );
        } else {
            let mut sameScreen: Boolean = 0;
            if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT
                && (*pCD).focusAutoRaiseDisabled == 0
                || wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER
                    && pCD == GetClientUnderPointer(&raw mut sameScreen)
            {
                Do_Raise(pCD, NULL as *mut ClientListEntry, STACK_NORMAL);
            }
        }
    }
    if !currentFocus.is_null() {
        ClearFocusIndication(
            currentFocus,
            (if focusFlags & REFRESH_LAST_FOCUS != 0 {
                True
            } else {
                False
            }) as Boolean,
        );
    }
    if wmGD.colormapFocusPolicy == CMAP_FOCUS_KEYBOARD && focusFlags & SCREEN_SWITCH_FOCUS == 0 {
        SetColormapFocus(wmGD.pActiveSD, pCD);
    }
    if !pCD.is_null() {
        (*pCD).focusPriority = (*wmGD.pActiveSD).focusPriority;
        SetFocusIndication(pCD);
        SetEwmhActiveWindow(pCD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ResetExplicitSelectHandling(mut pCD: *mut ClientData) {
    let mut bUnseen: Boolean = 0;
    bUnseen = (if (*pCD).clientState & UNSEEN_STATE != 0 {
        True
    } else {
        False
    }) as Boolean;
    if bUnseen != 0 {
        (*pCD).clientState &= !UNSEEN_STATE;
    }
    if (*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE {
        DoExplicitSelectGrab((*pCD).clientBaseWin);
    } else {
        (*pCD).clientState == MINIMIZED_STATE;
    }
    if bUnseen != 0 {
        (*pCD).clientState |= UNSEEN_STATE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn DoExplicitSelectGrab(mut window: Window) {
    WmGrabButton(
        wmGD.display,
        FOCUS_SELECT_BUTTON as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        window,
        False as ::core::ffi::c_uint,
        ButtonReleaseMask as ::core::ffi::c_int,
        GrabModeSync,
        GrabModeSync,
        None_0 as Window,
        None_0 as Cursor,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SetFocusIndication(mut pCD: *mut ClientData) {
    let mut saveCD = ::core::ptr::null_mut::<ClientData>();
    saveCD = wmGD.keyboardFocus;
    wmGD.keyboardFocus = pCD;
    if (*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE {
        ShowActiveClientFrame(pCD);
    } else if (*pCD).clientState == MINIMIZED_STATE {
        ShowActiveIcon(pCD);
    }
    wmGD.keyboardFocus = saveCD;
}
#[no_mangle]
pub unsafe extern "C" fn ClearFocusIndication(mut pCD: *mut ClientData, mut refresh: Boolean) {
    let mut saveCD = ::core::ptr::null_mut::<ClientData>();
    let mut bUnseen: Boolean = 0;
    saveCD = wmGD.keyboardFocus;
    wmGD.keyboardFocus = ::core::ptr::null_mut::<ClientData>();
    bUnseen = (if (*pCD).clientState & UNSEEN_STATE != 0 {
        True
    } else {
        False
    }) as Boolean;
    if bUnseen != 0 {
        (*pCD).clientState &= !UNSEEN_STATE;
    }
    if (*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE {
        ShowInactiveClientFrame(pCD);
    } else if (*pCD).clientState == MINIMIZED_STATE {
        ShowInactiveIcon(pCD, refresh);
    }
    if bUnseen != 0 {
        (*pCD).clientState |= UNSEEN_STATE;
    }
    wmGD.keyboardFocus = saveCD;
}
#[no_mangle]
pub unsafe extern "C" fn GetClientUnderPointer(mut pSameScreen: *mut Boolean) -> *mut ClientData {
    let mut root: Window = 0;
    let mut child: Window = 0;
    let mut rootX: ::core::ffi::c_int = 0;
    let mut rootY: ::core::ffi::c_int = 0;
    let mut winX: ::core::ffi::c_int = 0;
    let mut winY: ::core::ffi::c_int = 0;
    let mut mask: ::core::ffi::c_uint = 0;
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    *pSameScreen = XQueryPointer(
        wmGD.display,
        (*wmGD.pActiveSD).rootWindow,
        &raw mut root,
        &raw mut child,
        &raw mut rootX,
        &raw mut rootY,
        &raw mut winX,
        &raw mut winY,
        &raw mut mask,
    ) as Boolean;
    if *pSameScreen as ::core::ffi::c_int != False {
        if child != 0
            && XFindContext(
                wmGD.display,
                child as XID,
                wmGD.windowContextType,
                &raw mut pCD as *mut XPointer,
            ) == 0
        {
            return pCD;
        }
    }
    return ::core::ptr::null_mut::<ClientData>();
}
#[no_mangle]
pub unsafe extern "C" fn FocusNextWindow(
    mut type_0: ::core::ffi::c_ulong,
    mut focusTime: Time,
) -> Boolean {
    let mut pCurrentEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut pNextEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut focused = False as Boolean;
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    if type_0 & F_GROUP_TRANSIENT as ::core::ffi::c_ulong != 0 {
        focused = FocusNextTransient(wmGD.keyboardFocus, type_0, False as Boolean, focusTime);
    }
    if focused == 0 {
        if wmGD.systemModalActive != 0 {
            focused = True as Boolean;
        } else if !wmGD.keyboardFocus.is_null() {
            if !(*wmGD.keyboardFocus).transientLeader.is_null() {
                pCD = FindTransientTreeLeader(wmGD.keyboardFocus);
            } else {
                pCD = wmGD.keyboardFocus;
            }
            if (*pCD).clientState == MINIMIZED_STATE {
                pCurrentEntry = &raw mut (*pCD).iconEntry;
            } else {
                pCurrentEntry = &raw mut (*pCD).clientEntry;
            }
            pNextEntry = (*pCurrentEntry).nextSibling as *mut ClientListEntry;
            if pNextEntry.is_null() {
                pNextEntry = (*wmGD.pActiveSD).clientList;
            }
        } else {
            pCurrentEntry = (*wmGD.pActiveSD).clientList;
            pNextEntry = pCurrentEntry;
        }
    }
    while focused == 0 && !pNextEntry.is_null() {
        focused = CheckForKeyFocus(pNextEntry, type_0, True as Boolean, focusTime);
        if focused == 0 {
            pNextEntry = (*pNextEntry).nextSibling as *mut ClientListEntry;
        }
    }
    if focused == 0 {
        pNextEntry = (*wmGD.pActiveSD).clientList;
        while pNextEntry != pCurrentEntry && focused == 0 {
            focused = CheckForKeyFocus(pNextEntry, type_0, True as Boolean, focusTime);
            if focused == 0 {
                pNextEntry = (*pNextEntry).nextSibling as *mut ClientListEntry;
            }
        }
    }
    return focused;
}
#[no_mangle]
pub unsafe extern "C" fn FocusNextTransient(
    mut pCD: *mut ClientData,
    mut type_0: ::core::ffi::c_ulong,
    mut initiate: Boolean,
    mut focusTime: Time,
) -> Boolean {
    let mut focused = False as Boolean;
    let mut startAt: ::core::ffi::c_ulong = 0;
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut pcdLowerLeader = ::core::ptr::null_mut::<ClientData>();
    let mut pcdFocus = ::core::ptr::null_mut::<ClientData>();
    if initiate as ::core::ffi::c_int != 0
        && type_0 & F_GROUP_TRANSIENT as ::core::ffi::c_ulong == 0
    {
        if !(*pCD).transientChildren.is_null() {
            pcdFocus = FindLastTransientTreeFocus(pCD, NULL as *mut ClientData);
            if pcdFocus != wmGD.keyboardFocus {
                pcdLeader = FindTransientTreeLeader(pcdFocus);
                if !wmGD.keyboardFocus.is_null()
                    && (*wmGD.keyboardFocus).focusAutoRaise as ::core::ffi::c_int != 0
                    && wmGD.keyboardFocus != pcdLeader
                {
                    pcdLowerLeader = FindTransientTreeLeader(wmGD.keyboardFocus);
                    if pcdLowerLeader == pcdLeader {
                        if PutTransientBelowSiblings(wmGD.keyboardFocus) != 0 {
                            RestackTransients(pcdLeader);
                        }
                    } else {
                        F_Lower(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            wmGD.keyboardFocus,
                            NULL as *mut XEvent,
                        );
                    }
                }
                Do_Focus_Key(pcdFocus, focusTime, ALWAYS_SET_FOCUS);
            }
            focused = True as Boolean;
        } else {
            focused = False as Boolean;
        }
    } else if !pCD.is_null()
        && (*pCD).clientState != MINIMIZED_STATE
        && (!(*pCD).transientLeader.is_null() || !(*pCD).transientChildren.is_null())
    {
        startAt = if initiate as ::core::ffi::c_int != 0 {
            (*wmGD.pActiveSD)
                .clientCounter
                .wrapping_add(1 as ::core::ffi::c_ulong)
        } else {
            (*pCD).clientID
        };
        pcdLeader = FindTransientTreeLeader(pCD);
        pcdFocus = FindNextTFocusInSeq(pcdLeader, startAt);
        if pcdFocus.is_null() && type_0 == F_GROUP_TRANSIENT as ::core::ffi::c_ulong {
            pcdFocus = FindNextTFocusInSeq(
                pcdLeader,
                (*wmGD.pActiveSD)
                    .clientCounter
                    .wrapping_add(1 as ::core::ffi::c_ulong),
            );
        }
        if !pcdFocus.is_null() {
            if pcdFocus != wmGD.keyboardFocus {
                if !wmGD.keyboardFocus.is_null()
                    && (*wmGD.keyboardFocus).focusAutoRaise as ::core::ffi::c_int != 0
                    && wmGD.keyboardFocus != pcdLeader
                {
                    pcdLowerLeader = FindTransientTreeLeader(wmGD.keyboardFocus);
                    if pcdLowerLeader == pcdLeader {
                        if PutTransientBelowSiblings(wmGD.keyboardFocus) != 0 {
                            RestackTransients(pcdLeader);
                        }
                    } else {
                        F_Lower(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            wmGD.keyboardFocus,
                            NULL as *mut XEvent,
                        );
                    }
                }
                Do_Focus_Key(pcdFocus, focusTime, ALWAYS_SET_FOCUS);
            }
            focused = True as Boolean;
        }
    } else if type_0 == F_GROUP_TRANSIENT as ::core::ffi::c_ulong {
        focused = True as Boolean;
    }
    return focused;
}
#[no_mangle]
pub unsafe extern "C" fn FindLastTransientTreeFocus(
    mut pCD: *mut ClientData,
    mut pcdNoFocus: *mut ClientData,
) -> *mut ClientData {
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    let mut pcdFocus = ::core::ptr::null_mut::<ClientData>();
    let mut pcdLastFocus = ::core::ptr::null_mut::<ClientData>();
    pcdNext = (*pCD).transientChildren as *mut ClientData;
    while !pcdNext.is_null() {
        pcdFocus = FindLastTransientTreeFocus(pcdNext, pcdNoFocus);
        if !pcdFocus.is_null()
            && !((*pcdFocus).primaryModalCount != 0 || (*pcdFocus).fullModalCount != 0)
            && (pcdLastFocus.is_null() || (*pcdFocus).focusPriority > (*pcdLastFocus).focusPriority)
        {
            pcdLastFocus = pcdFocus;
        }
        pcdNext = (*pcdNext).transientSiblings as *mut ClientData;
    }
    if !((*pCD).primaryModalCount != 0 || (*pCD).fullModalCount != 0)
        && (pcdLastFocus.is_null() || (*pCD).focusPriority > (*pcdLastFocus).focusPriority)
    {
        pcdLastFocus = pCD;
    }
    return pcdLastFocus;
}
#[no_mangle]
pub unsafe extern "C" fn FindNextTFocusInSeq(
    mut pCD: *mut ClientData,
    mut startAt: ::core::ffi::c_ulong,
) -> *mut ClientData {
    let mut pcdNextFocus = ::core::ptr::null_mut::<ClientData>();
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    let mut pcdFocus = ::core::ptr::null_mut::<ClientData>();
    pcdNext = (*pCD).transientChildren as *mut ClientData;
    while !pcdNext.is_null() {
        pcdFocus = FindNextTFocusInSeq(pcdNext, startAt);
        if !pcdFocus.is_null() {
            if pcdNextFocus.is_null() || (*pcdFocus).clientID > (*pcdNextFocus).clientID {
                pcdNextFocus = pcdFocus;
            }
        }
        pcdNext = (*pcdNext).transientSiblings as *mut ClientData;
    }
    if pcdNextFocus.is_null() || (*pCD).clientID > (*pcdNextFocus).clientID {
        if !((*pCD).primaryModalCount != 0 || (*pCD).fullModalCount != 0)
            && (*pCD).clientID < startAt
        {
            pcdNextFocus = pCD;
        }
    }
    return pcdNextFocus;
}
#[no_mangle]
pub unsafe extern "C" fn FocusPrevWindow(
    mut type_0: ::core::ffi::c_ulong,
    mut focusTime: Time,
) -> Boolean {
    let mut pCurrentEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut pNextEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut focused = False as Boolean;
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    if type_0 & F_GROUP_TRANSIENT as ::core::ffi::c_ulong != 0 {
        focused = FocusPrevTransient(wmGD.keyboardFocus, type_0, False as Boolean, focusTime);
    }
    if focused == 0 {
        if wmGD.systemModalActive != 0 {
            focused = True as Boolean;
        } else if !wmGD.keyboardFocus.is_null() {
            if !(*wmGD.keyboardFocus).transientLeader.is_null() {
                pCD = FindTransientTreeLeader(wmGD.keyboardFocus);
            } else {
                pCD = wmGD.keyboardFocus;
            }
            if (*pCD).clientState == MINIMIZED_STATE {
                pCurrentEntry = &raw mut (*pCD).iconEntry;
            } else {
                pCurrentEntry = &raw mut (*pCD).clientEntry;
            }
            pNextEntry = (*pCurrentEntry).prevSibling as *mut ClientListEntry;
            if pNextEntry.is_null() {
                pNextEntry = (*wmGD.pActiveSD).lastClient;
            }
        } else {
            pCurrentEntry = (*wmGD.pActiveSD).lastClient;
            pNextEntry = pCurrentEntry;
        }
    }
    while focused == 0 && !pNextEntry.is_null() {
        focused = CheckForKeyFocus(pNextEntry, type_0, False as Boolean, focusTime);
        if focused == 0 {
            pNextEntry = (*pNextEntry).prevSibling as *mut ClientListEntry;
        }
    }
    if focused == 0 {
        pNextEntry = (*wmGD.pActiveSD).lastClient;
        while pNextEntry != pCurrentEntry && focused == 0 {
            focused = CheckForKeyFocus(pNextEntry, type_0, False as Boolean, focusTime);
            if focused == 0 {
                pNextEntry = (*pNextEntry).prevSibling as *mut ClientListEntry;
            }
        }
    }
    return focused;
}
#[no_mangle]
pub unsafe extern "C" fn FocusPrevTransient(
    mut pCD: *mut ClientData,
    mut type_0: ::core::ffi::c_ulong,
    mut initiate: Boolean,
    mut focusTime: Time,
) -> Boolean {
    let mut focused = False as Boolean;
    let mut startAt: ::core::ffi::c_ulong = 0;
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut pcdFocus = ::core::ptr::null_mut::<ClientData>();
    if initiate as ::core::ffi::c_int != 0
        && type_0 & F_GROUP_TRANSIENT as ::core::ffi::c_ulong == 0
    {
        if !(*pCD).transientChildren.is_null() {
            pcdFocus = FindLastTransientTreeFocus(pCD, NULL as *mut ClientData);
            if pcdFocus != wmGD.keyboardFocus {
                Do_Focus_Key(pcdFocus, focusTime, ALWAYS_SET_FOCUS);
            }
            focused = True as Boolean;
        } else {
            focused = False as Boolean;
        }
    } else if !pCD.is_null()
        && (*pCD).clientState != MINIMIZED_STATE
        && (!(*pCD).transientLeader.is_null() || !(*pCD).transientChildren.is_null())
    {
        startAt = if initiate as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_ulong
        } else {
            (*pCD).clientID
        };
        pcdLeader = FindTransientTreeLeader(pCD);
        pcdFocus = FindPrevTFocusInSeq(pcdLeader, startAt);
        if pcdFocus.is_null() && type_0 == F_GROUP_TRANSIENT as ::core::ffi::c_ulong {
            pcdFocus = FindPrevTFocusInSeq(pcdLeader, 0 as ::core::ffi::c_ulong);
        }
        if !pcdFocus.is_null() {
            if pcdFocus != wmGD.keyboardFocus {
                Do_Focus_Key(pcdFocus, focusTime, ALWAYS_SET_FOCUS);
            }
            focused = True as Boolean;
        }
    } else if type_0 == F_GROUP_TRANSIENT as ::core::ffi::c_ulong {
        focused = True as Boolean;
    }
    return focused;
}
#[no_mangle]
pub unsafe extern "C" fn FindPrevTFocusInSeq(
    mut pCD: *mut ClientData,
    mut startAt: ::core::ffi::c_ulong,
) -> *mut ClientData {
    let mut pcdNextFocus = ::core::ptr::null_mut::<ClientData>();
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    let mut pcdFocus = ::core::ptr::null_mut::<ClientData>();
    pcdNext = (*pCD).transientChildren as *mut ClientData;
    while !pcdNext.is_null() {
        pcdFocus = FindPrevTFocusInSeq(pcdNext, startAt);
        if !pcdFocus.is_null() {
            if pcdNextFocus.is_null() || (*pcdFocus).clientID < (*pcdNextFocus).clientID {
                pcdNextFocus = pcdFocus;
            }
        }
        pcdNext = (*pcdNext).transientSiblings as *mut ClientData;
    }
    if pcdNextFocus.is_null() || (*pCD).clientID < (*pcdNextFocus).clientID {
        if !((*pCD).primaryModalCount != 0 || (*pCD).fullModalCount != 0)
            && (*pCD).clientID > startAt
        {
            pcdNextFocus = pCD;
        }
    }
    return pcdNextFocus;
}
#[no_mangle]
pub unsafe extern "C" fn CheckForKeyFocus(
    mut pNextEntry: *mut ClientListEntry,
    mut type_0: ::core::ffi::c_ulong,
    mut focusNext: Boolean,
    mut focusTime: Time,
) -> Boolean {
    let mut pCD = (*pNextEntry).pCD as *mut ClientData;
    let mut windowType: ::core::ffi::c_ulong = 0;
    let mut focused = False as Boolean;
    if (*pCD).desktopSurface != 0 {
        return False as Boolean;
    }
    if (*pNextEntry).type_0 == NORMAL_STATE
        && (*pCD).clientState & UNSEEN_STATE == 0
        && (*pCD).clientState != MINIMIZED_STATE
        || (*pNextEntry).type_0 == MINIMIZED_STATE && (*pCD).clientState == MINIMIZED_STATE
    {
        if (*pCD).clientState == MINIMIZED_STATE {
            windowType = F_GROUP_ICON as ::core::ffi::c_ulong;
        } else {
            if focusNext != 0 {
                focused = FocusNextTransient(pCD, type_0, True as Boolean, focusTime);
            } else {
                focused = FocusPrevTransient(pCD, type_0, True as Boolean, focusTime);
            }
            windowType = F_GROUP_WINDOW as ::core::ffi::c_ulong;
            if !(*pCD).transientLeader.is_null() || !(*pCD).transientChildren.is_null() {
                windowType |= F_GROUP_TRANSIENT as ::core::ffi::c_ulong;
            }
        }
        if focused == 0 && type_0 & windowType != 0 {
            focused = True as Boolean;
            if focusNext as ::core::ffi::c_int != 0
                && !wmGD.keyboardFocus.is_null()
                && (*wmGD.keyboardFocus).focusAutoRaise as ::core::ffi::c_int != 0
            {
                F_Lower(
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    wmGD.keyboardFocus,
                    NULL as *mut XEvent,
                );
            }
            Do_Focus_Key(pCD, focusTime, ALWAYS_SET_FOCUS);
        }
    }
    return focused;
}
#[no_mangle]
pub unsafe extern "C" fn RepairFocus() {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut sameScreen: Boolean = 0;
    let mut event = _XEvent { type_0: 0 };
    if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER
        || wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER
    {
        XSync(wmGD.display, False);
        while XCheckMaskEvent(
            wmGD.display,
            EnterWindowMask | LeaveWindowMask,
            &raw mut event,
        ) != 0
        {}
        pCD = GetClientUnderPointer(&raw mut sameScreen);
        if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER {
            Do_Focus_Key(pCD, CurrentTime as Time, ALWAYS_SET_FOCUS);
        } else if wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER {
            SetColormapFocus(wmGD.pActiveSD, pCD);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn AutoResetKeyFocus(mut pcdNoFocus: *mut ClientData, mut focusTime: Time) {
    let mut pNextEntry = ::core::ptr::null_mut::<ClientListEntry>();
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut pcdLastFocus = ::core::ptr::null_mut::<ClientData>();
    let mut pcdFocus = ::core::ptr::null_mut::<ClientData>();
    pNextEntry = (*wmGD.pActiveSD).clientList;
    while !pNextEntry.is_null() {
        pCD = (*pNextEntry).pCD as *mut ClientData;
        if wmGD.systemModalActive == 0 || wmGD.systemModalClient == pCD {
            if (*pNextEntry).type_0 != MINIMIZED_STATE
                && (*pCD).clientState != MINIMIZED_STATE
                && (*pCD).clientState & UNSEEN_STATE == 0
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
    if !pcdLastFocus.is_null() {
        Do_Focus_Key(pcdLastFocus, focusTime, ALWAYS_SET_FOCUS);
    } else {
        Do_Focus_Key(NULL as *mut ClientData, focusTime, ALWAYS_SET_FOCUS);
    };
}
