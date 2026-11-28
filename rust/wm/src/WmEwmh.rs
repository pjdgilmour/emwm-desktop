/*
 * Copyright (C) 2018-2026 alx@fastestcode.org
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed_0;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_1;
use crate::abi::C2RustUnnamed_2_WmEwmh as C2RustUnnamed_2;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_3;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_4;
use crate::abi::C2RustUnnamed_WmArgb as C2RustUnnamed;
use crate::abi::WmWorkspaceData_WmBackdrop as WmWorkspaceData;
use crate::abi::*;
extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn ffs(__i: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ceilf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn floorf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn XCreateImage(
        _: *mut Display,
        _: *mut Visual,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut XImage;
    fn XInternAtom(_: *mut Display, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int) -> Atom;
    fn XInternAtoms(
        _: *mut Display,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut Atom,
    ) -> ::core::ffi::c_int;
    fn XCreateGC(_: *mut Display, _: Drawable, _: ::core::ffi::c_ulong, _: *mut XGCValues) -> GC;
    fn XCreatePixmap(
        _: *mut Display,
        _: Drawable,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> Pixmap;
    fn XGetSelectionOwner(_: *mut Display, _: Atom) -> Window;
    fn XDefaultVisual(_: *mut Display, _: ::core::ffi::c_int) -> *mut Visual;
    fn XBitmapPad(_: *mut Display) -> ::core::ffi::c_int;
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
    fn XClearArea(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XDefaultDepth(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XDisplayHeight(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XDisplayWidth(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XFree(_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn XFreeGC(_: *mut Display, _: GC) -> ::core::ffi::c_int;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> ::core::ffi::c_int;
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
    fn XLowerWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XMoveResizeWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XPutImage(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XImage,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XQueryColor(_: *mut Display, _: Colormap, _: *mut XColor) -> ::core::ffi::c_int;
    fn XResizeWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XSendEvent(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_long,
        _: *mut XEvent,
    ) -> ::core::ffi::c_int;
    fn XUnmapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XtWindowOfObject(_: Widget) -> Window;
    fn XtFree(_: *mut ::core::ffi::c_char);
    fn XmStringCopy(string: XmString) -> XmString;
    fn XmStringLength(string: XmString) -> ::core::ffi::c_int;
    fn XmStringFree(string: XmString);
    fn XmStringUnparse(
        string: XmString,
        tag: XmStringTag,
        tag_type: XmTextType,
        output_type: XmTextType,
        parse_table: XmParseTable,
        parse_count: Cardinal,
        parse_model: XmParseModel,
    ) -> XtPointer;
    fn XmStringGenerate(
        text: XtPointer,
        tag: XmStringTag,
        type_0: XmTextType,
        rendition: XmStringTag,
    ) -> XmString;
    static mut wmGD: WmGlobalData;
    fn Warning(message: *mut ::core::ffi::c_char);
    fn MakeClientIconPixmap(pCD: *mut ClientData, iconBitmap: Pixmap, iconMask: Pixmap) -> Pixmap;
    fn UpperBorderWidth(pcd: *mut ClientData) -> ::core::ffi::c_uint;
    fn LowerBorderWidth(pcd: *mut ClientData) -> ::core::ffi::c_uint;
    fn BaseWindowX(pcd: *mut ClientData) -> ::core::ffi::c_int;
    fn BaseWindowY(pcd: *mut ClientData) -> ::core::ffi::c_int;
    fn BaseWindowWidth(pcd: *mut ClientData) -> ::core::ffi::c_uint;
    fn BaseWindowHeight(pcd: *mut ClientData) -> ::core::ffi::c_uint;
    fn DrawWindowTitle(pcd: *mut ClientData, eraseFirst: Boolean);
    fn RegenerateClientFrame(pcd: *mut ClientData);
    fn GetXineramaScreenFromLocation(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
    fn GetXineramaScreenInfo(
        index: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
    fn RedisplayIconTitle(pcd: *mut ClientData);
    fn AdjustCoordinatesToGravity(
        pCD: *mut ClientData,
        gravity: ::core::ffi::c_int,
        px: *mut ::core::ffi::c_int,
        py: *mut ::core::ffi::c_int,
        pwidth: *mut ::core::ffi::c_uint,
        pheight: *mut ::core::ffi::c_uint,
    );
    fn CancelFrameConfig(pcd: *mut ClientData);
    fn HandleClientFrameMove(pcd: *mut ClientData, pev: *mut XEvent);
    fn HandleClientFrameResize(pcd: *mut ClientData, pev: *mut XEvent);
    fn ProcessNewConfiguration(
        pCD: *mut ClientData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        clientRequest: Boolean,
    );
    fn StartClientMove(pcd: *mut ClientData, pev: *mut XEvent) -> Boolean;
    fn StartClientResize(pcd: *mut ClientData, pev: *mut XEvent);
    fn StartResizeConfig(pcd: *mut ClientData, pev: *mut XEvent) -> Boolean;
    fn F_Raise(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Focus_Key(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Maximize(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Minimize(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Kill(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Restore(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn SendConfigureNotify(pCD: *mut ClientData);
    fn ChangeToWorkspace(pNewWS: *mut WmWorkspaceData);
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const XmNO_TEXT: XmTextType = 3;
pub const XmWIDECHAR_TEXT: XmTextType = 2;
pub const XmMULTIBYTE_TEXT: XmTextType = 1;
pub const XmCHARSET_TEXT: XmTextType = 0;
pub const XmOUTPUT_BOTH: XmParseModel = 4;
pub const XmOUTPUT_END: XmParseModel = 3;
pub const XmOUTPUT_BEGINNING: XmParseModel = 2;
pub const XmOUTPUT_BETWEEN: XmParseModel = 1;
pub const XmOUTPUT_ALL: XmParseModel = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const _NET_WM_NAME: ewmh_atom = 12;
pub const _NET_SUPPORTING_WM_CHECK: ewmh_atom = 1;
pub const _NUM_EWMH_ATOMS: ewmh_atom = 38;
pub const _NET_SUPPORTED: ewmh_atom = 0;
pub const _NET_FRAME_EXTENTS: ewmh_atom = 4;
pub const _NET_WM_ICON: ewmh_atom = 20;
pub const _NET_WM_ICON_NAME: ewmh_atom = 13;
pub const _NET_WM_WINDOW_TYPE_SPLASH: ewmh_atom = 33;
pub const _NET_WM_WINDOW_TYPE_DIALOG: ewmh_atom = 36;
pub const _NET_WM_WINDOW_TYPE_UTILITY: ewmh_atom = 35;
pub const _NET_WM_WINDOW_TYPE_TOOLBAR: ewmh_atom = 34;
pub const _NET_WM_WINDOW_TYPE: ewmh_atom = 32;
pub const _NET_ACTIVE_WINDOW: ewmh_atom = 5;
pub const _NET_WM_FULLSCREEN_MONITORS: ewmh_atom = 30;
pub const MOVE: C2RustUnnamed_3 = 8;
pub const SIZE_LEFT: C2RustUnnamed_3 = 7;
pub const SIZE_BOTTOMLEFT: C2RustUnnamed_3 = 6;
pub const SIZE_BOTTOM: C2RustUnnamed_3 = 5;
pub const SIZE_BOTTOMRIGHT: C2RustUnnamed_3 = 4;
pub const SIZE_RIGHT: C2RustUnnamed_3 = 3;
pub const SIZE_TOPRIGHT: C2RustUnnamed_3 = 2;
pub const SIZE_TOP: C2RustUnnamed_3 = 1;
pub const SIZE_TOPLEFT: C2RustUnnamed_3 = 0;
pub const MOVE_KEYBOARD: C2RustUnnamed_3 = 10;
pub const SIZE_KEYBOARD: C2RustUnnamed_3 = 9;
pub const CANCEL: C2RustUnnamed_3 = 11;
pub const _NET_WM_MOVERESIZE: ewmh_atom = 21;
pub const _NET_MOVERESIZE_WINDOW: ewmh_atom = 3;
pub const _NET_CLOSE_WINDOW: ewmh_atom = 2;
pub const _NET_WM_ALLOWED_ACTIONS: ewmh_atom = 22;
pub const _NET_WM_ACTION_CLOSE: ewmh_atom = 29;
pub const _NET_WM_ACTION_MINIMIZE: ewmh_atom = 25;
pub const _NET_WM_ACTION_FULLSCREEN: ewmh_atom = 28;
pub const _NET_WM_ACTION_MAXIMIZE_VERT: ewmh_atom = 27;
pub const _NET_WM_ACTION_MAXIMIZE_HORZ: ewmh_atom = 26;
pub const _NET_WM_ACTION_RESIZE: ewmh_atom = 24;
pub const _NET_WM_ACTION_MOVE: ewmh_atom = 23;
pub const _NET_WM_STATE: ewmh_atom = 14;
pub const _NET_WM_STATE_MODAL: ewmh_atom = 19;
pub const _NET_WM_STATE_HIDDEN: ewmh_atom = 18;
pub const _NET_WM_STATE_MAXIMIZED_VERT: ewmh_atom = 16;
pub const _NET_WM_STATE_MAXIMIZED_HORZ: ewmh_atom = 17;
pub const _NET_WM_STATE_FULLSCREEN: ewmh_atom = 15;
pub const REMOVE: C2RustUnnamed_4 = 0;
pub const TOGGLE: C2RustUnnamed_4 = 2;
pub const ADD: C2RustUnnamed_4 = 1;
pub const _NET_CURRENT_DESKTOP: ewmh_atom = 10;
pub const _NET_CLIENT_LIST: ewmh_atom = 6;
pub const _NET_DESKTOP_VIEWPORT: ewmh_atom = 9;
pub const _NET_DESKTOP_GEOMETRY: ewmh_atom = 8;
pub const _NET_NUMBER_OF_DESKTOPS: ewmh_atom = 7;
pub const _NET_DESKTOP_NAMES: ewmh_atom = 11;
pub const _NET_WM_WINDOW_TYPE_DESKTOP: ewmh_atom = 31;
pub const _NET_WM_FULL_PLACEMENT: ewmh_atom = 37;
pub const BUFSIZ: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const SubstructureNotifyMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 19 as ::core::ffi::c_int;
pub const KeyPress: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ButtonPress: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ClientMessage: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const Success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CWX: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const CWY: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const CWWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const CWHeight: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const StaticGravity: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PropModeReplace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GCGraphicsExposures: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int;
pub const ZPixmap: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TrueColor: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DirectColor: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
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
pub const MWM_DECOR_BORDER: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const MWM_DECOR_RESIZEH: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const MWM_DECOR_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const MWM_DECOR_MINIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const MWM_INPUT_PRIMARY_APPLICATION_MODAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WM_RESOURCE_NAME: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"emwm\0") };
pub const ICON_INTERNAL_SHADOW_WIDTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STRETCH_COUNT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const WM_DECOR_BORDER: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const WM_DECOR_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const WM_DECOR_MINIMIZE: ::core::ffi::c_long = WM_DECOR_TITLE | MWM_DECOR_MINIMIZE;
pub const FRAME_RESIZE_NW: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const FRAME_RESIZE_N: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const FRAME_RESIZE_NE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const FRAME_RESIZE_E: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const FRAME_RESIZE_SE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FRAME_RESIZE_S: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const FRAME_RESIZE_SW: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const FRAME_RESIZE_W: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const IB_MARGIN_HEIGHT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const IB_MARGIN_WIDTH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAXIMIZED_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const UNSEEN_STATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NO_ACTION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut ewmh_atom_names: [*mut ::core::ffi::c_char; 38] = [
    b"_NET_SUPPORTED\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_SUPPORTING_WM_CHECK\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_CLOSE_WINDOW\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_MOVERESIZE_WINDOW\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_FRAME_EXTENTS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_ACTIVE_WINDOW\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_CLIENT_LIST\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_NUMBER_OF_DESKTOPS\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_DESKTOP_GEOMETRY\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_DESKTOP_VIEWPORT\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_CURRENT_DESKTOP\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_DESKTOP_NAMES\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_WM_NAME\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_WM_ICON_NAME\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_WM_STATE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_WM_STATE_FULLSCREEN\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_STATE_MAXIMIZED_VERT\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_STATE_MAXIMIZED_HORZ\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_STATE_HIDDEN\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_STATE_MODAL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_WM_ICON\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_WM_MOVERESIZE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_WM_ALLOWED_ACTIONS\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_ACTION_MOVE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_WM_ACTION_RESIZE\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_ACTION_MINIMIZE\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_ACTION_MAXIMIZE_HORZ\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_ACTION_MAXIMIZE_VERT\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_ACTION_FULLSCREEN\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_ACTION_CLOSE\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_FULLSCREEN_MONITORS\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_WINDOW_TYPE_DESKTOP\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"_NET_WM_WINDOW_TYPE_SPLASH\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_WINDOW_TYPE_TOOLBAR\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_WINDOW_TYPE_UTILITY\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_WINDOW_TYPE_DIALOG\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"_NET_WM_FULL_PLACEMENT\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
];
static mut ewmh_atoms: [Atom; 38] = [0; 38];
static mut XA_UTF8_STRING: Atom = 0;
#[no_mangle]
pub unsafe extern "C" fn SetupWmEwmh() {
    let mut i: ::core::ffi::c_int = 0;
    XInternAtoms(
        wmGD.display,
        &raw mut ewmh_atom_names as *mut *mut ::core::ffi::c_char,
        _NUM_EWMH_ATOMS as ::core::ffi::c_int,
        False,
        &raw mut ewmh_atoms as *mut Atom,
    );
    XA_UTF8_STRING = XInternAtom(
        wmGD.display,
        b"UTF8_STRING\0" as *const u8 as *const ::core::ffi::c_char,
        False,
    );
    i = 0 as ::core::ffi::c_int;
    while i < wmGD.numScreens {
        let mut check_wnd = XtWindowOfObject((*wmGD.Screens.offset(i as isize)).screenTopLevelW);
        XChangeProperty(
            wmGD.display,
            (*wmGD.Screens.offset(i as isize)).rootWindow,
            ewmh_atoms[_NET_SUPPORTED as ::core::ffi::c_int as usize],
            XA_ATOM,
            32 as ::core::ffi::c_int,
            PropModeReplace,
            (&raw mut ewmh_atoms as *mut Atom).offset(1 as ::core::ffi::c_int as isize) as *mut Atom
                as *mut ::core::ffi::c_uchar,
            _NUM_EWMH_ATOMS as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
        );
        XChangeProperty(
            wmGD.display,
            (*wmGD.Screens.offset(i as isize)).rootWindow,
            ewmh_atoms[_NET_SUPPORTING_WM_CHECK as ::core::ffi::c_int as usize],
            XA_WINDOW,
            32 as ::core::ffi::c_int,
            PropModeReplace,
            &raw mut check_wnd as *mut ::core::ffi::c_uchar,
            1 as ::core::ffi::c_int,
        );
        XChangeProperty(
            wmGD.display,
            check_wnd,
            ewmh_atoms[_NET_SUPPORTING_WM_CHECK as ::core::ffi::c_int as usize],
            XA_WINDOW,
            32 as ::core::ffi::c_int,
            PropModeReplace,
            &raw mut check_wnd as *mut ::core::ffi::c_uchar,
            1 as ::core::ffi::c_int,
        );
        XChangeProperty(
            wmGD.display,
            check_wnd,
            ewmh_atoms[_NET_WM_NAME as ::core::ffi::c_int as usize],
            XA_STRING,
            8 as ::core::ffi::c_int,
            PropModeReplace,
            WM_RESOURCE_NAME.as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(WM_RESOURCE_NAME.as_ptr()) as ::core::ffi::c_int,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessEwmh(mut pCD: *mut ClientData) {
    let mut sz = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut icon: Pixmap = 0;
    let mut size = 0 as ::core::ffi::c_ulong;
    sz = FetchWindowProperty(
        (*pCD).client,
        ewmh_atoms[_NET_WM_NAME as ::core::ffi::c_int as usize],
        XA_UTF8_STRING,
        &raw mut size,
    ) as *mut ::core::ffi::c_char;
    if size != 0 {
        if !(*pCD).ewmhClientTitle.is_null() {
            XmStringFree((*pCD).ewmhClientTitle);
        }
        (*pCD).ewmhClientTitle = XmStringGenerate(
            sz as XtPointer,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            XmMULTIBYTE_TEXT,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
    }
    if !sz.is_null() {
        XFree(sz as *mut ::core::ffi::c_void);
    }
    sz = FetchWindowProperty(
        (*pCD).client,
        ewmh_atoms[_NET_WM_ICON_NAME as ::core::ffi::c_int as usize],
        XA_UTF8_STRING,
        &raw mut size,
    ) as *mut ::core::ffi::c_char;
    if size != 0 {
        if !(*pCD).ewmhIconTitle.is_null() {
            XmStringFree((*pCD).ewmhIconTitle);
        }
        (*pCD).ewmhIconTitle = XmStringGenerate(
            sz as XtPointer,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            XmMULTIBYTE_TEXT,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
    } else if !(*pCD).ewmhClientTitle.is_null() {
        (*pCD).ewmhIconTitle = XmStringCopy((*pCD).ewmhClientTitle);
    }
    if !sz.is_null() {
        XFree(sz as *mut ::core::ffi::c_void);
    }
    if (*pCD).ewmhIconPixmap != 0 {
        XFreePixmap(wmGD.display, (*pCD).ewmhIconPixmap);
    }
    (*pCD).ewmhIconPixmap = None_0 as Pixmap;
    if (*pCD).useClientIcon as ::core::ffi::c_int != 0 || (*pCD).iconImage.is_null() {
        icon = GetIconPixmap(pCD);
        if icon != 0 {
            (*pCD).ewmhIconPixmap = MakeClientIconPixmap(pCD, icon, None_0 as Pixmap);
            XFreePixmap(wmGD.display, icon);
        }
    }
    UpdateFrameExtents(pCD);
}
#[no_mangle]
pub unsafe extern "C" fn ProcessEwmhWindowType(mut pCD: *mut ClientData) {
    let mut atoms = ::core::ptr::null_mut::<Atom>();
    let mut count: ::core::ffi::c_ulong = 0;
    atoms = FetchWindowProperty(
        (*pCD).client,
        ewmh_atoms[_NET_WM_WINDOW_TYPE as ::core::ffi::c_int as usize],
        XA_ATOM,
        &raw mut count,
    ) as *mut Atom;
    if !atoms.is_null() {
        if *atoms.offset(0 as ::core::ffi::c_int as isize)
            == ewmh_atoms[_NET_WM_WINDOW_TYPE_TOOLBAR as ::core::ffi::c_int as usize]
            || *atoms.offset(0 as ::core::ffi::c_int as isize)
                == ewmh_atoms[_NET_WM_WINDOW_TYPE_UTILITY as ::core::ffi::c_int as usize]
        {
            (*pCD).clientDecoration = (*(*pCD).pSD).utilityDecoration;
            (*pCD).clientFunctions = (*(*pCD).pSD).utilityFunctions;
        } else if *atoms.offset(0 as ::core::ffi::c_int as isize)
            == ewmh_atoms[_NET_WM_WINDOW_TYPE_DIALOG as ::core::ffi::c_int as usize]
        {
            (*pCD).clientDecoration = (*(*pCD).pSD).transientDecoration;
            (*pCD).clientFunctions = (*(*pCD).pSD).transientFunctions;
        } else if *atoms.offset(0 as ::core::ffi::c_int as isize)
            == ewmh_atoms[_NET_WM_WINDOW_TYPE_SPLASH as ::core::ffi::c_int as usize]
        {
            (*pCD).clientDecoration = (WM_DECOR_MINIMIZE | WM_DECOR_BORDER) as ::core::ffi::c_int;
            (*pCD).clientFunctions = (MWM_FUNC_MINIMIZE | MWM_FUNC_MOVE) as ::core::ffi::c_int;
        }
        XtFree(atoms as *mut ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleEwmhCPropertyNotify(
    mut pCD: *mut ClientData,
    mut evt: *mut XPropertyEvent,
) {
    let mut size = 0 as ::core::ffi::c_ulong;
    if (*evt).atom == ewmh_atoms[_NET_WM_NAME as ::core::ffi::c_int as usize] {
        let mut sz = FetchWindowProperty(
            (*pCD).client,
            ewmh_atoms[_NET_WM_NAME as ::core::ffi::c_int as usize],
            XA_UTF8_STRING,
            &raw mut size,
        ) as *mut ::core::ffi::c_char;
        if size != 0 {
            if !(*pCD).ewmhClientTitle.is_null() {
                XmStringFree((*pCD).ewmhClientTitle);
            }
            (*pCD).ewmhClientTitle = XmStringGenerate(
                sz as XtPointer,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                XmMULTIBYTE_TEXT,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            DrawWindowTitle(pCD, True as Boolean);
        }
        if !sz.is_null() {
            XFree(sz as *mut ::core::ffi::c_void);
        }
    } else if (*evt).atom == ewmh_atoms[_NET_WM_ICON_NAME as ::core::ffi::c_int as usize] {
        let mut sz_0 = FetchWindowProperty(
            (*pCD).client,
            ewmh_atoms[_NET_WM_ICON_NAME as ::core::ffi::c_int as usize],
            XA_UTF8_STRING,
            &raw mut size,
        ) as *mut ::core::ffi::c_char;
        if size != 0 {
            if !(*pCD).ewmhIconTitle.is_null() {
                XmStringFree((*pCD).ewmhIconTitle);
            }
            (*pCD).ewmhIconTitle = XmStringGenerate(
                sz_0 as XtPointer,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                XmMULTIBYTE_TEXT,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            RedisplayIconTitle(pCD);
        } else if !(*pCD).ewmhClientTitle.is_null() {
            (*pCD).ewmhIconTitle = XmStringCopy((*pCD).ewmhClientTitle);
            RedisplayIconTitle(pCD);
        }
        if !sz_0.is_null() {
            XFree(sz_0 as *mut ::core::ffi::c_void);
        }
    } else if (*evt).atom == ewmh_atoms[_NET_WM_ICON as ::core::ffi::c_int as usize] {
        let mut icon: Pixmap = 0;
        if (*pCD).ewmhIconPixmap != 0 {
            XFreePixmap(wmGD.display, (*pCD).ewmhIconPixmap);
        }
        (*pCD).ewmhIconPixmap = None_0 as Pixmap;
        if (*pCD).useClientIcon as ::core::ffi::c_int != 0 || (*pCD).iconImage.is_null() {
            icon = GetIconPixmap(pCD);
            if icon != 0 {
                (*pCD).ewmhIconPixmap = MakeClientIconPixmap(pCD, icon, None_0 as Pixmap);
                XFreePixmap(wmGD.display, icon);
                XClearArea(
                    wmGD.display,
                    (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
                    IB_MARGIN_WIDTH,
                    IB_MARGIN_HEIGHT,
                    (*(*pCD).pSD).iconWidth as ::core::ffi::c_uint,
                    (*(*pCD).pSD).iconHeight as ::core::ffi::c_uint,
                    True,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleEwmhClientMessage(
    mut pCD: *mut ClientData,
    mut evt: *mut XClientMessageEvent,
) {
    if (*evt).message_type == ewmh_atoms[_NET_WM_STATE as ::core::ffi::c_int as usize] {
        let mut action = (*evt).data.l[0 as ::core::ffi::c_int as usize] as C2RustUnnamed_4;
        if (*evt).data.l[1 as ::core::ffi::c_int as usize] as Atom
            == ewmh_atoms[_NET_WM_STATE_FULLSCREEN as ::core::ffi::c_int as usize]
        {
            let mut set = False as Boolean;
            match action as ::core::ffi::c_uint {
                2 => {
                    set = (if (*pCD).fullScreen as ::core::ffi::c_int != 0 {
                        False
                    } else {
                        True
                    }) as Boolean;
                }
                1 => {
                    set = True as Boolean;
                }
                0 => {
                    set = False as Boolean;
                }
                _ => {}
            }
            ConfigureEwmhFullScreen(pCD, set);
        } else if (*evt).data.l[1 as ::core::ffi::c_int as usize] as Atom
            == ewmh_atoms[_NET_WM_STATE_MAXIMIZED_VERT as ::core::ffi::c_int as usize]
            && (*evt).data.l[2 as ::core::ffi::c_int as usize] as Atom
                == ewmh_atoms[_NET_WM_STATE_MAXIMIZED_HORZ as ::core::ffi::c_int as usize]
        {
            match action as ::core::ffi::c_uint {
                1 => {
                    if (*pCD).clientState != MAXIMIZED_STATE {
                        F_Maximize(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            pCD,
                            evt as *mut XEvent,
                        );
                    }
                }
                2 => {
                    if (*pCD).clientState == MAXIMIZED_STATE {
                        F_Restore(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            pCD,
                            evt as *mut XEvent,
                        );
                    } else {
                        F_Maximize(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            pCD,
                            evt as *mut XEvent,
                        );
                    }
                }
                0 => {
                    if (*pCD).clientState == MAXIMIZED_STATE {
                        F_Restore(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            pCD,
                            evt as *mut XEvent,
                        );
                    }
                }
                _ => {}
            }
            UpdateEwmhClientState(pCD);
        } else if (*evt).data.l[1 as ::core::ffi::c_int as usize] as Atom
            == ewmh_atoms[_NET_WM_STATE_HIDDEN as ::core::ffi::c_int as usize]
        {
            match action as ::core::ffi::c_uint {
                1 => {
                    if (*pCD).clientState != MINIMIZED_STATE {
                        F_Minimize(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            pCD,
                            evt as *mut XEvent,
                        );
                    }
                }
                2 => {
                    if (*pCD).clientState == MINIMIZED_STATE {
                        F_Restore(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            pCD,
                            evt as *mut XEvent,
                        );
                    } else {
                        F_Minimize(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            pCD,
                            evt as *mut XEvent,
                        );
                    }
                }
                0 => {
                    if (*pCD).clientState == MINIMIZED_STATE {
                        F_Restore(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            pCD,
                            evt as *mut XEvent,
                        );
                    }
                }
                _ => {}
            }
            UpdateEwmhClientState(pCD);
        }
    } else if (*evt).message_type == ewmh_atoms[_NET_ACTIVE_WINDOW as ::core::ffi::c_int as usize] {
        // A pager selects a particular window, including minimized clients.
        if (*evt).data.l[0] == 2 {
            if (*pCD).clientState == MINIMIZED_STATE {
                F_Restore(::core::ptr::null_mut(), pCD, ::core::ptr::null_mut());
            }
            F_Raise(::core::ptr::null_mut(), pCD, ::core::ptr::null_mut());
        }
        F_Focus_Key(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pCD,
            ::core::ptr::null_mut::<XEvent>(),
        );
    } else if (*evt).message_type == ewmh_atoms[_NET_CLOSE_WINDOW as ::core::ffi::c_int as usize] {
        F_Kill(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pCD,
            ::core::ptr::null_mut::<XEvent>(),
        );
    } else if (*evt).message_type
        == ewmh_atoms[_NET_MOVERESIZE_WINDOW as ::core::ffi::c_int as usize]
    {
        let mut gravity = ((*evt).data.l[0 as ::core::ffi::c_int as usize]
            & 0xff as ::core::ffi::c_long) as ::core::ffi::c_int;
        let mut mask = (((*evt).data.l[0 as ::core::ffi::c_int as usize]
            & 0xf00 as ::core::ffi::c_long)
            >> 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let mut src = (((*evt).data.l[0 as ::core::ffi::c_int as usize]
            & 0xf000 as ::core::ffi::c_long)
            >> 12 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if mask & (CWX | CWY | CWWidth | CWHeight) == 0 && src == 1 as ::core::ffi::c_int {
            SendConfigureNotify(pCD);
            return;
        }
        let mut cx = (if mask & CWX != 0 {
            (*evt).data.l[1 as ::core::ffi::c_int as usize]
        } else {
            (if (*pCD).maxConfig as ::core::ffi::c_int != 0 {
                (*pCD).maxX
            } else {
                (*pCD).clientX
            }) as ::core::ffi::c_long
        }) as ::core::ffi::c_int;
        let mut cy = (if mask & CWY != 0 {
            (*evt).data.l[2 as ::core::ffi::c_int as usize]
        } else {
            (if (*pCD).maxConfig as ::core::ffi::c_int != 0 {
                (*pCD).maxY
            } else {
                (*pCD).clientY
            }) as ::core::ffi::c_long
        }) as ::core::ffi::c_int;
        let mut width = (if mask & CWWidth != 0 {
            (*evt).data.l[3 as ::core::ffi::c_int as usize]
        } else {
            (*pCD).clientWidth as ::core::ffi::c_long
        }) as ::core::ffi::c_uint;
        let mut height = (if mask & CWHeight != 0 {
            (*evt).data.l[4 as ::core::ffi::c_int as usize]
        } else {
            (*pCD).clientHeight as ::core::ffi::c_long
        }) as ::core::ffi::c_uint;
        if (*pCD).windowGravity != StaticGravity {
            if mask & CWX == 0 {
                cx -= (*pCD).clientOffset.x as ::core::ffi::c_int;
            }
            if mask & CWY == 0 {
                cy -= (*pCD).clientOffset.y as ::core::ffi::c_int;
            }
        }
        AdjustCoordinatesToGravity(
            pCD,
            if gravity != 0 {
                gravity
            } else {
                (*pCD).windowGravity
            },
            &raw mut cx,
            &raw mut cy,
            &raw mut width,
            &raw mut height,
        );
        ProcessNewConfiguration(pCD, cx, cy, width, height, True as Boolean);
    } else if (*evt).message_type == ewmh_atoms[_NET_WM_MOVERESIZE as ::core::ffi::c_int as usize] {
        ProcessMoveResize(
            pCD,
            (*evt).data.l[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            (*evt).data.l[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            (*evt).data.l[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            (*evt).data.l[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            (*evt).data.l[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        );
    } else if (*evt).message_type
        == ewmh_atoms[_NET_WM_FULLSCREEN_MONITORS as ::core::ffi::c_int as usize]
    {
        (*pCD).fullScreenXineramaIndices[0 as ::core::ffi::c_int as usize] =
            (*evt).data.l[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        (*pCD).fullScreenXineramaIndices[1 as ::core::ffi::c_int as usize] =
            (*evt).data.l[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        (*pCD).fullScreenXineramaIndices[2 as ::core::ffi::c_int as usize] =
            (*evt).data.l[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        (*pCD).fullScreenXineramaIndices[3 as ::core::ffi::c_int as usize] =
            (*evt).data.l[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn ProcessMoveResize(
    mut pCD: *mut ClientData,
    mut x_root: ::core::ffi::c_int,
    mut y_root: ::core::ffi::c_int,
    mut dir: ::core::ffi::c_int,
    mut button: ::core::ffi::c_int,
    mut source: ::core::ffi::c_int,
) {
    let mut evt = _XEvent {
        type_0: 0 as ::core::ffi::c_int,
    };
    if dir == CANCEL as ::core::ffi::c_int {
        if wmGD.configAction != NO_ACTION {
            CancelFrameConfig(pCD);
        }
        return;
    }
    if !((*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE) {
        return;
    }
    if dir == MOVE as ::core::ffi::c_int || dir == MOVE_KEYBOARD as ::core::ffi::c_int {
        if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MOVE == 0 {
            return;
        }
    } else if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_RESIZE == 0 {
        return;
    }
    if dir == SIZE_KEYBOARD as ::core::ffi::c_int {
        evt.type_0 = KeyPress;
        StartClientResize(pCD, &raw mut evt);
        HandleClientFrameResize(pCD, &raw mut evt);
    } else if dir == MOVE_KEYBOARD as ::core::ffi::c_int {
        evt.type_0 = KeyPress;
        StartClientMove(pCD, &raw mut evt);
        HandleClientFrameMove(pCD, &raw mut evt);
    } else {
        evt.type_0 = ButtonPress;
        evt.xbutton.button = button as ::core::ffi::c_uint;
        evt.xbutton.x_root = x_root;
        evt.xbutton.y_root = y_root;
        match dir {
            0 => {
                wmGD.configPart = FRAME_RESIZE_NW as ::core::ffi::c_uint;
            }
            1 => {
                wmGD.configPart = FRAME_RESIZE_N as ::core::ffi::c_uint;
            }
            2 => {
                wmGD.configPart = FRAME_RESIZE_NE as ::core::ffi::c_uint;
            }
            3 => {
                wmGD.configPart = FRAME_RESIZE_E as ::core::ffi::c_uint;
            }
            4 => {
                wmGD.configPart = FRAME_RESIZE_SE as ::core::ffi::c_uint;
            }
            5 => {
                wmGD.configPart = FRAME_RESIZE_S as ::core::ffi::c_uint;
            }
            6 => {
                wmGD.configPart = FRAME_RESIZE_SW as ::core::ffi::c_uint;
            }
            7 => {
                wmGD.configPart = FRAME_RESIZE_W as ::core::ffi::c_uint;
            }
            _ => {}
        }
        if dir == MOVE as ::core::ffi::c_int {
            StartClientMove(pCD, &raw mut evt);
            HandleClientFrameMove(pCD, &raw mut evt);
        } else {
            StartResizeConfig(pCD, &raw mut evt);
            HandleClientFrameResize(pCD, &raw mut evt);
        }
    };
}
unsafe extern "C" fn GetMultiscreenCoords(
    mut pCD: *mut ClientData,
    mut xorg: *mut ::core::ffi::c_int,
    mut yorg: *mut ::core::ffi::c_int,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) -> Boolean {
    let mut xsi: [XineramaScreenInfo; 4] = [XineramaScreenInfo {
        screen_number: 0,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    }; 4];
    let mut i: ::core::ffi::c_int = 0;
    if (*pCD).fullScreenXineramaIndices[0 as ::core::ffi::c_int as usize]
        == -(1 as ::core::ffi::c_int)
    {
        return False as Boolean;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        if GetXineramaScreenInfo(
            (*pCD).fullScreenXineramaIndices[i as usize],
            (&raw mut xsi as *mut XineramaScreenInfo).offset(i as isize) as *mut XineramaScreenInfo,
        ) == 0
        {
            return False as Boolean;
        }
        i += 1;
    }
    *yorg = xsi[0 as ::core::ffi::c_int as usize].y_org as ::core::ffi::c_int;
    *height = xsi[1 as ::core::ffi::c_int as usize].y_org as ::core::ffi::c_int
        + xsi[1 as ::core::ffi::c_int as usize].height as ::core::ffi::c_int;
    *xorg = xsi[2 as ::core::ffi::c_int as usize].x_org as ::core::ffi::c_int;
    *width = xsi[3 as ::core::ffi::c_int as usize].x_org as ::core::ffi::c_int
        + xsi[3 as ::core::ffi::c_int as usize].width as ::core::ffi::c_int;
    return True as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ConfigureEwmhFullScreen(mut pCD: *mut ClientData, mut set: Boolean) {
    let mut xorg = 0 as ::core::ffi::c_int;
    let mut yorg = 0 as ::core::ffi::c_int;
    let mut swidth: ::core::ffi::c_int = 0;
    let mut sheight: ::core::ffi::c_int = 0;
    let mut xsi = XineramaScreenInfo {
        screen_number: 0,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    };
    let mut i: ::core::ffi::c_int = 0;
    if set as ::core::ffi::c_int == (*pCD).fullScreen as ::core::ffi::c_int {
        return;
    }
    if GetMultiscreenCoords(
        pCD,
        &raw mut xorg,
        &raw mut yorg,
        &raw mut swidth,
        &raw mut sheight,
    ) == 0
    {
        if GetXineramaScreenFromLocation((*pCD).clientX, (*pCD).clientY, &raw mut xsi) != 0 {
            xorg = xsi.x_org as ::core::ffi::c_int;
            yorg = xsi.y_org as ::core::ffi::c_int;
            swidth = xsi.width as ::core::ffi::c_int;
            sheight = xsi.height as ::core::ffi::c_int;
        } else {
            swidth = XDisplayWidth(wmGD.display, (*(*pCD).pSD).screen);
            sheight = XDisplayHeight(wmGD.display, (*(*pCD).pSD).screen);
        }
    }
    if set != 0 {
        (*pCD).normalClientFunctions = (*pCD).clientFunctions;
        (*pCD).fullScreenWidth = swidth;
        (*pCD).fullScreenHeight = sheight;
        (*pCD).fullScreenX = xorg;
        (*pCD).fullScreenY = yorg;
        (*pCD).clientFunctions = ((*pCD).clientFunctions as ::core::ffi::c_long
            & !(MWM_FUNC_RESIZE | MWM_FUNC_MOVE | MWM_FUNC_MAXIMIZE))
            as ::core::ffi::c_int;
        if (*pCD).decor & MWM_DECOR_TITLE != 0 {
            XUnmapWindow(wmGD.display, (*pCD).clientTitleWin);
        }
        if (*pCD).decor & MWM_DECOR_RESIZEH != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < STRETCH_COUNT {
                XUnmapWindow(wmGD.display, (*pCD).clientStretchWin[i as usize]);
                i += 1;
            }
        }
        XMoveResizeWindow(
            wmGD.display,
            (*pCD).clientFrameWin,
            xorg,
            yorg,
            swidth as ::core::ffi::c_uint,
            sheight as ::core::ffi::c_uint,
        );
        XMoveResizeWindow(
            wmGD.display,
            (*pCD).clientBaseWin,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            swidth as ::core::ffi::c_uint,
            sheight as ::core::ffi::c_uint,
        );
        XMoveResizeWindow(
            wmGD.display,
            (*pCD).client,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            swidth as ::core::ffi::c_uint,
            sheight as ::core::ffi::c_uint,
        );
        (*pCD).fullScreen = True as Boolean;
        UpdateEwmhClientState(pCD);
    } else {
        (*pCD).clientFunctions = (*pCD).normalClientFunctions;
        XMoveResizeWindow(
            wmGD.display,
            (*pCD).client,
            (*pCD).matteWidth,
            (*pCD).matteWidth,
            (*pCD).clientWidth as ::core::ffi::c_uint,
            (*pCD).clientHeight as ::core::ffi::c_uint,
        );
        XMoveResizeWindow(
            wmGD.display,
            (*pCD).clientBaseWin,
            BaseWindowX(pCD),
            BaseWindowY(pCD),
            BaseWindowWidth(pCD),
            BaseWindowHeight(pCD),
        );
        if (*pCD).maxConfig != 0 {
            XResizeWindow(
                wmGD.display,
                (*pCD).client,
                (*pCD).maxWidth as ::core::ffi::c_uint,
                (*pCD).maxHeight as ::core::ffi::c_uint,
            );
        } else {
            XResizeWindow(
                wmGD.display,
                (*pCD).client,
                (*pCD).clientWidth as ::core::ffi::c_uint,
                (*pCD).clientHeight as ::core::ffi::c_uint,
            );
        }
        RegenerateClientFrame(pCD);
        if (*pCD).decor & MWM_DECOR_TITLE != 0 {
            XMapWindow(wmGD.display, (*pCD).clientTitleWin);
        }
        if (*pCD).decor & MWM_DECOR_RESIZEH != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < STRETCH_COUNT {
                XMapWindow(wmGD.display, (*pCD).clientStretchWin[i as usize]);
                i += 1;
            }
        }
        (*pCD).fullScreen = False as Boolean;
        UpdateEwmhClientState(pCD);
    };
}
#[no_mangle]
pub unsafe extern "C" fn UpdateEwmhWorkspaceProperties(mut pSD: *mut WmScreenData) {
    let mut i: ::core::ffi::c_int = 0;
    let mut nws = (*pSD).numWorkspaces as ::core::ffi::c_ulong;
    let mut ws_geom: [::core::ffi::c_ulong; 2] = [0; 2];
    let mut names = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut names_len: size_t = 0 as size_t;
    i = 0 as ::core::ffi::c_int;
    while i < (*pSD).numWorkspaces {
        names_len = names_len.wrapping_add(
            (XmStringLength((*(*pSD).pWS.offset(i as isize)).title) + 1 as ::core::ffi::c_int)
                as size_t,
        );
        i += 1;
    }
    names = malloc(names_len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if !names.is_null() {
        let mut ptr = names;
        names_len = 0 as size_t;
        i = 0 as ::core::ffi::c_int;
        while i < (*pSD).numWorkspaces {
            let mut len: size_t = 0;
            let mut title_sz = XmStringUnparse(
                (*(*pSD).pWS.offset(i as isize)).title,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                XmMULTIBYTE_TEXT,
                XmMULTIBYTE_TEXT,
                ::core::ptr::null_mut::<XmParseMapping>(),
                0 as Cardinal,
                XmOUTPUT_ALL,
            ) as *mut ::core::ffi::c_char;
            len = strlen(title_sz);
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                title_sz as *const ::core::ffi::c_void,
                len,
            );
            ptr = ptr.offset(len as isize);
            *ptr.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
            ptr = ptr.offset(1);
            names_len = names_len.wrapping_add(len.wrapping_add(1 as size_t));
            XtFree(title_sz);
            i += 1;
        }
    } else {
        Warning(
            b"Failed to set EWMH workspace names. Memory allocation error.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        names_len = 1 as size_t;
        names = strdup(b"\0\0" as *const u8 as *const ::core::ffi::c_char);
    }
    XChangeProperty(
        wmGD.display,
        (*pSD).rootWindow,
        ewmh_atoms[_NET_DESKTOP_NAMES as ::core::ffi::c_int as usize],
        XA_UTF8_STRING,
        8 as ::core::ffi::c_int,
        PropModeReplace,
        names as *mut ::core::ffi::c_uchar,
        names_len as ::core::ffi::c_int,
    );
    XChangeProperty(
        wmGD.display,
        (*pSD).rootWindow,
        ewmh_atoms[_NET_NUMBER_OF_DESKTOPS as ::core::ffi::c_int as usize],
        XA_CARDINAL,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut nws as *mut ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int,
    );
    ws_geom[0 as ::core::ffi::c_int as usize] = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*pSD).screen as isize))
    .width as ::core::ffi::c_ulong;
    ws_geom[1 as ::core::ffi::c_int as usize] = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*pSD).screen as isize))
    .height as ::core::ffi::c_ulong;
    XChangeProperty(
        wmGD.display,
        (*pSD).rootWindow,
        ewmh_atoms[_NET_DESKTOP_GEOMETRY as ::core::ffi::c_int as usize],
        XA_CARDINAL,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut ws_geom as *mut ::core::ffi::c_ulong as *mut ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int,
    );
    ws_geom[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_ulong;
    ws_geom[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_ulong;
    XChangeProperty(
        wmGD.display,
        (*pSD).rootWindow,
        ewmh_atoms[_NET_DESKTOP_VIEWPORT as ::core::ffi::c_int as usize],
        XA_CARDINAL,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut ws_geom as *mut ::core::ffi::c_ulong as *mut ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int,
    );
    free(names as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn UpdateEwmhActiveWorkspace(
    mut pSD: *mut WmScreenData,
    mut id: WorkspaceID,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut data: ::core::ffi::c_ulong = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pSD).numWorkspaces {
        if (*(*pSD).pWS.offset(i as isize)).id == id {
            break;
        }
        i += 1;
    }
    data = i as ::core::ffi::c_ulong;
    XChangeProperty(
        wmGD.display,
        (*pSD).rootWindow,
        ewmh_atoms[_NET_CURRENT_DESKTOP as ::core::ffi::c_int as usize],
        XA_CARDINAL,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut data as *mut ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HandleEwmhRootClientMessage(
    mut pSD: *mut WmScreenData,
    mut evt: *mut XClientMessageEvent,
) -> Boolean {
    if (*evt).message_type == ewmh_atoms[_NET_CURRENT_DESKTOP as ::core::ffi::c_int as usize] {
        let mut wsi = (*evt).data.l[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        if wsi < (*pSD).numWorkspaces {
            ChangeToWorkspace((*pSD).pWS.offset(wsi as isize) as *mut WmWorkspaceData);
        } else {
            Warning(
                b"Invalid workspace index in _NET_CURRENT_DESKTOP request.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        return True as Boolean;
    }
    return False as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn SetEwmhActiveWindow(mut pCD: *mut ClientData) {
    XChangeProperty(
        wmGD.display,
        (*(*pCD).pSD).rootWindow,
        ewmh_atoms[_NET_ACTIVE_WINDOW as ::core::ffi::c_int as usize],
        XA_WINDOW,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut (*pCD).client as *mut ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn UpdateFrameExtents(mut pCD: *mut ClientData) {
    let mut data: [::core::ffi::c_ulong; 4] = [0; 4];
    data[3 as ::core::ffi::c_int as usize] = (if (*pCD).decor & MWM_DECOR_BORDER != 0 {
        LowerBorderWidth(pCD)
    } else {
        0 as ::core::ffi::c_uint
    }) as ::core::ffi::c_ulong;
    data[1 as ::core::ffi::c_int as usize] = data[3 as ::core::ffi::c_int as usize];
    data[0 as ::core::ffi::c_int as usize] = data[1 as ::core::ffi::c_int as usize];
    data[2 as ::core::ffi::c_int as usize] = (if (*pCD).decor & MWM_DECOR_TITLE != 0 {
        (*pCD).frameInfo.titleBarHeight
    } else {
        0 as ::core::ffi::c_uint
    })
    .wrapping_add(
        (if (*pCD).decor & MWM_DECOR_BORDER != 0 {
            UpperBorderWidth(pCD)
        } else {
            0 as ::core::ffi::c_uint
        }),
    ) as ::core::ffi::c_ulong;
    XChangeProperty(
        wmGD.display,
        (*pCD).client,
        ewmh_atoms[_NET_FRAME_EXTENTS as ::core::ffi::c_int as usize],
        XA_CARDINAL,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut data as *mut ::core::ffi::c_ulong as *mut ::core::ffi::c_uchar,
        4 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn UpdateEwmhClientList(mut pSD: *mut WmScreenData) {
    let mut e = (*pSD).clientList;
    let mut wlist = ::core::ptr::null_mut::<Window>();
    let mut n = 0 as ::core::ffi::c_uint;
    if e.is_null() {
        XChangeProperty(
            wmGD.display,
            (*pSD).rootWindow,
            ewmh_atoms[_NET_CLIENT_LIST as ::core::ffi::c_int as usize],
            XA_WINDOW,
            32 as ::core::ffi::c_int,
            PropModeReplace,
            ::core::ptr::null::<::core::ffi::c_uchar>(),
            0 as ::core::ffi::c_int,
        );
        return;
    }
    while !e.is_null() {
        if (*e).type_0 != MINIMIZED_STATE {
            n = n.wrapping_add(1);
        }
        e = (*e).nextSibling as *mut ClientListEntry;
    }
    wlist = malloc((::core::mem::size_of::<Window>() as size_t).wrapping_mul(n as size_t))
        as *mut Window;
    if wlist.is_null() {
        Warning(
            b"Failed to allocate memory for _NET_CLIENT_LIST\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    e = (*pSD).clientList;
    n = 0 as ::core::ffi::c_uint;
    while !e.is_null() {
        if (*e).type_0 != MINIMIZED_STATE {
            *wlist.offset(n as isize) = (*(*e).pCD).client;
            n = n.wrapping_add(1);
        }
        e = (*e).nextSibling as *mut ClientListEntry;
    }
    XChangeProperty(
        wmGD.display,
        (*pSD).rootWindow,
        ewmh_atoms[_NET_CLIENT_LIST as ::core::ffi::c_int as usize],
        XA_WINDOW,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        wlist as *mut ::core::ffi::c_uchar,
        n as ::core::ffi::c_int,
    );
    free(wlist as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn UpdateEwmhClientState(mut pCD: *mut ClientData) {
    let mut state: [Atom; 4] = [0; 4];
    let mut actions: [Atom; 10] = [0; 10];
    let mut i = 0 as ::core::ffi::c_uint;
    if (*pCD).fullScreen as ::core::ffi::c_int != 0 && (*pCD).clientState != MINIMIZED_STATE {
        let fresh0 = i;
        i = i.wrapping_add(1);
        state[fresh0 as usize] =
            ewmh_atoms[_NET_WM_STATE_FULLSCREEN as ::core::ffi::c_int as usize];
    } else if (*pCD).clientState == MAXIMIZED_STATE {
        let fresh1 = i;
        i = i.wrapping_add(1);
        state[fresh1 as usize] =
            ewmh_atoms[_NET_WM_STATE_MAXIMIZED_HORZ as ::core::ffi::c_int as usize];
        let fresh2 = i;
        i = i.wrapping_add(1);
        state[fresh2 as usize] =
            ewmh_atoms[_NET_WM_STATE_MAXIMIZED_VERT as ::core::ffi::c_int as usize];
    } else if (*pCD).clientState == MINIMIZED_STATE || (*pCD).clientState & UNSEEN_STATE != 0 {
        let fresh3 = i;
        i = i.wrapping_add(1);
        state[fresh3 as usize] = ewmh_atoms[_NET_WM_STATE_HIDDEN as ::core::ffi::c_int as usize];
    }
    if (*pCD).inputMode == MWM_INPUT_PRIMARY_APPLICATION_MODAL {
        let fresh4 = i;
        i = i.wrapping_add(1);
        state[fresh4 as usize] = ewmh_atoms[_NET_WM_STATE_MODAL as ::core::ffi::c_int as usize];
    }
    XChangeProperty(
        wmGD.display,
        (*pCD).client,
        ewmh_atoms[_NET_WM_STATE as ::core::ffi::c_int as usize],
        XA_ATOM,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut state as *mut Atom as *mut ::core::ffi::c_uchar,
        i as ::core::ffi::c_int,
    );
    i = 0 as ::core::ffi::c_uint;
    if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MOVE != 0 {
        let fresh5 = i;
        i = i.wrapping_add(1);
        actions[fresh5 as usize] = ewmh_atoms[_NET_WM_ACTION_MOVE as ::core::ffi::c_int as usize];
    }
    if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_RESIZE != 0 {
        let fresh6 = i;
        i = i.wrapping_add(1);
        actions[fresh6 as usize] = ewmh_atoms[_NET_WM_ACTION_RESIZE as ::core::ffi::c_int as usize];
    }
    if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MAXIMIZE != 0 {
        let fresh7 = i;
        i = i.wrapping_add(1);
        actions[fresh7 as usize] =
            ewmh_atoms[_NET_WM_ACTION_MAXIMIZE_HORZ as ::core::ffi::c_int as usize];
        let fresh8 = i;
        i = i.wrapping_add(1);
        actions[fresh8 as usize] =
            ewmh_atoms[_NET_WM_ACTION_MAXIMIZE_VERT as ::core::ffi::c_int as usize];
        let fresh9 = i;
        i = i.wrapping_add(1);
        actions[fresh9 as usize] =
            ewmh_atoms[_NET_WM_ACTION_FULLSCREEN as ::core::ffi::c_int as usize];
    }
    if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0 {
        let fresh10 = i;
        i = i.wrapping_add(1);
        actions[fresh10 as usize] =
            ewmh_atoms[_NET_WM_ACTION_MINIMIZE as ::core::ffi::c_int as usize];
    }
    if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0 {
        let fresh11 = i;
        i = i.wrapping_add(1);
        actions[fresh11 as usize] = ewmh_atoms[_NET_WM_ACTION_CLOSE as ::core::ffi::c_int as usize];
    }
    XChangeProperty(
        wmGD.display,
        (*pCD).client,
        ewmh_atoms[_NET_WM_ALLOWED_ACTIONS as ::core::ffi::c_int as usize],
        XA_ATOM,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut actions as *mut Atom as *mut ::core::ffi::c_uchar,
        i as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn GetIconPixmap(mut pCD: *const ClientData) -> Pixmap {
    let mut prop_data = ::core::ptr::null_mut::<::core::ffi::c_ulong>();
    let mut rgb_data = ::core::ptr::null_mut::<::core::ffi::c_ulong>();
    let mut prop_data_size: ::core::ffi::c_ulong = 0;
    let mut rgb_width = 0 as ::core::ffi::c_uint;
    let mut rgb_height = 0 as ::core::ffi::c_uint;
    let mut dest_img = ::core::ptr::null_mut::<XImage>();
    let mut dest_img_data = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut visual = ::core::ptr::null_mut::<Visual>();
    let mut depth: ::core::ffi::c_int = 0;
    let mut bg = XColor {
        pixel: 0 as ::core::ffi::c_ulong,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    let mut pixmap = None_0 as Pixmap;
    let mut red_shift = 0 as ::core::ffi::c_ushort;
    let mut green_shift = 0 as ::core::ffi::c_ushort;
    let mut blue_shift = 0 as ::core::ffi::c_ushort;
    let mut fptr = C2RustUnnamed_2 {
        i: ::core::ptr::null_mut::<::core::ffi::c_ulong>(),
    };
    let mut x: ::core::ffi::c_uint = 0;
    let mut y: ::core::ffi::c_uint = 0;
    let mut dx: ::core::ffi::c_float = 0.;
    let mut dy: ::core::ffi::c_float = 0.;
    let mut icon_width = ((*(*pCD).pSD).iconImageMaximum.width
        - (2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH
            + (if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }))) as ::core::ffi::c_uint;
    let mut icon_height = ((*(*pCD).pSD).iconImageMaximum.height
        - (2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH
            + (if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }))) as ::core::ffi::c_uint;
    visual = XDefaultVisual(wmGD.display, (*(*pCD).pSD).screen);
    depth = XDefaultDepth(wmGD.display, (*(*pCD).pSD).screen);
    if (*visual).class != TrueColor && (*visual).class != DirectColor {
        return None_0 as Pixmap;
    }
    prop_data = FetchWindowProperty(
        (*pCD).client,
        ewmh_atoms[_NET_WM_ICON as ::core::ffi::c_int as usize],
        XA_CARDINAL,
        &raw mut prop_data_size,
    ) as *mut ::core::ffi::c_ulong;
    if prop_data.is_null() {
        return None_0 as Pixmap;
    }
    if prop_data_size < 2 as ::core::ffi::c_ulong {
        XFree(prop_data as *mut ::core::ffi::c_void);
        return None_0 as Pixmap;
    }
    fptr.i = prop_data;
    rgb_width = *fptr.i.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    rgb_height = *fptr.i.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    while rgb_width < icon_width || rgb_height < icon_height {
        if rgb_width == 0
            || rgb_height == 0
            || fptr
                .i
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(rgb_width.wrapping_mul(rgb_height) as isize)
                >= prop_data.offset(prop_data_size as isize)
        {
            XFree(prop_data as *mut ::core::ffi::c_void);
            return None_0 as Pixmap;
        }
        fptr.i = fptr.i.offset(
            (2 as ::core::ffi::c_uint).wrapping_add(rgb_width.wrapping_mul(rgb_height)) as isize,
        );
        rgb_width = *fptr.i.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        rgb_height = *fptr.i.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    }
    rgb_data = fptr.i.offset(2 as ::core::ffi::c_int as isize);
    if rgb_data.offset(rgb_width.wrapping_mul(rgb_height) as isize)
        > prop_data.offset(prop_data_size as isize)
    {
        XFree(prop_data as *mut ::core::ffi::c_void);
        return None_0 as Pixmap;
    }
    dest_img_data = calloc(
        icon_width.wrapping_mul(icon_height) as size_t,
        (XBitmapPad(wmGD.display) / 8 as ::core::ffi::c_int) as size_t,
    );
    if !dest_img_data.is_null() {
        dest_img = XCreateImage(
            wmGD.display,
            visual,
            depth as ::core::ffi::c_uint,
            ZPixmap,
            0 as ::core::ffi::c_int,
            dest_img_data as *mut ::core::ffi::c_char,
            icon_width,
            icon_height,
            XBitmapPad(wmGD.display),
            0 as ::core::ffi::c_int,
        );
        if dest_img.is_null() {
            free(dest_img_data);
            XFree(prop_data as *mut ::core::ffi::c_void);
            return None_0 as Pixmap;
        }
    } else {
        XFree(prop_data as *mut ::core::ffi::c_void);
        return None_0 as Pixmap;
    }
    bg.pixel = (*(*pCD).pSD).iconAppearance.background as ::core::ffi::c_ulong;
    XQueryColor(wmGD.display, (*pCD).clientColormap, &raw mut bg);
    bg.red = (bg.red as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    bg.green = (bg.green as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    bg.blue = (bg.blue as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    red_shift = (if (*visual).red_mask != 0 {
        ffs((*visual).red_mask as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as ::core::ffi::c_ushort;
    green_shift = (if (*visual).green_mask != 0 {
        ffs((*visual).green_mask as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as ::core::ffi::c_ushort;
    blue_shift = (if (*visual).blue_mask != 0 {
        ffs((*visual).blue_mask as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as ::core::ffi::c_ushort;
    dx = rgb_width as ::core::ffi::c_float / (*dest_img).width as ::core::ffi::c_float;
    dy = rgb_height as ::core::ffi::c_float / (*dest_img).height as ::core::ffi::c_float;
    y = 0 as ::core::ffi::c_uint;
    while y < (*dest_img).height as ::core::ffi::c_uint {
        x = 0 as ::core::ffi::c_uint;
        while x < (*dest_img).width as ::core::ffi::c_uint {
            let mut r: [::core::ffi::c_uint; 4] = [0; 4];
            let mut g: [::core::ffi::c_uint; 4] = [0; 4];
            let mut b: [::core::ffi::c_uint; 4] = [0; 4];
            let mut a: [::core::ffi::c_uint; 4] = [0; 4];
            let mut na: ::core::ffi::c_float = 0.;
            let mut pixel: ::core::ffi::c_ulong = 0;
            fptr.i = rgb_data.offset(
                (floorf(y as ::core::ffi::c_float * dy) * rgb_width as ::core::ffi::c_float
                    + floorf(x as ::core::ffi::c_float * dx)) as ::core::ffi::c_uint
                    as isize,
            );
            a[0 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            r[0 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            g[0 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            b[0 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            fptr.i = rgb_data.offset(
                (floorf(y as ::core::ffi::c_float * dy) * rgb_width as ::core::ffi::c_float
                    + ceilf(x as ::core::ffi::c_float * dx)) as ::core::ffi::c_uint
                    as isize,
            );
            a[1 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            r[1 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            g[1 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            b[1 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            fptr.i = rgb_data.offset(
                (ceilf(y as ::core::ffi::c_float * dy) * rgb_width as ::core::ffi::c_float
                    + floorf(x as ::core::ffi::c_float * dx)) as ::core::ffi::c_uint
                    as isize,
            );
            a[2 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            r[2 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            g[2 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            b[2 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            fptr.i = rgb_data.offset(
                (ceilf(y as ::core::ffi::c_float * dy) * rgb_width as ::core::ffi::c_float
                    + ceilf(x as ::core::ffi::c_float * dx)) as ::core::ffi::c_uint
                    as isize,
            );
            a[3 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            r[3 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            g[3 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            b[3 as ::core::ffi::c_int as usize] =
                *fptr.b.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            na = a[0 as ::core::ffi::c_int as usize]
                .wrapping_add(a[1 as ::core::ffi::c_int as usize])
                .wrapping_add(a[2 as ::core::ffi::c_int as usize])
                .wrapping_add(a[3 as ::core::ffi::c_int as usize])
                .wrapping_div(4 as ::core::ffi::c_uint) as ::core::ffi::c_float
                / 256 as ::core::ffi::c_int as ::core::ffi::c_float;
            r[0 as ::core::ffi::c_int as usize] = r[0 as ::core::ffi::c_int as usize]
                .wrapping_add(r[1 as ::core::ffi::c_int as usize])
                .wrapping_add(r[2 as ::core::ffi::c_int as usize])
                .wrapping_add(r[3 as ::core::ffi::c_int as usize])
                .wrapping_div(4 as ::core::ffi::c_uint);
            g[0 as ::core::ffi::c_int as usize] = g[0 as ::core::ffi::c_int as usize]
                .wrapping_add(g[1 as ::core::ffi::c_int as usize])
                .wrapping_add(g[2 as ::core::ffi::c_int as usize])
                .wrapping_add(g[3 as ::core::ffi::c_int as usize])
                .wrapping_div(4 as ::core::ffi::c_uint);
            b[0 as ::core::ffi::c_int as usize] = b[0 as ::core::ffi::c_int as usize]
                .wrapping_add(b[1 as ::core::ffi::c_int as usize])
                .wrapping_add(b[2 as ::core::ffi::c_int as usize])
                .wrapping_add(b[3 as ::core::ffi::c_int as usize])
                .wrapping_div(4 as ::core::ffi::c_uint);
            r[0 as ::core::ffi::c_int as usize] = ((r[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_float
                * na) as ::core::ffi::c_double
                + bg.red as ::core::ffi::c_float as ::core::ffi::c_double
                    * (1.0f64 - na as ::core::ffi::c_double))
                as ::core::ffi::c_uint;
            g[0 as ::core::ffi::c_int as usize] = ((g[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_float
                * na) as ::core::ffi::c_double
                + bg.green as ::core::ffi::c_float as ::core::ffi::c_double
                    * (1.0f64 - na as ::core::ffi::c_double))
                as ::core::ffi::c_uint;
            b[0 as ::core::ffi::c_int as usize] = ((b[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_float
                * na) as ::core::ffi::c_double
                + bg.blue as ::core::ffi::c_float as ::core::ffi::c_double
                    * (1.0f64 - na as ::core::ffi::c_double))
                as ::core::ffi::c_uint;
            pixel = (r[0 as ::core::ffi::c_int as usize] << red_shift as ::core::ffi::c_int
                | g[0 as ::core::ffi::c_int as usize] << green_shift as ::core::ffi::c_int
                | b[0 as ::core::ffi::c_int as usize] << blue_shift as ::core::ffi::c_int)
                as ::core::ffi::c_ulong;
            Some((*dest_img).f.put_pixel.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                dest_img as *mut _XImage,
                x as ::core::ffi::c_int,
                y as ::core::ffi::c_int,
                pixel,
            );
            x = x.wrapping_add(1);
        }
        y = y.wrapping_add(1);
    }
    pixmap = XCreatePixmap(
        wmGD.display,
        (*(*pCD).pSD).rootWindow as Drawable,
        icon_width,
        icon_height,
        depth as ::core::ffi::c_uint,
    );
    if pixmap != 0 {
        let mut gc = ::core::ptr::null_mut::<_XGC>();
        let mut gcv = XGCValues {
            function: 0,
            plane_mask: 0,
            foreground: 0,
            background: 0,
            line_width: 0,
            line_style: 0,
            cap_style: 0,
            join_style: 0,
            fill_style: 0,
            fill_rule: 0,
            arc_mode: 0,
            tile: 0,
            stipple: 0,
            ts_x_origin: 0,
            ts_y_origin: 0,
            font: 0,
            subwindow_mode: 0,
            graphics_exposures: 0,
            clip_x_origin: 0,
            clip_y_origin: 0,
            clip_mask: 0,
            dash_offset: 0,
            dashes: 0,
        };
        gcv.graphics_exposures = False;
        gc = XCreateGC(
            wmGD.display,
            (*(*pCD).pSD).rootWindow as Drawable,
            GCGraphicsExposures as ::core::ffi::c_ulong,
            &raw mut gcv,
        );
        if !gc.is_null() {
            XPutImage(
                wmGD.display,
                pixmap as Drawable,
                gc,
                dest_img,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                icon_width,
                icon_height,
            );
            XFreeGC(wmGD.display, gc);
        } else {
            XFreePixmap(wmGD.display, pixmap);
            pixmap = None_0 as Pixmap;
        }
    }
    Some(
        (*dest_img)
            .f
            .destroy_image
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(dest_img as *mut _XImage);
    XFree(prop_data as *mut ::core::ffi::c_void);
    return pixmap;
}
unsafe extern "C" fn FetchWindowProperty(
    mut wnd: Window,
    mut prop: Atom,
    mut req_type: Atom,
    mut size: *mut ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    let mut ret_items: ::core::ffi::c_ulong = 0;
    let mut ret_bytes_left: ::core::ffi::c_ulong = 0;
    let mut result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ret_type: Atom = 0;
    let mut ret_fmt: ::core::ffi::c_int = 0;
    if XGetWindowProperty(
        wmGD.display,
        wnd,
        prop,
        0 as ::core::ffi::c_long,
        BUFSIZ as ::core::ffi::c_long,
        False,
        req_type,
        &raw mut ret_type,
        &raw mut ret_fmt,
        &raw mut ret_items,
        &raw mut ret_bytes_left,
        &raw mut result as *mut *mut ::core::ffi::c_uchar,
    ) != Success
    {
        return NULL;
    }
    if ret_type != req_type {
        if !result.is_null() {
            XFree(result as *mut ::core::ffi::c_void);
        }
        return NULL;
    }
    if ret_bytes_left != 0 {
        XFree(result as *mut ::core::ffi::c_void);
        if XGetWindowProperty(
            wmGD.display,
            wnd,
            prop,
            0 as ::core::ffi::c_long,
            (BUFSIZ as ::core::ffi::c_ulong)
                .wrapping_add(ret_bytes_left)
                .wrapping_add(1 as ::core::ffi::c_ulong) as ::core::ffi::c_long,
            False,
            req_type,
            &raw mut ret_type,
            &raw mut ret_fmt,
            &raw mut ret_items,
            &raw mut ret_bytes_left,
            &raw mut result as *mut *mut ::core::ffi::c_uchar,
        ) != Success
        {
            return NULL;
        }
    }
    if !size.is_null() {
        *size = ret_items;
    }
    return result as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn IsDesktopSurface(mut window: Window) -> Boolean {
    let mut count = 0 as ::core::ffi::c_ulong;
    let mut i: ::core::ffi::c_ulong = 0;
    let mut types = FetchWindowProperty(
        window,
        ewmh_atoms[_NET_WM_WINDOW_TYPE as ::core::ffi::c_int as usize],
        XA_ATOM,
        &raw mut count,
    ) as *mut Atom;
    let mut found = False as Boolean;
    i = 0 as ::core::ffi::c_ulong;
    while !types.is_null() && i < count {
        if *types.offset(i as isize)
            == ewmh_atoms[_NET_WM_WINDOW_TYPE_DESKTOP as ::core::ffi::c_int as usize]
        {
            found = True as Boolean;
        }
        i = i.wrapping_add(1);
    }
    if !types.is_null() {
        XFree(types as *mut ::core::ffi::c_void);
    }
    return found;
}
#[no_mangle]
pub unsafe extern "C" fn DesktopOwnsBackground(mut screen: *mut WmScreenData) -> Boolean {
    let mut entry = ::core::ptr::null_mut::<ClientListEntry>();
    entry = (*screen).clientList;
    while !entry.is_null() {
        if (*entry).type_0 == NORMAL_STATE
            && (*(*entry).pCD).desktopSurface as ::core::ffi::c_int != 0
        {
            return True as Boolean;
        }
        entry = (*entry).nextSibling as *mut ClientListEntry;
    }
    return False as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn RestackDesktopSurfaces(mut screen: *mut WmScreenData) {
    let mut entry = ::core::ptr::null_mut::<ClientListEntry>();
    entry = (*screen).clientList;
    while !entry.is_null() {
        if (*entry).type_0 == NORMAL_STATE
            && (*(*entry).pCD).desktopSurface as ::core::ffi::c_int != 0
        {
            XLowerWindow(wmGD.display, (*(*entry).pCD).clientFrameWin);
        }
        entry = (*entry).nextSibling as *mut ClientListEntry;
    }
    if DesktopOwnsBackground(screen) != 0 {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*screen).numWorkspaces {
            if (*(*screen).pWS.offset(i as isize)).backdrop.window != 0 {
                XUnmapWindow(
                    wmGD.display,
                    (*(*screen).pWS.offset(i as isize)).backdrop.window,
                );
            }
            i += 1;
        }
    } else if !(*screen).pActiveWS.is_null() && (*(*screen).pActiveWS).backdrop.window != 0 {
        let mut backdrop = (*(*screen).pActiveWS).backdrop.window;
        XLowerWindow(wmGD.display, backdrop);
        XMapWindow(wmGD.display, backdrop);
        (*screen).lastBackdropWin = backdrop;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PublishCompositorFrame(mut client: *mut ClientData) {
    let mut prop = XInternAtom(
        wmGD.display,
        b"_EMWM_FRAME_CLIENT\0" as *const u8 as *const ::core::ffi::c_char,
        False,
    );
    let mut value = (*client).client as ::core::ffi::c_ulong;
    XChangeProperty(
        wmGD.display,
        (*client).clientFrameWin,
        prop,
        XA_WINDOW,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut value as *mut ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NotifyCompositorTransition(
    mut client: *mut ClientData,
    mut kind: ::core::ffi::c_int,
    mut phase: ::core::ffi::c_int,
) {
    let mut event = _XEvent { type_0: 0 };
    let mut name: [::core::ffi::c_char; 64] = [0; 64];
    let mut selection: Atom = 0;
    if kind == 0 || (*client).desktopSurface as ::core::ffi::c_int != 0 {
        return;
    }
    snprintf(
        &raw mut name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
        b"_NET_WM_CM_S%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*(*client).pSD).screen,
    );
    selection = XInternAtom(
        wmGD.display,
        &raw mut name as *mut ::core::ffi::c_char,
        False,
    );
    if XGetSelectionOwner(wmGD.display, selection) == 0 {
        return;
    }
    memset(
        &raw mut event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<XEvent>() as size_t,
    );
    event.xclient.type_0 = ClientMessage;
    event.xclient.window = (*(*client).pSD).rootWindow;
    event.xclient.message_type = XInternAtom(
        wmGD.display,
        b"_EMWM_ANIMATION\0" as *const u8 as *const ::core::ffi::c_char,
        False,
    );
    event.xclient.format = 32 as ::core::ffi::c_int;
    event.xclient.data.l[0 as ::core::ffi::c_int as usize] =
        (*client).clientFrameWin as ::core::ffi::c_long;
    event.xclient.data.l[1 as ::core::ffi::c_int as usize] = kind as ::core::ffi::c_long;
    event.xclient.data.l[2 as ::core::ffi::c_int as usize] =
        (*(*client).pWsList.offset((*client).currentWsc as isize)).iconFrameWin
            as ::core::ffi::c_long;
    event.xclient.data.l[3 as ::core::ffi::c_int as usize] = phase as ::core::ffi::c_long;
    XSendEvent(
        wmGD.display,
        (*(*client).pSD).rootWindow,
        False,
        SubstructureNotifyMask,
        &raw mut event,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const XA_ATOM: Atom = 4 as ::core::ffi::c_int as Atom;
pub const XA_CARDINAL: Atom = 6 as ::core::ffi::c_int as Atom;
pub const XA_STRING: Atom = 31 as ::core::ffi::c_int as Atom;
pub const XA_WINDOW: Atom = 33 as ::core::ffi::c_int as Atom;
