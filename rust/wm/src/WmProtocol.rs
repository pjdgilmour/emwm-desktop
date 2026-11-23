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
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_1;
use crate::abi::*;
extern "C" {
    fn XInternAtoms(
        _: *mut Display,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut Atom,
    ) -> ::core::ffi::c_int;
    fn XFlush(_: *mut Display) -> ::core::ffi::c_int;
    fn XKillClient(_: *mut Display, _: XID) -> ::core::ffi::c_int;
    fn XSendEvent(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_long,
        _: *mut XEvent,
    ) -> ::core::ffi::c_int;
    fn XSetIconSizes(
        _: *mut Display,
        _: Window,
        _: *mut XIconSize,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XtAppAddTimeOut(
        _: XtAppContext,
        _: ::core::ffi::c_ulong,
        _: XtTimerCallbackProc,
        _: XtPointer,
    ) -> XtIntervalId;
    fn XtRemoveTimeOut(_: XtIntervalId);
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    static mut wmGD: WmGlobalData;
    fn Warning(message: *mut ::core::ffi::c_char);
    fn Do_Raise(pCD: *mut ClientData, pStackEntry: *mut ClientListEntry, flags: ::core::ffi::c_int);
    fn GetClientUnderPointer(pSameScreen: *mut Boolean) -> *mut ClientData;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const XA_COMPOUND_TEXT: C2RustUnnamed_1 = 11;
pub const XA_MOTIF_WM_OFFSET: C2RustUnnamed_1 = 10;
pub const XA_MWM_MESSAGES: C2RustUnnamed_1 = 9;
pub const XA_MWM_MENU: C2RustUnnamed_1 = 8;
pub const XA_MWM_HINTS: C2RustUnnamed_1 = 7;
pub const XA_WM_TAKE_FOCUS: C2RustUnnamed_1 = 6;
pub const XA_WM_COLORMAP_WINDOWS: C2RustUnnamed_1 = 5;
pub const XA_WM_DELETE_WINDOW: C2RustUnnamed_1 = 4;
pub const XA_WM_SAVE_YOURSELF: C2RustUnnamed_1 = 3;
pub const XA_WM_CHANGE_STATE: C2RustUnnamed_1 = 2;
pub const XA_WM_PROTOCOLS: C2RustUnnamed_1 = 1;
pub const XA_WM_STATE: C2RustUnnamed_1 = 0;
pub const NUM_ATOMS: C2RustUnnamed_1 = 12;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NoEventMask: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const StructureNotifyMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 17 as ::core::ffi::c_int;
pub const ConfigureNotify: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ClientMessage: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const NorthWestGravity: ::core::ffi::c_int = 1;
pub const NorthGravity: ::core::ffi::c_int = 2;
pub const NorthEastGravity: ::core::ffi::c_int = 3;
pub const WestGravity: ::core::ffi::c_int = 4;
pub const CenterGravity: ::core::ffi::c_int = 5;
pub const EastGravity: ::core::ffi::c_int = 6;
pub const SouthWestGravity: ::core::ffi::c_int = 7;
pub const SouthGravity: ::core::ffi::c_int = 8;
pub const SouthEastGravity: ::core::ffi::c_int = 9;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const _XA_MOTIF_WM_HINTS: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"_MOTIF_WM_HINTS\0") };
pub const _XA_MWM_HINTS: [::core::ffi::c_char; 16] = _XA_MOTIF_WM_HINTS;
pub const _XA_MOTIF_WM_MESSAGES: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"_MOTIF_WM_MESSAGES\0")
};
pub const _XA_MWM_MESSAGES: [::core::ffi::c_char; 19] = _XA_MOTIF_WM_MESSAGES;
pub const _XA_MOTIF_WM_OFFSET: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"_MOTIF_WM_OFFSET\0")
};
pub const _XA_MOTIF_WM_MENU: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"_MOTIF_WM_MENU\0") };
pub const _XA_MWM_MENU: [::core::ffi::c_char; 15] = _XA_MOTIF_WM_MENU;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _XA_WM_STATE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"WM_STATE\0") };
pub const _XA_WM_PROTOCOLS: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"WM_PROTOCOLS\0") };
pub const _XA_WM_CHANGE_STATE: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"WM_CHANGE_STATE\0") };
pub const _XA_WM_SAVE_YOURSELF: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"WM_SAVE_YOURSELF\0")
};
pub const _XA_WM_DELETE_WINDOW: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"WM_DELETE_WINDOW\0")
};
pub const _XA_WM_TAKE_FOCUS: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"WM_TAKE_FOCUS\0") };
pub const _XA_WM_COLORMAP_WINDOWS: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"WM_COLORMAP_WINDOWS\0")
};
pub const TIMER_QUIT: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TIMER_RAISE: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STACK_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn SetupWmICCC() {
    static mut atom_names: [*mut ::core::ffi::c_char; 12] = [
        _XA_WM_STATE.as_ptr() as *mut ::core::ffi::c_char,
        _XA_WM_PROTOCOLS.as_ptr() as *mut ::core::ffi::c_char,
        _XA_WM_CHANGE_STATE.as_ptr() as *mut ::core::ffi::c_char,
        _XA_WM_SAVE_YOURSELF.as_ptr() as *mut ::core::ffi::c_char,
        _XA_WM_DELETE_WINDOW.as_ptr() as *mut ::core::ffi::c_char,
        _XA_WM_COLORMAP_WINDOWS.as_ptr() as *mut ::core::ffi::c_char,
        _XA_WM_TAKE_FOCUS.as_ptr() as *mut ::core::ffi::c_char,
        _XA_MWM_HINTS.as_ptr() as *mut ::core::ffi::c_char,
        _XA_MWM_MENU.as_ptr() as *mut ::core::ffi::c_char,
        _XA_MWM_MESSAGES.as_ptr() as *mut ::core::ffi::c_char,
        _XA_MOTIF_WM_OFFSET.as_ptr() as *mut ::core::ffi::c_char,
        b"COMPOUND_TEXT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ];
    let mut sizeList = XIconSize {
        min_width: 0,
        min_height: 0,
        max_width: 0,
        max_height: 0,
        width_inc: 0,
        height_inc: 0,
    };
    let mut scr: ::core::ffi::c_int = 0;
    let mut atoms: [Atom; 12] = [0; 12];
    XInternAtoms(
        wmGD.display,
        &raw mut atom_names as *mut *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[*mut ::core::ffi::c_char; 12]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as Cardinal as ::core::ffi::c_int,
        False,
        &raw mut atoms as *mut Atom,
    );
    wmGD.xa_WM_STATE = atoms[XA_WM_STATE as ::core::ffi::c_int as usize];
    wmGD.xa_WM_PROTOCOLS = atoms[XA_WM_PROTOCOLS as ::core::ffi::c_int as usize];
    wmGD.xa_WM_CHANGE_STATE = atoms[XA_WM_CHANGE_STATE as ::core::ffi::c_int as usize];
    wmGD.xa_WM_SAVE_YOURSELF = atoms[XA_WM_SAVE_YOURSELF as ::core::ffi::c_int as usize];
    wmGD.xa_WM_DELETE_WINDOW = atoms[XA_WM_DELETE_WINDOW as ::core::ffi::c_int as usize];
    wmGD.xa_WM_COLORMAP_WINDOWS = atoms[XA_WM_COLORMAP_WINDOWS as ::core::ffi::c_int as usize];
    wmGD.xa_WM_TAKE_FOCUS = atoms[XA_WM_TAKE_FOCUS as ::core::ffi::c_int as usize];
    wmGD.xa_MWM_HINTS = atoms[XA_MWM_HINTS as ::core::ffi::c_int as usize];
    wmGD.xa_MWM_MENU = atoms[XA_MWM_MENU as ::core::ffi::c_int as usize];
    wmGD.xa_MWM_MESSAGES = atoms[XA_MWM_MESSAGES as ::core::ffi::c_int as usize];
    wmGD.xa_MWM_OFFSET = atoms[XA_MOTIF_WM_OFFSET as ::core::ffi::c_int as usize];
    wmGD.xa_COMPOUND_TEXT = atoms[XA_COMPOUND_TEXT as ::core::ffi::c_int as usize];
    sizeList.width_inc = 1 as ::core::ffi::c_int;
    sizeList.height_inc = 1 as ::core::ffi::c_int;
    scr = 0 as ::core::ffi::c_int;
    while scr < wmGD.numScreens {
        if (*wmGD.Screens.offset(scr as isize)).managed != 0 {
            sizeList.min_width = (*wmGD.Screens.offset(scr as isize)).iconImageMinimum.width;
            sizeList.min_height = (*wmGD.Screens.offset(scr as isize)).iconImageMinimum.height;
            sizeList.max_width = (*wmGD.Screens.offset(scr as isize)).iconImageMaximum.width;
            sizeList.max_height = (*wmGD.Screens.offset(scr as isize)).iconImageMaximum.height;
            XSetIconSizes(
                wmGD.display,
                (*wmGD.Screens.offset(scr as isize)).rootWindow,
                &raw mut sizeList,
                1 as ::core::ffi::c_int,
            );
        }
        scr += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SendConfigureNotify(mut pCD: *mut ClientData) {
    let mut notifyEvent = XConfigureEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: ::core::ptr::null_mut::<Display>(),
        event: 0,
        window: 0,
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        above: 0,
        override_redirect: 0,
    };
    notifyEvent.type_0 = ConfigureNotify;
    notifyEvent.display = wmGD.display;
    notifyEvent.event = (*pCD).client;
    notifyEvent.window = (*pCD).client;
    if (*pCD).fullScreen != 0 {
        notifyEvent.x = (*pCD).fullScreenX;
        notifyEvent.y = (*pCD).fullScreenY;
        notifyEvent.width = (*pCD).fullScreenWidth;
        notifyEvent.height = (*pCD).fullScreenHeight;
    } else if (*pCD).maxConfig != 0 {
        notifyEvent.x = (*pCD).maxX;
        notifyEvent.y = (*pCD).maxY;
        notifyEvent.width = (*pCD).maxWidth;
        notifyEvent.height = (*pCD).maxHeight;
    } else {
        notifyEvent.x = (*pCD).clientX;
        notifyEvent.y = (*pCD).clientY;
        notifyEvent.width = (*pCD).clientWidth;
        notifyEvent.height = (*pCD).clientHeight;
    }
    notifyEvent.border_width = 0 as ::core::ffi::c_int;
    notifyEvent.above = None_0 as Window;
    notifyEvent.override_redirect = False;
    XSendEvent(
        wmGD.display,
        (*pCD).client,
        False,
        StructureNotifyMask,
        &raw mut notifyEvent as *mut XEvent,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SendClientOffsetMessage(mut pCD: *mut ClientData) {
    let mut borderWidth = (*pCD).xBorderWidth as ::core::ffi::c_long;
    let mut offsetX = (*pCD).clientOffset.x as ::core::ffi::c_long;
    let mut offsetY = (*pCD).clientOffset.y as ::core::ffi::c_long;
    let mut clientMsgEvent = XClientMessageEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: ::core::ptr::null_mut::<Display>(),
        window: 0,
        message_type: 0,
        format: 0,
        data: C2RustUnnamed { b: [0; 20] },
    };
    clientMsgEvent.type_0 = ClientMessage;
    clientMsgEvent.window = (*pCD).client;
    clientMsgEvent.message_type = wmGD.xa_MWM_MESSAGES;
    clientMsgEvent.format = 32 as ::core::ffi::c_int;
    clientMsgEvent.data.l[0 as ::core::ffi::c_int as usize] =
        wmGD.xa_MWM_OFFSET as ::core::ffi::c_long;
    match (*pCD).windowGravity {
        NorthGravity => {
            clientMsgEvent.data.l[1 as ::core::ffi::c_int as usize] = borderWidth;
            clientMsgEvent.data.l[2 as ::core::ffi::c_int as usize] = offsetY;
        }
        NorthEastGravity => {
            clientMsgEvent.data.l[1 as ::core::ffi::c_int as usize] =
                -(offsetX - 2 as ::core::ffi::c_long * borderWidth);
            clientMsgEvent.data.l[2 as ::core::ffi::c_int as usize] = offsetY;
        }
        EastGravity => {
            clientMsgEvent.data.l[1 as ::core::ffi::c_int as usize] =
                -(offsetX - 2 as ::core::ffi::c_long * borderWidth);
            clientMsgEvent.data.l[2 as ::core::ffi::c_int as usize] =
                borderWidth + (offsetY - offsetX) / 2 as ::core::ffi::c_long;
        }
        SouthEastGravity => {
            clientMsgEvent.data.l[1 as ::core::ffi::c_int as usize] =
                -(offsetX - 2 as ::core::ffi::c_long * borderWidth);
            clientMsgEvent.data.l[2 as ::core::ffi::c_int as usize] =
                -(offsetX - 2 as ::core::ffi::c_long * borderWidth);
        }
        SouthGravity => {
            clientMsgEvent.data.l[1 as ::core::ffi::c_int as usize] = borderWidth;
            clientMsgEvent.data.l[2 as ::core::ffi::c_int as usize] =
                -(offsetX - 2 as ::core::ffi::c_long * borderWidth);
        }
        SouthWestGravity => {
            clientMsgEvent.data.l[1 as ::core::ffi::c_int as usize] = offsetX;
            clientMsgEvent.data.l[2 as ::core::ffi::c_int as usize] =
                -(offsetX - 2 as ::core::ffi::c_long * borderWidth);
        }
        WestGravity => {
            clientMsgEvent.data.l[1 as ::core::ffi::c_int as usize] = offsetX;
            clientMsgEvent.data.l[2 as ::core::ffi::c_int as usize] =
                borderWidth + (offsetY - offsetX) / 2 as ::core::ffi::c_long;
        }
        CenterGravity => {
            clientMsgEvent.data.l[2 as ::core::ffi::c_int as usize] =
                (offsetY - offsetX) / 2 as ::core::ffi::c_long;
        }
        NorthWestGravity | _ => {
            clientMsgEvent.data.l[1 as ::core::ffi::c_int as usize] = offsetX;
            clientMsgEvent.data.l[2 as ::core::ffi::c_int as usize] = offsetY;
        }
    }
    XSendEvent(
        wmGD.display,
        (*pCD).client,
        False,
        NoEventMask,
        &raw mut clientMsgEvent as *mut XEvent,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SendClientMsg(
    mut window: Window,
    mut type_0: ::core::ffi::c_long,
    mut data0: ::core::ffi::c_long,
    mut time: Time,
    mut pData: *mut ::core::ffi::c_long,
    mut dataLen: ::core::ffi::c_int,
) {
    let mut clientMsgEvent = XClientMessageEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: ::core::ptr::null_mut::<Display>(),
        window: 0,
        message_type: 0,
        format: 0,
        data: C2RustUnnamed { b: [0; 20] },
    };
    let mut i: ::core::ffi::c_int = 0;
    clientMsgEvent.type_0 = ClientMessage;
    clientMsgEvent.window = window;
    clientMsgEvent.message_type = type_0 as Atom;
    clientMsgEvent.format = 32 as ::core::ffi::c_int;
    clientMsgEvent.data.l[0 as ::core::ffi::c_int as usize] = data0;
    clientMsgEvent.data.l[1 as ::core::ffi::c_int as usize] = time as ::core::ffi::c_long;
    if !pData.is_null() {
        if dataLen > 3 as ::core::ffi::c_int {
            dataLen = 3 as ::core::ffi::c_int;
        }
        i = 2 as ::core::ffi::c_int;
        while i < 2 as ::core::ffi::c_int + dataLen {
            clientMsgEvent.data.l[i as usize] = *pData.offset(i as isize);
            i += 1;
        }
    }
    XSendEvent(
        wmGD.display,
        window,
        False,
        NoEventMask,
        &raw mut clientMsgEvent as *mut XEvent,
    );
    XFlush(wmGD.display);
}
#[no_mangle]
pub unsafe extern "C" fn AddWmTimer(
    mut timerType: ::core::ffi::c_uint,
    mut timerInterval: ::core::ffi::c_ulong,
    mut pCD: *mut ClientData,
) -> Boolean {
    let mut pWmTimer = ::core::ptr::null_mut::<WmTimer>();
    pWmTimer = XtMalloc(::core::mem::size_of::<WmTimer>() as Cardinal) as *mut WmTimer;
    if pWmTimer.is_null() {
        Warning(
            b"Insufficient memory for window manager data\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as Boolean;
    }
    (*pWmTimer).timerId = XtAppAddTimeOut(
        wmGD.mwmAppContext,
        timerInterval,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(XtPointer, *mut XtIntervalId) -> ()>,
            XtTimerCallbackProc,
        >(Some(
            TimeoutProc as unsafe extern "C" fn(XtPointer, *mut XtIntervalId) -> (),
        )),
        pCD as XtPointer,
    );
    (*pWmTimer).timerCD = pCD as *mut _ClientData;
    (*pWmTimer).timerType = timerType;
    (*pWmTimer).nextWmTimer = wmGD.wmTimers as *mut _WmTimer;
    wmGD.wmTimers = pWmTimer;
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn DeleteClientWmTimers(mut pCD: *mut ClientData) {
    let mut pPrevTimer = ::core::ptr::null_mut::<WmTimer>();
    let mut pWmTimer = ::core::ptr::null_mut::<WmTimer>();
    let mut pRemoveTimer = ::core::ptr::null_mut::<WmTimer>();
    pPrevTimer = ::core::ptr::null_mut::<WmTimer>();
    pWmTimer = wmGD.wmTimers;
    while !pWmTimer.is_null() {
        if (*pWmTimer).timerCD == pCD {
            if !pPrevTimer.is_null() {
                (*pPrevTimer).nextWmTimer = (*pWmTimer).nextWmTimer;
            } else {
                wmGD.wmTimers = (*pWmTimer).nextWmTimer as *mut WmTimer;
            }
            pRemoveTimer = pWmTimer;
            pWmTimer = (*pWmTimer).nextWmTimer as *mut WmTimer;
            XtRemoveTimeOut((*pRemoveTimer).timerId);
            XtFree(pRemoveTimer as *mut ::core::ffi::c_char);
        } else {
            pPrevTimer = pWmTimer;
            pWmTimer = (*pWmTimer).nextWmTimer as *mut WmTimer;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn TimeoutProc(mut client_data: XtPointer, mut id: *mut XtIntervalId) {
    let mut pPrevTimer = ::core::ptr::null_mut::<WmTimer>();
    let mut pWmTimer = ::core::ptr::null_mut::<WmTimer>();
    pPrevTimer = ::core::ptr::null_mut::<WmTimer>();
    pWmTimer = wmGD.wmTimers;
    while !pWmTimer.is_null() {
        if (*pWmTimer).timerId == *id {
            break;
        }
        pPrevTimer = pWmTimer;
        pWmTimer = (*pWmTimer).nextWmTimer as *mut WmTimer;
    }
    if !pWmTimer.is_null() {
        match (*pWmTimer).timerType {
            1 => {
                XKillClient(wmGD.display, (*(*pWmTimer).timerCD).client as XID);
            }
            2 => {
                let mut sameScreen: Boolean = 0;
                if wmGD.keyboardFocus == (*pWmTimer).timerCD
                    && (*(*pWmTimer).timerCD).focusPriority
                        == (*(*(*pWmTimer).timerCD).pSD).focusPriority
                    && wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER
                    && (*pWmTimer).timerCD == GetClientUnderPointer(&raw mut sameScreen)
                {
                    Do_Raise(
                        (*pWmTimer).timerCD as *mut ClientData,
                        NULL as *mut ClientListEntry,
                        STACK_NORMAL,
                    );
                }
            }
            _ => {}
        }
        if !pPrevTimer.is_null() {
            (*pPrevTimer).nextWmTimer = (*pWmTimer).nextWmTimer;
        } else {
            wmGD.wmTimers = (*pWmTimer).nextWmTimer as *mut WmTimer;
        }
        XtFree(pWmTimer as *mut ::core::ffi::c_char);
    }
    XtRemoveTimeOut(*id);
}
