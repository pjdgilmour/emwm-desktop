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
use crate::abi::_ObjectPart_WmEvent as _ObjectPart;
use crate::abi::_ObjectRec_WmEvent as _ObjectRec;
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_0;
use crate::abi::ObjectPart_WmEvent as ObjectPart;
use crate::abi::Object_WmEvent as Object;
use crate::abi::*;
extern "C" {
    fn XAllowEvents(_: *mut Display, _: ::core::ffi::c_int, _: Time) -> ::core::ffi::c_int;
    fn XChangeProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: Atom,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XChangeWindowAttributes(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> ::core::ffi::c_int;
    fn XCheckMaskEvent(
        _: *mut Display,
        _: ::core::ffi::c_long,
        _: *mut XEvent,
    ) -> ::core::ffi::c_int;
    fn XCheckWindowEvent(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_long,
        _: *mut XEvent,
    ) -> ::core::ffi::c_int;
    fn XConfigureWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_uint,
        _: *mut XWindowChanges,
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
    fn XGrabKey(
        _: *mut Display,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XPutBackEvent(_: *mut Display, _: *mut XEvent) -> ::core::ffi::c_int;
    fn XSendEvent(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_long,
        _: *mut XEvent,
    ) -> ::core::ffi::c_int;
    fn XSetCommand(
        _: *mut Display,
        _: Window,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XSetInputFocus(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: Time,
    ) -> ::core::ffi::c_int;
    fn XSync(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XUngrabButton(
        _: *mut Display,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: Window,
    ) -> ::core::ffi::c_int;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
    fn XtDispatchEvent(_: *mut XEvent) -> Boolean;
    fn XtIsSensitive(_: Widget) -> Boolean;
    fn XtCalloc(_: Cardinal, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    static mut wmGD: WmGlobalData;
    fn IdentifyFramePart(
        pCD: *mut ClientData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn PushGadgetIn(pcd: *mut ClientData, gadget: ::core::ffi::c_int);
    fn PopGadgetOut(pcd: *mut ClientData, gadget: ::core::ffi::c_int);
    fn ReloadTitleGradients(screen: *mut WmScreenData);
    fn CheckButtonPressBuiltin(
        buttonEvent: *mut XButtonEvent,
        context: Context,
        subContext: Context,
        partContext: ::core::ffi::c_int,
        pCD: *mut ClientData,
    );
    fn CheckButtonReleaseBuiltin(
        buttonEvent: *mut XButtonEvent,
        context: Context,
        subContext: Context,
        pCD: *mut ClientData,
    );
    fn GetScreenForWindow(win: Window) -> *mut WmScreenData;
    fn HandleCConfigureRequest(pCD: *mut ClientData, configureRequest: *mut XConfigureRequestEvent);
    fn WmDispatchClientEvent(event: *mut XEvent) -> Boolean;
    fn SetColormapFocus(pSD: *mut WmScreenData, pCD: *mut ClientData);
    fn Do_Focus_Key(pCD: *mut ClientData, focusTime: Time, flags: ::core::ffi::c_long);
    fn F_Pass_Key(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Menu(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Post_SMenu(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Kill(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Restore_And_Raise(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn GetClientUnderPointer(pSameScreen: *mut Boolean) -> *mut ClientData;
    fn SetKeyboardFocus(pCD: *mut ClientData, focusFlags: ::core::ffi::c_long);
    fn ManageWindow(pSD: *mut WmScreenData, clientWindow: Window, manageFlags: ::core::ffi::c_long);
    fn UnManageWindow(pCD: *mut ClientData);
    fn UnpostMenu(menuSpec: *mut MenuSpec);
    fn GetWmRequest(
        pSD: *mut WmScreenData,
        pszReq: *mut *mut ::core::ffi::c_char,
        pmore: *mut Boolean,
    );
    fn WmGetWindowAttributes(window: Window) -> Boolean;
    fn SetClientState(pCD: *mut ClientData, newState: ::core::ffi::c_int, setTime: Time);
    fn _XmGetMenuState(_: Widget) -> XmMenuState;
    fn HandleEwmhRootClientMessage(_: *mut WmScreenData, _: *mut XClientMessageEvent) -> Boolean;
    fn ProcessWmRequest(pSD: *mut WmScreenData, pchReq: *mut ::core::ffi::c_char);
    static mut buttonModifierMasks: [::core::ffi::c_uint; 0];
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const PointerRoot: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
pub const CurrentTime: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ButtonPressMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ButtonReleaseMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const EnterWindowMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const LeaveWindowMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const ExposureMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 15 as ::core::ffi::c_int;
pub const VisibilityChangeMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int;
pub const StructureNotifyMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 17 as ::core::ffi::c_int;
pub const SubstructureRedirectMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 20 as ::core::ffi::c_int;
pub const FocusChangeMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 21 as ::core::ffi::c_int;
pub const PropertyChangeMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 22 as ::core::ffi::c_int;
pub const KeyPress: ::core::ffi::c_int = 2;
pub const ButtonPress: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ButtonRelease: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const EnterNotify: ::core::ffi::c_int = 7;
pub const LeaveNotify: ::core::ffi::c_int = 8;
pub const FocusIn: ::core::ffi::c_int = 9;
pub const FocusOut: ::core::ffi::c_int = 10;
pub const Expose: ::core::ffi::c_int = 12;
pub const UnmapNotify: ::core::ffi::c_int = 18;
pub const MapRequest: ::core::ffi::c_int = 20;
pub const ConfigureNotify: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ConfigureRequest: ::core::ffi::c_int = 23;
pub const PropertyNotify: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const ClientMessage: ::core::ffi::c_int = 33;
pub const Button1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NotifyNormal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NotifyUngrab: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NotifyWhileGrabbed: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NotifyInferior: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NotifyNonlinear: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NotifyNonlinearVirtual: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NotifyPointerRoot: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const NotifyDetailNone: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const GrabModeSync: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GrabModeAsync: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AsyncKeyboard: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ReplayKeyboard: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CWEventMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CWSibling: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const CWStackMode: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const PropModeAppend: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_FUNC_CLOSE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XA_RESOURCE_MANAGER: Atom = 23 as ::core::ffi::c_int as Atom;
pub const SELECT_BUTTON: ::core::ffi::c_int = Button1;
pub const MANAGEW_NORMAL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ALWAYS_SET_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const REFRESH_LAST_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const SCREEN_SWITCH_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const F_CONTEXT_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_CONTEXT_ROOT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const F_CONTEXT_ICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const F_CONTEXT_NORMAL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const F_CONTEXT_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const F_CONTEXT_ICONBOX: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const F_CONTEXT_WINDOW: ::core::ffi::c_long = F_CONTEXT_NORMAL | F_CONTEXT_MAXIMIZE;
pub const F_CONTEXT_ALL: ::core::ffi::c_long = F_CONTEXT_ROOT | F_CONTEXT_ICON | F_CONTEXT_WINDOW;
pub const WINDOW_PART_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FRAME_CLIENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FRAME_SYSTEM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICON_PART_ALL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ROOT_PART_ALL: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const ICONBOX_PART_IICON: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const ICONBOX_PART_WICON: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const F_SUBCONTEXT_NONE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << WINDOW_PART_NONE;
pub const F_SUBCONTEXT_I_ALL: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << ICON_PART_ALL;
pub const F_SUBCONTEXT_IB_IICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_IICON;
pub const F_SUBCONTEXT_IB_WICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_WICON;
pub const F_SUBCONTEXT_W_SYSTEM: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << FRAME_SYSTEM;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAXIMIZED_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CMAP_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn InitEventHandling() {
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    let mut setAttributes = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    let mut base_mask: ::core::ffi::c_ulong = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut scr: ::core::ffi::c_uint = 0;
    base_mask =
        (SubstructureRedirectMask | FocusChangeMask | PropertyChangeMask) as ::core::ffi::c_ulong;
    base_mask |= (EnterWindowMask | LeaveWindowMask) as ::core::ffi::c_ulong;
    scr = 0 as ::core::ffi::c_uint;
    while scr < wmGD.numScreens as ::core::ffi::c_uint {
        pSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
        if (*pSD).managed != 0 {
            setAttributes.event_mask = base_mask as ::core::ffi::c_long;
            if !(*pSD).buttonBindings.is_null() {
                setAttributes.event_mask |= ButtonPressMask | ButtonReleaseMask;
            }
            XChangeWindowAttributes(
                wmGD.display,
                (*pSD).rootWindow,
                CWEventMask as ::core::ffi::c_ulong,
                &raw mut setAttributes,
            );
            if !(*pSD).keySpecs.is_null() {
                SetupKeyBindings(
                    (*pSD).keySpecs,
                    (*pSD).rootWindow,
                    GrabModeSync,
                    F_CONTEXT_ALL,
                );
            }
            if (*pSD).acceleratorMenuCount != 0 {
                n = 0 as ::core::ffi::c_uint;
                while n < (*pSD).acceleratorMenuCount {
                    SetupKeyBindings(
                        (**(*pSD).acceleratorMenuSpecs.offset(n as isize)).accelKeySpecs,
                        (*pSD).rootWindow,
                        GrabModeSync,
                        F_CONTEXT_ALL,
                    );
                    n = n.wrapping_add(1);
                }
            }
        }
        scr = scr.wrapping_add(1);
    }
}
unsafe extern "C" fn _WmGrabMasks(
    mut modifiers: ::core::ffi::c_uint,
    mut pnum_masks: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uint {
    static mut pRetMasks: *mut ::core::ffi::c_uint =
        ::core::ptr::null::<::core::ffi::c_uint>() as *mut ::core::ffi::c_uint;
    static mut len_ret_masks: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num_masks: ::core::ffi::c_int = 0;
    let mut num_ret_masks: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut mask: ::core::ffi::c_uint = 0;
    num_masks = 0 as ::core::ffi::c_int;
    while *wmGD.pLockMaskSequence.offset(num_masks as isize) != 0 as ::core::ffi::c_uint {
        num_masks += 1;
    }
    if pRetMasks.is_null() || len_ret_masks < num_masks + 2 as ::core::ffi::c_int {
        if !pRetMasks.is_null() {
            XtFree(pRetMasks as *mut ::core::ffi::c_char);
        }
        len_ret_masks = num_masks + 2 as ::core::ffi::c_int;
        pRetMasks = XtCalloc(
            len_ret_masks as Cardinal,
            ::core::mem::size_of::<::core::ffi::c_uint>() as Cardinal,
        ) as *mut ::core::ffi::c_uint;
    }
    num_ret_masks = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < num_masks {
        mask = modifiers | *wmGD.pLockMaskSequence.offset(i as isize);
        if !(mask == modifiers) {
            j = 0 as ::core::ffi::c_int;
            while j < num_ret_masks {
                if mask == *pRetMasks.offset(j as isize) {
                    break;
                }
                j += 1;
            }
            if j >= num_ret_masks {
                *pRetMasks.offset(num_ret_masks as isize) = mask;
                num_ret_masks += 1;
            }
        }
        i += 1;
    }
    let fresh0 = num_ret_masks;
    num_ret_masks = num_ret_masks + 1;
    *pRetMasks.offset(fresh0 as isize) = modifiers;
    *pRetMasks.offset(num_ret_masks as isize) = 0 as ::core::ffi::c_uint;
    *pnum_masks = num_ret_masks;
    return pRetMasks;
}
#[no_mangle]
pub unsafe extern "C" fn WmGrabKey(
    mut display: *mut Display,
    mut keycode: ::core::ffi::c_int,
    mut modifiers: ::core::ffi::c_uint,
    mut grab_window: Window,
    mut owner_events: ::core::ffi::c_int,
    mut pointer_mode: ::core::ffi::c_int,
    mut keyboard_mode: ::core::ffi::c_int,
) {
    let mut pGrabMasks = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut i: ::core::ffi::c_int = 0;
    let mut num_masks: ::core::ffi::c_int = 0;
    pGrabMasks = _WmGrabMasks(modifiers, &raw mut num_masks);
    i = 0 as ::core::ffi::c_int;
    while i < num_masks {
        XGrabKey(
            display,
            keycode,
            *pGrabMasks,
            grab_window,
            owner_events,
            pointer_mode,
            keyboard_mode,
        );
        i += 1;
        pGrabMasks = pGrabMasks.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WmGrabButton(
    mut display: *mut Display,
    mut button: ::core::ffi::c_uint,
    mut modifiers: ::core::ffi::c_uint,
    mut grab_window: Window,
    mut event_mask: ::core::ffi::c_uint,
    mut owner_events: ::core::ffi::c_int,
    mut pointer_mode: ::core::ffi::c_int,
    mut keyboard_mode: ::core::ffi::c_int,
    mut confine_to: Window,
    mut cursor: Cursor,
) {
    let mut pGrabMasks = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut i: ::core::ffi::c_int = 0;
    let mut num_masks: ::core::ffi::c_int = 0;
    pGrabMasks = _WmGrabMasks(modifiers, &raw mut num_masks);
    i = 0 as ::core::ffi::c_int;
    while i < num_masks {
        XGrabButton(
            display,
            button,
            *pGrabMasks,
            grab_window,
            event_mask as ::core::ffi::c_int,
            owner_events as ::core::ffi::c_uint,
            pointer_mode,
            keyboard_mode,
            confine_to,
            cursor,
        );
        i += 1;
        pGrabMasks = pGrabMasks.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WmUngrabButton(
    mut display: *mut Display,
    mut button: ::core::ffi::c_uint,
    mut modifiers: ::core::ffi::c_uint,
    mut grab_window: Window,
) {
    let mut pGrabMasks = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut i: ::core::ffi::c_int = 0;
    let mut num_masks: ::core::ffi::c_int = 0;
    pGrabMasks = _WmGrabMasks(modifiers, &raw mut num_masks);
    i = 0 as ::core::ffi::c_int;
    while i < num_masks {
        XUngrabButton(display, button, *pGrabMasks, grab_window);
        i += 1;
        pGrabMasks = pGrabMasks.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetupKeyBindings(
    mut keySpecs: *mut KeySpec,
    mut grabWindow: Window,
    mut keyboardMode: ::core::ffi::c_int,
    mut context: ::core::ffi::c_long,
) -> ::core::ffi::c_int {
    let mut keySpec = ::core::ptr::null_mut::<KeySpec>();
    let mut setCount = 0 as ::core::ffi::c_int;
    let mut iconContext: Boolean = 0;
    iconContext = (context == F_CONTEXT_ICON) as ::core::ffi::c_int as Boolean;
    keySpec = keySpecs;
    while !keySpec.is_null() {
        if F_CONTEXT_ICON as Context
            == (*keySpec).context
                ^ (F_CONTEXT_ICONBOX | F_SUBCONTEXT_IB_IICON | F_SUBCONTEXT_IB_WICON) as Context
            && iconContext as ::core::ffi::c_int != 0
            || F_CONTEXT_ICON as Context
                != (*keySpec).context
                    ^ (F_CONTEXT_ICONBOX | F_SUBCONTEXT_IB_IICON | F_SUBCONTEXT_IB_WICON) as Context
                && iconContext == 0
        {
            WmGrabKey(
                wmGD.display,
                (*keySpec).keycode as ::core::ffi::c_int,
                (*keySpec).state,
                grabWindow,
                False,
                GrabModeAsync,
                keyboardMode,
            );
            setCount += 1;
        }
        keySpec = (*keySpec).nextKeySpec as *mut KeySpec;
    }
    return setCount;
}
#[no_mangle]
pub unsafe extern "C" fn WmDispatchMenuEvent(mut event: *mut XButtonEvent) -> Boolean {
    let mut pCD = wmGD.menuClient;
    let mut doXtDispatchEvent = True as Boolean;
    let mut checkContext: Boolean = 0;
    let mut context = 0 as Context;
    let mut keyEvent = ::core::ptr::null_mut::<XKeyEvent>();
    let mut keySpecs = ::core::ptr::null_mut::<KeySpec>();
    let mut menuBtnPtr = ::core::ptr::null_mut::<MenuButton>();
    if (*event).type_0 == KeyPress {
        if !(*wmGD.menuActive).accelKeySpecs.is_null() {
            checkContext = (pCD.is_null() || (*pCD).systemMenuSpec != wmGD.menuActive)
                as ::core::ffi::c_int as Boolean;
            if checkContext != 0 {
                if !pCD.is_null() {
                    if (*pCD).clientState == MINIMIZED_STATE {
                        context = F_CONTEXT_ICON as Context;
                    } else if (*pCD).clientState == NORMAL_STATE {
                        context = F_CONTEXT_NORMAL as Context;
                    } else {
                        context = F_CONTEXT_MAXIMIZE as Context;
                    }
                } else {
                    context = F_CONTEXT_ROOT as Context;
                }
            }
            keySpecs = (*wmGD.menuActive).accelKeySpecs;
            keyEvent = event as *mut XKeyEvent;
            menuBtnPtr = (*wmGD.menuActive).menuButtons.offset(
                (*wmGD.menuActive)
                    .menuButtonSize
                    .wrapping_sub(1 as ::core::ffi::c_uint) as isize,
            );
            while !keySpecs.is_null() {
                if (*keyEvent).keycode == (*keySpecs).keycode as ::core::ffi::c_uint
                    && ((*keyEvent).state == (*keySpecs).state
                        || (*keyEvent).state
                            & !wmGD.lockingModMask
                            & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                            == (*keySpecs).state)
                    && (checkContext == 0 || context & (*keySpecs).context != 0)
                {
                    doXtDispatchEvent = XtIsSensitive((*menuBtnPtr).buttonWidget);
                    break;
                } else {
                    keySpecs = (*keySpecs).nextKeySpec as *mut KeySpec;
                    menuBtnPtr = menuBtnPtr.offset(-1);
                }
            }
            doXtDispatchEvent = (doXtDispatchEvent as ::core::ffi::c_int != 0
                && HandleKeyPress(
                    event as *mut XKeyEvent,
                    (*wmGD.menuActive).accelKeySpecs,
                    checkContext,
                    context,
                    TRUE as Boolean,
                    NULL as *mut ClientData,
                ) as ::core::ffi::c_int
                    != 0) as ::core::ffi::c_int as Boolean;
        }
        if !wmGD.menuActive.is_null() && !wmGD.menuUnpostKeySpec.is_null() {
            if (*wmGD.menuUnpostKeySpec).keycode as ::core::ffi::c_uint == (*event).button
                && ((*wmGD.menuUnpostKeySpec).state == (*event).state
                    || (*wmGD.menuUnpostKeySpec).state
                        == (*event).state
                            & !wmGD.lockingModMask
                            & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
            {
                UnpostMenu(wmGD.menuActive);
                doXtDispatchEvent = False as Boolean;
            }
        }
    } else if wmGD.checkHotspot as ::core::ffi::c_int != 0
        && ((*event).type_0 == ButtonPress || (*event).type_0 == ButtonRelease)
        && (*event).x_root >= wmGD.hotspotRectangle.x as ::core::ffi::c_int
        && (*event).y_root >= wmGD.hotspotRectangle.y as ::core::ffi::c_int
        && (*event).x_root
            < wmGD.hotspotRectangle.x as ::core::ffi::c_int
                + wmGD.hotspotRectangle.width as ::core::ffi::c_short as ::core::ffi::c_int
        && (*event).y_root
            < wmGD.hotspotRectangle.y as ::core::ffi::c_int
                + wmGD.hotspotRectangle.height as ::core::ffi::c_short as ::core::ffi::c_int
        && (!pCD.is_null()
            || wmGD.rootButtonClick as ::core::ffi::c_int != 0
                && wmGD.clickData.clickPending as ::core::ffi::c_int != 0)
    {
        if (*event).type_0 == ButtonRelease {
            if !pCD.is_null() {
                ProcessClickBRelease(
                    event,
                    pCD,
                    wmGD.clickData.context,
                    wmGD.clickData.subContext,
                );
                if wmGD.clickData.context == F_SUBCONTEXT_W_SYSTEM as Context {
                    PopGadgetOut(pCD, FRAME_SYSTEM);
                }
                (*_XmGetMenuState(
                    (*((*(*pCD).systemMenuSpec).menuWidget as Object))
                        .object
                        .parent,
                ))
                .MS_LastManagedMenuTime = (*event).time;
                doXtDispatchEvent = True as Boolean;
            } else if wmGD.clickData.pCD.is_null()
                && (*event).button == wmGD.clickData.button
                && ((*event).state == wmGD.clickData.releaseState
                    || (*event).state
                        & !wmGD.lockingModMask
                        & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                        == wmGD.clickData.releaseState)
            {
                let mut timeDiff: Time = 0;
                if (*event).time > wmGD.clickData.time {
                    timeDiff = (*event).time.wrapping_sub(wmGD.clickData.time);
                } else {
                    timeDiff = (!wmGD.clickData.time)
                        .wrapping_add((*event).time)
                        .wrapping_add(1 as Time);
                }
                if timeDiff < wmGD.doubleClickTime as Time {
                    (*_XmGetMenuState(
                        (*((*wmGD.menuActive).menuWidget as Object)).object.parent,
                    ))
                    .MS_LastManagedMenuTime = (*event).time;
                    doXtDispatchEvent = True as Boolean;
                }
                wmGD.clickData.clickPending = False as Boolean;
            }
        } else {
            ProcessClickBPress(
                event,
                pCD,
                wmGD.clickData.context,
                wmGD.clickData.subContext,
            );
            if wmGD.clickData.subContext == F_SUBCONTEXT_W_SYSTEM as Context {
                PushGadgetIn(pCD, FRAME_SYSTEM);
            }
            if wmGD.clickData.doubleClickContext == F_SUBCONTEXT_W_SYSTEM as Context {
                if wmGD.systemButtonClick2 as ::core::ffi::c_int != 0
                    && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
                {
                    UnpostMenu((*pCD).systemMenuSpec);
                    F_Kill(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        pCD,
                        event as *mut XEvent,
                    );
                    doXtDispatchEvent = False as Boolean;
                }
            } else if wmGD.clickData.doubleClickContext == F_SUBCONTEXT_I_ALL as Context {
                let mut newState: ::core::ffi::c_int = 0;
                UnpostMenu((*pCD).systemMenuSpec);
                if (*pCD).maxConfig != 0 {
                    newState = MAXIMIZED_STATE;
                } else {
                    newState = NORMAL_STATE;
                }
                SetClientState(pCD, newState, (*event).time);
                wmGD.clickData.clickPending = False as Boolean;
                wmGD.clickData.doubleClickPending = False as Boolean;
                doXtDispatchEvent = False as Boolean;
            } else if wmGD.clickData.doubleClickContext == F_SUBCONTEXT_IB_IICON as Context
                || wmGD.clickData.doubleClickContext == F_SUBCONTEXT_IB_WICON as Context
            {
                UnpostMenu((*pCD).systemMenuSpec);
                F_Restore_And_Raise(NULL as String_0, pCD, NULL as *mut XEvent);
                doXtDispatchEvent = False as Boolean;
            }
        }
    }
    return doXtDispatchEvent;
}
#[no_mangle]
pub unsafe extern "C" fn WmDispatchWsEvent(mut event: *mut XEvent) -> Boolean {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut dispatchEvent = False as Boolean;
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    match (*event).type_0 {
        PropertyNotify => {
            if (*event).xproperty.atom == XA_RESOURCE_MANAGER {
                ReloadTitleGradients(wmGD.pActiveSD);
            }
        }
        KeyPress => {
            dispatchEvent = HandleWsKeyPress(event as *mut XKeyEvent);
        }
        ButtonPress => {
            if !wmGD.menuActive.is_null() {
                dispatchEvent = True as Boolean;
            } else {
                HandleWsButtonPress(event as *mut XButtonEvent);
            }
        }
        ButtonRelease => {
            if !wmGD.menuActive.is_null() {
                dispatchEvent = True as Boolean;
            } else {
                HandleWsButtonRelease(event as *mut XButtonEvent);
            }
        }
        UnmapNotify => {
            if XFindContext(
                wmGD.display,
                (*event).xunmap.window as XID,
                wmGD.windowContextType,
                &raw mut pCD as *mut XPointer,
            ) == 0
                && (*(event as *mut XUnmapEvent)).window == (*pCD).client
            {
                UnManageWindow(pCD);
            }
        }
        EnterNotify => {
            HandleWsEnterNotify(event as *mut XEnterWindowEvent);
        }
        LeaveNotify => {
            HandleWsLeaveNotify(event as *mut XLeaveWindowEvent);
        }
        ConfigureRequest => {
            HandleWsConfigureRequest(event as *mut XConfigureRequestEvent);
        }
        MapRequest => {
            if XFindContext(
                wmGD.display,
                (*event).xmaprequest.window as XID,
                wmGD.windowContextType,
                &raw mut pCD as *mut XPointer,
            ) != 0
                && {
                    pSD = GetScreenForWindow((*event).xmaprequest.window);
                    !pSD.is_null()
                }
            {
                ManageWindow(pSD, (*event).xmaprequest.window, MANAGEW_NORMAL);
            }
        }
        FocusIn => {
            HandleWsFocusIn(event as *mut XFocusInEvent);
        }
        ClientMessage => {
            if HandleEwmhRootClientMessage(wmGD.pActiveSD, &raw mut (*event).xclient) != 0 {
                dispatchEvent = False as Boolean;
            }
        }
        FocusOut | _ => {}
    }
    return dispatchEvent;
}
#[no_mangle]
pub unsafe extern "C" fn HandleWsKeyPress(mut keyEvent: *mut XKeyEvent) -> Boolean {
    let mut dispatchEvent = False as Boolean;
    let mut checkKeyEvent = True as Boolean;
    let mut n: ::core::ffi::c_uint = 0;
    let mut context: Context = 0;
    if !wmGD.menuActive.is_null() {
        dispatchEvent = True as Boolean;
        checkKeyEvent = False as Boolean;
    }
    if wmGD.passKeysActive != 0 {
        if !wmGD.passKeysKeySpec.is_null()
            && ((*wmGD.passKeysKeySpec).state == (*keyEvent).state
                || (*wmGD.passKeysKeySpec).state
                    == (*keyEvent).state
                        & !wmGD.lockingModMask
                        & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint)
            && (*wmGD.passKeysKeySpec).keycode as ::core::ffi::c_uint == (*keyEvent).keycode
        {
            F_Pass_Key(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                NULL as *mut ClientData,
                NULL as *mut XEvent,
            );
            XAllowEvents(wmGD.display, AsyncKeyboard, CurrentTime as Time);
        } else {
            XAllowEvents(wmGD.display, ReplayKeyboard, CurrentTime as Time);
        }
        checkKeyEvent = False as Boolean;
    }
    if checkKeyEvent != 0 {
        if !wmGD.keyboardFocus.is_null() {
            if (*wmGD.keyboardFocus).clientState == MINIMIZED_STATE {
                context = F_CONTEXT_ICON as Context;
            } else if (*wmGD.keyboardFocus).clientState == NORMAL_STATE {
                context = F_CONTEXT_NORMAL as Context;
            } else {
                context = F_CONTEXT_MAXIMIZE as Context;
            }
        } else {
            context = F_CONTEXT_ROOT as Context;
        }
        if HandleKeyPress(
            keyEvent,
            (*wmGD.pActiveSD).keySpecs,
            TRUE as Boolean,
            context,
            FALSE as Boolean,
            NULL as *mut ClientData,
        ) as ::core::ffi::c_int
            != 0
            && (*wmGD.pActiveSD).acceleratorMenuCount != 0
        {
            n = 0 as ::core::ffi::c_uint;
            while (*keyEvent).keycode != 0 as ::core::ffi::c_uint
                && n < (*wmGD.pActiveSD).acceleratorMenuCount
            {
                if HandleKeyPress(
                    keyEvent,
                    (**(*wmGD.pActiveSD).acceleratorMenuSpecs.offset(n as isize)).accelKeySpecs,
                    TRUE as Boolean,
                    context,
                    TRUE as Boolean,
                    NULL as *mut ClientData,
                ) == 0
                {
                    break;
                }
                n = n.wrapping_add(1);
            }
        }
        XAllowEvents(wmGD.display, AsyncKeyboard, CurrentTime as Time);
    }
    return dispatchEvent;
}
#[no_mangle]
pub unsafe extern "C" fn HandleKeyPress(
    mut keyEvent: *mut XKeyEvent,
    mut keySpecs: *mut KeySpec,
    mut checkContext: Boolean,
    mut context: Context,
    mut onlyFirst: Boolean,
    mut pCD: *mut ClientData,
) -> Boolean {
    let mut processKey = True as Boolean;
    let mut functionClient = ::core::ptr::null_mut::<ClientData>();
    let mut haveRootBinding = False as Boolean;
    let mut haveWindowBinding = False as Boolean;
    while processKey as ::core::ffi::c_int != 0 && !keySpecs.is_null() {
        if ((*keyEvent).state == (*keySpecs).state
            || (*keyEvent).state
                & !wmGD.lockingModMask
                & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as ::core::ffi::c_uint
                == (*keySpecs).state)
            && (*keyEvent).keycode == (*keySpecs).keycode as ::core::ffi::c_uint
        {
            if checkContext == 0 || context & (*keySpecs).context != 0 {
                if !pCD.is_null() {
                    functionClient = pCD;
                } else {
                    functionClient = wmGD.keyboardFocus;
                }
                if !wmGD.menuActive.is_null() {
                    functionClient = wmGD.menuClient;
                    UnpostMenu(wmGD.menuActive);
                    processKey = False as Boolean;
                } else if onlyFirst != 0 {
                    processKey = False as Boolean;
                }
                if (*keySpecs).wmFunction.is_some_and(|action| {
                    std::ptr::fn_addr_eq(
                        action,
                        F_Menu
                            as unsafe extern "C" fn(
                                String_0,
                                *mut ClientData,
                                *mut XEvent,
                            ) -> Boolean,
                    )
                }) || (*keySpecs).wmFunction.is_some_and(|action| {
                    std::ptr::fn_addr_eq(
                        action,
                        F_Post_SMenu
                            as unsafe extern "C" fn(
                                String_0,
                                *mut ClientData,
                                *mut XEvent,
                            ) -> Boolean,
                    )
                }) {
                    wmGD.menuUnpostKeySpec = keySpecs;
                } else if (*keySpecs).wmFunction.is_some_and(|action| {
                    std::ptr::fn_addr_eq(
                        action,
                        F_Pass_Key
                            as unsafe extern "C" fn(
                                String_0,
                                *mut ClientData,
                                *mut XEvent,
                            ) -> Boolean,
                    )
                }) {
                    wmGD.passKeysKeySpec = keySpecs;
                }
                if (*keySpecs).wmFunction.expect("non-null function pointer")(
                    (*keySpecs).wmFuncArgs,
                    functionClient as *mut ClientData,
                    keyEvent as *mut XEvent,
                ) == 0
                {
                    processKey = False as Boolean;
                }
                if context & (F_CONTEXT_NORMAL | F_CONTEXT_MAXIMIZE) as Context != 0 {
                    haveWindowBinding = True as Boolean;
                }
            } else if context & (F_CONTEXT_NORMAL | F_CONTEXT_MAXIMIZE) as Context != 0
                && (*keySpecs).context & F_CONTEXT_ROOT as Context != 0
            {
                haveRootBinding = True as Boolean;
            }
        }
        keySpecs = (*keySpecs).nextKeySpec as *mut KeySpec;
    }
    if haveRootBinding as ::core::ffi::c_int != 0 && haveWindowBinding == 0 {
        XAllowEvents(wmGD.display, ReplayKeyboard, CurrentTime as Time);
    }
    return processKey;
}
#[no_mangle]
pub unsafe extern "C" fn HandleWsButtonPress(mut buttonEvent: *mut XButtonEvent) {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut context: Context = 0;
    let mut partContext: ::core::ffi::c_int = 0;
    let mut subContext: Context = 0;
    if (*buttonEvent).subwindow == None_0 as Window
        || XFindContext(
            wmGD.display,
            (*buttonEvent).subwindow as XID,
            wmGD.windowContextType,
            &raw mut pCD as *mut XPointer,
        ) != 0
    {
        pCD = ::core::ptr::null_mut::<ClientData>();
    }
    IdentifyEventContext(buttonEvent, pCD, &raw mut context, &raw mut partContext);
    subContext = ((1 as ::core::ffi::c_long) << partContext) as Context;
    ProcessClickBPress(buttonEvent, pCD, context, subContext);
    if CheckForButtonAction(buttonEvent, context, subContext, pCD) as ::core::ffi::c_int != 0
        && !pCD.is_null()
    {
        CheckButtonPressBuiltin(buttonEvent, context, subContext, partContext, pCD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleWsButtonRelease(mut buttonEvent: *mut XButtonEvent) {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut context: Context = 0;
    let mut partContext: ::core::ffi::c_int = 0;
    let mut subContext: Context = 0;
    if (*buttonEvent).subwindow == None_0 as Window
        || XFindContext(
            wmGD.display,
            (*buttonEvent).subwindow as XID,
            wmGD.windowContextType,
            &raw mut pCD as *mut XPointer,
        ) != 0
    {
        pCD = ::core::ptr::null_mut::<ClientData>();
    }
    IdentifyEventContext(buttonEvent, pCD, &raw mut context, &raw mut partContext);
    subContext = ((1 as ::core::ffi::c_long) << partContext) as Context;
    ProcessClickBRelease(buttonEvent, pCD, context, subContext);
    if CheckForButtonAction(buttonEvent, context, subContext, pCD) as ::core::ffi::c_int != 0
        && !pCD.is_null()
    {
        CheckButtonReleaseBuiltin(buttonEvent, context, subContext, pCD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CheckForButtonAction(
    mut buttonEvent: *mut XButtonEvent,
    mut context: Context,
    mut subContext: Context,
    mut pCD: *mut ClientData,
) -> Boolean {
    let mut buttonSpec = ::core::ptr::null_mut::<ButtonSpec>();
    buttonSpec = (*wmGD.pActiveSD).buttonSpecs;
    while !buttonSpec.is_null() {
        if (*buttonEvent).button == (*buttonSpec).button
            && ((*buttonEvent).state == (*buttonSpec).state
                || (*buttonEvent).state
                    & !wmGD.lockingModMask
                    & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                    == (*buttonSpec).state)
        {
            if (*buttonEvent).type_0 as ::core::ffi::c_uint == (*buttonSpec).eventType
                && context & (*buttonSpec).context != 0
                && subContext & (*buttonSpec).subContext != 0
            {
                if (*buttonEvent).type_0 == ButtonRelease {
                    if (*buttonSpec).subContext | wmGD.clickData.clickContext
                        != (*buttonSpec).subContext
                    {
                        buttonSpec = (*buttonSpec).nextButtonSpec as *mut ButtonSpec;
                        continue;
                    }
                } else if (*buttonSpec).click as ::core::ffi::c_int != 0
                    && (*buttonEvent).type_0 == ButtonPress
                {
                    if (*buttonSpec).subContext | wmGD.clickData.doubleClickContext
                        != (*buttonSpec).subContext
                    {
                        buttonSpec = (*buttonSpec).nextButtonSpec as *mut ButtonSpec;
                        continue;
                    } else {
                        wmGD.clickData.clickPending = False as Boolean;
                        wmGD.clickData.doubleClickPending = False as Boolean;
                    }
                }
                if (*buttonSpec).wmFunction.expect("non-null function pointer")(
                    (*buttonSpec).wmFuncArgs,
                    pCD as *mut ClientData,
                    buttonEvent as *mut XEvent,
                ) == 0
                {
                    return 0 as Boolean;
                }
            }
        }
        buttonSpec = (*buttonSpec).nextButtonSpec as *mut ButtonSpec;
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn IdentifyEventContext(
    mut event: *mut XButtonEvent,
    mut pCD: *mut ClientData,
    mut pContext: *mut Context,
    mut pPartContext: *mut ::core::ffi::c_int,
) {
    let mut eventOnRoot: Boolean = 0;
    let mut actionWindow: Window = 0;
    let mut clientX: ::core::ffi::c_int = 0;
    let mut clientY: ::core::ffi::c_int = 0;
    let mut framePart: ::core::ffi::c_int = 0;
    eventOnRoot = (if (*event).window == (*wmGD.pActiveSD).rootWindow {
        True
    } else {
        False
    }) as Boolean;
    if !pCD.is_null() {
        actionWindow = if eventOnRoot as ::core::ffi::c_int != 0 {
            (*event).subwindow
        } else {
            (*event).window
        };
        if actionWindow == (*pCD).clientFrameWin {
            *pContext = F_CONTEXT_WINDOW as Context;
            if eventOnRoot != 0 {
                clientX = (*event).x
                    - (if (*pCD).maxConfig as ::core::ffi::c_int != 0 {
                        (*pCD).maxX
                    } else {
                        (*pCD).clientX
                    })
                    + (*pCD).clientOffset.x as ::core::ffi::c_int;
                clientY = (*event).y
                    - (if (*pCD).maxConfig as ::core::ffi::c_int != 0 {
                        (*pCD).maxY
                    } else {
                        (*pCD).clientY
                    })
                    + (*pCD).clientOffset.y as ::core::ffi::c_int;
            } else {
                clientX = (*event).x;
                clientY = (*event).y;
            }
            framePart = IdentifyFramePart(pCD, clientX, clientY);
            *pPartContext = framePart;
        } else if actionWindow == (*pCD).clientBaseWin {
            *pContext = F_CONTEXT_WINDOW as Context;
            *pPartContext = FRAME_CLIENT;
        } else if actionWindow == (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin
            || actionWindow == (*wmGD.pActiveSD).activeIconTextWin
        {
            if !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
                .pIconBox
                .is_null()
            {
                *pContext = F_CONTEXT_ICONBOX as Context;
                if (*pCD).clientState == MINIMIZED_STATE {
                    *pPartContext = ICONBOX_PART_IICON;
                } else {
                    *pPartContext = ICONBOX_PART_WICON;
                }
            } else {
                *pContext = F_CONTEXT_ICON as Context;
                *pPartContext = ICON_PART_ALL;
            }
        } else {
            *pContext = F_CONTEXT_ROOT as Context;
            *pPartContext = ROOT_PART_ALL;
        }
        if *pContext & F_CONTEXT_WINDOW as Context != 0
            && (*pCD).clientState != NORMAL_STATE
            && (*pCD).clientState != MAXIMIZED_STATE
            || *pContext & F_CONTEXT_ICON as Context != 0 && (*pCD).clientState != MINIMIZED_STATE
        {
            *pContext = F_CONTEXT_NONE as Context;
        }
    } else {
        *pContext = F_CONTEXT_ROOT as Context;
        *pPartContext = ROOT_PART_ALL;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ProcessClickBPress(
    mut buttonEvent: *mut XButtonEvent,
    mut pCD: *mut ClientData,
    mut context: Context,
    mut subContext: Context,
) {
    let mut timeDiff: Time = 0;
    let mut passButton: Boolean = 0;
    wmGD.clickData.doubleClickContext = F_SUBCONTEXT_NONE as Context;
    if wmGD.clickData.doubleClickPending as ::core::ffi::c_int != 0
        && (*buttonEvent).button == wmGD.clickData.button
        && ((*buttonEvent).state == wmGD.clickData.state
            || (*buttonEvent).state
                & !wmGD.lockingModMask
                & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as ::core::ffi::c_uint
                == wmGD.clickData.state)
        && pCD == wmGD.clickData.pCD
        && context == wmGD.clickData.context
    {
        if (*buttonEvent).time > wmGD.clickData.time {
            timeDiff = (*buttonEvent).time.wrapping_sub(wmGD.clickData.time);
        } else {
            timeDiff = (!wmGD.clickData.time)
                .wrapping_add((*buttonEvent).time)
                .wrapping_add(1 as Time);
        }
        if timeDiff < wmGD.doubleClickTime as Time {
            wmGD.clickData.doubleClickContext = subContext | wmGD.clickData.subContext;
        }
    }
    if (*buttonEvent).button == SELECT_BUTTON as ::core::ffi::c_uint
        && ((*buttonEvent).state == 0 as ::core::ffi::c_uint
            || (*buttonEvent).state
                & !wmGD.lockingModMask
                & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint)
    {
        passButton = wmGD.passSelectButton;
    } else {
        passButton = wmGD.passButtons;
    }
    if !(!pCD.is_null()
        && (*buttonEvent).window == (*pCD).clientBaseWin
        && passButton as ::core::ffi::c_int != 0)
    {
        wmGD.clickData.button = (*buttonEvent).button;
        wmGD.clickData.state = (*buttonEvent).state;
        wmGD.clickData.releaseState = (*buttonEvent).state
            | *(&raw mut buttonModifierMasks as *mut ::core::ffi::c_uint)
                .offset((*buttonEvent).button as isize);
        wmGD.clickData.pCD = pCD as *mut _ClientData;
        wmGD.clickData.context = context;
        wmGD.clickData.subContext = subContext;
        wmGD.clickData.time = (*buttonEvent).time;
        wmGD.clickData.clickPending = True as Boolean;
        wmGD.clickData.doubleClickPending = True as Boolean;
        wmGD.clickData.bReplayed = wmGD.bReplayedButton;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessClickBRelease(
    mut buttonEvent: *mut XButtonEvent,
    mut pCD: *mut ClientData,
    mut context: Context,
    mut subContext: Context,
) {
    if !pCD.is_null() && wmGD.gadgetClient == pCD && (*pCD).decorFlags != 0 {
        PopGadgetOut(pCD, wmGD.gadgetDepressed);
    }
    wmGD.clickData.clickContext = F_SUBCONTEXT_NONE as Context;
    if wmGD.clickData.clickPending as ::core::ffi::c_int != 0
        && (*buttonEvent).button == wmGD.clickData.button
        && (*buttonEvent).state == wmGD.clickData.releaseState
        && pCD == wmGD.clickData.pCD
        && context == wmGD.clickData.context
    {
        wmGD.clickData.clickContext = subContext | wmGD.clickData.subContext;
    } else {
        wmGD.clickData.doubleClickPending = False as Boolean;
    }
    wmGD.clickData.clickPending = False as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn HandleWmClientMessage(mut clientEvent: *mut XClientMessageEvent) {
    let mut scr: ::core::ffi::c_int = 0;
    if (*clientEvent).message_type == wmGD.xa_WM_PROTOCOLS {
        if (*clientEvent).data.l[0 as ::core::ffi::c_int as usize] as Atom
            == wmGD.xa_WM_SAVE_YOURSELF
        {
            scr = 0 as ::core::ffi::c_int;
            while scr < wmGD.numScreens {
                (*wmGD.Screens.offset(scr as isize)).managed != 0;
                scr += 1;
            }
            XSetCommand(
                wmGD.display,
                wmGD.commandWindow,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleMwmRequest(mut pSD: *mut WmScreenData, mut pev: *mut XEvent) {
    let mut more = True as Boolean;
    let mut pchReq = ::core::ptr::null_mut::<::core::ffi::c_char>();
    while more != 0 {
        GetWmRequest(pSD, &raw mut pchReq, &raw mut more);
        if !pchReq.is_null() {
            ProcessWmRequest(pSD, pchReq);
            XtFree(pchReq);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleWsEnterNotify(mut enterEvent: *mut XEnterWindowEvent) {
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    if wmGD.queryScreen as ::core::ffi::c_int != 0
        && XFindContext(
            wmGD.display,
            (*enterEvent).window as XID,
            wmGD.screenContextType,
            &raw mut pSD as *mut XPointer,
        ) == 0
    {
        wmGD.pActiveSD = pSD;
        wmGD.queryScreen = False as Boolean;
    }
    if wmGD.menuActive.is_null()
        && ((*enterEvent).mode == NotifyNormal
            || (*enterEvent).mode == NotifyUngrab
            || (*enterEvent).mode == NotifyWhileGrabbed)
    {
        if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER {
            Do_Focus_Key(
                NULL as *mut ClientData,
                (*enterEvent).time,
                ALWAYS_SET_FOCUS,
            );
        } else if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT
            && ((*enterEvent).detail == NotifyNonlinearVirtual
                || (*enterEvent).detail == NotifyNonlinear)
            && wmGD.keyboardFocus.is_null()
            && (*enterEvent).focus != 0
        {
            Do_Focus_Key(
                NULL as *mut ClientData,
                (*enterEvent).time,
                ALWAYS_SET_FOCUS,
            );
        }
        if wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER {
            SetColormapFocus(wmGD.pActiveSD, NULL as *mut ClientData);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleWsLeaveNotify(mut leaveEvent: *mut XLeaveWindowEvent) {
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    if wmGD.menuActive.is_null()
        && ((*leaveEvent).detail == NotifyNonlinear
            || (*leaveEvent).detail == NotifyNonlinearVirtual)
    {
        if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER
            || wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER
        {
            if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER {
                Do_Focus_Key(
                    NULL as *mut ClientData,
                    (*leaveEvent).time,
                    SCREEN_SWITCH_FOCUS | ALWAYS_SET_FOCUS,
                );
            }
            if wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER {
                SetColormapFocus(wmGD.pActiveSD, NULL as *mut ClientData);
            }
        }
        if XFindContext(
            wmGD.display,
            (*leaveEvent).root as XID,
            wmGD.screenContextType,
            &raw mut pSD as *mut XPointer,
        ) == 0
        {
            wmGD.pActiveSD = pSD;
            wmGD.queryScreen = False as Boolean;
        } else {
            wmGD.queryScreen = True as Boolean;
            XSetInputFocus(
                wmGD.display,
                PointerRoot as Window,
                PointerRoot as ::core::ffi::c_int,
                (*leaveEvent).time,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleWsConfigureRequest(mut configureEvent: *mut XConfigureRequestEvent) {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
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
    let mut configChanged: Boolean = 0;
    let mut values = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    if XFindContext(
        wmGD.display,
        (*configureEvent).window as XID,
        wmGD.windowContextType,
        &raw mut pCD as *mut XPointer,
    ) != 0
    {
        if WmGetWindowAttributes((*configureEvent).window) != 0 {
            configChanged = (wmGD.windowAttributes.x != (*configureEvent).x
                || wmGD.windowAttributes.y != (*configureEvent).y
                || wmGD.windowAttributes.width != (*configureEvent).width
                || wmGD.windowAttributes.height != (*configureEvent).height
                || wmGD.windowAttributes.border_width != (*configureEvent).border_width
                || (*configureEvent).value_mask & (CWSibling | CWStackMode) as ::core::ffi::c_ulong
                    != 0) as ::core::ffi::c_int as Boolean;
            if configChanged != 0 {
                values.x = (*configureEvent).x;
                values.y = (*configureEvent).y;
                values.width = (*configureEvent).width;
                values.height = (*configureEvent).height;
                values.border_width = (*configureEvent).border_width;
                values.sibling = (*configureEvent).above;
                values.stack_mode = (*configureEvent).detail;
                XConfigureWindow(
                    wmGD.display,
                    (*configureEvent).window,
                    (*configureEvent).value_mask as ::core::ffi::c_uint,
                    &raw mut values,
                );
            }
            if configChanged == 0 {
                notifyEvent.type_0 = ConfigureNotify;
                notifyEvent.display = wmGD.display;
                notifyEvent.event = (*configureEvent).window;
                notifyEvent.window = (*configureEvent).window;
                notifyEvent.x = (*configureEvent).x;
                notifyEvent.y = (*configureEvent).y;
                notifyEvent.width = (*configureEvent).width;
                notifyEvent.height = (*configureEvent).height;
                notifyEvent.border_width = (*configureEvent).border_width;
                notifyEvent.above = None_0 as Window;
                notifyEvent.override_redirect = False;
                XSendEvent(
                    wmGD.display,
                    (*configureEvent).window,
                    False,
                    StructureNotifyMask,
                    &raw mut notifyEvent as *mut XEvent,
                );
            }
        }
    } else {
        HandleCConfigureRequest(pCD, configureEvent);
    };
}
#[no_mangle]
pub unsafe extern "C" fn HandleWsFocusIn(mut focusEvent: *mut XFocusInEvent) {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut sameScreen: Boolean = 0;
    if ((*focusEvent).mode == NotifyNormal || (*focusEvent).mode == NotifyUngrab)
        && ((*focusEvent).detail == NotifyPointerRoot
            || (*focusEvent).detail == NotifyDetailNone
            || (*focusEvent).detail == NotifyInferior)
    {
        pCD = GetClientUnderPointer(&raw mut sameScreen);
        if !wmGD.keyboardFocus.is_null() && (*focusEvent).detail != NotifyInferior {
            if sameScreen != 0 {
                if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
                    Do_Focus_Key(wmGD.keyboardFocus, GetTimestamp(), ALWAYS_SET_FOCUS);
                } else if !pCD.is_null() || (*focusEvent).detail == NotifyDetailNone {
                    Do_Focus_Key(pCD, GetTimestamp(), ALWAYS_SET_FOCUS);
                }
                SetKeyboardFocus(NULL as *mut ClientData, REFRESH_LAST_FOCUS);
            } else {
                SetKeyboardFocus(NULL as *mut ClientData, REFRESH_LAST_FOCUS);
            }
        } else if sameScreen != 0 {
            if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
                if ((*focusEvent).detail == NotifyInferior
                    || (*focusEvent).detail == NotifyPointerRoot)
                    && wmGD.keyboardFocus != wmGD.nextKeyboardFocus
                {
                    Do_Focus_Key(wmGD.nextKeyboardFocus, GetTimestamp(), ALWAYS_SET_FOCUS);
                } else {
                    Do_Focus_Key(pCD, GetTimestamp(), ALWAYS_SET_FOCUS);
                }
            } else if !pCD.is_null() || (*focusEvent).detail != NotifyPointerRoot {
                Do_Focus_Key(pCD, GetTimestamp(), ALWAYS_SET_FOCUS);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetTimestamp() -> Time {
    let mut timestamp: Time = 0;
    let mut pSD = wmGD.pActiveSD;
    let mut event = _XEvent { type_0: 0 };
    let mut property = 0 as ::core::ffi::c_long;
    XChangeProperty(
        wmGD.display,
        (*pSD).wmWorkspaceWin,
        wmGD.xa_WM_STATE,
        wmGD.xa_WM_STATE,
        32 as ::core::ffi::c_int,
        PropModeAppend,
        &raw mut property as *mut ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int,
    );
    XSync(wmGD.display, False);
    if XCheckWindowEvent(
        wmGD.display,
        (*pSD).wmWorkspaceWin,
        PropertyChangeMask,
        &raw mut event,
    ) != 0
    {
        if event.type_0 == PropertyNotify {
            timestamp = event.xproperty.time;
        } else {
            timestamp = CurrentTime as Time;
        }
        if event.type_0 != PropertyNotify || event.xproperty.atom != wmGD.xa_WM_STATE {
            XPutBackEvent(wmGD.display, &raw mut event);
        }
    } else {
        timestamp = CurrentTime as Time;
    }
    return timestamp;
}
#[no_mangle]
pub unsafe extern "C" fn PullExposureEvents() {
    let mut event = _XEvent { type_0: 0 };
    let mut dispatchEvent: Boolean = 0;
    XSync(wmGD.display, False);
    XSync(wmGD.display1, False);
    while XCheckMaskEvent(
        wmGD.display,
        ExposureMask | VisibilityChangeMask,
        &raw mut event,
    ) != 0
        || XCheckMaskEvent(
            wmGD.display1,
            ExposureMask | VisibilityChangeMask,
            &raw mut event,
        ) != 0
    {
        match event.type_0 {
            Expose => {
                if event.xany.window == (*wmGD.pActiveSD).rootWindow {
                    dispatchEvent = WmDispatchWsEvent(&raw mut event);
                } else {
                    dispatchEvent = WmDispatchClientEvent(&raw mut event);
                }
            }
            _ => {}
        }
        dispatchEvent = True as Boolean;
        if dispatchEvent != 0 {
            XtDispatchEvent(&raw mut event);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReplayedButtonEvent(
    mut pevB1: *mut XButtonEvent,
    mut pevB2: *mut XButtonEvent,
) -> Boolean {
    let mut rval = False as Boolean;
    if (*pevB1).type_0 == (*pevB2).type_0
        && (*pevB1).send_event == (*pevB2).send_event
        && (*pevB1).display == (*pevB2).display
        && (*pevB1).window == (*pevB2).window
        && (*pevB1).root == (*pevB2).root
        && (*pevB1).subwindow == (*pevB2).subwindow
        && (*pevB1).time == (*pevB2).time
        && (*pevB1).x == (*pevB2).x
        && (*pevB1).y == (*pevB2).y
        && (*pevB1).x_root == (*pevB2).x_root
        && (*pevB1).y_root == (*pevB2).y_root
        && (*pevB1).state == (*pevB2).state
        && (*pevB1).button == (*pevB2).button
        && (*pevB1).same_screen == (*pevB2).same_screen
    {
        rval = True as Boolean;
    }
    return rval;
}
