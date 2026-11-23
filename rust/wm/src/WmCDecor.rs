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
use crate::abi::_WmGlobalData_WmArgb as _WmGlobalData;
use crate::abi::_WmScreenData_WmArgb as _WmScreenData;
use crate::abi::_WmTimer_WmArgb as _WmTimer;
use crate::abi::_WmWorkspaceData_WmArgb as _WmWorkspaceData;
use crate::abi::_WsClientData_WmArgb as _WsClientData;
use crate::abi::_WsPresenceData_WmArgb as _WsPresenceData;
use crate::abi::ButtonSpec_WmArgb as ButtonSpec;
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed_0;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_1;
use crate::abi::C2RustUnnamed_WmArgb as C2RustUnnamed;
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
use crate::abi::WmFunction_WmArgb as WmFunction;
use crate::abi::WmGlobalData_WmArgb as WmGlobalData;
use crate::abi::WmScreenData_WmArgb as WmScreenData;
use crate::abi::WmTimer_WmArgb as WmTimer;
use crate::abi::WsPresenceData_WmArgb as WsPresenceData;
use crate::abi::*;
extern "C" {
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
    fn XChangeSaveSet(_: *mut Display, _: Window, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XChangeWindowAttributes(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_ulong,
        _: *mut XSetWindowAttributes,
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
    fn XClearWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XConfigureWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_uint,
        _: *mut XWindowChanges,
    ) -> ::core::ffi::c_int;
    fn XDestroyWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XDisplayHeight(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XDisplayWidth(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XFillRectangles(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XRectangle,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XGetGCValues(
        _: *mut Display,
        _: GC,
        _: ::core::ffi::c_ulong,
        _: *mut XGCValues,
    ) -> ::core::ffi::c_int;
    fn XMapSubwindows(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XMoveResizeWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XRaiseWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
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
    fn XSetForeground(_: *mut Display, _: GC, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn XShapeCombineMask(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: Pixmap,
        _: ::core::ffi::c_int,
    );
    fn XShapeCombineShape(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn XShapeQueryExtents(
        _: *mut Display,
        _: Window,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XShapeSelectInput(_: *mut Display, _: Window, _: ::core::ffi::c_ulong);
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    static mut wmGD: WmGlobalData;
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn BaseWindowX(pcd: *mut ClientData) -> ::core::ffi::c_int;
    fn BaseWindowY(pcd: *mut ClientData) -> ::core::ffi::c_int;
    fn BaseWindowWidth(pcd: *mut ClientData) -> ::core::ffi::c_uint;
    fn BaseWindowHeight(pcd: *mut ClientData) -> ::core::ffi::c_uint;
    fn GetFramePartInfo(
        pcd: *mut ClientData,
        part: ::core::ffi::c_int,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
        pWidth: *mut ::core::ffi::c_uint,
        pHeight: *mut ::core::ffi::c_uint,
    ) -> Boolean;
    fn GetDepressInfo(
        pcd: *mut ClientData,
        part: ::core::ffi::c_int,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
        pWidth: *mut ::core::ffi::c_uint,
        pHeight: *mut ::core::ffi::c_uint,
        pInvertWidth: *mut ::core::ffi::c_uint,
    ) -> Boolean;
    fn SetFrameInfo(pcd: *mut ClientData);
    fn SetClientOffset(pcd: *mut ClientData);
    fn XBorderIsShowing(pcd: *mut ClientData) -> Boolean;
    fn InitArgbFrame(pcd: *mut ClientData);
    fn ClientDecorDrawable(pcd: *mut ClientData, base: Boolean) -> Drawable;
    fn PaintArgbBackground(pcd: *mut ClientData, base: Boolean, area: *mut XRectangle);
    fn PresentArgbDecoration(pcd: *mut ClientData, base: Boolean);
    fn Warning(message: *mut ::core::ffi::c_char);
    fn AllocateRList(amt: ::core::ffi::c_uint) -> *mut RList;
    fn BevelDepressedRectangle(
        prTop: *mut RList,
        prBot: *mut RList,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        top_wid: ::core::ffi::c_uint,
        right_wid: ::core::ffi::c_uint,
        bot_wid: ::core::ffi::c_uint,
        left_wid: ::core::ffi::c_uint,
        in_wid: ::core::ffi::c_uint,
    );
    fn BevelRectangle(
        prTop: *mut RList,
        prBot: *mut RList,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        top_wid: ::core::ffi::c_uint,
        right_wid: ::core::ffi::c_uint,
        bot_wid: ::core::ffi::c_uint,
        left_wid: ::core::ffi::c_uint,
    );
    fn ExtendRList(prl: *mut RList, amt: ::core::ffi::c_uint) -> Boolean;
    fn StretcherCorner(
        prTop: *mut RList,
        prBot: *mut RList,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        cnum: ::core::ffi::c_int,
        swidth: ::core::ffi::c_uint,
        cwidth: ::core::ffi::c_uint,
        cheight: ::core::ffi::c_uint,
    );
    fn WmDrawXmString(
        dpy: *mut Display,
        w: Window,
        xmfontlist: XmFontList,
        xmstring: XmString,
        gc: GC,
        x: Position,
        y: Position,
        width: Dimension,
        pbox: *mut XRectangle,
        bCenter: Boolean,
    );
    fn GetIconBoxIconRootXY(
        pCD: *mut ClientData,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
    );
    fn FixWindowSize(
        pCD: *mut ClientData,
        pWidth: *mut ::core::ffi::c_uint,
        pHeight: *mut ::core::ffi::c_uint,
        widthInc: ::core::ffi::c_uint,
        heightInc: ::core::ffi::c_uint,
    );
    fn WmGetWindowAttributes(window: Window) -> Boolean;
    fn ProcessNewConfiguration(
        pCD: *mut ClientData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        clientRequest: Boolean,
    );
    fn GetXineramaScreenFromLocation(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
    fn EmwmReloadTitleGradient(_: *mut Display, _: ::core::ffi::c_int, _: Window);
    fn EmwmTitleTextPixel(
        _: *mut Display,
        _: ::core::ffi::c_int,
        _: Window,
        _: ::core::ffi::c_int,
        _: *mut Pixel,
    ) -> ::core::ffi::c_int;
    fn EmwmPaintTitleGradient(
        _: *mut Display,
        _: ::core::ffi::c_int,
        _: Window,
        _: ::core::ffi::c_int,
        _: Drawable,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const XRectangle,
    );
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CopyFromParent: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ButtonPressMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ButtonReleaseMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const EnterWindowMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const LeaveWindowMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const Button1MotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int;
pub const Button2MotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const ExposureMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 15 as ::core::ffi::c_int;
pub const SubstructureNotifyMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 19 as ::core::ffi::c_int;
pub const SubstructureRedirectMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 20 as ::core::ffi::c_int;
pub const FocusChangeMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 21 as ::core::ffi::c_int;
pub const InputOutput: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const InputOnly: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CWBackPixmap: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const CWBackPixel: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const CWBorderPixel: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const CWSaveUnder: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 10 as ::core::ffi::c_int;
pub const CWEventMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CWColormap: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 13 as ::core::ffi::c_int;
pub const CWCursor: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 14 as ::core::ffi::c_int;
pub const CWWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const CWHeight: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const CWBorderWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const SetModeInsert: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GCForeground: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ShapeSet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ShapeUnion: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ShapeBounding: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ShapeClip: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ShapeNotifyMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_FUNC_CLOSE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const MWM_DECOR_BORDER: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const MWM_DECOR_RESIZEH: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const MWM_DECOR_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const MWM_DECOR_MENU: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const MWM_DECOR_MINIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const MWM_DECOR_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SELECT_BUTTON_MOTION_MASK: ::core::ffi::c_long = Button1MotionMask;
pub const DMANIP_BUTTON_MOTION_MASK: ::core::ffi::c_long = Button2MotionMask;
pub const NUM_TITLE_GADGET_RECTS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const NUM_MATTE_TS_RECTS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const NUM_MATTE_BS_RECTS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const FRAME_INTERNAL_SHADOW_WIDTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_TOP_TITLE_PADDING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_BOTTOM_TITLE_PADDING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_TOP_TITLE_SHADOW: ::core::ffi::c_int = FRAME_INTERNAL_SHADOW_WIDTH;
pub const WM_BOTTOM_TITLE_SHADOW: ::core::ffi::c_int = FRAME_INTERNAL_SHADOW_WIDTH;
pub const STRETCH_NORTH_WEST: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STRETCH_NORTH: ::core::ffi::c_int = 1;
pub const STRETCH_NORTH_EAST: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STRETCH_EAST: ::core::ffi::c_int = 3;
pub const STRETCH_SOUTH_EAST: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const STRETCH_SOUTH: ::core::ffi::c_int = 5;
pub const STRETCH_SOUTH_WEST: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const STRETCH_WEST: ::core::ffi::c_int = 7;
pub const STRETCH_COUNT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const FRAME_SYSTEM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FRAME_TITLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FRAME_MINIMIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FRAME_MAXIMIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FRAME_RESIZE_NW: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const FRAME_RESIZE_N: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const FRAME_RESIZE_NE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const FRAME_RESIZE_E: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const FRAME_RESIZE_SE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FRAME_RESIZE_S: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const FRAME_RESIZE_SW: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const FRAME_RESIZE_W: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FRAME_CLOSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const ICONBOX_PART_WICON: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const F_SUBCONTEXT_IB_WICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_WICON;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CLIENT_REPARENTED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const CLIENT_IN_SAVE_SET: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const ICON_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CONFIRM_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 12 as ::core::ffi::c_int;
pub const CLIENT_WM_CLIENTS: ::core::ffi::c_long = ICON_BOX | CONFIRM_BOX;
pub const SYSTEM_DEPRESSED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const TITLE_DEPRESSED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const MINIMIZE_DEPRESSED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const MAXIMIZE_DEPRESSED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const CLOSE_DEPRESSED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const MAX_INTERNAL_BEVEL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CMAP_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn PaintTitleGradient(mut pcd: *mut ClientData, mut clip: *mut XRectangle) {
    let mut separate: Boolean = 0;
    if (*pcd).decor & MWM_DECOR_TITLE == 0 || (*pcd).fullScreen as ::core::ffi::c_int != 0 {
        return;
    }
    separate = (*(*pcd).pSD).decoupleTitleAppearance;
    EmwmPaintTitleGradient(
        wmGD.display,
        (*(*pcd).pSD).screen,
        (*(*pcd).pSD).rootWindow,
        (pcd == wmGD.keyboardFocus) as ::core::ffi::c_int,
        if separate as ::core::ffi::c_int != 0 {
            (*pcd).clientTitleWin as Drawable
        } else {
            ClientDecorDrawable(pcd, False as Boolean)
        },
        (if separate as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_uint
        } else {
            (*pcd).frameInfo.upperBorderWidth
        }) as ::core::ffi::c_int,
        (*pcd).frameInfo.width.wrapping_sub(
            (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.upperBorderWidth),
        ) as ::core::ffi::c_int,
        (*pcd).frameInfo.titleBarHeight as ::core::ffi::c_int,
        clip,
    );
}
unsafe extern "C" fn RedrawGradientTree(mut pcd: *mut ClientData) {
    let mut child = ::core::ptr::null_mut::<ClientData>();
    if (*pcd).desktopSurface == 0 && (*pcd).decor & MWM_DECOR_TITLE != 0 && (*pcd).fullScreen == 0 {
        if pcd == wmGD.keyboardFocus {
            ShowActiveClientFrame(pcd);
        } else {
            ShowInactiveClientFrame(pcd);
        }
    }
    child = (*pcd).transientChildren as *mut ClientData;
    while !child.is_null() {
        RedrawGradientTree(child);
        child = (*child).transientSiblings as *mut ClientData;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReloadTitleGradients(mut screen: *mut WmScreenData) {
    let mut entry = ::core::ptr::null_mut::<ClientListEntry>();
    EmwmReloadTitleGradient(wmGD.display, (*screen).screen, (*screen).rootWindow);
    entry = (*screen).clientList;
    while !entry.is_null() {
        if (*entry).type_0 == NORMAL_STATE {
            RedrawGradientTree((*entry).pCD as *mut ClientData);
        }
        entry = (*entry).nextSibling as *mut ClientListEntry;
    }
}
static mut Bevels: [Bevel_Count; 14] = [
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 0 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 0 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 0 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 0 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 4 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 3 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 2 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 4 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 3 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 4 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 3 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 2 as ::core::ffi::c_int,
            join: 0 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 2 as ::core::ffi::c_int,
            internal: 2 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 1 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 1 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 1 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 1 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 2 as ::core::ffi::c_int,
            internal: 2 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 2 as ::core::ffi::c_int,
            join: 0 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 1 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 1 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 1 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
    },
    Bevel_Count {
        top: Single_Bevel_Count {
            external: 1 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 0 as ::core::ffi::c_int,
        },
        bottom: Single_Bevel_Count {
            external: 0 as ::core::ffi::c_int,
            join: 1 as ::core::ffi::c_int,
            internal: 1 as ::core::ffi::c_int,
        },
    },
];
#[no_mangle]
pub unsafe extern "C" fn FrameWindow(mut pcd: *mut ClientData) -> Boolean {
    if ConstructFrame(pcd) == 0 {
        return 0 as Boolean;
    }
    GenerateFrameDisplayLists(pcd);
    AdoptClient(pcd);
    if wmGD.hasShape as ::core::ffi::c_int != 0 && (*pcd).wShaped as ::core::ffi::c_int != 0 {
        SetFrameShape(pcd);
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn FrameExposureProc(mut pcd: *mut ClientData) {
    let mut topGC = ::core::ptr::null_mut::<_XGC>();
    let mut botGC = ::core::ptr::null_mut::<_XGC>();
    let mut win = ClientDecorDrawable(pcd, False as Boolean);
    if (*pcd).fullScreen != 0 {
        XClearWindow(wmGD.display, (*pcd).clientFrameWin);
        return;
    }
    PaintArgbBackground(pcd, False as Boolean, ::core::ptr::null_mut::<XRectangle>());
    PaintTitleGradient(pcd, ::core::ptr::null_mut::<XRectangle>());
    if pcd == wmGD.keyboardFocus {
        topGC = (*(*pcd).pSD).clientAppearance.activeTopShadowGC;
        botGC = (*(*pcd).pSD).clientAppearance.activeBottomShadowGC;
    } else {
        topGC = (*(*pcd).pSD).clientAppearance.inactiveTopShadowGC;
        botGC = (*(*pcd).pSD).clientAppearance.inactiveBottomShadowGC;
    }
    if !(*pcd).pclientTopShadows.is_null() {
        XFillRectangles(
            wmGD.display,
            win,
            topGC,
            (*(*pcd).pclientTopShadows).prect,
            (*(*pcd).pclientTopShadows).used,
        );
    }
    if !(*pcd).pclientBottomShadows.is_null() {
        XFillRectangles(
            wmGD.display,
            win,
            botGC,
            (*(*pcd).pclientBottomShadows).prect,
            (*(*pcd).pclientBottomShadows).used,
        );
    }
    if (*(*pcd).pSD).decoupleTitleAppearance as ::core::ffi::c_int != 0
        && (*pcd).decor & MWM_DECOR_TITLE != 0
    {
        if pcd == wmGD.keyboardFocus {
            topGC = (*(*pcd).pSD).clientTitleAppearance.activeTopShadowGC;
            botGC = (*(*pcd).pSD).clientTitleAppearance.activeBottomShadowGC;
        } else {
            topGC = (*(*pcd).pSD).clientTitleAppearance.inactiveTopShadowGC;
            botGC = (*(*pcd).pSD).clientTitleAppearance.inactiveBottomShadowGC;
        }
        if !(*pcd).pclientTitleTopShadows.is_null() {
            XFillRectangles(
                wmGD.display,
                (*pcd).clientTitleWin as Drawable,
                topGC,
                (*(*pcd).pclientTitleTopShadows).prect,
                (*(*pcd).pclientTitleTopShadows).used,
            );
        }
        if !(*pcd).pclientTitleBottomShadows.is_null() {
            XFillRectangles(
                wmGD.display,
                (*pcd).clientTitleWin as Drawable,
                botGC,
                (*(*pcd).pclientTitleBottomShadows).prect,
                (*(*pcd).pclientTitleBottomShadows).used,
            );
        }
    }
    DrawWindowTitle(pcd, False as Boolean);
    PresentArgbDecoration(pcd, False as Boolean);
}
#[no_mangle]
pub unsafe extern "C" fn BaseWinExposureProc(mut pcd: *mut ClientData) {
    let mut drawable = ClientDecorDrawable(pcd, True as Boolean);
    PaintArgbBackground(pcd, True as Boolean, ::core::ptr::null_mut::<XRectangle>());
    if (*pcd).matteWidth > 0 as ::core::ffi::c_int {
        if !(*pcd).pclientMatteTopShadows.is_null() {
            XFillRectangles(
                wmGD.display,
                drawable,
                (*pcd).clientMatteTopShadowGC,
                (*(*pcd).pclientMatteTopShadows).prect,
                (*(*pcd).pclientMatteTopShadows).used,
            );
        }
        if !(*pcd).pclientMatteBottomShadows.is_null() {
            XFillRectangles(
                wmGD.display,
                drawable,
                (*pcd).clientMatteBottomShadowGC,
                (*(*pcd).pclientMatteBottomShadows).prect,
                (*(*pcd).pclientMatteBottomShadows).used,
            );
        }
    }
    PresentArgbDecoration(pcd, True as Boolean);
}
#[no_mangle]
pub unsafe extern "C" fn UpdateClientDecorations(mut pcd: *mut ClientData) -> Boolean {
    SetClientOffset(pcd);
    if ConstructFrame(pcd) == 0 {
        return False as Boolean;
    }
    if (*pcd).decor & MWM_DECOR_TITLE != 0
        && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
    {
        let mut boxes = (2 as ::core::ffi::c_int
            + ((*pcd).decor & MWM_DECOR_MENU != 0) as ::core::ffi::c_int
            + ((*pcd).decor & MWM_DECOR_MINIMIZE != 0) as ::core::ffi::c_int
            + ((*pcd).decor & MWM_DECOR_MAXIMIZE != 0) as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        let mut minimum = boxes.wrapping_mul((*pcd).frameInfo.titleBarHeight);
        minimum = if minimum > (2 as ::core::ffi::c_int * (*pcd).matteWidth) as ::core::ffi::c_uint
        {
            minimum
                .wrapping_sub((2 as ::core::ffi::c_int * (*pcd).matteWidth) as ::core::ffi::c_uint)
        } else {
            0 as ::core::ffi::c_uint
        };
        if ((*pcd).minWidth as ::core::ffi::c_uint) < minimum {
            let mut remainder = minimum
                .wrapping_sub((*pcd).baseWidth as ::core::ffi::c_uint)
                .wrapping_rem((*pcd).widthInc as ::core::ffi::c_uint);
            (*pcd).minWidth = minimum.wrapping_add(
                (if remainder != 0 {
                    ((*pcd).widthInc as ::core::ffi::c_uint).wrapping_sub(remainder)
                } else {
                    0 as ::core::ffi::c_uint
                }),
            ) as ::core::ffi::c_int;
            if (*pcd).maxWidth < (*pcd).minWidth {
                (*pcd).maxWidth = (*pcd).minWidth;
            }
        }
        if (*pcd).clientWidth < (*pcd).minWidth {
            if (*pcd).maxConfig != 0 {
                FixWindowSize(
                    pcd,
                    &raw mut (*pcd).clientWidth as *mut ::core::ffi::c_uint,
                    &raw mut (*pcd).clientHeight as *mut ::core::ffi::c_uint,
                    (*pcd).widthInc as ::core::ffi::c_uint,
                    (*pcd).heightInc as ::core::ffi::c_uint,
                );
            } else {
                ProcessNewConfiguration(
                    pcd,
                    (*pcd).clientX,
                    (*pcd).clientY,
                    (*pcd).minWidth as ::core::ffi::c_uint,
                    (*pcd).clientHeight as ::core::ffi::c_uint,
                    True as Boolean,
                );
            }
        }
    }
    RegenerateClientFrame(pcd);
    if pcd == wmGD.keyboardFocus {
        ShowActiveClientFrame(pcd);
    } else {
        ShowInactiveClientFrame(pcd);
    }
    return True as Boolean;
}
unsafe extern "C" fn ConstructFrame(mut pcd: *mut ClientData) -> Boolean {
    let mut decoration = (*pcd).decor as ::core::ffi::c_ulong;
    let mut wclass: ::core::ffi::c_uint = 0;
    let mut attr_mask: ::core::ffi::c_ulong = 0;
    let mut window_attribs = XSetWindowAttributes {
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
    let mut frmX: ::core::ffi::c_int = 0;
    let mut frmY: ::core::ffi::c_int = 0;
    SetFrameInfo(pcd);
    if AllocateFrameDisplayLists(pcd) == 0 {
        return 0 as Boolean;
    }
    if (*pcd).clientFrameWin == 0 {
        InitArgbFrame(pcd);
        attr_mask = CWEventMask as ::core::ffi::c_ulong;
        window_attribs.event_mask = ButtonPressMask
            | ButtonReleaseMask
            | SELECT_BUTTON_MOTION_MASK
            | DMANIP_BUTTON_MOTION_MASK
            | ExposureMask;
        if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER
            || wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER
        {
            window_attribs.event_mask |= EnterWindowMask | LeaveWindowMask;
        }
        if (*(*pcd).pSD).clientAppearance.backgroundPixmap != 0 {
            attr_mask |= CWBackPixmap as ::core::ffi::c_ulong;
            window_attribs.background_pixmap = (*(*pcd).pSD).clientAppearance.backgroundPixmap;
        } else {
            attr_mask |= CWBackPixel as ::core::ffi::c_ulong;
            window_attribs.background_pixel =
                (*(*pcd).pSD).clientAppearance.background as ::core::ffi::c_ulong;
        }
        attr_mask |= CWCursor as ::core::ffi::c_ulong;
        window_attribs.cursor = wmGD.workspaceCursor;
        frmY = (*pcd).frameInfo.y;
        frmX = (*pcd).frameInfo.x;
        if (*(*pcd).pSD).clientAppearance.saveUnder as ::core::ffi::c_int != 0
            && WmGetWindowAttributes((*pcd).client) as ::core::ffi::c_int != 0
            && wmGD.windowAttributes.save_under != 0
        {
            attr_mask |= CWSaveUnder as ::core::ffi::c_ulong;
            window_attribs.save_under = True;
        }
        if !(*pcd).argbFrame.is_null() {
            attr_mask &= !CWBackPixmap as ::core::ffi::c_ulong;
            attr_mask |= (CWBackPixel | CWBorderPixel | CWColormap) as ::core::ffi::c_ulong;
            window_attribs.background_pixel = 0 as ::core::ffi::c_ulong;
            window_attribs.border_pixel = 0 as ::core::ffi::c_ulong;
            window_attribs.colormap = (*(*pcd).argbFrame).colormap;
        }
        (*pcd).clientFrameWin = XCreateWindow(
            wmGD.display,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(*pcd).pSD).screen as isize))
            .root,
            frmX,
            frmY,
            (*pcd).frameInfo.width,
            (*pcd).frameInfo.height,
            0 as ::core::ffi::c_uint,
            (if !(*pcd).argbFrame.is_null() {
                (*(*pcd).argbFrame).depth as ::core::ffi::c_long
            } else {
                CopyFromParent
            }) as ::core::ffi::c_int,
            InputOutput as ::core::ffi::c_uint,
            if !(*pcd).argbFrame.is_null() {
                (*(*pcd).argbFrame).visual
            } else {
                ::core::ptr::null_mut::<Visual>()
            },
            attr_mask,
            &raw mut window_attribs,
        );
    } else {
        XMoveResizeWindow(
            wmGD.display,
            (*pcd).clientFrameWin,
            (*pcd).frameInfo.x,
            (*pcd).frameInfo.y,
            (*pcd).frameInfo.width,
            (*pcd).frameInfo.height,
        );
    }
    if (*(*pcd).pSD).resizeCursors as ::core::ffi::c_int != 0
        && decoration & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0
    {
        if (*pcd).clientStretchWin[0 as ::core::ffi::c_int as usize] == 0 {
            CreateStretcherWindows(pcd);
        }
    } else if (*pcd).clientStretchWin[0 as ::core::ffi::c_int as usize] != 0 {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < STRETCH_COUNT {
            XDestroyWindow(wmGD.display, (*pcd).clientStretchWin[i as usize]);
            (*pcd).clientStretchWin[i as usize] = None_0 as Window;
            i += 1;
        }
    }
    if decoration & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0 && (*pcd).clientTitleWin == 0 {
        attr_mask = CWCursor as ::core::ffi::c_ulong;
        window_attribs.cursor = wmGD.workspaceCursor;
        if (*(*pcd).pSD).decoupleTitleAppearance != 0 {
            wclass = InputOutput as ::core::ffi::c_uint;
            attr_mask |= CWEventMask as ::core::ffi::c_ulong;
            window_attribs.event_mask = ExposureMask;
            if (*(*pcd).pSD).clientTitleAppearance.backgroundPixmap != 0 {
                attr_mask |= CWBackPixmap as ::core::ffi::c_ulong;
                window_attribs.background_pixmap =
                    (*(*pcd).pSD).clientTitleAppearance.backgroundPixmap;
            } else {
                attr_mask |= CWBackPixel as ::core::ffi::c_ulong;
                window_attribs.background_pixel =
                    (*(*pcd).pSD).clientTitleAppearance.background as ::core::ffi::c_ulong;
            }
        } else {
            wclass = InputOnly as ::core::ffi::c_uint;
        }
        if !(*pcd).argbFrame.is_null() && wclass == InputOutput as ::core::ffi::c_uint {
            attr_mask |= (CWColormap | CWBorderPixel) as ::core::ffi::c_ulong;
            window_attribs.colormap = (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(*pcd).pSD).screen as isize))
            .cmap;
            window_attribs.border_pixel = 0 as ::core::ffi::c_ulong;
        }
        (*pcd).clientTitleWin = XCreateWindow(
            wmGD.display,
            (*pcd).clientFrameWin,
            (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int,
            (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int,
            (*pcd).frameInfo.width.wrapping_sub(
                (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.upperBorderWidth),
            ),
            (*pcd).frameInfo.titleBarHeight,
            0 as ::core::ffi::c_uint,
            (if !(*pcd).argbFrame.is_null() && wclass == InputOutput as ::core::ffi::c_uint {
                (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*(*pcd).pSD).screen as isize))
                .root_depth as ::core::ffi::c_long
            } else {
                CopyFromParent
            }) as ::core::ffi::c_int,
            wclass,
            if !(*pcd).argbFrame.is_null() && wclass == InputOutput as ::core::ffi::c_uint {
                (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*(*pcd).pSD).screen as isize))
                .root_visual
            } else {
                ::core::ptr::null_mut::<Visual>()
            },
            attr_mask,
            &raw mut window_attribs,
        );
    } else if decoration & MWM_DECOR_TITLE as ::core::ffi::c_ulong == 0
        && (*pcd).clientTitleWin != 0
    {
        XDestroyWindow(wmGD.display, (*pcd).clientTitleWin);
        (*pcd).clientTitleWin = None_0 as Window;
    } else if (*pcd).clientTitleWin != 0 {
        XRaiseWindow(wmGD.display, (*pcd).clientTitleWin);
    }
    if AllocateGadgetRectangles(pcd) == 0 {
        return 0 as Boolean;
    }
    ComputeGadgetRectangles(pcd);
    if (*pcd).clientBaseWin == None_0 as Window {
        window_attribs.event_mask =
            SubstructureRedirectMask | SubstructureNotifyMask | FocusChangeMask;
        if (*pcd).matteWidth > 0 as ::core::ffi::c_int {
            window_attribs.event_mask |= ExposureMask;
            window_attribs.background_pixel = (*pcd).matteBackground as ::core::ffi::c_ulong;
        } else {
            window_attribs.background_pixel =
                (*(*pcd).pSD).clientTitleAppearance.background as ::core::ffi::c_ulong;
        }
        attr_mask = (CWBackPixel | CWEventMask) as ::core::ffi::c_ulong;
        if !(*pcd).argbFrame.is_null() {
            window_attribs.background_pixel = 0 as ::core::ffi::c_ulong;
        }
        (*pcd).clientBaseWin = XCreateWindow(
            wmGD.display,
            (*pcd).clientFrameWin,
            BaseWindowX(pcd),
            BaseWindowY(pcd),
            BaseWindowWidth(pcd),
            BaseWindowHeight(pcd),
            0 as ::core::ffi::c_uint,
            CopyFromParent as ::core::ffi::c_int,
            InputOutput as ::core::ffi::c_uint,
            ::core::ptr::null_mut::<Visual>(),
            attr_mask,
            &raw mut window_attribs,
        );
    } else {
        XRaiseWindow(wmGD.display, (*pcd).clientBaseWin);
        XMoveResizeWindow(
            wmGD.display,
            (*pcd).clientBaseWin,
            BaseWindowX(pcd),
            BaseWindowY(pcd),
            BaseWindowWidth(pcd),
            BaseWindowHeight(pcd),
        );
    }
    XMapSubwindows(wmGD.display, (*pcd).clientFrameWin);
    return 1 as Boolean;
}
unsafe extern "C" fn GenerateFrameDisplayLists(mut pcd: *mut ClientData) {
    let mut decoration = (*pcd).decor as ::core::ffi::c_ulong;
    let mut matte_width = (*pcd).matteWidth;
    let mut insideBevel: ::core::ffi::c_int = 0;
    let mut inset: ::core::ffi::c_int = 0;
    let mut diffBevel: ::core::ffi::c_int = 0;
    let mut nTitleBevel: ::core::ffi::c_uint = 0;
    let mut sTitleBevel: ::core::ffi::c_uint = 0;
    let mut eTitleBevel: ::core::ffi::c_uint = 0;
    let mut wTitleBevel: ::core::ffi::c_uint = 0;
    let mut meTitleBevel: ::core::ffi::c_uint = 0;
    let mut inWidth: ::core::ffi::c_uint = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut xAdj = 0 as ::core::ffi::c_int;
    let mut yAdj = 0 as ::core::ffi::c_int;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut prlTop = ::core::ptr::null_mut::<RList>();
    let mut prlBot = ::core::ptr::null_mut::<RList>();
    let mut jX: ::core::ffi::c_int = 0;
    let mut jY: ::core::ffi::c_int = 0;
    let mut jW: ::core::ffi::c_uint = 0;
    let mut jH: ::core::ffi::c_uint = 0;
    if !(*pcd).pclientTopShadows.is_null() {
        (*(*pcd).pclientTopShadows).used = 0 as ::core::ffi::c_int;
    }
    if !(*pcd).pclientBottomShadows.is_null() {
        (*(*pcd).pclientBottomShadows).used = 0 as ::core::ffi::c_int;
    }
    if !(*pcd).pclientTitleTopShadows.is_null() {
        (*(*pcd).pclientTitleTopShadows).used = 0 as ::core::ffi::c_int;
    }
    if !(*pcd).pclientTitleBottomShadows.is_null() {
        (*(*pcd).pclientTitleBottomShadows).used = 0 as ::core::ffi::c_int;
    }
    if !(*pcd).pclientMatteTopShadows.is_null() {
        (*(*pcd).pclientMatteTopShadows).used = 0 as ::core::ffi::c_int;
    }
    if !(*pcd).pclientMatteBottomShadows.is_null() {
        (*(*pcd).pclientMatteBottomShadows).used = 0 as ::core::ffi::c_int;
    }
    if wmGD.frameStyle as ::core::ffi::c_uint
        == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
        && matte_width > 0 as ::core::ffi::c_int
    {
        insideBevel = (*(*pcd).pSD).joinBevel;
    } else {
        insideBevel = (*pcd).internalBevel;
    }
    diffBevel = insideBevel - 1 as ::core::ffi::c_int;
    if decoration & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
        if (*pcd).internalBevel > 1 as ::core::ffi::c_int
            && decoration & MWM_DECOR_TITLE as ::core::ffi::c_ulong == 0
        {
            inset = 1 as ::core::ffi::c_int;
        } else {
            inset = 0 as ::core::ffi::c_int;
        }
        GetFramePartInfo(
            pcd,
            FRAME_RESIZE_NW,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        StretcherCorner(
            (*pcd).pclientTopShadows,
            (*pcd).pclientBottomShadows,
            x,
            y,
            STRETCH_NORTH_WEST,
            (*pcd)
                .frameInfo
                .upperBorderWidth
                .wrapping_sub(inset as ::core::ffi::c_uint),
            width,
            height,
        );
        GetFramePartInfo(
            pcd,
            FRAME_RESIZE_N,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        if width as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            BevelRectangle(
                (*pcd).pclientTopShadows,
                (*pcd).pclientBottomShadows,
                x,
                y,
                width,
                height.wrapping_sub(inset as ::core::ffi::c_uint),
                2 as ::core::ffi::c_uint,
                1 as ::core::ffi::c_uint,
                (if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
                1 as ::core::ffi::c_uint,
            );
        }
        GetFramePartInfo(
            pcd,
            FRAME_RESIZE_NE,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        StretcherCorner(
            (*pcd).pclientTopShadows,
            (*pcd).pclientBottomShadows,
            x,
            y,
            STRETCH_NORTH_EAST,
            (*pcd)
                .frameInfo
                .upperBorderWidth
                .wrapping_sub(inset as ::core::ffi::c_uint),
            width,
            height,
        );
        GetFramePartInfo(
            pcd,
            FRAME_RESIZE_E,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        if height as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            BevelRectangle(
                (*pcd).pclientTopShadows,
                (*pcd).pclientBottomShadows,
                x + diffBevel,
                y,
                width.wrapping_sub(diffBevel as ::core::ffi::c_uint),
                height,
                1 as ::core::ffi::c_uint,
                2 as ::core::ffi::c_uint,
                1 as ::core::ffi::c_uint,
                (if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            );
        }
        GetFramePartInfo(
            pcd,
            FRAME_RESIZE_SE,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        StretcherCorner(
            (*pcd).pclientTopShadows,
            (*pcd).pclientBottomShadows,
            x,
            y,
            STRETCH_SOUTH_EAST,
            (*pcd)
                .frameInfo
                .upperBorderWidth
                .wrapping_sub(inset as ::core::ffi::c_uint),
            width,
            height,
        );
        GetFramePartInfo(
            pcd,
            FRAME_RESIZE_S,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        if width as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            BevelRectangle(
                (*pcd).pclientTopShadows,
                (*pcd).pclientBottomShadows,
                x,
                y + diffBevel,
                width,
                height.wrapping_sub(diffBevel as ::core::ffi::c_uint),
                (if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
                1 as ::core::ffi::c_uint,
                2 as ::core::ffi::c_uint,
                1 as ::core::ffi::c_uint,
            );
        }
        GetFramePartInfo(
            pcd,
            FRAME_RESIZE_SW,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        StretcherCorner(
            (*pcd).pclientTopShadows,
            (*pcd).pclientBottomShadows,
            x,
            y,
            STRETCH_SOUTH_WEST,
            (*pcd)
                .frameInfo
                .upperBorderWidth
                .wrapping_sub(inset as ::core::ffi::c_uint),
            width,
            height,
        );
        GetFramePartInfo(
            pcd,
            FRAME_RESIZE_W,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        if height as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            BevelRectangle(
                (*pcd).pclientTopShadows,
                (*pcd).pclientBottomShadows,
                x,
                y,
                width.wrapping_sub(diffBevel as ::core::ffi::c_uint),
                height,
                1 as ::core::ffi::c_uint,
                (if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
                1 as ::core::ffi::c_uint,
                2 as ::core::ffi::c_uint,
            );
        }
        if diffBevel != 0 {
            BevelRectangle(
                (*pcd).pclientBottomShadows,
                (*pcd).pclientTopShadows,
                (*pcd)
                    .frameInfo
                    .lowerBorderWidth
                    .wrapping_sub(diffBevel as ::core::ffi::c_uint)
                    as ::core::ffi::c_int,
                (*pcd).clientOffset.y as ::core::ffi::c_int - diffBevel,
                (*pcd)
                    .frameInfo
                    .width
                    .wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.lowerBorderWidth),
                    )
                    .wrapping_add((2 as ::core::ffi::c_int * diffBevel) as ::core::ffi::c_uint),
                (*pcd)
                    .frameInfo
                    .height
                    .wrapping_sub((*pcd).clientOffset.y as ::core::ffi::c_uint)
                    .wrapping_sub((*pcd).frameInfo.lowerBorderWidth)
                    .wrapping_add((2 as ::core::ffi::c_int * diffBevel) as ::core::ffi::c_uint),
                diffBevel as ::core::ffi::c_uint,
                diffBevel as ::core::ffi::c_uint,
                diffBevel as ::core::ffi::c_uint,
                diffBevel as ::core::ffi::c_uint,
            );
        }
    } else if decoration & MWM_DECOR_BORDER as ::core::ffi::c_ulong != 0 {
        BevelRectangle(
            (*pcd).pclientTopShadows,
            (*pcd).pclientBottomShadows,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*pcd).frameInfo.width,
            (*pcd).frameInfo.height,
            wmGD.frameExternalShadowWidth as ::core::ffi::c_uint,
            wmGD.frameExternalShadowWidth as ::core::ffi::c_uint,
            wmGD.frameExternalShadowWidth as ::core::ffi::c_uint,
            wmGD.frameExternalShadowWidth as ::core::ffi::c_uint,
        );
        if (*pcd).internalBevel > 1 as ::core::ffi::c_int
            && matte_width == 0
            && decoration & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0
        {
            GetFramePartInfo(
                pcd,
                FRAME_TITLE,
                &raw mut x,
                &raw mut y,
                &raw mut width,
                &raw mut height,
            );
            inset = (1 as ::core::ffi::c_uint).wrapping_add(
                (*pcd)
                    .frameInfo
                    .lowerBorderWidth
                    .wrapping_sub((*pcd).frameInfo.upperBorderWidth),
            ) as ::core::ffi::c_int;
            BevelRectangle(
                (*pcd).pclientBottomShadows,
                (*pcd).pclientTopShadows,
                (*pcd)
                    .frameInfo
                    .lowerBorderWidth
                    .wrapping_sub(inset as ::core::ffi::c_uint)
                    as ::core::ffi::c_int,
                (*pcd)
                    .frameInfo
                    .lowerBorderWidth
                    .wrapping_sub(inset as ::core::ffi::c_uint)
                    as ::core::ffi::c_int,
                (*pcd)
                    .frameInfo
                    .width
                    .wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.lowerBorderWidth),
                    )
                    .wrapping_add((2 as ::core::ffi::c_int * inset) as ::core::ffi::c_uint),
                (*pcd)
                    .frameInfo
                    .height
                    .wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.lowerBorderWidth),
                    )
                    .wrapping_add((2 as ::core::ffi::c_int * inset) as ::core::ffi::c_uint),
                1 as ::core::ffi::c_uint,
                1 as ::core::ffi::c_uint,
                1 as ::core::ffi::c_uint,
                1 as ::core::ffi::c_uint,
            );
            BevelRectangle(
                (*pcd).pclientBottomShadows,
                (*pcd).pclientTopShadows,
                (*pcd)
                    .frameInfo
                    .lowerBorderWidth
                    .wrapping_sub(diffBevel as ::core::ffi::c_uint)
                    as ::core::ffi::c_int,
                (*pcd).clientOffset.y as ::core::ffi::c_int - diffBevel,
                (*pcd)
                    .frameInfo
                    .width
                    .wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.lowerBorderWidth),
                    )
                    .wrapping_add((2 as ::core::ffi::c_int * diffBevel) as ::core::ffi::c_uint),
                (*pcd)
                    .frameInfo
                    .height
                    .wrapping_sub((*pcd).clientOffset.y as ::core::ffi::c_uint)
                    .wrapping_sub((*pcd).frameInfo.lowerBorderWidth)
                    .wrapping_add((2 as ::core::ffi::c_int * diffBevel) as ::core::ffi::c_uint),
                diffBevel as ::core::ffi::c_uint,
                diffBevel as ::core::ffi::c_uint,
                diffBevel as ::core::ffi::c_uint,
                diffBevel as ::core::ffi::c_uint,
            );
        } else {
            BevelRectangle(
                (*pcd).pclientBottomShadows,
                (*pcd).pclientTopShadows,
                (*pcd)
                    .frameInfo
                    .lowerBorderWidth
                    .wrapping_sub(insideBevel as ::core::ffi::c_uint)
                    as ::core::ffi::c_int,
                (*pcd)
                    .frameInfo
                    .lowerBorderWidth
                    .wrapping_sub(insideBevel as ::core::ffi::c_uint)
                    as ::core::ffi::c_int,
                (*pcd)
                    .frameInfo
                    .width
                    .wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.lowerBorderWidth),
                    )
                    .wrapping_add((2 as ::core::ffi::c_int * insideBevel) as ::core::ffi::c_uint),
                (*pcd)
                    .frameInfo
                    .height
                    .wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.lowerBorderWidth),
                    )
                    .wrapping_add((2 as ::core::ffi::c_int * insideBevel) as ::core::ffi::c_uint),
                insideBevel as ::core::ffi::c_uint,
                insideBevel as ::core::ffi::c_uint,
                insideBevel as ::core::ffi::c_uint,
                insideBevel as ::core::ffi::c_uint,
            );
        }
    }
    if decoration & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) as ::core::ffi::c_ulong != 0 {
        nTitleBevel = (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint;
        if wmGD.frameStyle as ::core::ffi::c_uint
            == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            sTitleBevel = (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint;
        } else {
            sTitleBevel = insideBevel as ::core::ffi::c_uint;
        }
        eTitleBevel = (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint;
        wTitleBevel = (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint;
        meTitleBevel = (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint;
    } else {
        nTitleBevel = (*(*pcd).pSD).externalBevel as ::core::ffi::c_uint;
        if wmGD.frameStyle as ::core::ffi::c_uint
            == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            sTitleBevel = (if matte_width > 0 as ::core::ffi::c_int {
                (*(*pcd).pSD).joinBevel
            } else {
                (*(*pcd).pSD).externalBevel
            }) as ::core::ffi::c_uint;
        } else {
            sTitleBevel = (if matte_width > 0 as ::core::ffi::c_int {
                insideBevel
            } else {
                (*(*pcd).pSD).externalBevel
            }) as ::core::ffi::c_uint;
        }
        eTitleBevel =
            (if decoration & (MWM_DECOR_MINIMIZE | MWM_DECOR_MAXIMIZE) as ::core::ffi::c_ulong != 0
                || (*pcd).decor & MWM_DECOR_TITLE != 0
                    && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
            {
                (*(*pcd).pSD).joinBevel
            } else {
                (*(*pcd).pSD).externalBevel
            }) as ::core::ffi::c_uint;
        wTitleBevel = (if decoration & MWM_DECOR_MENU as ::core::ffi::c_ulong != 0 {
            (*(*pcd).pSD).joinBevel
        } else {
            (*(*pcd).pSD).externalBevel
        }) as ::core::ffi::c_uint;
        meTitleBevel = (if decoration & MWM_DECOR_MAXIMIZE as ::core::ffi::c_ulong != 0
            || (*pcd).decor & MWM_DECOR_TITLE != 0
                && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
        {
            (*(*pcd).pSD).joinBevel
        } else {
            (*(*pcd).pSD).externalBevel
        }) as ::core::ffi::c_uint;
    }
    if decoration & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0 {
        if (*(*pcd).pSD).decoupleTitleAppearance != 0 {
            prlTop = (*pcd).pclientTitleTopShadows;
            prlBot = (*pcd).pclientTitleBottomShadows;
            yAdj = (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int;
            xAdj = yAdj;
        } else {
            prlTop = (*pcd).pclientTopShadows;
            prlBot = (*pcd).pclientBottomShadows;
            yAdj = 0 as ::core::ffi::c_int;
            xAdj = yAdj;
        }
        GetFramePartInfo(
            pcd,
            FRAME_TITLE,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        if (*pcd).decorFlags & TITLE_DEPRESSED != 0 {
            GetDepressInfo(
                pcd,
                FRAME_TITLE,
                &raw mut jX,
                &raw mut jY,
                &raw mut jW,
                &raw mut jH,
                &raw mut inWidth,
            );
            BevelDepressedRectangle(
                prlTop,
                prlBot,
                x - xAdj,
                y - yAdj,
                width,
                height,
                nTitleBevel,
                eTitleBevel,
                sTitleBevel,
                wTitleBevel,
                inWidth,
            );
        } else {
            BevelRectangle(
                prlTop,
                prlBot,
                x - xAdj,
                y - yAdj,
                width,
                height,
                nTitleBevel,
                eTitleBevel,
                sTitleBevel,
                wTitleBevel,
            );
        }
    }
    if decoration & MWM_DECOR_MENU as ::core::ffi::c_ulong != 0 {
        GetFramePartInfo(
            pcd,
            FRAME_SYSTEM,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        if (*pcd).decorFlags & SYSTEM_DEPRESSED != 0 {
            GetDepressInfo(
                pcd,
                FRAME_SYSTEM,
                &raw mut jX,
                &raw mut jY,
                &raw mut jW,
                &raw mut jH,
                &raw mut inWidth,
            );
            BevelDepressedRectangle(
                prlTop,
                prlBot,
                x - xAdj,
                y - yAdj,
                width,
                height,
                nTitleBevel,
                wTitleBevel,
                sTitleBevel,
                nTitleBevel,
                inWidth,
            );
        } else {
            BevelRectangle(
                prlTop,
                prlBot,
                x - xAdj,
                y - yAdj,
                width,
                height,
                nTitleBevel,
                wTitleBevel,
                sTitleBevel,
                nTitleBevel,
            );
        }
        BevelSystemButton(prlTop, prlBot, x - xAdj, y - yAdj, width, height);
    }
    if decoration & MWM_DECOR_MINIMIZE as ::core::ffi::c_ulong != 0 {
        GetFramePartInfo(
            pcd,
            FRAME_MINIMIZE,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        if (*pcd).decorFlags & MINIMIZE_DEPRESSED != 0 {
            GetDepressInfo(
                pcd,
                FRAME_MINIMIZE,
                &raw mut jX,
                &raw mut jY,
                &raw mut jW,
                &raw mut jH,
                &raw mut inWidth,
            );
            BevelDepressedRectangle(
                prlTop,
                prlBot,
                x - xAdj,
                y - yAdj,
                width,
                height,
                nTitleBevel,
                meTitleBevel,
                sTitleBevel,
                eTitleBevel,
                inWidth,
            );
        } else {
            BevelRectangle(
                prlTop,
                prlBot,
                x - xAdj,
                y - yAdj,
                width,
                height,
                nTitleBevel,
                meTitleBevel,
                sTitleBevel,
                eTitleBevel,
            );
        }
        BevelMinimizeButton(prlTop, prlBot, x - xAdj, y - yAdj, height);
    }
    if decoration & MWM_DECOR_MAXIMIZE as ::core::ffi::c_ulong != 0 {
        GetFramePartInfo(
            pcd,
            FRAME_MAXIMIZE,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        if (*pcd).decorFlags & MAXIMIZE_DEPRESSED != 0 {
            GetDepressInfo(
                pcd,
                FRAME_MAXIMIZE,
                &raw mut jX,
                &raw mut jY,
                &raw mut jW,
                &raw mut jH,
                &raw mut inWidth,
            );
            BevelDepressedRectangle(
                prlTop,
                prlBot,
                x - xAdj,
                y - yAdj,
                width,
                height,
                nTitleBevel,
                if (*pcd).decor & MWM_DECOR_TITLE != 0
                    && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
                {
                    (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint
                } else {
                    nTitleBevel
                },
                sTitleBevel,
                eTitleBevel,
                inWidth,
            );
        } else {
            BevelRectangle(
                prlTop,
                prlBot,
                x - xAdj,
                y - yAdj,
                width,
                height,
                nTitleBevel,
                if (*pcd).decor & MWM_DECOR_TITLE != 0
                    && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
                {
                    (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint
                } else {
                    nTitleBevel
                },
                sTitleBevel,
                eTitleBevel,
            );
        }
        if (*pcd).maxConfig != 0 {
            BevelMaximizeButton(prlBot, prlTop, x - xAdj, y - yAdj, height);
        } else {
            BevelMaximizeButton(prlTop, prlBot, x - xAdj, y - yAdj, height);
        }
    }
    if (*pcd).decor & MWM_DECOR_TITLE != 0
        && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
    {
        GetFramePartInfo(
            pcd,
            FRAME_CLOSE,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        if (*pcd).decorFlags & CLOSE_DEPRESSED != 0 {
            GetDepressInfo(
                pcd,
                FRAME_CLOSE,
                &raw mut jX,
                &raw mut jY,
                &raw mut jW,
                &raw mut jH,
                &raw mut inWidth,
            );
            BevelDepressedRectangle(
                prlTop,
                prlBot,
                x - xAdj,
                y - yAdj,
                width,
                height,
                nTitleBevel,
                nTitleBevel,
                sTitleBevel,
                (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint,
                inWidth,
            );
        } else {
            BevelRectangle(
                prlTop,
                prlBot,
                x - xAdj,
                y - yAdj,
                width,
                height,
                nTitleBevel,
                nTitleBevel,
                sTitleBevel,
                (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint,
            );
        }
        BevelCloseButton(prlTop, prlBot, x - xAdj, y - yAdj, height);
    }
    if matte_width > 0 as ::core::ffi::c_int {
        let mut mWidth: ::core::ffi::c_uint = 0;
        let mut mHeight: ::core::ffi::c_uint = 0;
        let mut exMatteBevel: ::core::ffi::c_uint = 0;
        let mut tMatteBevel: ::core::ffi::c_uint = 0;
        mWidth = BaseWindowWidth(pcd);
        mHeight = BaseWindowHeight(pcd);
        if decoration & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) as ::core::ffi::c_ulong != 0 {
            exMatteBevel = (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint;
            tMatteBevel = (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint;
        } else {
            exMatteBevel = (*(*pcd).pSD).externalBevel as ::core::ffi::c_uint;
            tMatteBevel = (if decoration & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0 {
                (*(*pcd).pSD).joinBevel
            } else {
                (*(*pcd).pSD).externalBevel
            }) as ::core::ffi::c_uint;
        }
        BevelRectangle(
            (*pcd).pclientMatteTopShadows,
            (*pcd).pclientMatteBottomShadows,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            mWidth,
            mHeight,
            tMatteBevel,
            exMatteBevel,
            exMatteBevel,
            exMatteBevel,
        );
        BevelRectangle(
            (*pcd).pclientMatteBottomShadows,
            (*pcd).pclientMatteTopShadows,
            matte_width - (*pcd).internalBevel,
            matte_width - (*pcd).internalBevel,
            mWidth
                .wrapping_sub((2 as ::core::ffi::c_int * matte_width) as ::core::ffi::c_uint)
                .wrapping_add(
                    (2 as ::core::ffi::c_int * (*pcd).internalBevel) as ::core::ffi::c_uint,
                ),
            mHeight
                .wrapping_sub((2 as ::core::ffi::c_int * matte_width) as ::core::ffi::c_uint)
                .wrapping_add(
                    (2 as ::core::ffi::c_int * (*pcd).internalBevel) as ::core::ffi::c_uint,
                ),
            (*pcd).internalBevel as ::core::ffi::c_uint,
            (*pcd).internalBevel as ::core::ffi::c_uint,
            (*pcd).internalBevel as ::core::ffi::c_uint,
            (*pcd).internalBevel as ::core::ffi::c_uint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn AdoptClient(mut pcd: *mut ClientData) {
    let mut windowChanges = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    let mut mask: ::core::ffi::c_uint = 0;
    if (*pcd).clientFlags & CLIENT_WM_CLIENTS == 0 {
        XChangeSaveSet(wmGD.display, (*pcd).client, SetModeInsert);
        (*pcd).clientFlags |= CLIENT_IN_SAVE_SET;
    }
    mask = (CWWidth | CWHeight) as ::core::ffi::c_uint;
    windowChanges.width = (*pcd).clientWidth;
    windowChanges.height = (*pcd).clientHeight;
    if (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) != 0
        || (*pcd).matteWidth > 0 as ::core::ffi::c_int
    {
        mask |= CWBorderWidth as ::core::ffi::c_uint;
        windowChanges.border_width = 0 as ::core::ffi::c_int;
    }
    XConfigureWindow(wmGD.display, (*pcd).client, mask, &raw mut windowChanges);
    if wmGD.hasShape != 0 {
        let mut xws: ::core::ffi::c_int = 0;
        let mut yws: ::core::ffi::c_int = 0;
        let mut xbs: ::core::ffi::c_int = 0;
        let mut ybs: ::core::ffi::c_int = 0;
        let mut wws: ::core::ffi::c_uint = 0;
        let mut hws: ::core::ffi::c_uint = 0;
        let mut wbs: ::core::ffi::c_uint = 0;
        let mut hbs: ::core::ffi::c_uint = 0;
        let mut boundingShaped: ::core::ffi::c_int = 0;
        let mut clipShaped: ::core::ffi::c_int = 0;
        XShapeSelectInput(
            wmGD.display,
            (*pcd).client,
            ShapeNotifyMask as ::core::ffi::c_ulong,
        );
        XShapeQueryExtents(
            wmGD.display,
            (*pcd).client,
            &raw mut boundingShaped,
            &raw mut xws,
            &raw mut yws,
            &raw mut wws,
            &raw mut hws,
            &raw mut clipShaped,
            &raw mut xbs,
            &raw mut ybs,
            &raw mut wbs,
            &raw mut hbs,
        );
        (*pcd).wShaped = boundingShaped as ::core::ffi::c_short;
    }
    XReparentWindow(
        wmGD.display,
        (*pcd).client,
        (*pcd).clientBaseWin,
        (*pcd).matteWidth,
        (*pcd).matteWidth,
    );
    (*pcd).clientFlags |= CLIENT_REPARENTED;
}
#[no_mangle]
pub unsafe extern "C" fn GetTextBox(mut pcd: *mut ClientData, mut pBox: *mut XRectangle) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    if GetFramePartInfo(
        pcd,
        FRAME_TITLE,
        &raw mut x,
        &raw mut y,
        &raw mut width,
        &raw mut height,
    ) == 0
    {
        (*pBox).x = 0 as ::core::ffi::c_short;
        (*pBox).y = 0 as ::core::ffi::c_short;
        (*pBox).width = 0 as ::core::ffi::c_ushort;
        (*pBox).height = 0 as ::core::ffi::c_ushort;
        return;
    }
    x += WM_TOP_TITLE_SHADOW + WM_TOP_TITLE_PADDING;
    y += WM_TOP_TITLE_SHADOW + WM_TOP_TITLE_PADDING;
    width = width.wrapping_sub(
        (WM_TOP_TITLE_SHADOW
            + WM_BOTTOM_TITLE_SHADOW
            + WM_TOP_TITLE_PADDING
            + WM_BOTTOM_TITLE_PADDING) as ::core::ffi::c_uint,
    );
    height = height.wrapping_sub(
        (WM_TOP_TITLE_SHADOW
            + WM_BOTTOM_TITLE_SHADOW
            + WM_TOP_TITLE_PADDING
            + WM_BOTTOM_TITLE_PADDING) as ::core::ffi::c_uint,
    );
    (*pBox).x = x as ::core::ffi::c_short;
    (*pBox).y = y as ::core::ffi::c_short;
    (*pBox).width = width as ::core::ffi::c_ushort;
    (*pBox).height = height as ::core::ffi::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn DrawWindowTitle(mut pcd: *mut ClientData, mut eraseFirst: Boolean) {
    let mut clientGC = ::core::ptr::null_mut::<_XGC>();
    let mut decoration = (*pcd).decor as ::core::ffi::c_ulong;
    let mut textBox = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut win: Window = 0;
    let mut renderTable = ::core::ptr::null_mut::<*mut __XmRenderTableRec>();
    let mut titlePixel: Pixel = 0;
    let mut useGradient: Boolean = 0;
    let mut oldGC = XGCValues {
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
    let mut gradientText = False as Boolean;
    let mut cleanText = (*wmGD.pActiveSD).cleanText;
    if decoration & MWM_DECOR_TITLE as ::core::ffi::c_ulong == 0 {
        return;
    }
    if (*(*pcd).pSD).decoupleTitleAppearance != 0 {
        if pcd == wmGD.keyboardFocus {
            clientGC = (*(*pcd).pSD).clientTitleAppearance.activeGC;
        } else {
            clientGC = (*(*pcd).pSD).clientTitleAppearance.inactiveGC;
        }
        GetTextBox(pcd, &raw mut textBox);
        textBox.x = (textBox.x as ::core::ffi::c_int
            - (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_short as ::core::ffi::c_int)
            as ::core::ffi::c_short;
        textBox.y = (textBox.y as ::core::ffi::c_int
            - (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_short as ::core::ffi::c_int)
            as ::core::ffi::c_short;
        win = (*pcd).clientTitleWin;
        renderTable = (*(*pcd).pSD).clientTitleAppearance.renderTable;
    } else {
        if pcd == wmGD.keyboardFocus {
            clientGC = (*(*pcd).pSD).clientAppearance.activeGC;
        } else {
            clientGC = (*(*pcd).pSD).clientAppearance.inactiveGC;
        }
        GetTextBox(pcd, &raw mut textBox);
        win = ClientDecorDrawable(pcd, False as Boolean) as Window;
        renderTable = (*(*pcd).pSD).clientAppearance.renderTable;
    }
    useGradient = EmwmTitleTextPixel(
        wmGD.display,
        (*(*pcd).pSD).screen,
        (*(*pcd).pSD).rootWindow,
        (pcd == wmGD.keyboardFocus) as ::core::ffi::c_int,
        &raw mut titlePixel,
    ) as Boolean;
    if eraseFirst != 0 {
        if useGradient != 0 {
            PaintTitleGradient(pcd, &raw mut textBox);
        } else if !(*pcd).argbFrame.is_null() && (*(*pcd).pSD).decoupleTitleAppearance == 0 {
            PaintArgbBackground(pcd, False as Boolean, &raw mut textBox);
        } else {
            XClearArea(
                wmGD.display,
                win,
                textBox.x as ::core::ffi::c_int,
                textBox.y as ::core::ffi::c_int,
                textBox.width as ::core::ffi::c_uint,
                textBox.height as ::core::ffi::c_uint,
                FALSE,
            );
        }
    }
    if (*(*pcd).pSD).titleLeft != 0 {
        let mut padding = (*(*pcd).pSD)
            .clientTitleAppearance
            .fontHeight
            .wrapping_div(3 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        textBox.x = (textBox.x as ::core::ffi::c_int + padding) as ::core::ffi::c_short;
        textBox.width = (textBox.width as ::core::ffi::c_int - padding) as ::core::ffi::c_ushort;
    }
    if useGradient as ::core::ffi::c_int != 0
        && XGetGCValues(
            wmGD.display,
            clientGC,
            GCForeground as ::core::ffi::c_ulong,
            &raw mut oldGC,
        ) != 0
    {
        gradientText = True as Boolean;
        (*wmGD.pActiveSD).cleanText = False as Boolean;
        XSetForeground(wmGD.display, clientGC, titlePixel as ::core::ffi::c_ulong);
    }
    WmDrawXmString(
        wmGD.display,
        win,
        renderTable as XmFontList,
        if !(*pcd).ewmhClientTitle.is_null() {
            (*pcd).ewmhClientTitle
        } else {
            (*pcd).clientTitle
        },
        clientGC,
        textBox.x as Position,
        textBox.y as Position,
        textBox.width as Dimension,
        &raw mut textBox,
        (if (*(*pcd).pSD).titleLeft as ::core::ffi::c_int != 0 {
            False
        } else {
            True
        }) as Boolean,
    );
    if gradientText != 0 {
        XSetForeground(wmGD.display, clientGC, oldGC.foreground);
        (*wmGD.pActiveSD).cleanText = cleanText;
    }
    if eraseFirst != 0 {
        PresentArgbDecoration(pcd, False as Boolean);
    }
}
unsafe extern "C" fn CreateStretcherWindows(mut pcd: *mut ClientData) {
    let mut iWin: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut win_attribs = XSetWindowAttributes {
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
    let mut attr_mask: ::core::ffi::c_ulong = 0;
    iWin = 0 as ::core::ffi::c_int;
    while iWin < STRETCH_COUNT {
        match iWin {
            STRETCH_NORTH_WEST => {
                GetFramePartInfo(
                    pcd,
                    FRAME_RESIZE_NW,
                    &raw mut x,
                    &raw mut y,
                    &raw mut width,
                    &raw mut height,
                );
            }
            STRETCH_NORTH => {
                GetFramePartInfo(
                    pcd,
                    FRAME_RESIZE_N,
                    &raw mut x,
                    &raw mut y,
                    &raw mut width,
                    &raw mut height,
                );
            }
            STRETCH_NORTH_EAST => {
                GetFramePartInfo(
                    pcd,
                    FRAME_RESIZE_NE,
                    &raw mut x,
                    &raw mut y,
                    &raw mut width,
                    &raw mut height,
                );
            }
            STRETCH_EAST => {
                GetFramePartInfo(
                    pcd,
                    FRAME_RESIZE_E,
                    &raw mut x,
                    &raw mut y,
                    &raw mut width,
                    &raw mut height,
                );
            }
            STRETCH_SOUTH_EAST => {
                GetFramePartInfo(
                    pcd,
                    FRAME_RESIZE_SE,
                    &raw mut x,
                    &raw mut y,
                    &raw mut width,
                    &raw mut height,
                );
            }
            STRETCH_SOUTH => {
                GetFramePartInfo(
                    pcd,
                    FRAME_RESIZE_S,
                    &raw mut x,
                    &raw mut y,
                    &raw mut width,
                    &raw mut height,
                );
            }
            STRETCH_SOUTH_WEST => {
                GetFramePartInfo(
                    pcd,
                    FRAME_RESIZE_SW,
                    &raw mut x,
                    &raw mut y,
                    &raw mut width,
                    &raw mut height,
                );
            }
            STRETCH_WEST => {
                GetFramePartInfo(
                    pcd,
                    FRAME_RESIZE_W,
                    &raw mut x,
                    &raw mut y,
                    &raw mut width,
                    &raw mut height,
                );
            }
            _ => {}
        }
        attr_mask = CWCursor as ::core::ffi::c_ulong;
        win_attribs.cursor = wmGD.stretchCursors[iWin as usize];
        (*pcd).clientStretchWin[iWin as usize] = XCreateWindow(
            wmGD.display,
            (*pcd).clientFrameWin,
            x,
            y,
            width,
            height,
            0 as ::core::ffi::c_uint,
            CopyFromParent as ::core::ffi::c_int,
            InputOnly as ::core::ffi::c_uint,
            ::core::ptr::null_mut::<Visual>(),
            attr_mask,
            &raw mut win_attribs,
        );
        iWin += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CountFrameRectangles(mut pSD: *mut WmScreenData) {
    let mut i: ::core::ffi::c_int = 0;
    (*pSD).Num_Title_Bs_Elements = 0 as ::core::ffi::c_int;
    (*pSD).Num_Title_Ts_Elements = (*pSD).Num_Title_Bs_Elements;
    i = FRAME_SYSTEM;
    while i <= FRAME_MAXIMIZE {
        (*pSD).Num_Title_Ts_Elements += Bevels[i as usize].top.external * (*pSD).externalBevel
            + Bevels[i as usize].top.internal * MAX_INTERNAL_BEVEL
            + Bevels[i as usize].top.join * (*pSD).joinBevel;
        (*pSD).Num_Title_Bs_Elements += Bevels[i as usize].bottom.external * (*pSD).externalBevel
            + Bevels[i as usize].bottom.internal * MAX_INTERNAL_BEVEL
            + Bevels[i as usize].bottom.join * (*pSD).joinBevel;
        i += 1;
    }
    (*pSD).Num_Title_Ts_Elements += 4 as ::core::ffi::c_int * (*pSD).joinBevel;
    (*pSD).Num_Title_Bs_Elements += 3 as ::core::ffi::c_int * (*pSD).joinBevel + MAX_INTERNAL_BEVEL;
    (*pSD).Num_Resize_Bs_Elements = 0 as ::core::ffi::c_int;
    (*pSD).Num_Resize_Ts_Elements = (*pSD).Num_Resize_Bs_Elements;
    i = FRAME_RESIZE_NW;
    while i <= FRAME_RESIZE_W {
        (*pSD).Num_Resize_Ts_Elements += Bevels[i as usize].top.external * (*pSD).externalBevel
            + Bevels[i as usize].top.internal * MAX_INTERNAL_BEVEL
            + Bevels[i as usize].top.join * (*pSD).joinBevel;
        (*pSD).Num_Resize_Bs_Elements += Bevels[i as usize].bottom.external * (*pSD).externalBevel
            + Bevels[i as usize].bottom.internal * MAX_INTERNAL_BEVEL
            + Bevels[i as usize].bottom.join * (*pSD).joinBevel;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn AllocateFrameDisplayLists(mut pcd: *mut ClientData) -> Boolean {
    let mut frame_top_count: ::core::ffi::c_int = 0;
    let mut frame_bottom_count: ::core::ffi::c_int = 0;
    if (*(*pcd).pSD).decoupleTitleAppearance as ::core::ffi::c_int != 0
        && (*pcd).decor & MWM_DECOR_TITLE != 0
        && (*pcd).pclientTitleTopShadows.is_null()
    {
        (*pcd).pclientTitleTopShadows =
            AllocateRList((*(*pcd).pSD).Num_Title_Ts_Elements as ::core::ffi::c_uint);
        if (*pcd).pclientTitleTopShadows.is_null() || {
            (*pcd).pclientTitleBottomShadows =
                AllocateRList((*(*pcd).pSD).Num_Title_Bs_Elements as ::core::ffi::c_uint);
            (*pcd).pclientTitleBottomShadows.is_null()
        } {
            Warning(
                b"Insufficient memory for client window framing\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return 0 as Boolean;
        }
        frame_top_count = (*(*pcd).pSD).Num_Resize_Ts_Elements;
        frame_bottom_count = (*(*pcd).pSD).Num_Resize_Bs_Elements;
    } else {
        frame_top_count =
            (*(*pcd).pSD).Num_Resize_Ts_Elements + (*(*pcd).pSD).Num_Title_Ts_Elements;
        frame_bottom_count =
            (*(*pcd).pSD).Num_Resize_Bs_Elements + (*(*pcd).pSD).Num_Resize_Bs_Elements;
    }
    if (*pcd).pclientTopShadows.is_null() && {
        (*pcd).pclientTopShadows = AllocateRList(frame_top_count as ::core::ffi::c_uint);
        (*pcd).pclientTopShadows.is_null()
    } {
        Warning(
            b"Insufficient memory for client window framing\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as Boolean;
    }
    if (*pcd).pclientBottomShadows.is_null() && {
        (*pcd).pclientBottomShadows = AllocateRList(frame_bottom_count as ::core::ffi::c_uint);
        (*pcd).pclientBottomShadows.is_null()
    } {
        Warning(
            b"Insufficient memory for client window framing\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as Boolean;
    }
    if (*pcd).matteWidth != 0 && (*pcd).pclientMatteTopShadows.is_null() && {
        (*pcd).pclientMatteTopShadows = AllocateRList(NUM_MATTE_TS_RECTS as ::core::ffi::c_uint);
        (*pcd).pclientMatteTopShadows.is_null()
    } {
        Warning(
            b"Insufficient memory for client window framing\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as Boolean;
    }
    if (*pcd).matteWidth != 0 && (*pcd).pclientMatteBottomShadows.is_null() && {
        (*pcd).pclientMatteBottomShadows = AllocateRList(NUM_MATTE_BS_RECTS as ::core::ffi::c_uint);
        (*pcd).pclientMatteBottomShadows.is_null()
    } {
        Warning(
            b"Insufficient memory for client window framing\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as Boolean;
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn InitClientDecoration(mut pSD: *mut WmScreenData) {
    CountFrameRectangles(pSD);
}
unsafe extern "C" fn AllocateGadgetRectangles(mut pcd: *mut ClientData) -> Boolean {
    let mut decor = (*pcd).decor as ::core::ffi::c_ulong;
    let mut pgr = ::core::ptr::null_mut::<GadgetRectangle>();
    if decor & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0 {
        if (*pcd).pTitleGadgets.is_null() {
            pgr = XtMalloc(
                (NUM_TITLE_GADGET_RECTS as usize)
                    .wrapping_mul(::core::mem::size_of::<GadgetRectangle>() as usize)
                    as Cardinal,
            ) as *mut GadgetRectangle;
            if pgr.is_null() {
                Warning(
                    b"Insufficient memory for client window framing\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 0 as Boolean;
            }
            (*pcd).pTitleGadgets = pgr;
            (*pcd).cTitleGadgets = 0 as ::core::ffi::c_int;
        }
    }
    if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
        if (*pcd).pResizeGadgets.is_null() {
            pgr = XtMalloc(
                (STRETCH_COUNT as usize)
                    .wrapping_mul(::core::mem::size_of::<GadgetRectangle>() as usize)
                    as Cardinal,
            ) as *mut GadgetRectangle;
            if pgr.is_null() {
                Warning(
                    b"Insufficient memory for client window framing\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 0 as Boolean;
            }
            (*pcd).pResizeGadgets = pgr;
        }
    }
    return 1 as Boolean;
}
unsafe extern "C" fn ComputeGadgetRectangles(mut pcd: *mut ClientData) {
    let mut decor = (*pcd).decor as ::core::ffi::c_ulong;
    let mut pgr = ::core::ptr::null_mut::<GadgetRectangle>();
    let mut fpX: ::core::ffi::c_int = 0;
    let mut fpY: ::core::ffi::c_int = 0;
    let mut fpWidth: ::core::ffi::c_uint = 0;
    let mut fpHeight: ::core::ffi::c_uint = 0;
    let mut igr: ::core::ffi::c_int = 0;
    let mut clientWidth = if (*pcd).maxConfig as ::core::ffi::c_int != 0 {
        (*pcd).maxWidth
    } else {
        (*pcd).clientWidth
    };
    if decor & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0 {
        pgr = (*pcd).pTitleGadgets;
        if pgr.is_null() {
            return;
        }
        (*pcd).titleRectangle.x = (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_short;
        (*pcd).titleRectangle.y = (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_short;
        (*pcd).titleRectangle.width = (clientWidth
            + (if XBorderIsShowing(pcd) as ::core::ffi::c_int != 0 {
                2 as ::core::ffi::c_int * (*pcd).xBorderWidth
            } else {
                2 as ::core::ffi::c_int * (*pcd).matteWidth
            })) as ::core::ffi::c_ushort;
        (*pcd).titleRectangle.height = (*pcd).frameInfo.titleBarHeight as ::core::ffi::c_ushort;
        igr = 0 as ::core::ffi::c_int;
        (*pgr.offset(igr as isize)).id = FRAME_TITLE as ::core::ffi::c_short;
        GetFramePartInfo(
            pcd,
            FRAME_TITLE,
            &raw mut fpX,
            &raw mut fpY,
            &raw mut fpWidth,
            &raw mut fpHeight,
        );
        (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
        (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
        (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
        (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
        igr += 1 as ::core::ffi::c_int;
        if decor & MWM_DECOR_MENU as ::core::ffi::c_ulong != 0 {
            (*pgr.offset(igr as isize)).id = FRAME_SYSTEM as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_SYSTEM,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
            (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
            igr += 1 as ::core::ffi::c_int;
        }
        if decor & MWM_DECOR_MINIMIZE as ::core::ffi::c_ulong != 0 {
            (*pgr.offset(igr as isize)).id = FRAME_MINIMIZE as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_MINIMIZE,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
            (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
            igr += 1 as ::core::ffi::c_int;
        }
        if decor & MWM_DECOR_MAXIMIZE as ::core::ffi::c_ulong != 0 {
            (*pgr.offset(igr as isize)).id = FRAME_MAXIMIZE as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_MAXIMIZE,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
            (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
            igr += 1 as ::core::ffi::c_int;
        }
        if (*pcd).decor & MWM_DECOR_TITLE != 0
            && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
        {
            (*pgr.offset(igr as isize)).id = FRAME_CLOSE as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_CLOSE,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
            (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
            igr += 1 as ::core::ffi::c_int;
        }
        (*pcd).pTitleGadgets = pgr;
        (*pcd).cTitleGadgets = igr;
    }
    if decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) as ::core::ffi::c_ulong != 0 {
        (*pcd).matteRectangle.x = (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_short;
        (*pcd).matteRectangle.y = (*pcd)
            .frameInfo
            .upperBorderWidth
            .wrapping_add((*pcd).frameInfo.titleBarHeight)
            as ::core::ffi::c_short;
        (*pcd).matteRectangle.width = (*pcd).frameInfo.width.wrapping_sub(
            (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.lowerBorderWidth),
        ) as ::core::ffi::c_ushort;
        (*pcd).matteRectangle.height = (*pcd)
            .frameInfo
            .height
            .wrapping_sub((*pcd).frameInfo.upperBorderWidth)
            .wrapping_sub((*pcd).frameInfo.lowerBorderWidth)
            .wrapping_sub((*pcd).frameInfo.titleBarHeight)
            as ::core::ffi::c_ushort;
    } else {
        (*pcd).matteRectangle.x = 0 as ::core::ffi::c_short;
        (*pcd).matteRectangle.y = (*pcd).frameInfo.titleBarHeight as ::core::ffi::c_short;
        (*pcd).matteRectangle.width = (*pcd).frameInfo.width as ::core::ffi::c_ushort;
        (*pcd).matteRectangle.height = (*pcd)
            .frameInfo
            .height
            .wrapping_sub((*pcd).frameInfo.titleBarHeight)
            as ::core::ffi::c_ushort;
    }
    if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
        pgr = (*pcd).pResizeGadgets;
        if pgr.is_null() {
            return;
        }
        igr = 0 as ::core::ffi::c_int;
        if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
            (*pgr.offset(igr as isize)).id = FRAME_RESIZE_NW as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_RESIZE_NW,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
            (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
            igr += 1 as ::core::ffi::c_int;
            (*pgr.offset(igr as isize)).id = FRAME_RESIZE_N as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_RESIZE_N,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            if fpWidth as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
                (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
                (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
                (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
                igr += 1 as ::core::ffi::c_int;
            }
            (*pgr.offset(igr as isize)).id = FRAME_RESIZE_NE as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_RESIZE_NE,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
            (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
            igr += 1 as ::core::ffi::c_int;
            (*pgr.offset(igr as isize)).id = FRAME_RESIZE_W as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_RESIZE_W,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            if fpHeight as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
                (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
                (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
                (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
                igr += 1 as ::core::ffi::c_int;
            }
            (*pgr.offset(igr as isize)).id = FRAME_RESIZE_E as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_RESIZE_E,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            if fpHeight as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
                (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
                (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
                (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
                igr += 1 as ::core::ffi::c_int;
            }
            (*pgr.offset(igr as isize)).id = FRAME_RESIZE_SW as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_RESIZE_SW,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
            (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
            igr += 1 as ::core::ffi::c_int;
            (*pgr.offset(igr as isize)).id = FRAME_RESIZE_S as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_RESIZE_S,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            if fpWidth as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
                (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
                (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
                (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
                igr += 1 as ::core::ffi::c_int;
            }
            (*pgr.offset(igr as isize)).id = FRAME_RESIZE_SE as ::core::ffi::c_short;
            GetFramePartInfo(
                pcd,
                FRAME_RESIZE_SE,
                &raw mut fpX,
                &raw mut fpY,
                &raw mut fpWidth,
                &raw mut fpHeight,
            );
            (*pgr.offset(igr as isize)).rect.x = fpX as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.y = fpY as ::core::ffi::c_short;
            (*pgr.offset(igr as isize)).rect.width = fpWidth as ::core::ffi::c_ushort;
            (*pgr.offset(igr as isize)).rect.height = fpHeight as ::core::ffi::c_ushort;
        }
        (*pcd).pResizeGadgets = pgr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetSystemMenuPosition(
    mut pcd: *mut ClientData,
    mut px: *mut ::core::ffi::c_int,
    mut py: *mut ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
    mut context: Context,
) {
    let mut xsi = XineramaScreenInfo {
        screen_number: 0,
        x_org: 0 as ::core::ffi::c_short,
        y_org: 0 as ::core::ffi::c_short,
        width: XDisplayWidth(wmGD.display, (*(*pcd).pSD).screen) as ::core::ffi::c_short,
        height: XDisplayHeight(wmGD.display, (*(*pcd).pSD).screen) as ::core::ffi::c_short,
    };
    if (*pcd).clientState == MINIMIZED_STATE
        || (*pcd).clientState != MINIMIZED_STATE && context == F_SUBCONTEXT_IB_WICON as Context
    {
        if (*(*pcd).pSD).useIconBox as ::core::ffi::c_int != 0
            && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                .pIconBox
                .is_null()
        {
            GetIconBoxIconRootXY(pcd, px, py);
            GetXineramaScreenFromLocation(*px, *py, &raw mut xsi);
            wmGD.hotspotRectangle.x = *px as ::core::ffi::c_short;
            wmGD.hotspotRectangle.y = *py as ::core::ffi::c_short;
            *py = (*py as ::core::ffi::c_uint).wrapping_sub(height) as ::core::ffi::c_int
                as ::core::ffi::c_int;
            if *py < 0 as ::core::ffi::c_int {
                *py = (*py as ::core::ffi::c_uint).wrapping_add(
                    height.wrapping_add((*(*pcd).pSD).iconHeight as ::core::ffi::c_uint),
                ) as ::core::ffi::c_int as ::core::ffi::c_int;
                if (*py as ::core::ffi::c_uint).wrapping_add(height)
                    >= (xsi.y_org as ::core::ffi::c_int + xsi.height as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                {
                    wmGD.checkHotspot = FALSE as Boolean;
                }
            }
        } else {
            *px = (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconX;
            *py = ((*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconY
                as ::core::ffi::c_uint)
                .wrapping_sub(height) as ::core::ffi::c_int;
            GetXineramaScreenFromLocation(*px, *py, &raw mut xsi);
            if *py < 0 as ::core::ffi::c_int {
                *py = (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconY
                    + (*(*pcd).pSD).iconHeight;
                if (*py as ::core::ffi::c_uint).wrapping_add(height)
                    >= (xsi.y_org as ::core::ffi::c_int + xsi.height as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                {
                    wmGD.checkHotspot = FALSE as Boolean;
                }
            }
            wmGD.hotspotRectangle.x =
                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconX as ::core::ffi::c_short;
            wmGD.hotspotRectangle.y =
                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconY as ::core::ffi::c_short;
        }
        if (*px as ::core::ffi::c_uint).wrapping_add(width)
            > (xsi.x_org as ::core::ffi::c_int + xsi.width as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            *px = (*px as ::core::ffi::c_uint).wrapping_sub(
                (*px as ::core::ffi::c_uint)
                    .wrapping_add(width)
                    .wrapping_sub(
                        (xsi.x_org as ::core::ffi::c_int + xsi.width as ::core::ffi::c_int)
                            as ::core::ffi::c_uint,
                    ),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        wmGD.hotspotRectangle.width = (*(*pcd).pSD).iconWidth as ::core::ffi::c_ushort;
        wmGD.hotspotRectangle.height = (*(*pcd).pSD).iconHeight as ::core::ffi::c_ushort;
    } else {
        GetXineramaScreenFromLocation((*pcd).clientX, (*pcd).clientY, &raw mut xsi);
        if (*pcd).fullScreen != 0 {
            *px = (*pcd).fullScreenX;
            *py = (*pcd).fullScreenY;
        } else if (*pcd).decor & MWM_DECOR_TITLE != 0
            && (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) == 0
        {
            *px = (*pcd).frameInfo.x;
            *py = ((*pcd).frameInfo.y as ::core::ffi::c_uint)
                .wrapping_add((*pcd).frameInfo.titleBarHeight)
                as ::core::ffi::c_int;
        } else {
            *px = ((*pcd).frameInfo.x as ::core::ffi::c_uint)
                .wrapping_add((*pcd).frameInfo.lowerBorderWidth)
                as ::core::ffi::c_int;
            *py = ((*pcd).frameInfo.y as ::core::ffi::c_uint)
                .wrapping_add((*pcd).frameInfo.upperBorderWidth)
                .wrapping_add((*pcd).frameInfo.titleBarHeight)
                as ::core::ffi::c_int;
        }
        if (*py as ::core::ffi::c_uint).wrapping_add(height)
            >= (xsi.y_org as ::core::ffi::c_int + xsi.height as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            if (*pcd).decor & MWM_DECOR_TITLE != 0
                && (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) == 0
            {
                *py = ((*pcd).frameInfo.y as ::core::ffi::c_uint).wrapping_sub(height)
                    as ::core::ffi::c_int;
            } else {
                *py = ((*pcd).frameInfo.y as ::core::ffi::c_uint)
                    .wrapping_add((*pcd).frameInfo.upperBorderWidth)
                    .wrapping_sub(height) as ::core::ffi::c_int;
            }
            if *py < 0 as ::core::ffi::c_int {
                wmGD.checkHotspot = FALSE as Boolean;
            }
        }
        if (*px as ::core::ffi::c_uint).wrapping_add(width)
            > (xsi.x_org as ::core::ffi::c_int + xsi.width as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            *px = (*px as ::core::ffi::c_uint).wrapping_sub(
                (*px as ::core::ffi::c_uint)
                    .wrapping_add(width)
                    .wrapping_sub(
                        (xsi.x_org as ::core::ffi::c_int + xsi.width as ::core::ffi::c_int)
                            as ::core::ffi::c_uint,
                    ),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        wmGD.hotspotRectangle.x = ((*pcd).frameInfo.x as ::core::ffi::c_uint)
            .wrapping_add((*pcd).frameInfo.lowerBorderWidth)
            as ::core::ffi::c_short;
        wmGD.hotspotRectangle.y = ((*pcd).frameInfo.y as ::core::ffi::c_uint)
            .wrapping_add((*pcd).frameInfo.upperBorderWidth)
            as ::core::ffi::c_short;
        wmGD.hotspotRectangle.width = (*pcd).frameInfo.titleBarHeight as ::core::ffi::c_ushort;
        wmGD.hotspotRectangle.height = (*pcd).frameInfo.titleBarHeight as ::core::ffi::c_ushort;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ShowActiveClientFrame(mut pcd: *mut ClientData) {
    let mut attr_mask = 0 as ::core::ffi::c_ulong;
    let mut window_attribs = XSetWindowAttributes {
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
    if (*(*pcd).pSD).decoupleTitleAppearance as ::core::ffi::c_int != 0
        && (*pcd).decor & MWM_DECOR_TITLE != 0
    {
        if (*(*pcd).pSD).clientTitleAppearance.activeBackgroundPixmap != 0 {
            attr_mask |= CWBackPixmap as ::core::ffi::c_ulong;
            window_attribs.background_pixmap =
                (*(*pcd).pSD).clientTitleAppearance.activeBackgroundPixmap;
        } else {
            attr_mask |= CWBackPixel as ::core::ffi::c_ulong;
            window_attribs.background_pixel =
                (*(*pcd).pSD).clientTitleAppearance.activeBackground as ::core::ffi::c_ulong;
        }
        XChangeWindowAttributes(
            wmGD.display,
            (*pcd).clientTitleWin,
            attr_mask,
            &raw mut window_attribs,
        );
        XClearWindow(wmGD.display, (*pcd).clientTitleWin);
    }
    if (*(*pcd).pSD).clientAppearance.activeBackgroundPixmap != 0 {
        attr_mask |= CWBackPixmap as ::core::ffi::c_ulong;
        window_attribs.background_pixmap = (*(*pcd).pSD).clientAppearance.activeBackgroundPixmap;
    } else {
        attr_mask |= CWBackPixel as ::core::ffi::c_ulong;
        window_attribs.background_pixel =
            (*(*pcd).pSD).clientAppearance.activeBackground as ::core::ffi::c_ulong;
    }
    if (*pcd).argbFrame.is_null() {
        XChangeWindowAttributes(
            wmGD.display,
            (*pcd).clientFrameWin,
            attr_mask,
            &raw mut window_attribs,
        );
    }
    XClearWindow(wmGD.display, (*pcd).clientFrameWin);
    FrameExposureProc(pcd);
}
#[no_mangle]
pub unsafe extern "C" fn ShowInactiveClientFrame(mut pcd: *mut ClientData) {
    let mut attr_mask = 0 as ::core::ffi::c_ulong;
    let mut window_attribs = XSetWindowAttributes {
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
    if (*(*pcd).pSD).decoupleTitleAppearance as ::core::ffi::c_int != 0
        && (*pcd).decor & MWM_DECOR_TITLE != 0
    {
        if (*(*pcd).pSD).clientTitleAppearance.backgroundPixmap != 0 {
            attr_mask |= CWBackPixmap as ::core::ffi::c_ulong;
            window_attribs.background_pixmap = (*(*pcd).pSD).clientTitleAppearance.backgroundPixmap;
        } else {
            attr_mask |= CWBackPixel as ::core::ffi::c_ulong;
            window_attribs.background_pixel =
                (*(*pcd).pSD).clientTitleAppearance.background as ::core::ffi::c_ulong;
        }
        XChangeWindowAttributes(
            wmGD.display,
            (*pcd).clientTitleWin,
            attr_mask,
            &raw mut window_attribs,
        );
        XClearWindow(wmGD.display, (*pcd).clientTitleWin);
        attr_mask = 0 as ::core::ffi::c_ulong;
    }
    if (*(*pcd).pSD).clientAppearance.backgroundPixmap != 0 {
        attr_mask |= CWBackPixmap as ::core::ffi::c_ulong;
        window_attribs.background_pixmap = (*(*pcd).pSD).clientAppearance.backgroundPixmap;
    } else {
        attr_mask |= CWBackPixel as ::core::ffi::c_ulong;
        window_attribs.background_pixel =
            (*(*pcd).pSD).clientAppearance.background as ::core::ffi::c_ulong;
    }
    if (*pcd).argbFrame.is_null() {
        XChangeWindowAttributes(
            wmGD.display,
            (*pcd).clientFrameWin,
            attr_mask,
            &raw mut window_attribs,
        );
    }
    XClearWindow(wmGD.display, (*pcd).clientFrameWin);
    FrameExposureProc(pcd);
}
#[no_mangle]
pub unsafe extern "C" fn RegenerateClientFrame(mut pcd: *mut ClientData) {
    let mut decor = (*pcd).decor as ::core::ffi::c_ulong;
    SetFrameInfo(pcd);
    XMoveResizeWindow(
        wmGD.display,
        (*pcd).clientFrameWin,
        (*pcd).frameInfo.x,
        (*pcd).frameInfo.y,
        (*pcd).frameInfo.width,
        (*pcd).frameInfo.height,
    );
    if decor & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0 {
        XMoveResizeWindow(
            wmGD.display,
            (*pcd).clientTitleWin,
            (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int,
            (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int,
            (*pcd).frameInfo.width.wrapping_sub(
                (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.upperBorderWidth),
            ),
            (*pcd).frameInfo.titleBarHeight,
        );
    }
    XResizeWindow(
        wmGD.display,
        (*pcd).clientBaseWin,
        BaseWindowWidth(pcd),
        BaseWindowHeight(pcd),
    );
    if (*(*pcd).pSD).resizeCursors as ::core::ffi::c_int != 0
        && decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0
    {
        XMoveResizeWindow(
            wmGD.display,
            (*pcd).clientStretchWin[STRETCH_NORTH_WEST as usize],
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*pcd).frameInfo.cornerWidth,
            (*pcd).frameInfo.cornerHeight,
        );
        XMoveResizeWindow(
            wmGD.display,
            (*pcd).clientStretchWin[STRETCH_NORTH as usize],
            (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*pcd).frameInfo.width.wrapping_sub(
                (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerWidth),
            ),
            (*pcd).frameInfo.upperBorderWidth,
        );
        XMoveResizeWindow(
            wmGD.display,
            (*pcd).clientStretchWin[STRETCH_NORTH_EAST as usize],
            (*pcd)
                .frameInfo
                .width
                .wrapping_sub((*pcd).frameInfo.cornerWidth) as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*pcd).frameInfo.cornerWidth,
            (*pcd).frameInfo.cornerHeight,
        );
        XMoveResizeWindow(
            wmGD.display,
            (*pcd).clientStretchWin[STRETCH_EAST as usize],
            (*pcd)
                .frameInfo
                .width
                .wrapping_sub((*pcd).frameInfo.lowerBorderWidth) as ::core::ffi::c_int,
            (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int,
            (*pcd).frameInfo.lowerBorderWidth,
            (*pcd).frameInfo.height.wrapping_sub(
                (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerHeight),
            ),
        );
        XMoveResizeWindow(
            wmGD.display,
            (*pcd).clientStretchWin[STRETCH_SOUTH_EAST as usize],
            (*pcd)
                .frameInfo
                .width
                .wrapping_sub((*pcd).frameInfo.cornerWidth) as ::core::ffi::c_int,
            (*pcd)
                .frameInfo
                .height
                .wrapping_sub((*pcd).frameInfo.cornerHeight) as ::core::ffi::c_int,
            (*pcd).frameInfo.cornerWidth,
            (*pcd).frameInfo.cornerHeight,
        );
        XMoveResizeWindow(
            wmGD.display,
            (*pcd).clientStretchWin[STRETCH_SOUTH as usize],
            (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int,
            (*pcd)
                .frameInfo
                .height
                .wrapping_sub((*pcd).frameInfo.lowerBorderWidth) as ::core::ffi::c_int,
            (*pcd).frameInfo.width.wrapping_sub(
                (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerWidth),
            ),
            (*pcd).frameInfo.lowerBorderWidth,
        );
        XMoveResizeWindow(
            wmGD.display,
            (*pcd).clientStretchWin[STRETCH_SOUTH_WEST as usize],
            0 as ::core::ffi::c_int,
            (*pcd)
                .frameInfo
                .height
                .wrapping_sub((*pcd).frameInfo.cornerHeight) as ::core::ffi::c_int,
            (*pcd).frameInfo.cornerWidth,
            (*pcd).frameInfo.cornerHeight,
        );
        XMoveResizeWindow(
            wmGD.display,
            (*pcd).clientStretchWin[STRETCH_WEST as usize],
            0 as ::core::ffi::c_int,
            (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int,
            (*pcd).frameInfo.lowerBorderWidth,
            (*pcd).frameInfo.height.wrapping_sub(
                (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerHeight),
            ),
        );
    }
    ComputeGadgetRectangles(pcd);
    GenerateFrameDisplayLists(pcd);
    if wmGD.hasShape as ::core::ffi::c_int != 0 && (*pcd).wShaped as ::core::ffi::c_int != 0 {
        SetFrameShape(pcd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BevelSystemButton(
    mut prTop: *mut RList,
    mut prBot: *mut RList,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
) {
    let mut offset1: ::core::ffi::c_int = 0;
    let mut offset2: ::core::ffi::c_int = 0;
    let mut dim1: ::core::ffi::c_uint = 0;
    let mut dim2: ::core::ffi::c_uint = 0;
    match height {
        5 | 6 => {
            offset2 = 2 as ::core::ffi::c_int;
            offset1 = offset2;
            dim2 = height.wrapping_sub(4 as ::core::ffi::c_uint);
            dim1 = dim2;
        }
        7 => {
            offset2 = 2 as ::core::ffi::c_int;
            offset1 = offset2;
            dim1 = 3 as ::core::ffi::c_uint;
            dim2 = 2 as ::core::ffi::c_uint;
        }
        8 | 9 => {
            offset1 = 2 as ::core::ffi::c_int;
            offset2 = 3 as ::core::ffi::c_int;
            dim1 = width.wrapping_sub(4 as ::core::ffi::c_uint);
            dim2 = height.wrapping_sub(6 as ::core::ffi::c_uint);
        }
        10 | 11 => {
            offset1 = 3 as ::core::ffi::c_int;
            offset2 = 4 as ::core::ffi::c_int;
            dim1 = width.wrapping_sub(6 as ::core::ffi::c_uint);
            dim2 = height.wrapping_sub(8 as ::core::ffi::c_uint);
        }
        12 | 13 => {
            offset1 = 3 as ::core::ffi::c_int;
            offset2 = height
                .wrapping_sub(3 as ::core::ffi::c_uint)
                .wrapping_div(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            dim1 = width.wrapping_sub(6 as ::core::ffi::c_uint);
            dim2 = 3 as ::core::ffi::c_uint;
        }
        _ => {
            offset1 = 4 as ::core::ffi::c_int;
            offset2 = height
                .wrapping_sub(4 as ::core::ffi::c_uint)
                .wrapping_div(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            dim1 = width.wrapping_sub(8 as ::core::ffi::c_uint);
            dim2 = 4 as ::core::ffi::c_uint;
        }
    }
    if height >= 5 as ::core::ffi::c_uint {
        BevelRectangle(
            prTop,
            prBot,
            x + offset1,
            y + offset2,
            dim1,
            dim2,
            1 as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn BevelMinimizeButton(
    mut prTop: *mut RList,
    mut prBot: *mut RList,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut height: ::core::ffi::c_uint,
) {
    let mut offset1: ::core::ffi::c_int = 0;
    let mut offset2: ::core::ffi::c_int = 0;
    let mut dim1: ::core::ffi::c_uint = 0;
    let mut dim2: ::core::ffi::c_uint = 0;
    match height {
        7 | 8 | 9 => {
            offset2 = 3 as ::core::ffi::c_int;
            offset1 = offset2;
            dim2 = height.wrapping_sub(6 as ::core::ffi::c_uint);
            dim1 = dim2;
        }
        10 | 11 | 12 => {
            offset2 = height
                .wrapping_sub(3 as ::core::ffi::c_uint)
                .wrapping_div(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            offset1 = offset2;
            dim2 = 3 as ::core::ffi::c_uint;
            dim1 = dim2;
        }
        _ => {
            offset2 = height
                .wrapping_sub(4 as ::core::ffi::c_uint)
                .wrapping_div(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            offset1 = offset2;
            dim2 = 4 as ::core::ffi::c_uint;
            dim1 = dim2;
        }
    }
    if height >= 7 as ::core::ffi::c_uint {
        BevelRectangle(
            prTop,
            prBot,
            x + offset1,
            y + offset2,
            dim1,
            dim2,
            1 as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn BevelMaximizeButton(
    mut prTop: *mut RList,
    mut prBot: *mut RList,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut height: ::core::ffi::c_uint,
) {
    let mut offset1: ::core::ffi::c_int = 0;
    let mut offset2: ::core::ffi::c_int = 0;
    let mut dim1: ::core::ffi::c_uint = 0;
    let mut dim2: ::core::ffi::c_uint = 0;
    match height {
        5 | 6 | 7 | 8 | 9 | 10 | 11 => {
            offset2 = 2 as ::core::ffi::c_int;
            offset1 = offset2;
            dim2 = height.wrapping_sub(4 as ::core::ffi::c_uint);
            dim1 = dim2;
        }
        12 | 13 | 14 | 15 => {
            offset2 = 3 as ::core::ffi::c_int;
            offset1 = offset2;
            dim2 = height.wrapping_sub(6 as ::core::ffi::c_uint);
            dim1 = dim2;
        }
        _ => {
            offset2 = 4 as ::core::ffi::c_int;
            offset1 = offset2;
            dim2 = height.wrapping_sub(8 as ::core::ffi::c_uint);
            dim1 = dim2;
        }
    }
    BevelRectangle(
        prTop,
        prBot,
        x + offset1,
        y + offset2,
        dim1,
        dim2,
        1 as ::core::ffi::c_uint,
        1 as ::core::ffi::c_uint,
        1 as ::core::ffi::c_uint,
        1 as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn CloseButtonPixel(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
) -> Boolean {
    return (x >= 0 as ::core::ffi::c_int
        && y >= 0 as ::core::ffi::c_int
        && x < size
        && y < size
        && (abs(x - y) <= 1 as ::core::ffi::c_int
            || abs(x + y - size + 1 as ::core::ffi::c_int) <= 1 as ::core::ffi::c_int))
        as ::core::ffi::c_int as Boolean;
}
unsafe extern "C" fn BevelCloseButton(
    mut prTop: *mut RList,
    mut prBot: *mut RList,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut height: ::core::ffi::c_uint,
) {
    let mut row: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_uint = 0;
    if height < 7 as ::core::ffi::c_uint {
        return;
    }
    offset = if height < 12 as ::core::ffi::c_uint {
        2 as ::core::ffi::c_int
    } else if height < 16 as ::core::ffi::c_uint {
        3 as ::core::ffi::c_int
    } else {
        4 as ::core::ffi::c_int
    };
    size = height.wrapping_sub((2 as ::core::ffi::c_int * offset) as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    x += offset;
    y += offset;
    count = (6 as ::core::ffi::c_int * size) as ::core::ffi::c_uint;
    if (((*prTop).allocated - (*prTop).used) as ::core::ffi::c_uint) < count
        && ExtendRList(prTop, count) == 0
        || (((*prBot).allocated - (*prBot).used) as ::core::ffi::c_uint) < count
            && ExtendRList(prBot, count) == 0
    {
        return;
    }
    row = 0 as ::core::ffi::c_int;
    while row < size {
        let mut current_block_16: u64;
        col = 0 as ::core::ffi::c_int;
        while col < size {
            let mut list = ::core::ptr::null_mut::<RList>();
            let mut rect = ::core::ptr::null_mut::<XRectangle>();
            if !(CloseButtonPixel(col, row, size) == 0) {
                if CloseButtonPixel(col - 1 as ::core::ffi::c_int, row, size) == 0
                    || CloseButtonPixel(col, row - 1 as ::core::ffi::c_int, size) == 0
                {
                    list = prTop;
                    current_block_16 = 7976072742316086414;
                } else if CloseButtonPixel(col + 1 as ::core::ffi::c_int, row, size) == 0
                    || CloseButtonPixel(col, row + 1 as ::core::ffi::c_int, size) == 0
                {
                    list = prBot;
                    current_block_16 = 7976072742316086414;
                } else {
                    current_block_16 = 5720623009719927633;
                }
                match current_block_16 {
                    5720623009719927633 => {}
                    _ => {
                        let fresh0 = (*list).used;
                        (*list).used = (*list).used + 1;
                        rect = (*list).prect.offset(fresh0 as isize) as *mut XRectangle;
                        (*rect).x = (x + col) as ::core::ffi::c_short;
                        (*rect).y = (y + row) as ::core::ffi::c_short;
                        (*rect).height = 1 as ::core::ffi::c_ushort;
                        (*rect).width = (*rect).height;
                    }
                }
            }
            col += 1;
        }
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn DepressGadget(
    mut pcd: *mut ClientData,
    mut gadget: ::core::ffi::c_int,
    mut depressed: Boolean,
) -> Boolean {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut invertWidth: ::core::ffi::c_uint = 0;
    static mut pTopRect: *mut RList = ::core::ptr::null::<RList>() as *mut RList;
    static mut pBotRect: *mut RList = ::core::ptr::null::<RList>() as *mut RList;
    let mut topGC = ::core::ptr::null_mut::<_XGC>();
    let mut botGC = ::core::ptr::null_mut::<_XGC>();
    let mut win: Window = 0;
    match gadget {
        FRAME_TITLE | FRAME_SYSTEM | FRAME_MINIMIZE | FRAME_MAXIMIZE | FRAME_CLOSE => {
            if GetDepressInfo(
                pcd,
                gadget,
                &raw mut x,
                &raw mut y,
                &raw mut width,
                &raw mut height,
                &raw mut invertWidth,
            ) == 0
            {
                return 0 as Boolean;
            }
        }
        _ => return 0 as Boolean,
    }
    if (*(*pcd).pSD).decoupleTitleAppearance as ::core::ffi::c_int != 0
        && (*pcd).decor & MWM_DECOR_TITLE != 0
    {
        x -= (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_short as ::core::ffi::c_int;
        y -= (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_short as ::core::ffi::c_int;
        if pcd == wmGD.keyboardFocus {
            topGC = (*(*pcd).pSD).clientTitleAppearance.activeTopShadowGC;
            botGC = (*(*pcd).pSD).clientTitleAppearance.activeBottomShadowGC;
        } else {
            topGC = (*(*pcd).pSD).clientTitleAppearance.inactiveTopShadowGC;
            botGC = (*(*pcd).pSD).clientTitleAppearance.inactiveBottomShadowGC;
        }
        win = (*pcd).clientTitleWin;
    } else {
        if pcd == wmGD.keyboardFocus {
            topGC = (*(*pcd).pSD).clientAppearance.activeTopShadowGC;
            botGC = (*(*pcd).pSD).clientAppearance.activeBottomShadowGC;
        } else {
            topGC = (*(*pcd).pSD).clientAppearance.inactiveTopShadowGC;
            botGC = (*(*pcd).pSD).clientAppearance.inactiveBottomShadowGC;
        }
        win = ClientDecorDrawable(pcd, False as Boolean) as Window;
    }
    if !pTopRect.is_null() && !pBotRect.is_null() || {
        pTopRect = AllocateRList(2 as ::core::ffi::c_uint);
        !pTopRect.is_null() && {
            pBotRect = AllocateRList(2 as ::core::ffi::c_uint);
            !pBotRect.is_null()
        }
    } {
        (*pTopRect).used = 0 as ::core::ffi::c_int;
        (*pBotRect).used = 0 as ::core::ffi::c_int;
        BevelRectangle(
            pTopRect,
            pBotRect,
            x,
            y,
            width,
            height,
            invertWidth,
            invertWidth,
            invertWidth,
            invertWidth,
        );
    }
    if depressed != 0 {
        XFillRectangles(
            wmGD.display,
            win as Drawable,
            botGC,
            (*pTopRect).prect,
            (*pTopRect).used,
        );
        XFillRectangles(
            wmGD.display,
            win as Drawable,
            topGC,
            (*pBotRect).prect,
            (*pBotRect).used,
        );
    } else {
        XFillRectangles(
            wmGD.display,
            win as Drawable,
            topGC,
            (*pTopRect).prect,
            (*pTopRect).used,
        );
        XFillRectangles(
            wmGD.display,
            win as Drawable,
            botGC,
            (*pBotRect).prect,
            (*pBotRect).used,
        );
    }
    PresentArgbDecoration(pcd, False as Boolean);
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn PushGadgetIn(mut pcd: *mut ClientData, mut gadget: ::core::ffi::c_int) {
    match gadget {
        FRAME_SYSTEM => {
            (*pcd).decorFlags |= SYSTEM_DEPRESSED;
        }
        FRAME_TITLE => {
            (*pcd).decorFlags |= TITLE_DEPRESSED;
        }
        FRAME_MINIMIZE => {
            (*pcd).decorFlags |= MINIMIZE_DEPRESSED;
        }
        FRAME_MAXIMIZE => {
            (*pcd).decorFlags |= MAXIMIZE_DEPRESSED;
        }
        FRAME_CLOSE => {
            (*pcd).decorFlags |= CLOSE_DEPRESSED;
        }
        _ => return,
    }
    GenerateFrameDisplayLists(pcd);
    DepressGadget(pcd, gadget, TRUE as Boolean);
    wmGD.gadgetClient = pcd;
    wmGD.gadgetDepressed = gadget;
}
#[no_mangle]
pub unsafe extern "C" fn PopGadgetOut(mut pcd: *mut ClientData, mut gadget: ::core::ffi::c_int) {
    match gadget {
        FRAME_SYSTEM => {
            (*pcd).decorFlags &= !SYSTEM_DEPRESSED;
        }
        FRAME_TITLE => {
            (*pcd).decorFlags &= !TITLE_DEPRESSED;
        }
        FRAME_MINIMIZE => {
            (*pcd).decorFlags &= !MINIMIZE_DEPRESSED;
        }
        FRAME_MAXIMIZE => {
            (*pcd).decorFlags &= !MAXIMIZE_DEPRESSED;
        }
        FRAME_CLOSE => {
            (*pcd).decorFlags &= !CLOSE_DEPRESSED;
        }
        _ => return,
    }
    GenerateFrameDisplayLists(pcd);
    DepressGadget(pcd, gadget, FALSE as Boolean);
    wmGD.gadgetClient = ::core::ptr::null_mut::<ClientData>();
    wmGD.gadgetDepressed = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetFrameShape(mut pcd: *mut ClientData) {
    let mut xOffset = 0 as ::core::ffi::c_int;
    let mut yOffset = 0 as ::core::ffi::c_int;
    if XBorderIsShowing(pcd) != 0 {
        xOffset = (*pcd).xBorderWidth;
        yOffset = (*pcd).xBorderWidth;
    } else if (*pcd).matteWidth > 0 as ::core::ffi::c_int {
        xOffset = (*pcd).matteWidth;
        yOffset = (*pcd).matteWidth;
    }
    if (*pcd).wShaped != 0 {
        XShapeCombineShape(
            wmGD.display,
            (*pcd).clientBaseWin,
            ShapeBounding,
            xOffset,
            yOffset,
            (*pcd).client,
            ShapeBounding,
            ShapeSet,
        );
        XShapeCombineShape(
            wmGD.display,
            (*pcd).clientFrameWin,
            ShapeBounding,
            BaseWindowX(pcd),
            BaseWindowY(pcd),
            (*pcd).clientBaseWin,
            ShapeBounding,
            ShapeSet,
        );
        if (*pcd).decor & MWM_DECOR_TITLE != 0 {
            XShapeCombineShape(
                wmGD.display,
                (*pcd).clientFrameWin,
                ShapeBounding,
                (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int,
                (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int,
                (*pcd).clientTitleWin,
                ShapeBounding,
                ShapeUnion,
            );
        }
    } else {
        XShapeCombineMask(
            wmGD.display,
            (*pcd).clientFrameWin,
            ShapeBounding,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            None_0 as Pixmap,
            ShapeSet,
        );
        XShapeCombineMask(
            wmGD.display,
            (*pcd).clientFrameWin,
            ShapeClip,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            None_0 as Pixmap,
            ShapeSet,
        );
        XShapeCombineMask(
            wmGD.display,
            (*pcd).clientBaseWin,
            ShapeBounding,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            None_0 as Pixmap,
            ShapeSet,
        );
        XShapeCombineMask(
            wmGD.display,
            (*pcd).clientBaseWin,
            ShapeClip,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            None_0 as Pixmap,
            ShapeSet,
        );
    };
}
