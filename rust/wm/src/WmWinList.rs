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
    fn XCreatePixmapCursor(
        _: *mut Display,
        _: Pixmap,
        _: Pixmap,
        _: *mut XColor,
        _: *mut XColor,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> Cursor;
    fn XCreateBitmapFromData(
        _: *mut Display,
        _: Drawable,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> Pixmap;
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
    fn XConfigureWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_uint,
        _: *mut XWindowChanges,
    ) -> ::core::ffi::c_int;
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
    fn XGrabKeyboard(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: Time,
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
    fn XMapRaised(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XQueryColors(
        _: *mut Display,
        _: Colormap,
        _: *mut XColor,
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
    fn XUngrabKeyboard(_: *mut Display, _: Time) -> ::core::ffi::c_int;
    fn XUngrabPointer(_: *mut Display, _: Time) -> ::core::ffi::c_int;
    fn XUnmapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut wmGD: WmGlobalData;
    fn SetupCButtonBindings(window: Window, buttonSpecs: *mut ButtonSpec);
    fn F_Raise(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn AutoResetKeyFocus(pcdNoFocus: *mut ClientData, focusTime: Time);
    fn DoExplicitSelectGrab(window: Window);
    fn SetKeyboardFocus(pCD: *mut ClientData, focusFlags: ::core::ffi::c_long);
    fn UnpostMenu(menuSpec: *mut MenuSpec);
    fn WmMalloc(
        ptr: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_char;
    fn GetWsClientData(pWS: *mut WmWorkspaceData, pCD: *mut ClientData) -> *mut WsClientData;
    fn GetTimestamp() -> Time;
    fn UpdateEwmhClientList(_: *mut WmScreenData);
    fn RestackDesktopSurfaces(screen: *mut WmScreenData);
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CopyFromParent: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const PointerRoot: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
pub const AnyButton: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CurrentTime: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ButtonPressMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const EnterWindowMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const ButtonMotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 13 as ::core::ffi::c_int;
pub const AnyModifier: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int;
pub const GrabModeAsync: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GrabFrozen: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const InputOnly: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CWOverrideRedirect: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const CWEventMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CWCursor: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 14 as ::core::ffi::c_int;
pub const CWSibling: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const CWStackMode: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const Above: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Below: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MWM_INPUT_PRIMARY_APPLICATION_MODAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_INPUT_SYSTEM_MODAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MWM_INPUT_FULL_APPLICATION_MODAL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const REFRESH_LAST_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const F_GROUP_WINDOW: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const F_GROUP_ICON: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const UNSEEN_STATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICON_ACTIVE_LABEL_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const noenter32_width: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const noenter32_height: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const noenter32_x_hot: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const noenter32_y_hot: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
static mut noenter32_bits: [::core::ffi::c_uchar; 128] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut noenter32m_bits: [::core::ffi::c_uchar; 128] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
pub const noenter16_width: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const noenter16_height: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const noenter16_x_hot: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const noenter16_y_hot: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
static mut noenter16_bits: [::core::ffi::c_uchar; 32] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut noenter16m_bits: [::core::ffi::c_uchar; 32] = [
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn AddClientToList(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
    mut onTop: Boolean,
) {
    let mut belowSystemModal = False as Boolean;
    let mut windowChanges = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    let mut pSD = (*pWS).pSD;
    let mut pWsc = GetWsClientData(pWS, pCD);
    if (*pCD).desktopSurface != 0 {
        onTop = False as Boolean;
    }
    if (*pCD).inputMode == MWM_INPUT_SYSTEM_MODAL {
        SetupSystemModalState(pCD);
        if wmGD.systemModalActive == 0 || wmGD.systemModalClient != pCD {
            (*pCD).inputMode = MWM_INPUT_FULL_APPLICATION_MODAL;
        }
    } else if wmGD.systemModalActive as ::core::ffi::c_int != 0
        && (*FindTransientTreeLeader(pCD)).inputMode != MWM_INPUT_SYSTEM_MODAL
    {
        windowChanges.sibling = (*pSD).inputScreenWindow;
        windowChanges.stack_mode = Below;
        XConfigureWindow(
            wmGD.display,
            (*pCD).clientFrameWin,
            (CWSibling | CWStackMode) as ::core::ffi::c_uint,
            &raw mut windowChanges,
        );
        belowSystemModal = True as Boolean;
    }
    if !(*pCD).transientLeader.is_null() {
        AddTransient(pWS, pCD);
    } else {
        (*pCD).clientEntry.type_0 = NORMAL_STATE;
        (*pCD).clientEntry.pCD = pCD as *mut _ClientData;
        if belowSystemModal as ::core::ffi::c_int != 0 && !wmGD.systemModalClient.is_null() {
            AddEntryToList(
                pWS,
                &raw mut (*pCD).clientEntry,
                False as Boolean,
                (*pSD).clientList,
            );
        } else if onTop != 0 {
            AddEntryToList(
                pWS,
                &raw mut (*pCD).clientEntry,
                True as Boolean,
                ::core::ptr::null_mut::<ClientListEntry>(),
            );
        } else {
            AddEntryToList(
                pWS,
                &raw mut (*pCD).clientEntry,
                False as Boolean,
                ::core::ptr::null_mut::<ClientListEntry>(),
            );
        }
        if (*pWsc).pIconBox.is_null() && (*pWsc).iconFrameWin != 0 {
            if (*(*pSD).lastClient).type_0 == MINIMIZED_STATE {
                let mut pWsib = ::core::ptr::null_mut::<WsClientData>();
                pWsib = (*(*(*pSD).lastClient).pCD)
                    .pWsList
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut _WsClientData as *mut WsClientData;
                windowChanges.sibling = (*pWsib).iconFrameWin;
            } else {
                windowChanges.sibling = (*(*(*pSD).lastClient).pCD).clientFrameWin;
            }
            windowChanges.stack_mode = Below;
            XConfigureWindow(
                wmGD.display,
                (*pWsc).iconFrameWin,
                (CWSibling | CWStackMode) as ::core::ffi::c_uint,
                &raw mut windowChanges,
            );
            (*pCD).iconEntry.type_0 = MINIMIZED_STATE;
            (*pCD).iconEntry.pCD = pCD as *mut _ClientData;
            (*pCD).iconEntry.nextSibling = ::core::ptr::null_mut::<_ClientListEntry>();
            (*pCD).iconEntry.prevSibling = (*pSD).lastClient as *mut _ClientListEntry;
            (*(*pSD).lastClient).nextSibling = &raw mut (*pCD).iconEntry as *mut _ClientListEntry;
            (*pSD).lastClient = &raw mut (*pCD).iconEntry;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AddEntryToList(
    mut pWS: *mut WmWorkspaceData,
    mut pEntry: *mut ClientListEntry,
    mut onTop: Boolean,
    mut pStackEntry: *mut ClientListEntry,
) {
    let mut pSD = (*pWS).pSD;
    if onTop != 0 {
        if !pStackEntry.is_null() {
            if pEntry != pStackEntry {
                (*pEntry).nextSibling = pStackEntry as *mut _ClientListEntry;
                (*pEntry).prevSibling = (*pStackEntry).prevSibling;
                (*pStackEntry).prevSibling = pEntry as *mut _ClientListEntry;
                if !(*pEntry).prevSibling.is_null() {
                    (*(*pEntry).prevSibling).nextSibling = pEntry as *mut _ClientListEntry;
                } else {
                    (*pSD).clientList = pEntry;
                }
            }
        } else if (*pSD).clientList != pEntry {
            (*pEntry).nextSibling = (*pSD).clientList as *mut _ClientListEntry;
            (*pEntry).prevSibling = ::core::ptr::null_mut::<_ClientListEntry>();
            if !(*pSD).clientList.is_null() {
                (*(*pSD).clientList).prevSibling = pEntry as *mut _ClientListEntry;
            } else {
                (*pSD).lastClient = pEntry;
            }
            (*pSD).clientList = pEntry;
        }
    } else if !pStackEntry.is_null() {
        if pEntry != pStackEntry {
            (*pEntry).nextSibling = (*pStackEntry).nextSibling;
            (*pEntry).prevSibling = pStackEntry as *mut _ClientListEntry;
            (*pStackEntry).nextSibling = pEntry as *mut _ClientListEntry;
            if !(*pEntry).nextSibling.is_null() {
                (*(*pEntry).nextSibling).prevSibling = pEntry as *mut _ClientListEntry;
            } else {
                (*pSD).lastClient = pEntry;
            }
        }
    } else if (*pSD).lastClient != pEntry {
        (*pEntry).nextSibling = ::core::ptr::null_mut::<_ClientListEntry>();
        (*pEntry).prevSibling = (*pSD).lastClient as *mut _ClientListEntry;
        if !(*pSD).clientList.is_null() {
            (*(*pSD).lastClient).nextSibling = pEntry as *mut _ClientListEntry;
        } else {
            (*pSD).clientList = pEntry;
        }
        (*pSD).lastClient = pEntry;
    }
    UpdateEwmhClientList(pSD);
}
#[no_mangle]
pub unsafe extern "C" fn MoveEntryInList(
    mut pWS: *mut WmWorkspaceData,
    mut pEntry: *mut ClientListEntry,
    mut onTop: Boolean,
    mut pStackEntry: *mut ClientListEntry,
) {
    DeleteEntryFromList(pWS, pEntry);
    AddEntryToList(pWS, pEntry, onTop, pStackEntry);
}
#[no_mangle]
pub unsafe extern "C" fn DeleteEntryFromList(
    mut pWS: *mut WmWorkspaceData,
    mut pListEntry: *mut ClientListEntry,
) {
    if !(*pListEntry).prevSibling.is_null() {
        (*(*pListEntry).prevSibling).nextSibling = (*pListEntry).nextSibling;
    } else {
        (*(*pWS).pSD).clientList = (*pListEntry).nextSibling as *mut ClientListEntry;
    }
    if !(*pListEntry).nextSibling.is_null() {
        (*(*pListEntry).nextSibling).prevSibling = (*pListEntry).prevSibling;
    } else {
        (*(*pWS).pSD).lastClient = (*pListEntry).prevSibling as *mut ClientListEntry;
    };
}
#[no_mangle]
pub unsafe extern "C" fn DeleteClientFromList(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
) {
    let mut pWsc = GetWsClientData(pWS, pCD);
    let mut pSD = (*pWS).pSD;
    if !(*pCD).transientLeader.is_null() {
        DeleteTransient(pCD);
    } else {
        if (*pCD).inputMode == MWM_INPUT_SYSTEM_MODAL {
            UndoSystemModalState();
        }
        if (*pWsc).pIconBox.is_null() && (*pWsc).iconFrameWin != 0 {
            if !(*pCD).iconEntry.prevSibling.is_null() {
                (*(*pCD).iconEntry.prevSibling).nextSibling = (*pCD).iconEntry.nextSibling;
            } else {
                (*pSD).clientList = (*pCD).iconEntry.nextSibling as *mut ClientListEntry;
            }
            if !(*pCD).iconEntry.nextSibling.is_null() {
                (*(*pCD).iconEntry.nextSibling).prevSibling = (*pCD).iconEntry.prevSibling;
            } else {
                (*pSD).lastClient = (*pCD).iconEntry.prevSibling as *mut ClientListEntry;
            }
        }
        if !(*pCD).clientEntry.prevSibling.is_null() {
            (*(*pCD).clientEntry.prevSibling).nextSibling = (*pCD).clientEntry.nextSibling;
        } else {
            (*pSD).clientList = (*pCD).clientEntry.nextSibling as *mut ClientListEntry;
        }
        if !(*pCD).clientEntry.nextSibling.is_null() {
            (*(*pCD).clientEntry.nextSibling).prevSibling = (*pCD).clientEntry.prevSibling;
        } else {
            (*pSD).lastClient = (*pCD).clientEntry.prevSibling as *mut ClientListEntry;
        }
    }
    UpdateEwmhClientList(pSD);
    if (*pCD).desktopSurface != 0 {
        RestackDesktopSurfaces(pSD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn AddTransient(mut pWS: *mut WmWorkspaceData, mut pCD: *mut ClientData) {
    let mut pcdLeader = (*pCD).transientLeader as *mut ClientData;
    let mut pcdTop = FindTransientTreeLeader(pCD);
    let mut restackTransients: Boolean = 0;
    let mut pSD = (*pWS).pSD;
    (*pCD).transientSiblings = (*pcdLeader).transientChildren;
    (*pcdLeader).transientChildren = pCD as *mut _ClientData;
    restackTransients = PutTransientOnTop(pCD);
    if (*pCD).inputMode == MWM_INPUT_PRIMARY_APPLICATION_MODAL {
        while !pcdLeader.is_null() {
            MarkModalTransient(pcdLeader, pCD);
            pcdLeader = (*pcdLeader).transientLeader as *mut ClientData;
        }
    } else if (*pCD).inputMode == MWM_INPUT_FULL_APPLICATION_MODAL {
        MarkModalSubtree(pcdTop, pCD);
    } else if (*pcdTop).fullModalCount != 0 {
        pcdLeader = (*pCD).transientLeader as *mut ClientData;
        if (*pcdLeader).inputMode != MWM_INPUT_FULL_APPLICATION_MODAL
            || ((*pcdLeader).primaryModalCount != 0 || (*pcdLeader).fullModalCount != 0)
        {
            (*pCD).fullModalCount = (*pcdLeader).fullModalCount;
        }
    }
    if (*pSD).clientList != &raw mut (*pcdTop).clientEntry && wmGD.systemModalActive == 0 {
        F_Raise(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pCD,
            ::core::ptr::null_mut::<XEvent>(),
        );
    } else if restackTransients != 0 {
        RestackTransientsAtWindow(pCD);
    } else if pCD != FindTransientOnTop(pcdTop) {
        StackTransientWindow(pCD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn MarkModalSubtree(
    mut pcdTree: *mut ClientData,
    mut pcdAvoid: *mut ClientData,
) {
    if !(*pcdTree).transientChildren.is_null() {
        MarkModalSubtree((*pcdTree).transientChildren as *mut ClientData, pcdAvoid);
    }
    if pcdTree != pcdAvoid {
        MarkModalTransient(pcdTree, pcdAvoid);
    }
    if !(*pcdTree).transientSiblings.is_null() {
        MarkModalSubtree((*pcdTree).transientSiblings as *mut ClientData, pcdAvoid);
    }
}
#[no_mangle]
pub unsafe extern "C" fn MarkModalTransient(
    mut pcdLeader: *mut ClientData,
    mut pCD: *mut ClientData,
) {
    if !((*pcdLeader).primaryModalCount != 0 || (*pcdLeader).fullModalCount != 0) {
        XGrabButton(
            wmGD.display,
            AnyButton as ::core::ffi::c_uint,
            AnyModifier as ::core::ffi::c_uint,
            (*pcdLeader).clientBaseWin,
            True,
            (ButtonPressMask | ButtonMotionMask) as ::core::ffi::c_uint,
            GrabModeAsync,
            GrabModeAsync,
            None_0 as Window,
            wmGD.workspaceCursor,
        );
    }
    if (*pCD).inputMode == MWM_INPUT_FULL_APPLICATION_MODAL {
        (*pcdLeader).fullModalCount += 1;
    } else {
        (*pcdLeader).primaryModalCount += 1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn DeleteTransient(mut pCD: *mut ClientData) {
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut pcdPrev = ::core::ptr::null_mut::<ClientData>();
    let mut modalCount: ::core::ffi::c_int = 0;
    modalCount = (*pCD).primaryModalCount;
    if (*pCD).inputMode == MWM_INPUT_PRIMARY_APPLICATION_MODAL {
        modalCount += 1 as ::core::ffi::c_int;
    }
    if modalCount != 0 {
        pcdLeader = (*pCD).transientLeader as *mut ClientData;
        while !pcdLeader.is_null() {
            if modalCount != 0 {
                UnMarkModalTransient(pcdLeader, modalCount, pCD);
            }
            pcdLeader = (*pcdLeader).transientLeader as *mut ClientData;
        }
    }
    pcdLeader = FindTransientTreeLeader(pCD);
    if !(*pCD).transientChildren.is_null() {
        DeleteFullAppModalChildren(pcdLeader, (*pCD).transientChildren as *mut ClientData);
    }
    if (*pCD).inputMode == MWM_INPUT_FULL_APPLICATION_MODAL {
        FixupFullAppModalCounts(pcdLeader, pCD);
    }
    pcdPrev = (*(*pCD).transientLeader).transientChildren as *mut ClientData;
    if !pcdPrev.is_null() {
        if pcdPrev == pCD {
            (*(*pCD).transientLeader).transientChildren = (*pCD).transientSiblings;
        } else {
            while !pcdPrev.is_null() && (*pcdPrev).transientSiblings != pCD {
                pcdPrev = (*pcdPrev).transientSiblings as *mut ClientData;
            }
            if !pcdPrev.is_null() {
                (*pcdPrev).transientSiblings = (*pCD).transientSiblings;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn DeleteFullAppModalChildren(
    mut pcdLeader: *mut ClientData,
    mut pCD: *mut ClientData,
) {
    if !(*pCD).transientChildren.is_null() {
        DeleteFullAppModalChildren(pcdLeader, (*pCD).transientChildren as *mut ClientData);
    }
    FixupFullAppModalCounts(pcdLeader, pCD);
    if !(*pCD).transientSiblings.is_null() {
        DeleteFullAppModalChildren(pcdLeader, (*pCD).transientSiblings as *mut ClientData);
    }
}
#[no_mangle]
pub unsafe extern "C" fn FixupFullAppModalCounts(
    mut pcdLeader: *mut ClientData,
    mut pcdDelete: *mut ClientData,
) {
    if !(*pcdLeader).transientChildren.is_null() {
        FixupFullAppModalCounts((*pcdLeader).transientChildren as *mut ClientData, pcdDelete);
    }
    if (*pcdLeader).fullModalCount > (*pcdDelete).fullModalCount {
        UnMarkModalTransient(pcdLeader, 1 as ::core::ffi::c_int, pcdDelete);
    }
    if !(*pcdLeader).transientSiblings.is_null() {
        FixupFullAppModalCounts((*pcdLeader).transientSiblings as *mut ClientData, pcdDelete);
    }
}
#[no_mangle]
pub unsafe extern "C" fn UnMarkModalTransient(
    mut pcdModee: *mut ClientData,
    mut modalCount: ::core::ffi::c_int,
    mut pcdModal: *mut ClientData,
) {
    if (*pcdModal).inputMode == MWM_INPUT_FULL_APPLICATION_MODAL {
        (*pcdModee).fullModalCount -= modalCount;
    } else if (*pcdModal).inputMode == MWM_INPUT_PRIMARY_APPLICATION_MODAL {
        (*pcdModee).primaryModalCount -= modalCount;
    }
    if !((*pcdModee).primaryModalCount != 0 || (*pcdModee).fullModalCount != 0) {
        XUngrabButton(
            wmGD.display,
            AnyButton as ::core::ffi::c_uint,
            AnyModifier as ::core::ffi::c_uint,
            (*pcdModee).clientBaseWin,
        );
        SetupCButtonBindings((*pcdModee).clientBaseWin, (*(*pcdModee).pSD).buttonSpecs);
        if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT && wmGD.keyboardFocus != pcdModee {
            DoExplicitSelectGrab((*pcdModee).clientBaseWin);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn PutTransientOnTop(mut pcd: *mut ClientData) -> Boolean {
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut pcdPrev = ::core::ptr::null_mut::<ClientData>();
    let mut restack = False as Boolean;
    pcdLeader = (*pcd).transientLeader as *mut ClientData;
    if !pcdLeader.is_null() {
        pcdPrev = (*pcdLeader).transientChildren as *mut ClientData;
        if pcdPrev != pcd {
            while (*pcdPrev).transientSiblings != pcd {
                pcdPrev = (*pcdPrev).transientSiblings as *mut ClientData;
            }
            (*pcdPrev).transientSiblings = (*pcd).transientSiblings;
            (*pcd).transientSiblings = (*pcdLeader).transientChildren;
            (*pcdLeader).transientChildren = pcd as *mut _ClientData;
            restack = True as Boolean;
        }
        if PutTransientOnTop(pcdLeader) != 0 {
            restack = True as Boolean;
        }
        if BumpPrimaryToBottom(pcdLeader) != 0 {
            restack = True as Boolean;
        }
    }
    return restack;
}
#[no_mangle]
pub unsafe extern "C" fn PutTransientBelowSiblings(mut pcd: *mut ClientData) -> Boolean {
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    let mut restack = False as Boolean;
    pcdLeader = (*pcd).transientLeader as *mut ClientData;
    if !pcdLeader.is_null() {
        if !(*pcd).transientSiblings.is_null() || (*pcdLeader).transientChildren != pcd {
            restack = True as Boolean;
            if (*pcdLeader).transientChildren == pcd {
                (*pcdLeader).transientChildren = (*pcd).transientSiblings;
            }
            pcdNext = (*pcdLeader).transientChildren as *mut ClientData;
            while !(*pcdNext).transientSiblings.is_null() {
                if (*pcdNext).transientSiblings == pcd {
                    (*pcdNext).transientSiblings = (*pcd).transientSiblings;
                } else {
                    pcdNext = (*pcdNext).transientSiblings as *mut ClientData;
                }
            }
            (*pcdNext).transientSiblings = pcd as *mut _ClientData;
        }
        (*pcd).transientSiblings = ::core::ptr::null_mut::<_ClientData>();
    }
    return restack;
}
#[no_mangle]
pub unsafe extern "C" fn RestackTransients(mut pcd: *mut ClientData) {
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut count: ::core::ffi::c_int = 0;
    static mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut windows: *mut Window = ::core::ptr::null::<Window>() as *mut Window;
    let mut windowChanges = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    let mut i: ::core::ffi::c_int = 0;
    let mut leaderIndex: ::core::ffi::c_int = 0;
    pcdLeader = FindTransientTreeLeader(pcd);
    count = CountTransientChildren(pcdLeader);
    if count < 2 as ::core::ffi::c_int {
        return;
    }
    if count > size {
        windows = WmMalloc(
            windows as *mut ::core::ffi::c_char,
            ((count + 5 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<Window>() as usize)
                as ::core::ffi::c_uint,
        ) as *mut Window;
        if windows.is_null() {
            size = 0 as ::core::ffi::c_int;
            return;
        }
        size = count + 5 as ::core::ffi::c_int;
    }
    MakeTransientFamilyStackingList(windows, pcdLeader);
    leaderIndex = 0 as ::core::ffi::c_int;
    while leaderIndex < count {
        if *windows.offset(leaderIndex as isize) == (*pcdLeader).clientFrameWin {
            break;
        }
        leaderIndex += 1;
    }
    if leaderIndex >= count {
        leaderIndex = count - 1 as ::core::ffi::c_int;
    }
    windowChanges.stack_mode = Above;
    i = leaderIndex;
    while i > 0 as ::core::ffi::c_int {
        windowChanges.sibling = *windows.offset(i as isize);
        XConfigureWindow(
            wmGD.display,
            *windows.offset((i - 1 as ::core::ffi::c_int) as isize),
            (CWSibling | CWStackMode) as ::core::ffi::c_uint,
            &raw mut windowChanges,
        );
        i -= 1;
    }
    windowChanges.stack_mode = Below;
    i = leaderIndex;
    while i < count - 1 as ::core::ffi::c_int {
        windowChanges.sibling = *windows.offset(i as isize);
        XConfigureWindow(
            wmGD.display,
            *windows.offset((i + 1 as ::core::ffi::c_int) as isize),
            (CWSibling | CWStackMode) as ::core::ffi::c_uint,
            &raw mut windowChanges,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RestackTransientsAtWindow(mut pcd: *mut ClientData) {
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut windowChanges = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    pcdLeader = FindTransientTreeLeader(pcd);
    if !pcdLeader.is_null() && pcdLeader != pcd {
        windowChanges.sibling = (*pcd).clientFrameWin;
        windowChanges.stack_mode = Below;
        XConfigureWindow(
            wmGD.display,
            (*pcdLeader).clientFrameWin,
            (CWSibling | CWStackMode) as ::core::ffi::c_uint,
            &raw mut windowChanges,
        );
    }
    RestackTransients(pcd);
}
#[no_mangle]
pub unsafe extern "C" fn FindTransientTreeLeader(mut pcd: *mut ClientData) -> *mut ClientData {
    while !(*pcd).transientLeader.is_null() {
        pcd = (*pcd).transientLeader as *mut ClientData;
    }
    return pcd;
}
#[no_mangle]
pub unsafe extern "C" fn CountTransientChildren(mut pcd: *mut ClientData) -> ::core::ffi::c_int {
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    let mut count = 1 as ::core::ffi::c_int;
    pcdNext = (*pcd).transientChildren as *mut ClientData;
    while !pcdNext.is_null() {
        if !(*pcdNext).transientChildren.is_null() {
            count += CountTransientChildren(pcdNext);
        } else {
            count += 1;
        }
        pcdNext = (*pcdNext).transientSiblings as *mut ClientData;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn MakeTransientWindowList(
    mut windows: *mut Window,
    mut pcd: *mut ClientData,
) -> *mut Window {
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    pcdNext = (*pcd).transientChildren as *mut ClientData;
    while !pcdNext.is_null() {
        if !(*pcdNext).transientChildren.is_null() {
            windows = MakeTransientWindowList(windows, pcdNext);
        }
        *windows = (*pcdNext).clientFrameWin;
        windows = windows.offset(1);
        pcdNext = (*pcdNext).transientSiblings as *mut ClientData;
    }
    return windows;
}
#[no_mangle]
pub unsafe extern "C" fn FindTransientFocus(mut pcd: *mut ClientData) -> *mut ClientData {
    let mut pcdFocus = ::core::ptr::null_mut::<ClientData>();
    pcdFocus = pcd;
    while !(*pcdFocus).transientChildren.is_null()
        && ((*pcdFocus).primaryModalCount != 0 || (*pcdFocus).fullModalCount != 0)
    {
        pcdFocus = (*pcdFocus).transientChildren as *mut ClientData;
    }
    if (*pcdFocus).primaryModalCount != 0 || (*pcdFocus).fullModalCount != 0 {
        let mut pcdSibling = ::core::ptr::null_mut::<ClientData>();
        pcdFocus = pcd;
        while !pcdFocus.is_null()
            && ((*pcdFocus).primaryModalCount != 0 || (*pcdFocus).fullModalCount != 0)
        {
            pcdSibling = pcdFocus;
            while !(*pcdSibling).transientSiblings.is_null()
                && ((*pcdFocus).primaryModalCount != 0 || (*pcdFocus).fullModalCount != 0)
            {
                pcdSibling = (*pcdSibling).transientSiblings as *mut ClientData;
            }
            if (*pcdSibling).primaryModalCount != 0 || (*pcdSibling).fullModalCount != 0 {
                pcdFocus = (*pcdFocus).transientChildren as *mut ClientData;
            } else {
                pcdFocus = pcdSibling;
                break;
            }
        }
    }
    return if !pcdFocus.is_null() {
        pcdFocus
    } else {
        wmGD.keyboardFocus
    };
}
#[no_mangle]
pub unsafe extern "C" fn FindTransientOnTop(mut pcd: *mut ClientData) -> *mut ClientData {
    pcd = FindTransientTreeLeader(pcd);
    if (*pcd).secondariesOnTop == 0 && LeaderOnTop(pcd) as ::core::ffi::c_int != 0 {
        let mut pcdSub = ::core::ptr::null_mut::<ClientData>();
        if LeaderOnTop(pcd) != 0 {
            return pcd;
        } else {
            pcdSub = FindSubLeaderToTop(pcd);
            if !pcdSub.is_null() {
                return pcdSub;
            }
        }
    }
    while !(*pcd).transientChildren.is_null() {
        pcd = (*pcd).transientChildren as *mut ClientData;
    }
    return pcd;
}
#[no_mangle]
pub unsafe extern "C" fn StackWindow(
    mut pWS: *mut WmWorkspaceData,
    mut pEntry: *mut ClientListEntry,
    mut onTop: Boolean,
    mut pStackEntry: *mut ClientListEntry,
) {
    let mut stackWindow: Window = 0;
    let mut stackTransientTreeWindows = False as Boolean;
    let mut activeIconWindow: Window = 0;
    let mut window: Window = 0;
    let mut changes = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    let mut pSD = (*pWS).pSD;
    if !pStackEntry.is_null() {
        if (*pStackEntry).type_0 == MINIMIZED_STATE {
            stackWindow = (*(*(*pStackEntry).pCD)
                .pWsList
                .offset((*(*pStackEntry).pCD).currentWsc as isize))
            .iconFrameWin;
        } else {
            stackWindow = (*(*pStackEntry).pCD).clientFrameWin;
        }
    } else {
        stackWindow = 0 as ::core::ffi::c_int as Window;
    }
    if (*pEntry).type_0 == MINIMIZED_STATE {
        window = (*(*(*pEntry).pCD)
            .pWsList
            .offset((*(*pEntry).pCD).currentWsc as isize))
        .iconFrameWin;
    } else if !(*(*pEntry).pCD).transientLeader.is_null()
        || !(*(*pEntry).pCD).transientChildren.is_null()
    {
        stackTransientTreeWindows = True as Boolean;
        window = (*FindTransientOnTop((*pEntry).pCD as *mut ClientData)).clientFrameWin;
    } else {
        window = (*(*pEntry).pCD).clientFrameWin;
    }
    if (*pEntry).type_0 == MINIMIZED_STATE
        && (*pEntry).pCD == wmGD.keyboardFocus
        && (*(*(*pEntry).pCD).pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART
            != 0
        && (*wmGD.pActiveSD).activeIconTextWin != 0
    {
        activeIconWindow = (*wmGD.pActiveSD).activeIconTextWin;
    } else {
        activeIconWindow = 0 as ::core::ffi::c_int as Window;
    }
    if onTop != 0 {
        if stackWindow == 0 as Window && !(*pSD).clientList.is_null() {
            if (*(*pSD).clientList).type_0 == MINIMIZED_STATE {
                stackWindow = (*(*(*(*pSD).clientList).pCD)
                    .pWsList
                    .offset((*(*(*pSD).clientList).pCD).currentWsc as isize))
                .iconFrameWin;
            } else if !(*(*(*pSD).clientList).pCD).transientChildren.is_null() {
                stackWindow = (*FindTransientOnTop((*(*pSD).clientList).pCD as *mut ClientData))
                    .clientFrameWin;
            } else {
                stackWindow = (*(*(*pSD).clientList).pCD).clientFrameWin;
            }
        }
        if activeIconWindow != 0 {
            changes.sibling = stackWindow;
            changes.stack_mode = Above;
            XConfigureWindow(
                wmGD.display,
                activeIconWindow,
                (CWSibling | CWStackMode) as ::core::ffi::c_uint,
                &raw mut changes,
            );
            changes.sibling = activeIconWindow;
            changes.stack_mode = Below;
            XConfigureWindow(
                wmGD.display,
                window,
                (CWSibling | CWStackMode) as ::core::ffi::c_uint,
                &raw mut changes,
            );
        } else {
            changes.sibling = stackWindow;
            changes.stack_mode = Above;
            XConfigureWindow(
                wmGD.display,
                window,
                (CWSibling | CWStackMode) as ::core::ffi::c_uint,
                &raw mut changes,
            );
            if stackTransientTreeWindows != 0 {
                changes.sibling = window;
                changes.stack_mode = Below;
                XConfigureWindow(
                    wmGD.display,
                    (*(*pEntry).pCD).clientFrameWin,
                    (CWSibling | CWStackMode) as ::core::ffi::c_uint,
                    &raw mut changes,
                );
                RestackTransients((*pEntry).pCD as *mut ClientData);
            }
        }
    } else {
        if !pStackEntry.is_null() && !(*(*pStackEntry).pCD).transientChildren.is_null() {
            stackWindow = LowestWindowInTransientFamily((*pStackEntry).pCD as *mut ClientData);
        }
        if stackWindow == 0 as Window {
            if (*(*pSD).lastClient).type_0 == MINIMIZED_STATE {
                stackWindow = (*(*(*(*pSD).lastClient).pCD)
                    .pWsList
                    .offset((*(*(*pSD).lastClient).pCD).currentWsc as isize))
                .iconFrameWin;
            } else if !(*(*(*pSD).lastClient).pCD).transientChildren.is_null() {
                stackWindow =
                    LowestWindowInTransientFamily((*(*pSD).lastClient).pCD as *mut ClientData);
            } else {
                stackWindow = (*(*(*pSD).lastClient).pCD).clientFrameWin;
            }
        }
        if activeIconWindow != 0 {
            changes.sibling = stackWindow;
            changes.stack_mode = Below;
            XConfigureWindow(
                wmGD.display,
                activeIconWindow,
                (CWSibling | CWStackMode) as ::core::ffi::c_uint,
                &raw mut changes,
            );
            changes.sibling = activeIconWindow;
            changes.stack_mode = Below;
            XConfigureWindow(
                wmGD.display,
                window,
                (CWSibling | CWStackMode) as ::core::ffi::c_uint,
                &raw mut changes,
            );
        } else {
            changes.sibling = stackWindow;
            changes.stack_mode = Below;
            XConfigureWindow(
                wmGD.display,
                window,
                (CWSibling | CWStackMode) as ::core::ffi::c_uint,
                &raw mut changes,
            );
            if stackTransientTreeWindows != 0 {
                changes.sibling = window;
                changes.stack_mode = Below;
                XConfigureWindow(
                    wmGD.display,
                    (*(*pEntry).pCD).clientFrameWin,
                    (CWSibling | CWStackMode) as ::core::ffi::c_uint,
                    &raw mut changes,
                );
                RestackTransients((*pEntry).pCD as *mut ClientData);
            }
        }
    }
    RestackDesktopSurfaces(pSD);
}
#[no_mangle]
pub unsafe extern "C" fn StackTransientWindow(mut pcd: *mut ClientData) {
    let mut changes = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    let mut pcdPrev = ::core::ptr::null_mut::<ClientData>();
    if (*(*pcd).transientLeader).transientChildren == pcd {
        if !(*pcd).transientSiblings.is_null() {
            changes.sibling = (*(*pcd).transientSiblings).clientFrameWin;
        } else {
            changes.sibling = (*(*pcd).transientLeader).clientFrameWin;
        }
        changes.stack_mode = Above;
    } else {
        pcdPrev = (*pcd).transientLeader as *mut ClientData;
        while (*pcdPrev).transientSiblings != pcd {
            pcdPrev = (*pcdPrev).transientSiblings as *mut ClientData;
        }
        changes.sibling = (*pcdPrev).clientFrameWin;
        changes.stack_mode = Below;
    }
    XConfigureWindow(
        wmGD.display,
        (*pcd).clientFrameWin,
        (CWSibling | CWStackMode) as ::core::ffi::c_uint,
        &raw mut changes,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CheckIfClientObscuring(
    mut pcdTop: *mut ClientData,
    mut pcd: *mut ClientData,
) -> Boolean {
    let mut obscuring = False as Boolean;
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    if !(*pcdTop).transientChildren.is_null() && (*pcdTop).clientState != MINIMIZED_STATE {
        pcdNext = (*pcdTop).transientChildren as *mut ClientData;
        while !pcdNext.is_null() && obscuring == 0 {
            obscuring = CheckIfClientObscuring(pcdNext, pcd);
            pcdNext = (*pcdNext).transientSiblings as *mut ClientData;
        }
    }
    if obscuring == 0
        && !(*pcd).transientChildren.is_null()
        && (*pcd).clientState != MINIMIZED_STATE
    {
        pcdNext = (*pcd).transientChildren as *mut ClientData;
        while !pcdNext.is_null() && obscuring == 0 {
            obscuring = CheckIfClientObscuring(pcdTop, pcdNext);
            pcdNext = (*pcdNext).transientSiblings as *mut ClientData;
        }
    }
    if obscuring == 0 {
        obscuring = CheckIfObscuring(pcdTop, pcd);
    }
    return obscuring;
}
#[no_mangle]
pub unsafe extern "C" fn CheckIfObscuring(
    mut pcdA: *mut ClientData,
    mut pcdB: *mut ClientData,
) -> Boolean {
    let mut obscuring = False as Boolean;
    let mut aX1: ::core::ffi::c_int = 0;
    let mut aX2: ::core::ffi::c_int = 0;
    let mut aY1: ::core::ffi::c_int = 0;
    let mut aY2: ::core::ffi::c_int = 0;
    let mut bX1: ::core::ffi::c_int = 0;
    let mut bX2: ::core::ffi::c_int = 0;
    let mut bY1: ::core::ffi::c_int = 0;
    let mut bY2: ::core::ffi::c_int = 0;
    if (*pcdA).clientState & UNSEEN_STATE != 0 || (*pcdB).clientState & UNSEEN_STATE != 0 {
        return 0 as Boolean;
    }
    if (*pcdA).clientState == NORMAL_STATE {
        aX1 = (*pcdA).clientX - (*pcdA).clientOffset.x as ::core::ffi::c_int;
        aY1 = (*pcdA).clientY - (*pcdA).clientOffset.y as ::core::ffi::c_int;
        aX2 = aX1
            + (*pcdA).clientWidth
            + 2 as ::core::ffi::c_int * (*pcdA).clientOffset.x as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        aY2 = aY1
            + (*pcdA).clientHeight
            + (*pcdA).clientOffset.y as ::core::ffi::c_int
            + (*pcdA).clientOffset.x as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
    } else if (*pcdA).clientState == MINIMIZED_STATE {
        aX1 = (*(*pcdA).pWsList.offset((*pcdA).currentWsc as isize)).iconX;
        aY1 = (*(*pcdA).pWsList.offset((*pcdA).currentWsc as isize)).iconY;
        aX2 = aX1 + (*(*pcdA).pSD).iconWidth - 1 as ::core::ffi::c_int;
        aY2 = aY1 + (*(*pcdA).pSD).iconHeight - 1 as ::core::ffi::c_int;
    } else {
        aX1 = (*pcdA).maxX - (*pcdA).clientOffset.x as ::core::ffi::c_int;
        aY1 = (*pcdA).maxY - (*pcdA).clientOffset.y as ::core::ffi::c_int;
        aX2 = aX1
            + (*pcdA).maxWidth
            + 2 as ::core::ffi::c_int * (*pcdA).clientOffset.x as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        aY2 = aY1
            + (*pcdA).maxHeight
            + (*pcdA).clientOffset.y as ::core::ffi::c_int
            + (*pcdA).clientOffset.x as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
    }
    if (*pcdB).clientState == NORMAL_STATE {
        bX1 = (*pcdB).clientX - (*pcdB).clientOffset.x as ::core::ffi::c_int;
        bY1 = (*pcdB).clientY - (*pcdB).clientOffset.y as ::core::ffi::c_int;
        bX2 = bX1
            + (*pcdB).clientWidth
            + 2 as ::core::ffi::c_int * (*pcdB).clientOffset.x as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        bY2 = bY1
            + (*pcdB).clientHeight
            + (*pcdB).clientOffset.y as ::core::ffi::c_int
            + (*pcdB).clientOffset.x as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
    } else if (*pcdB).clientState == MINIMIZED_STATE {
        bX1 = (*(*pcdB).pWsList.offset((*pcdB).currentWsc as isize)).iconX;
        bY1 = (*(*pcdB).pWsList.offset((*pcdB).currentWsc as isize)).iconY;
        bX2 = bX1 + (*(*pcdB).pSD).iconWidth - 1 as ::core::ffi::c_int;
        bY2 = bY1 + (*(*pcdB).pSD).iconHeight - 1 as ::core::ffi::c_int;
    } else {
        bX1 = (*pcdB).maxX - (*pcdB).clientOffset.x as ::core::ffi::c_int;
        bY1 = (*pcdB).maxY - (*pcdB).clientOffset.y as ::core::ffi::c_int;
        bX2 = bX1
            + (*pcdB).maxWidth
            + 2 as ::core::ffi::c_int * (*pcdB).clientOffset.x as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        bY2 = bY1
            + (*pcdB).maxHeight
            + (*pcdB).clientOffset.y as ::core::ffi::c_int
            + (*pcdB).clientOffset.x as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
    }
    if aX1 >= bX1 && aX1 <= bX2
        || aX2 >= bX1 && aX2 <= bX2
        || bX1 >= aX1 && bX1 <= aX2
        || bX2 >= aX1 && bX2 <= aX2
    {
        if aY1 >= bY1 && aY1 <= bY2
            || aY2 >= bY1 && aY2 <= bY2
            || bY1 >= aY1 && bY1 <= aY2
            || bY2 >= aY1 && bY2 <= aY2
        {
            obscuring = True as Boolean;
        }
    }
    return obscuring;
}
#[no_mangle]
pub unsafe extern "C" fn CheckIfClientObscuredByAny(mut pcd: *mut ClientData) -> Boolean {
    let mut obscured = False as Boolean;
    let mut pListEntry = ::core::ptr::null_mut::<ClientListEntry>();
    pListEntry = (*wmGD.pActiveSD).clientList;
    while !pListEntry.is_null() && obscured == 0 {
        if (*pListEntry).pCD == pcd {
            if (*pListEntry).type_0 == MINIMIZED_STATE
                && (*(*pListEntry).pCD).clientState == MINIMIZED_STATE
                || (*pListEntry).type_0 != MINIMIZED_STATE
                    && (*(*pListEntry).pCD).clientState != MINIMIZED_STATE
            {
                pListEntry = ::core::ptr::null_mut::<ClientListEntry>();
            }
        } else if (*pListEntry).type_0 == MINIMIZED_STATE
            && (*(*pListEntry).pCD).clientState == MINIMIZED_STATE
            || (*pListEntry).type_0 != MINIMIZED_STATE
                && (*(*pListEntry).pCD).clientState != MINIMIZED_STATE
        {
            obscured = CheckIfClientObscuring((*pListEntry).pCD as *mut ClientData, pcd);
        }
        if !pListEntry.is_null() {
            pListEntry = (*pListEntry).nextSibling as *mut ClientListEntry;
        }
    }
    return obscured;
}
#[no_mangle]
pub unsafe extern "C" fn CheckIfClientObscuringAny(mut pcd: *mut ClientData) -> Boolean {
    let mut obscuring = False as Boolean;
    let mut pListEntry = ::core::ptr::null_mut::<ClientListEntry>();
    pListEntry = if (*pcd).clientState == MINIMIZED_STATE {
        &raw mut (*pcd).iconEntry
    } else {
        &raw mut (*pcd).clientEntry
    };
    while !pListEntry.is_null() && obscuring == 0 {
        if (*pListEntry).pCD != pcd
            && ((*pListEntry).type_0 == MINIMIZED_STATE
                && (*(*pListEntry).pCD).clientState == MINIMIZED_STATE
                || (*pListEntry).type_0 != MINIMIZED_STATE
                    && (*(*pListEntry).pCD).clientState != MINIMIZED_STATE)
        {
            obscuring = CheckIfClientObscuring(pcd, (*pListEntry).pCD as *mut ClientData);
        }
        pListEntry = (*pListEntry).nextSibling as *mut ClientListEntry;
    }
    return obscuring;
}
#[no_mangle]
pub unsafe extern "C" fn SetupSystemModalState(mut pCD: *mut ClientData) {
    let mut windowChanges = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut x_hot: ::core::ffi::c_uint = 0;
    let mut y_hot: ::core::ffi::c_uint = 0;
    let mut bits = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut mask_bits = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    let mut scr: ::core::ffi::c_int = 0;
    if !wmGD.menuActive.is_null() {
        UnpostMenu(wmGD.menuActive);
        XSync(wmGD.display, False);
    }
    if XGrabPointer(
        wmGD.display,
        (*(*pCD).pSD).rootWindow,
        FALSE,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        GrabModeAsync,
        GrabModeAsync,
        None_0 as Window,
        None_0 as Cursor,
        CurrentTime as Time,
    ) == GrabFrozen
    {
        return;
    } else {
        XUngrabPointer(wmGD.display, CurrentTime as Time);
    }
    if XGrabKeyboard(
        wmGD.display,
        (*(*pCD).pSD).rootWindow,
        FALSE,
        GrabModeAsync,
        GrabModeAsync,
        CurrentTime as Time,
    ) == GrabFrozen
    {
        return;
    } else {
        XUngrabKeyboard(wmGD.display, CurrentTime as Time);
    }
    if wmGD.useLargeCursors != 0 {
        width = noenter32_width as ::core::ffi::c_uint;
        height = noenter32_height as ::core::ffi::c_uint;
        x_hot = noenter32_x_hot as ::core::ffi::c_uint;
        y_hot = noenter32_y_hot as ::core::ffi::c_uint;
        bits = &raw mut noenter32_bits as *mut ::core::ffi::c_uchar;
        mask_bits = &raw mut noenter32m_bits as *mut ::core::ffi::c_uchar;
    } else {
        width = noenter16_width as ::core::ffi::c_uint;
        height = noenter16_height as ::core::ffi::c_uint;
        x_hot = noenter16_x_hot as ::core::ffi::c_uint;
        y_hot = noenter16_y_hot as ::core::ffi::c_uint;
        bits = &raw mut noenter16_bits as *mut ::core::ffi::c_uchar;
        mask_bits = &raw mut noenter16m_bits as *mut ::core::ffi::c_uchar;
    }
    scr = 0 as ::core::ffi::c_int;
    while scr < wmGD.numScreens {
        pSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
        if (*pSD).managed as ::core::ffi::c_int != 0 && (*pSD).inputScreenWindow == 0 as Window {
            let mut windowAttributes = XSetWindowAttributes {
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
            let mut pixmap: Pixmap = 0;
            let mut maskPixmap: Pixmap = 0;
            let mut xcolors: [XColor; 2] = [XColor {
                pixel: 0,
                red: 0,
                green: 0,
                blue: 0,
                flags: 0,
                pad: 0,
            }; 2];
            windowAttributes.event_mask = ButtonPressMask;
            if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER {
                windowAttributes.event_mask |= EnterWindowMask;
            }
            windowAttributes.override_redirect = True;
            pixmap = XCreateBitmapFromData(
                wmGD.display,
                (*pSD).rootWindow as Drawable,
                bits as *mut ::core::ffi::c_char,
                width,
                height,
            );
            maskPixmap = XCreateBitmapFromData(
                wmGD.display,
                (*pSD).rootWindow as Drawable,
                mask_bits as *mut ::core::ffi::c_char,
                width,
                height,
            );
            xcolors[0 as ::core::ffi::c_int as usize].pixel = (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .black_pixel;
            xcolors[1 as ::core::ffi::c_int as usize].pixel = (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .white_pixel;
            XQueryColors(
                wmGD.display,
                (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*pSD).screen as isize))
                .cmap,
                &raw mut xcolors as *mut XColor,
                2 as ::core::ffi::c_int,
            );
            windowAttributes.cursor = XCreatePixmapCursor(
                wmGD.display,
                pixmap,
                maskPixmap,
                (&raw mut xcolors as *mut XColor).offset(0 as ::core::ffi::c_int as isize)
                    as *mut XColor,
                (&raw mut xcolors as *mut XColor).offset(1 as ::core::ffi::c_int as isize)
                    as *mut XColor,
                x_hot,
                y_hot,
            );
            XFreePixmap(wmGD.display, pixmap);
            XFreePixmap(wmGD.display, maskPixmap);
            (*pSD).inputScreenWindow = XCreateWindow(
                wmGD.display,
                (*pSD).rootWindow,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*pSD).screen as isize))
                .width as ::core::ffi::c_uint,
                (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*pSD).screen as isize))
                .height as ::core::ffi::c_uint,
                0 as ::core::ffi::c_uint,
                0 as ::core::ffi::c_int,
                InputOnly as ::core::ffi::c_uint,
                ::core::ptr::null_mut::<Visual>(),
                (CWEventMask | CWOverrideRedirect | CWCursor) as ::core::ffi::c_ulong,
                &raw mut windowAttributes,
            );
        }
        if (*pSD).managed as ::core::ffi::c_int != 0 && pSD != wmGD.pActiveSD {
            XMapRaised(wmGD.display, (*pSD).inputScreenWindow);
        }
        scr += 1;
    }
    if !pCD.is_null() {
        wmGD.systemModalWindow = (*pCD).clientFrameWin;
    } else {
        SetKeyboardFocus(::core::ptr::null_mut::<ClientData>(), REFRESH_LAST_FOCUS);
        XSetInputFocus(
            wmGD.display,
            wmGD.systemModalWindow,
            PointerRoot as ::core::ffi::c_int,
            CurrentTime as Time,
        );
    }
    windowChanges.sibling = wmGD.systemModalWindow;
    windowChanges.stack_mode = Below;
    XConfigureWindow(
        wmGD.display,
        (*wmGD.pActiveSD).inputScreenWindow,
        (CWSibling | CWStackMode) as ::core::ffi::c_uint,
        &raw mut windowChanges,
    );
    XMapWindow(wmGD.display, (*wmGD.pActiveSD).inputScreenWindow);
    wmGD.systemModalActive = True as Boolean;
    wmGD.systemModalClient = pCD;
}
#[no_mangle]
pub unsafe extern "C" fn UndoSystemModalState() {
    let mut scr: ::core::ffi::c_int = 0;
    scr = 0 as ::core::ffi::c_int;
    while scr < wmGD.numScreens {
        if (*wmGD.Screens.offset(scr as isize)).managed != 0 {
            XUnmapWindow(
                wmGD.display,
                (*wmGD.Screens.offset(scr as isize)).inputScreenWindow,
            );
        }
        scr += 1;
    }
    if wmGD.systemModalClient.is_null() {
        AutoResetKeyFocus(::core::ptr::null_mut::<ClientData>(), GetTimestamp());
    }
    wmGD.systemModalActive = False as Boolean;
    wmGD.systemModalClient = ::core::ptr::null_mut::<ClientData>();
    wmGD.systemModalWindow = 0 as Window;
}
#[no_mangle]
pub unsafe extern "C" fn FindClientNameMatch(
    mut pEntry: *mut ClientListEntry,
    mut toNext: Boolean,
    mut clientName: String_0,
    mut types: ::core::ffi::c_ulong,
) -> *mut ClientListEntry {
    let mut foundMatch = False as Boolean;
    let mut checkEntry: Boolean = 0;
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    while foundMatch == 0 && !pEntry.is_null() {
        checkEntry = False as Boolean;
        pCD = (*pEntry).pCD as *mut ClientData;
        if (*pEntry).type_0 == MINIMIZED_STATE {
            if (*pCD).clientState == MINIMIZED_STATE
                && types & F_GROUP_ICON as ::core::ffi::c_ulong != 0
            {
                checkEntry = True as Boolean;
            }
        } else if (*pCD).clientState != MINIMIZED_STATE
            && types & F_GROUP_WINDOW as ::core::ffi::c_ulong != 0
        {
            checkEntry = True as Boolean;
        }
        if checkEntry as ::core::ffi::c_int != 0
            && (!(*pCD).clientName.is_null()
                && strcmp(
                    clientName as *const ::core::ffi::c_char,
                    (*pCD).clientName as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || !(*pCD).clientClass.is_null()
                    && strcmp(
                        clientName as *const ::core::ffi::c_char,
                        (*pCD).clientClass as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int)
        {
            foundMatch = True as Boolean;
        } else {
            pEntry = (if toNext as ::core::ffi::c_int != 0 {
                (*pEntry).nextSibling
            } else {
                (*pEntry).prevSibling
            }) as *mut ClientListEntry;
        }
    }
    return pEntry;
}
#[no_mangle]
pub unsafe extern "C" fn BumpPrimaryToTop(mut pcdLeader: *mut ClientData) -> Boolean {
    let mut count: ::core::ffi::c_int = 0;
    let mut rval: Boolean = 0;
    count = CountTransientChildren(pcdLeader);
    if (*pcdLeader).primaryStackPosition != count - 1 as ::core::ffi::c_int {
        (*pcdLeader).primaryStackPosition = count - 1 as ::core::ffi::c_int;
        rval = True as Boolean;
    } else {
        rval = False as Boolean;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn BumpPrimaryToBottom(mut pcdLeader: *mut ClientData) -> Boolean {
    let mut rval: Boolean = 0;
    if (*pcdLeader).primaryStackPosition != 0 as ::core::ffi::c_int {
        (*pcdLeader).primaryStackPosition = 0 as ::core::ffi::c_int;
        rval = True as Boolean;
    } else {
        rval = False as Boolean;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn LowestWindowInTransientFamily(mut pcdLeader: *mut ClientData) -> Window {
    let mut count: ::core::ffi::c_int = 0;
    static mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut windows: *mut Window = ::core::ptr::null::<Window>() as *mut Window;
    let mut wReturn = None_0 as Window;
    count = CountTransientChildren(pcdLeader);
    if count > size {
        windows = WmMalloc(
            windows as *mut ::core::ffi::c_char,
            ((count + 5 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<Window>() as usize)
                as ::core::ffi::c_uint,
        ) as *mut Window;
        if windows.is_null() {
            size = 0 as ::core::ffi::c_int;
            return None_0 as Window;
        }
        size = count + 5 as ::core::ffi::c_int;
    }
    MakeTransientFamilyStackingList(windows, pcdLeader);
    if count > 0 as ::core::ffi::c_int {
        wReturn = *windows.offset((count - 1 as ::core::ffi::c_int) as isize);
    } else {
        wReturn = None_0 as Window;
    }
    return wReturn;
}
#[no_mangle]
pub unsafe extern "C" fn FindSubLeaderToTop(mut pcd: *mut ClientData) -> *mut ClientData {
    let mut pcdRet = ::core::ptr::null_mut::<ClientData>();
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    pcdNext = (*pcd).transientChildren as *mut ClientData;
    while !pcdNext.is_null() && pcdRet.is_null() {
        if !(*pcdNext).transientChildren.is_null() {
            if LeaderOnTop(pcdNext) != 0 {
                pcdRet = pcdNext;
            } else {
                pcdRet = FindSubLeaderToTop(pcdNext);
            }
        }
        pcdNext = (*pcdNext).transientSiblings as *mut ClientData;
    }
    return pcdRet;
}
#[no_mangle]
pub unsafe extern "C" fn MakeTransientFamilyStackingList(
    mut windows: *mut Window,
    mut pcdLeader: *mut ClientData,
) {
    let mut pcdSub = ::core::ptr::null_mut::<ClientData>();
    let mut nextWindow = ::core::ptr::null_mut::<Window>();
    let mut wSave: Window = 0;
    let mut wTemp: Window = 0;
    let mut wTop: Window = 0;
    let mut count = CountTransientChildren(pcdLeader);
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    nextWindow = MakeTransientWindowList(windows, pcdLeader);
    if (*pcdLeader).secondariesOnTop == 0 {
        if (*pcdLeader).primaryStackPosition > 0 as ::core::ffi::c_int
            && (*pcdLeader).primaryStackPosition < count
        {
            i = 0 as ::core::ffi::c_int;
            while i < (*pcdLeader).primaryStackPosition {
                j = count - i - 1 as ::core::ffi::c_int;
                *windows.offset(j as isize) =
                    *windows.offset((j - 1 as ::core::ffi::c_int) as isize);
                i += 1;
            }
            j = count - (*pcdLeader).primaryStackPosition - 1 as ::core::ffi::c_int;
            *windows.offset(j as isize) = (*pcdLeader).clientFrameWin;
        } else {
            *nextWindow = (*pcdLeader).clientFrameWin;
            pcdSub = FindSubLeaderToTop(pcdLeader);
            if !pcdSub.is_null() && (*pcdSub).clientFrameWin != None_0 as Window {
                wSave = (*pcdSub).clientFrameWin;
                wTop = wSave;
                i = 0 as ::core::ffi::c_int;
                while i < count {
                    wTemp = *windows.offset(i as isize);
                    *windows.offset(i as isize) = wSave;
                    wSave = wTemp;
                    if wTop == wSave {
                        break;
                    }
                    i += 1;
                }
            }
        }
    } else {
        *nextWindow = (*pcdLeader).clientFrameWin;
    };
}
#[no_mangle]
pub unsafe extern "C" fn NormalizeTransientTreeStacking(mut pcdLeader: *mut ClientData) -> Boolean {
    let mut pcdNext = ::core::ptr::null_mut::<ClientData>();
    let mut bChanged = False as Boolean;
    pcdNext = (*pcdLeader).transientChildren as *mut ClientData;
    bChanged = BumpPrimaryToBottom(pcdLeader);
    while !pcdNext.is_null() {
        if !(*pcdNext).transientChildren.is_null() {
            bChanged = (bChanged as ::core::ffi::c_int
                | BumpPrimaryToBottom(pcdNext) as ::core::ffi::c_int)
                as Boolean;
            bChanged = (bChanged as ::core::ffi::c_int
                | NormalizeTransientTreeStacking(pcdNext) as ::core::ffi::c_int)
                as Boolean;
        }
        pcdNext = (*pcdNext).transientSiblings as *mut ClientData;
    }
    return bChanged;
}
#[no_mangle]
pub unsafe extern "C" fn LeaderOnTop(mut pcdLeader: *mut ClientData) -> Boolean {
    let mut bOnTop = False as Boolean;
    let mut count = CountTransientChildren(pcdLeader);
    if (*pcdLeader).primaryStackPosition > 0 as ::core::ffi::c_int
        && (*pcdLeader).primaryStackPosition < count
    {
        bOnTop = True as Boolean;
    }
    return bOnTop;
}
