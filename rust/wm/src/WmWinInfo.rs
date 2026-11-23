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
    fn XGetAtomName(_: *mut Display, _: Atom) -> *mut ::core::ffi::c_char;
    fn XCreateBitmapFromData(
        _: *mut Display,
        _: Drawable,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> Pixmap;
    fn XChangeSaveSet(_: *mut Display, _: Window, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XChangeWindowAttributes(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> ::core::ffi::c_int;
    fn XFree(_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> ::core::ffi::c_int;
    fn XGetGeometry(
        _: *mut Display,
        _: Drawable,
        _: *mut Window,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XGetTransientForHint(_: *mut Display, _: Window, _: *mut Window) -> ::core::ffi::c_int;
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
    fn XGetWindowAttributes(
        _: *mut Display,
        _: Window,
        _: *mut XWindowAttributes,
    ) -> ::core::ffi::c_int;
    fn XParseGeometry(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
    fn XGetClassHint(_: *mut Display, _: Window, _: *mut XClassHint) -> ::core::ffi::c_int;
    fn XGetWMHints(_: *mut Display, _: Window) -> *mut XWMHints;
    fn XGetWMIconName(_: *mut Display, _: Window, _: *mut XTextProperty) -> ::core::ffi::c_int;
    fn XGetWMName(_: *mut Display, _: Window, _: *mut XTextProperty) -> ::core::ffi::c_int;
    fn XSaveContext(
        _: *mut Display,
        _: XID,
        _: XContext,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtCalloc(_: Cardinal, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    fn XmCvtTextPropertyToXmStringTable(
        display: *mut Display,
        text_prop: *mut XTextProperty,
        string_table_return: *mut XmStringTable,
        count_return: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XmCvtCTToXmString(text: *mut ::core::ffi::c_char) -> XmString;
    fn XmStringCreateLocalized(text: String_0) -> XmString;
    fn XmStringFree(string: XmString);
    fn XmStringWidth(fontlist: XmFontList, string: XmString) -> Dimension;
    static mut wmGD: WmGlobalData;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn TitleTextHeight(pcd: *mut ClientData) -> ::core::ffi::c_uint;
    fn SetFrameInfo(pcd: *mut ClientData);
    fn SetClientOffset(pcd: *mut ClientData);
    fn DrawWindowTitle(pcd: *mut ClientData, eraseFirst: Boolean);
    fn PlaceWindowInteractively(pcd: *mut ClientData);
    fn Warning(message: *mut ::core::ffi::c_char);
    fn IconExposureProc(pcd: *mut ClientData, clearFirst: Boolean);
    fn RedisplayIconTitle(pcd: *mut ClientData);
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
    fn PositionToPlacementData(
        pWS: *mut WmWorkspaceData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    ) -> *mut IconPlacementData;
    fn GetIconBoxMenuItems(pSD: *mut WmScreenData) -> *mut MenuItem;
    fn MakeIconBox(pWS: *mut WmWorkspaceData, pCD: *mut ClientData) -> Boolean;
    fn SetIconBoxInfo(pWS: *mut WmWorkspaceData, pCD: *mut ClientData);
    fn MakeClientIconPixmap(pCD: *mut ClientData, iconBitmap: Pixmap, iconMask: Pixmap) -> Pixmap;
    fn MakeIconPixmap(
        pCD: *mut ClientData,
        bitmap: Pixmap,
        mask: Pixmap,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        depth: ::core::ffi::c_uint,
    ) -> Pixmap;
    fn MakeNamedIconPixmap(pCD: *mut ClientData, iconName: String_0) -> Pixmap;
    fn UnManageWindow(pCD: *mut ClientData);
    fn MakeMenu(
        pSD: *mut WmScreenData,
        menuName: String_0,
        initialContext: Context,
        accelContext: Context,
        moreMenuItems: *mut MenuItem,
        fSystemMenu: Boolean,
    ) -> *mut MenuSpec;
    fn GetNormalHints(pCD: *mut ClientData) -> *mut SizeHints;
    fn ProcessWmProtocols(pCD: *mut ClientData);
    fn ProcessMwmMessages(pCD: *mut ClientData);
    fn GetMwmHints(pCD: *mut ClientData) -> *mut PropMwmHints;
    fn ProcessWmColormapWindows(pCD: *mut ClientData);
    fn GetMwmMenuItems(pCD: *mut ClientData) -> *mut MenuItem;
    fn GetInitialPropertyList(pCD: *mut ClientData);
    fn HasProperty(pCD: *mut ClientData, property: Atom) -> Boolean;
    fn ProcessClientResources(pCD: *mut ClientData);
    static mut builtinSystemMenuName: [::core::ffi::c_char; 0];
    fn GetClientWorkspaceInfo(pCD: *mut ClientData, manageFlags: ::core::ffi::c_long) -> Boolean;
    fn PutClientIntoWorkspace(pWS: *mut WmWorkspaceData, pCD: *mut ClientData);
    fn GetWorkspaceData(pSD: *mut WmScreenData, wsID: WorkspaceID) -> *mut WmWorkspaceData;
    fn ClientInWorkspace(pWS: *mut WmWorkspaceData, pCD: *mut ClientData) -> Boolean;
    fn FindTransientTreeLeader(pcd: *mut ClientData) -> *mut ClientData;
    fn FindClientDBMatch(_: *mut ClientData, _: *mut *mut ::core::ffi::c_char) -> Boolean;
    fn LoadClientIconPositions(_: *mut ClientData);
    fn GetXineramaScreenFromLocation(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
    fn GetXineramaScreenFromPointer(xsi: *mut XineramaScreenInfo) -> ::core::ffi::c_int;
    fn GetActiveXineramaScreen(xsi: *mut XineramaScreenInfo) -> ::core::ffi::c_int;
    fn GetPrimaryXineramaScreen(xsi: *mut XineramaScreenInfo) -> ::core::ffi::c_int;
    fn ProcessEwmh(_: *mut ClientData);
    fn ProcessEwmhWindowType(_: *mut ClientData);
    fn IsDesktopSurface(window: Window) -> Boolean;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const AnyPropertyType: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const PropertyChangeMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 22 as ::core::ffi::c_int;
pub const ColormapChangeMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 23 as ::core::ffi::c_int;
pub const Success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CWEventMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const ForgetGravity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NorthWestGravity: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NorthGravity: ::core::ffi::c_int = 2;
pub const NorthEastGravity: ::core::ffi::c_int = 3;
pub const WestGravity: ::core::ffi::c_int = 4;
pub const CenterGravity: ::core::ffi::c_int = 5;
pub const EastGravity: ::core::ffi::c_int = 6;
pub const SouthWestGravity: ::core::ffi::c_int = 7;
pub const SouthGravity: ::core::ffi::c_int = 8;
pub const SouthEastGravity: ::core::ffi::c_int = 9;
pub const StaticGravity: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SetModeInsert: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XValue: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const YValue: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WidthValue: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const HeightValue: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const InputHint: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const StateHint: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const IconPixmapHint: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const IconWindowHint: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const IconPositionHint: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const IconMaskHint: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const WindowGroupHint: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const IconicState: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const XNoMemory: ::core::ffi::c_int = -1;
pub const XLocaleNotSupported: ::core::ffi::c_int = -2;
pub const XConverterNotFound: ::core::ffi::c_int = -3;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_HINTS_FUNCTIONS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const MWM_HINTS_DECORATIONS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const MWM_HINTS_INPUT_MODE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const MWM_HINTS_STATUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const MWM_FUNC_ALL: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
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
pub const MWM_DECOR_ALL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
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
pub const MWM_INPUT_MODELESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MWM_INPUT_PRIMARY_APPLICATION_MODAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_INPUT_SYSTEM_MODAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MWM_INPUT_FULL_APPLICATION_MODAL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MWM_TEAROFF_WINDOW: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XA_STRING: Atom = 31 as ::core::ffi::c_int as Atom;
pub const XA_WM_HINTS: Atom = 35 as ::core::ffi::c_int as Atom;
pub const XA_WM_ICON_NAME: Atom = 37 as ::core::ffi::c_int as Atom;
pub const XA_WM_NAME: Atom = 39 as ::core::ffi::c_int as Atom;
pub const XA_WM_CLASS: Atom = 67 as ::core::ffi::c_int as Atom;
pub const XA_WM_TRANSIENT_FOR: Atom = 68 as ::core::ffi::c_int as Atom;
pub const MANAGEW_NORMAL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const MANAGEW_ICON_BOX: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const MANAGEW_CONFIRM_BOX: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const MANAGEW_WM_CLIENTS: ::core::ffi::c_long = MANAGEW_ICON_BOX | MANAGEW_CONFIRM_BOX;
pub const FRAME_INTERNAL_SHADOW_WIDTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_TOP_TITLE_PADDING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_BOTTOM_TITLE_PADDING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_TOP_TITLE_SHADOW: ::core::ffi::c_int = FRAME_INTERNAL_SHADOW_WIDTH;
pub const WM_BOTTOM_TITLE_SHADOW: ::core::ffi::c_int = FRAME_INTERNAL_SHADOW_WIDTH;
pub const STRETCH_COUNT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const WM_FUNC_DEFAULT: ::core::ffi::c_long = MWM_FUNC_ALL;
pub const WM_FUNC_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WM_FUNC_ALL: ::core::ffi::c_long =
    MWM_FUNC_RESIZE | MWM_FUNC_MOVE | MWM_FUNC_MINIMIZE | MWM_FUNC_MAXIMIZE | MWM_FUNC_CLOSE;
pub const WM_DECOR_DEFAULT: ::core::ffi::c_long = MWM_DECOR_ALL;
pub const WM_DECOR_BORDER: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const WM_DECOR_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const WM_DECOR_SYSTEM: ::core::ffi::c_long = WM_DECOR_TITLE | MWM_DECOR_MENU;
pub const WM_DECOR_MINIMIZE: ::core::ffi::c_long = WM_DECOR_TITLE | MWM_DECOR_MINIMIZE;
pub const WM_DECOR_MAXIMIZE: ::core::ffi::c_long = WM_DECOR_TITLE | MWM_DECOR_MAXIMIZE;
pub const WM_DECOR_TITLEBAR: ::core::ffi::c_long =
    WM_DECOR_SYSTEM | WM_DECOR_MINIMIZE | WM_DECOR_MAXIMIZE;
pub const WM_DECOR_RESIZEH: ::core::ffi::c_long = WM_DECOR_BORDER | MWM_DECOR_RESIZEH;
pub const WM_DECOR_ALL: ::core::ffi::c_long = WM_DECOR_TITLEBAR | WM_DECOR_RESIZEH;
pub const ICON_BOX_FUNCTIONS: ::core::ffi::c_long =
    MWM_FUNC_RESIZE | MWM_FUNC_MOVE | MWM_FUNC_MINIMIZE | MWM_FUNC_MAXIMIZE;
pub const WSM_FUNCTION_OCCUPY_WS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int;
pub const F_CONTEXT_ICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const F_CONTEXT_NORMAL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const F_CONTEXT_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const F_CONTEXT_WINDOW: ::core::ffi::c_long = F_CONTEXT_NORMAL | F_CONTEXT_MAXIMIZE;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const UNSEEN_STATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CLIENT_HINTS_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const CLIENT_TRANSIENT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const WM_INITIALIZATION: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const ICON_IN_SAVE_SET: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const ICON_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CONFIRM_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 12 as ::core::ffi::c_int;
pub const SM_X: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 18 as ::core::ffi::c_int;
pub const SM_Y: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 19 as ::core::ffi::c_int;
pub const SM_WIDTH: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 20 as ::core::ffi::c_int;
pub const SM_HEIGHT: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 21 as ::core::ffi::c_int;
pub const SM_CLIENT_STATE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 22 as ::core::ffi::c_int;
pub const SM_ICON_X: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 23 as ::core::ffi::c_int;
pub const SM_ICON_Y: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 24 as ::core::ffi::c_int;
pub const CLIENT_WM_CLIENTS: ::core::ffi::c_long = ICON_BOX | CONFIRM_BOX;
pub const ICON_HINTS_POSITION: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const ICON_HINTS_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const ICON_HINTS_PIXMAP: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const PROTOCOL_MWM_MESSAGES: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const XRS_FOCUS_POINTER: ::core::ffi::c_int = 0;
pub const XRS_FOCUS_KEYBOARD: ::core::ffi::c_int = 1;
pub const XRS_FOCUS_PRIMARY: ::core::ffi::c_int = 2;
pub const ICON_IMAGE_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const NO_ICON_PLACE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const CLIENT_DATA_TYPE: ::core::ffi::c_int = 1002 as ::core::ffi::c_int;
pub const USE_PPOSITION_ON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const USE_PPOSITION_NONZERO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BIGSIZE: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
pub const ICCC_R2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ICCC_CURRENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const US_POSITION: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const US_SIZE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const P_POSITION: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const P_SIZE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const P_MIN_SIZE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const P_MAX_SIZE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const P_RESIZE_INC: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const P_ASPECT: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
pub const P_BASE_SIZE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int;
pub const P_WIN_GRAVITY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const WmNconfirmbox: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"confirmbox\0") };
pub const WmCConfirmbox: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"Confirmbox\0") };
pub const iconBox_width: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const iconBox_height: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
static mut iconBox_bits: [::core::ffi::c_uchar; 350] = [
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const LOCALE_MSG: [::core::ffi::c_char; 80] = unsafe {
    ::core::mem::transmute::<[u8; 80], [::core::ffi::c_char; 80]>(
        *b"[XmbTextPropertyToTextList]:\n     Locale (%.100s) not supported. (Check $LANG).\0",
    )
};
#[no_mangle]
pub static mut pIconBoxInitialWS: *mut WmWorkspaceData =
    ::core::ptr::null::<WmWorkspaceData>() as *mut WmWorkspaceData;
#[no_mangle]
pub unsafe extern "C" fn GetClientInfo(
    mut pSD: *mut WmScreenData,
    mut clientWindow: Window,
    mut manageFlags: ::core::ffi::c_long,
) -> *mut ClientData {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut sAttributes = XSetWindowAttributes {
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
    pCD = XtCalloc(
        1 as Cardinal,
        ::core::mem::size_of::<ClientData>() as Cardinal,
    ) as *mut ClientData;
    if pCD.is_null() {
        Warning(
            b"Insufficient memory for client data\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<ClientData>();
    }
    (*pCD).client = clientWindow;
    (*pSD).clientCounter = (*pSD).clientCounter.wrapping_add(1);
    (*pCD).clientID = (*pSD).clientCounter;
    (*pCD).clientFlags = WM_INITIALIZATION;
    (*pCD).iconFlags = 0 as ::core::ffi::c_long;
    (*pCD).thisIconBox = ::core::ptr::null_mut::<IconBoxData>();
    (*pCD).wmUnmapCount = 0 as ::core::ffi::c_int;
    (*pCD).transientFor = 0 as ::core::ffi::c_long as Window;
    (*pCD).transientLeader = ::core::ptr::null_mut::<_ClientData>();
    (*pCD).transientChildren = ::core::ptr::null_mut::<_ClientData>();
    (*pCD).transientSiblings = ::core::ptr::null_mut::<_ClientData>();
    (*pCD).primaryStackPosition = 0 as ::core::ffi::c_int;
    (*pCD).fullModalCount = 0 as ::core::ffi::c_int;
    (*pCD).primaryModalCount = 0 as ::core::ffi::c_int;
    (*pCD).focusPriority = 0 as ::core::ffi::c_long;
    (*pCD).focusAutoRaiseDisabled = False as Boolean;
    (*pCD).focusAutoRaiseDisablePending = False as Boolean;
    (*pCD).clientClass = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pCD).clientName = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pCD).clientTitleWin = 0 as ::core::ffi::c_long as Window;
    (*pCD).clientFrameWin = 0 as ::core::ffi::c_long as Window;
    (*pCD).clientBaseWin = 0 as ::core::ffi::c_long as Window;
    (*pCD).pTitleGadgets = ::core::ptr::null_mut::<GadgetRectangle>();
    (*pCD).pResizeGadgets = ::core::ptr::null_mut::<GadgetRectangle>();
    (*pCD).pclientTitleTopShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).pclientTitleBottomShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).pclientTopShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).pclientBottomShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).pclientMatteTopShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).pclientMatteBottomShadows = ::core::ptr::null_mut::<RList>();
    memset(
        &raw mut (*pCD).clientStretchWin as *mut Window as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<Window>() as size_t).wrapping_mul(STRETCH_COUNT as size_t),
    );
    (*pCD).iconWindow = 0 as ::core::ffi::c_long as Window;
    (*pCD).iconPixmap = 0 as ::core::ffi::c_long as Pixmap;
    (*pCD).clientProtocols = ::core::ptr::null_mut::<Atom>();
    (*pCD).clientProtocolCount = 0 as ::core::ffi::c_int;
    (*pCD).mwmMessages = ::core::ptr::null_mut::<::core::ffi::c_long>();
    (*pCD).mwmMessagesCount = 0 as ::core::ffi::c_int;
    (*pCD).clientCmapCount = 0 as ::core::ffi::c_int;
    (*pCD).clientCmapIndex = 0 as ::core::ffi::c_int;
    (*pCD).clientCmapFlagsInitialized = FALSE;
    (*pCD).systemMenuSpec = ::core::ptr::null_mut::<MenuSpec>();
    (*pCD).putInAll = False as Boolean;
    (*pCD).desktopSurface = False as Boolean;
    (*pCD).pWorkspaceHints = ::core::ptr::null_mut::<WorkspaceID>();
    (*pCD).numInhabited = 0 as ::core::ffi::c_int;
    (*pCD).pWsList = ::core::ptr::null_mut::<_WsClientData>();
    (*pCD).wsmFunctions = WSM_FUNCTION_OCCUPY_WS;
    (*pCD).paInitialProperties = ::core::ptr::null_mut::<Atom>();
    (*pCD).numInitialProperties = 0 as ::core::ffi::c_int;
    (*pCD).decorFlags = 0 as ::core::ffi::c_long;
    (*pCD).pTitleGadgets = ::core::ptr::null_mut::<GadgetRectangle>();
    (*pCD).cTitleGadgets = 0 as ::core::ffi::c_int;
    (*pCD).pResizeGadgets = ::core::ptr::null_mut::<GadgetRectangle>();
    (*pCD).clientTitleWin = 0 as ::core::ffi::c_long as Window;
    (*pCD).pclientTopShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).pclientBottomShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).pclientTitleTopShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).pclientTitleBottomShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).pclientMatteTopShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).pclientMatteBottomShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).piconTopShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).piconBottomShadows = ::core::ptr::null_mut::<RList>();
    (*pCD).internalBevel = if wmGD.frameStyle as ::core::ffi::c_uint
        == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        0 as ::core::ffi::c_int
    } else {
        FRAME_INTERNAL_SHADOW_WIDTH
    };
    (*pCD).maxWidthLimit = BIGSIZE;
    (*pCD).maxWidth = (*pCD).maxWidthLimit;
    (*pCD).maxHeightLimit = BIGSIZE;
    (*pCD).maxHeight = (*pCD).maxHeightLimit;
    (*pCD).maxConfig = FALSE as Boolean;
    (*pCD).pSD = pSD;
    (*pCD).dataType = CLIENT_DATA_TYPE;
    (*pCD).window_status = 0 as ::core::ffi::c_long;
    (*pCD).clientEntry.nextSibling = ::core::ptr::null_mut::<_ClientListEntry>();
    (*pCD).clientEntry.prevSibling = ::core::ptr::null_mut::<_ClientListEntry>();
    (*pCD).clientEntry.pCD = ::core::ptr::null_mut::<_ClientData>();
    (*pCD).smClientID = NULL as String_0;
    (*pCD).fullScreenXineramaIndices[0 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
    if manageFlags & MANAGEW_WM_CLIENTS != 0 {
        let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
        if manageFlags & MANAGEW_ICON_BOX != 0 {
            pWS = pIconBoxInitialWS;
        } else {
            pWS = (*pSD).pActiveWS as *mut WmWorkspaceData;
        }
        return GetWmClientInfo(pWS, pCD, manageFlags);
    }
    XSaveContext(
        wmGD.display,
        clientWindow as XID,
        wmGD.windowContextType,
        pCD as XPointer as *const ::core::ffi::c_char,
    );
    sAttributes.event_mask = PropertyChangeMask | ColormapChangeMask;
    XChangeWindowAttributes(
        wmGD.display,
        (*pCD).client,
        CWEventMask as ::core::ffi::c_ulong,
        &raw mut sAttributes,
    );
    if WmGetWindowAttributes(clientWindow) == 0 {
        UnManageWindow(pCD);
        return ::core::ptr::null_mut::<ClientData>();
    }
    (*pCD).xBorderWidth = wmGD.windowAttributes.border_width;
    GetInitialPropertyList(pCD);
    ProcessWmClass(pCD);
    ProcessWmTransientFor(pCD);
    ProcessClientResources(pCD);
    (*pCD).desktopSurface = IsDesktopSurface((*pCD).client);
    ProcessSmClientID(pCD);
    ProcessWmSaveHint(pCD);
    if GetClientWorkspaceInfo(pCD, manageFlags) == 0 {
        XtFree(pCD as *mut ::core::ffi::c_char);
        return ::core::ptr::null_mut::<ClientData>();
    }
    LoadClientIconPositions(pCD);
    FindClientDBMatch(pCD, NULL as *mut *mut ::core::ffi::c_char);
    ProcessMwmHints(pCD, True as Boolean);
    ProcessWmHints(pCD, True as Boolean);
    SetClientOffset(pCD);
    ProcessWmNormalHints(pCD, True as Boolean, manageFlags);
    ProcessWmWindowTitle(pCD, TRUE as Boolean);
    ProcessWmIconTitle(pCD, TRUE as Boolean);
    ProcessWmProtocols(pCD);
    if (*pCD).protocolFlags & PROTOCOL_MWM_MESSAGES != 0 {
        ProcessMwmMessages(pCD);
    }
    if !(*pCD).systemMenu.is_null() {
        MakeSystemMenu(pCD);
    } else {
        (*pCD).systemMenuSpec = ::core::ptr::null_mut::<MenuSpec>();
    }
    InitCColormapData(pCD);
    ProcessEwmh(pCD);
    return pCD;
}
#[no_mangle]
pub unsafe extern "C" fn GetWmClientInfo(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
    mut manageFlags: ::core::ffi::c_long,
) -> *mut ClientData {
    let mut iconBitmap: Pixmap = 0;
    let mut i: ::core::ffi::c_int = 0;
    if manageFlags & MANAGEW_ICON_BOX != 0 {
        SetIconBoxInfo(pWS, pCD);
    } else if manageFlags & MANAGEW_CONFIRM_BOX != 0 {
        (*pCD).clientClass = WmCConfirmbox.as_ptr() as String_0;
        (*pCD).clientName = WmNconfirmbox.as_ptr() as String_0;
        (*pCD).iconImage = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*pCD).useClientIcon = False as Boolean;
        (*pCD).focusAutoRaise = True as Boolean;
        (*pCD).internalBevel = if wmGD.frameStyle as ::core::ffi::c_uint
            == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            0 as ::core::ffi::c_int
        } else {
            FRAME_INTERNAL_SHADOW_WIDTH
        };
        (*pCD).matteWidth = 0 as ::core::ffi::c_int;
        (*pCD).maximumClientSize.width = 0 as ::core::ffi::c_int;
        (*pCD).maximumClientSize.height = 0 as ::core::ffi::c_int;
        (*pCD).systemMenu = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    manageFlags & MANAGEW_ICON_BOX != 0;
    (*pCD).inputFocusModel = True;
    (*pCD).clientState = NORMAL_STATE;
    if (*(*pCD).pSD).iconDecoration as ::core::ffi::c_long & ICON_IMAGE_PART != 0 {
        if manageFlags & MANAGEW_ICON_BOX != 0 {
            (*pCD).clientFlags |= ICON_BOX;
        }
        if (*pCD).useClientIcon == 0 && !(*pCD).iconImage.is_null() {
            (*pCD).iconPixmap = MakeNamedIconPixmap(pCD, (*pCD).iconImage);
        }
        if (*pCD).iconPixmap == 0 {
            if manageFlags & MANAGEW_ICON_BOX != 0 {
                iconBitmap = XCreateBitmapFromData(
                    wmGD.display,
                    (*(*pCD).pSD).rootWindow as Drawable,
                    &raw mut iconBox_bits as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
                    iconBox_width as ::core::ffi::c_uint,
                    iconBox_height as ::core::ffi::c_uint,
                );
                (*pCD).iconPixmap = MakeIconPixmap(
                    pCD,
                    iconBitmap,
                    0 as ::core::ffi::c_long as Pixmap,
                    iconBox_width as ::core::ffi::c_uint,
                    iconBox_height as ::core::ffi::c_uint,
                    1 as ::core::ffi::c_uint,
                );
            }
        }
    }
    (*pCD).pWsList = XtMalloc(
        ((*(*pCD).pSD).numWorkspaces as usize)
            .wrapping_mul(::core::mem::size_of::<WsClientData>() as usize) as Cardinal,
    ) as *mut WsClientData as *mut _WsClientData;
    if (*pCD).pWsList.is_null() {
        Warning(
            b"Insufficient memory for client data\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<ClientData>();
    }
    (*pCD).sizeWsList = (*(*pCD).pSD).numWorkspaces;
    i = 0 as ::core::ffi::c_int;
    while i < (*(*pCD).pSD).numWorkspaces {
        (*(*pCD).pWsList.offset(i as isize)).wsID = None_0 as WorkspaceID;
        let ref mut fresh0 = (*(*pCD).pWsList.offset(i as isize)).IPData;
        *fresh0 = ::core::ptr::null_mut::<IconPlacementData>();
        (*(*pCD).pWsList.offset(i as isize)).iconPlace = NO_ICON_PLACE;
        (*(*pCD).pWsList.offset(i as isize)).iconX = 0 as ::core::ffi::c_int;
        (*(*pCD).pWsList.offset(i as isize)).iconY = 0 as ::core::ffi::c_int;
        (*(*pCD).pWsList.offset(i as isize)).iconFrameWin = 0 as ::core::ffi::c_long as Window;
        let ref mut fresh1 = (*(*pCD).pWsList.offset(i as isize)).pIconBox;
        *fresh1 = ::core::ptr::null_mut::<IconBoxData>();
        i += 1;
    }
    (*pCD).numInhabited = 0 as ::core::ffi::c_int;
    (*pCD).windowGroup = 0 as XID;
    if manageFlags & MANAGEW_CONFIRM_BOX != 0 {
        (*pCD).clientFunctions = WM_FUNC_NONE;
        (*pCD).clientDecoration = WM_DECOR_BORDER as ::core::ffi::c_int;
    } else {
        if (*pCD).clientFunctions as ::core::ffi::c_long & WM_FUNC_DEFAULT != 0 {
            (*pCD).clientFunctions = WM_FUNC_ALL as ::core::ffi::c_int;
        }
        if (*pCD).clientDecoration as ::core::ffi::c_long & WM_DECOR_DEFAULT != 0 {
            (*pCD).clientDecoration = WM_DECOR_ALL as ::core::ffi::c_int;
        }
        if manageFlags & MANAGEW_ICON_BOX != 0 {
            (*pCD).clientFunctions = ((*pCD).clientFunctions as ::core::ffi::c_long
                & ICON_BOX_FUNCTIONS) as ::core::ffi::c_int;
            (*pCD).wsmFunctions &= !WSM_FUNCTION_OCCUPY_WS;
        }
        if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_RESIZE == 0 {
            (*pCD).clientDecoration = ((*pCD).clientDecoration as ::core::ffi::c_long
                & !MWM_DECOR_RESIZEH) as ::core::ffi::c_int;
        }
        if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE == 0 {
            (*pCD).clientDecoration = ((*pCD).clientDecoration as ::core::ffi::c_long
                & !MWM_DECOR_MINIMIZE) as ::core::ffi::c_int;
        }
        if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MAXIMIZE == 0 {
            (*pCD).clientDecoration = ((*pCD).clientDecoration as ::core::ffi::c_long
                & !MWM_DECOR_MAXIMIZE) as ::core::ffi::c_int;
        }
    }
    (*pCD).decor = (*pCD).clientDecoration as ::core::ffi::c_long;
    if manageFlags & MANAGEW_ICON_BOX != 0 {
        (*pCD).inputMode = MWM_INPUT_MODELESS;
    } else if manageFlags & MANAGEW_CONFIRM_BOX != 0 {
        (*pCD).inputMode = MWM_INPUT_SYSTEM_MODAL;
    }
    (*pCD).icccVersion = ICCC_CURRENT;
    (*pCD).sizeFlags = US_POSITION | US_SIZE;
    (*pCD).windowGravity = NorthWestGravity;
    if manageFlags & MANAGEW_ICON_BOX != 0 {
        if MakeIconBox(pWS, pCD) == 0 {
            Warning(
                b"Couldn't make icon box\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<ClientData>();
        }
        PutClientIntoWorkspace(pWS, pCD);
    } else if manageFlags & MANAGEW_CONFIRM_BOX != 0 {
        let mut root: Window = 0;
        let mut cbWidth: ::core::ffi::c_uint = 0;
        let mut cbHeight: ::core::ffi::c_uint = 0;
        let mut depth: ::core::ffi::c_uint = 0;
        XGetGeometry(
            wmGD.display,
            (*pCD).client as Drawable,
            &raw mut root,
            &raw mut (*pCD).clientX,
            &raw mut (*pCD).clientY,
            &raw mut cbWidth,
            &raw mut cbHeight,
            &raw mut (*pCD).xBorderWidth as *mut ::core::ffi::c_uint,
            &raw mut depth,
        );
        (*pCD).clientWidth = cbWidth as ::core::ffi::c_int;
        (*pCD).clientHeight = cbHeight as ::core::ffi::c_int;
        (*pCD).maxWidth = (*pCD).clientWidth;
        (*pCD).baseWidth = (*pCD).maxWidth;
        (*pCD).minWidth = (*pCD).baseWidth;
        (*pCD).maxHeight = (*pCD).clientHeight;
        (*pCD).baseHeight = (*pCD).maxHeight;
        (*pCD).minHeight = (*pCD).baseHeight;
        (*pCD).oldMaxWidth = (*pCD).maxWidth;
        (*pCD).oldMaxHeight = (*pCD).maxHeight;
        (*pCD).widthInc = 1 as ::core::ffi::c_int;
        (*pCD).heightInc = 1 as ::core::ffi::c_int;
        (*pCD).clientFlags |= CONFIRM_BOX;
        PutClientIntoWorkspace((*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData, pCD);
    }
    SetFrameInfo(pCD);
    XSaveContext(
        wmGD.display,
        (*pCD).client as XID,
        wmGD.windowContextType,
        pCD as XPointer as *const ::core::ffi::c_char,
    );
    (*pCD).clientProtocolCount = 0 as ::core::ffi::c_int;
    (*pCD).protocolFlags = 0 as ::core::ffi::c_long;
    if manageFlags & MANAGEW_ICON_BOX != 0 {
        (*pCD).systemMenuSpec = MakeMenu(
            (*pCD).pSD,
            (*pCD).systemMenu,
            F_CONTEXT_WINDOW as Context,
            (F_CONTEXT_WINDOW | F_CONTEXT_ICON) as Context,
            GetIconBoxMenuItems((*pCD).pSD),
            TRUE as Boolean,
        );
    }
    (*pCD).clientColormap = (*(*pCD).pSD).workspaceColormap;
    return pCD;
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWmClass(mut pCD: *mut ClientData) {
    let mut classHint = XClassHint {
        res_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        res_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if HasProperty(pCD, XA_WM_CLASS) as ::core::ffi::c_int != 0
        && XGetClassHint(wmGD.display, (*pCD).client, &raw mut classHint) != 0
    {
        (*pCD).clientClass = classHint.res_class as String_0;
        (*pCD).clientName = classHint.res_name as String_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessSmClientID(mut pCD: *mut ClientData) {
    let mut actualType: Atom = 0;
    let mut actualFormat: ::core::ffi::c_int = 0;
    let mut nitems: ::core::ffi::c_ulong = 0;
    let mut leftover: ::core::ffi::c_ulong = 0;
    let mut clientID = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*pCD).smClientID.is_null() {
        XFree((*pCD).smClientID as *mut ::core::ffi::c_void);
        (*pCD).smClientID = NULL as String_0;
    }
    if XGetWindowProperty(
        wmGD.display,
        (*pCD).client,
        wmGD.xa_SM_CLIENT_ID,
        0 as ::core::ffi::c_long,
        1000000 as ::core::ffi::c_int as ::core::ffi::c_long,
        False,
        AnyPropertyType as Atom,
        &raw mut actualType,
        &raw mut actualFormat,
        &raw mut nitems,
        &raw mut leftover,
        &raw mut clientID as *mut *mut ::core::ffi::c_uchar,
    ) == Success
        && actualType != None_0 as Atom
        && actualFormat == 8 as ::core::ffi::c_int
    {
        (*pCD).smClientID = clientID as String_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWmSaveHint(mut pCD: *mut ClientData) {
    let mut actualType: Atom = 0;
    let mut actualFormat: ::core::ffi::c_int = 0;
    let mut nitems: ::core::ffi::c_ulong = 0;
    let mut leftover: ::core::ffi::c_ulong = 0;
    let mut saveHintFlags = NULL as *mut BITS32;
    if XGetWindowProperty(
        wmGD.display,
        (*pCD).client,
        wmGD.xa_WMSAVE_HINT,
        0 as ::core::ffi::c_long,
        1000000 as ::core::ffi::c_int as ::core::ffi::c_long,
        False,
        AnyPropertyType as Atom,
        &raw mut actualType,
        &raw mut actualFormat,
        &raw mut nitems,
        &raw mut leftover,
        &raw mut saveHintFlags as *mut *mut ::core::ffi::c_uchar,
    ) == Success
        && actualType != None_0 as Atom
        && actualFormat == 32 as ::core::ffi::c_int
    {
        (*pCD).wmSaveHintFlags = *saveHintFlags as ::core::ffi::c_int;
    } else {
        (*pCD).wmSaveHintFlags = 0 as ::core::ffi::c_int;
    }
    if !saveHintFlags.is_null() {
        XFree(saveHintFlags as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWmHints(mut pCD: *mut ClientData, mut firstTime: Boolean) {
    let mut pXWMHints = ::core::ptr::null_mut::<XWMHints>();
    let mut flags: ::core::ffi::c_long = 0;
    let mut iconPixmap: Pixmap = 0;
    let mut iconMask: Pixmap = 0;
    let mut pWsc = ::core::ptr::null_mut::<WsClientData>();
    let mut iws: ::core::ffi::c_int = 0;
    let mut tmpIconX: ::core::ffi::c_int = 0;
    let mut tmpIconY: ::core::ffi::c_int = 0;
    if firstTime as ::core::ffi::c_int != 0 && HasProperty(pCD, XA_WM_HINTS) == 0 {
        pXWMHints = ::core::ptr::null_mut::<XWMHints>();
    } else {
        pXWMHints = XGetWMHints(wmGD.display, (*pCD).client);
    }
    if !pXWMHints.is_null() {
        flags = (*pXWMHints).flags;
    } else {
        flags = 0 as ::core::ffi::c_long;
    }
    if firstTime != 0 {
        if flags & InputHint != 0 {
            (*pCD).inputFocusModel = (*pXWMHints).input;
        } else {
            (*pCD).inputFocusModel = True;
        }
        if (*pCD).clientFlags & SM_CLIENT_STATE != 0 {
            if (*pCD).clientState == MINIMIZED_STATE
                && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE == 0
            {
                (*pCD).clientState = NORMAL_STATE;
            }
        } else if flags & StateHint != 0
            && (*pXWMHints).initial_state == IconicState
            && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0
        {
            (*pCD).clientState = MINIMIZED_STATE;
        } else {
            (*pCD).clientState = NORMAL_STATE;
        }
        if ClientInWorkspace((*(*pCD).pSD).pActiveWS as *mut WmWorkspaceData, pCD) == 0 {
            (*pCD).clientState |= UNSEEN_STATE;
        }
        if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0
            && (*pCD).transientLeader.is_null()
        {
            if (*(*pCD).pSD).iconDecoration as ::core::ffi::c_long & ICON_IMAGE_PART != 0
                && ((*pCD).useClientIcon as ::core::ffi::c_int != 0 || (*pCD).iconImage.is_null())
            {
                if flags & IconWindowHint != 0 && (*pXWMHints).icon_window != (*pCD).client {
                    SetupClientIconWindow(pCD, (*pXWMHints).icon_window) == 0;
                }
                if (*pCD).iconWindow == 0 && flags & IconPixmapHint != 0 {
                    iconMask = if flags & IconMaskHint != 0 {
                        (*pXWMHints).icon_mask
                    } else {
                        NULL as Pixmap
                    };
                    (*pCD).iconPixmap =
                        MakeClientIconPixmap(pCD, (*pXWMHints).icon_pixmap, iconMask);
                    if (*pCD).iconPixmap != None_0 as Pixmap {
                        (*pCD).iconFlags |= ICON_HINTS_PIXMAP;
                    }
                }
            }
            if (*(*pCD).pSD).iconDecoration as ::core::ffi::c_long & ICON_IMAGE_PART != 0
                && (*pCD).iconPixmap == 0
            {
                if !(*pCD).iconImage.is_null() {
                    (*pCD).iconPixmap = MakeNamedIconPixmap(pCD, (*pCD).iconImage);
                }
                if (*pCD).iconPixmap == 0 {
                    (*pCD).iconPixmap =
                        MakeNamedIconPixmap(pCD, ::core::ptr::null_mut::<::core::ffi::c_char>());
                }
            }
            if flags & IconPositionHint != 0 || (*pCD).clientFlags & (SM_ICON_X | SM_ICON_Y) != 0 {
                (*pCD).iconFlags |= ICON_HINTS_POSITION;
                if wmGD.iconAutoPlace != 0 {
                    iws = 0 as ::core::ffi::c_int;
                    while iws < (*pCD).numInhabited {
                        let mut pWsd = ::core::ptr::null_mut::<WmWorkspaceData>();
                        pWsc = (*pCD).pWsList.offset(iws as isize) as *mut _WsClientData
                            as *mut WsClientData;
                        pWsd = GetWorkspaceData((*pCD).pSD, (*pWsc).wsID);
                        tmpIconX = if (*pCD).clientFlags & SM_ICON_X != 0 {
                            (*pWsc).iconX
                        } else {
                            (*pXWMHints).icon_x
                        };
                        tmpIconY = if (*pCD).clientFlags & SM_ICON_Y != 0 {
                            (*pWsc).iconY
                        } else {
                            (*pXWMHints).icon_y
                        };
                        (*pWsc).IPData = PositionToPlacementData(pWsd, tmpIconX, tmpIconY);
                        (*pWsc).iconPlace = FindIconPlace(pCD, (*pWsc).IPData, tmpIconX, tmpIconY);
                        if (*pWsc).iconPlace != NO_ICON_PLACE {
                            CvtIconPlaceToPosition(
                                (*pWsc).IPData,
                                (*pWsc).iconPlace,
                                &raw mut (*pWsc).iconX,
                                &raw mut (*pWsc).iconY,
                            );
                        }
                        iws += 1;
                    }
                } else {
                    iws = 0 as ::core::ffi::c_int;
                    while iws < (*pCD).numInhabited {
                        pWsc = (*pCD).pWsList.offset(iws as isize) as *mut _WsClientData
                            as *mut WsClientData;
                        if (*pCD).clientFlags & SM_ICON_X == 0 {
                            (*pWsc).iconX = (*pXWMHints).icon_x;
                        }
                        if (*pCD).clientFlags & SM_ICON_Y == 0 {
                            (*pWsc).iconY = (*pXWMHints).icon_y;
                        }
                        iws += 1;
                    }
                }
            } else if wmGD.iconAutoPlace != 0 {
                iws = 0 as ::core::ffi::c_int;
                while iws < (*pCD).numInhabited {
                    pWsc = (*pCD).pWsList.offset(iws as isize) as *mut _WsClientData
                        as *mut WsClientData;
                    (*pWsc).iconPlace = NO_ICON_PLACE;
                    (*pWsc).iconX = 0 as ::core::ffi::c_int;
                    (*pWsc).iconY = 0 as ::core::ffi::c_int;
                    iws += 1;
                }
            }
        }
        if flags & WindowGroupHint != 0 {
            (*pCD).windowGroup = (*pXWMHints).window_group;
        } else {
            (*pCD).windowGroup = 0 as XID;
        }
    } else if flags & IconPixmapHint != 0
        && ((*pCD).useClientIcon as ::core::ffi::c_int != 0 || (*pCD).iconImage.is_null())
    {
        iconMask = if flags & IconMaskHint != 0 {
            (*pXWMHints).icon_mask
        } else {
            NULL as Pixmap
        };
        iconPixmap = MakeClientIconPixmap(pCD, (*pXWMHints).icon_pixmap, iconMask);
        if iconPixmap != None_0 as Pixmap {
            if (*pCD).iconFlags & ICON_HINTS_PIXMAP != 0 {
                if (*pCD).iconPixmap != 0 {
                    XFreePixmap(wmGD.display, (*pCD).iconPixmap);
                }
            } else {
                (*pCD).iconFlags |= ICON_HINTS_PIXMAP;
            }
            (*pCD).iconPixmap = iconPixmap;
            if ((*pCD).clientState == MINIMIZED_STATE
                || (*(*pCD).pSD).useIconBox as ::core::ffi::c_int != 0
                    && !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
                        .pIconBox
                        .is_null())
                && (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin != 0
            {
                IconExposureProc(pCD, True as Boolean);
            }
        }
    }
    if !pXWMHints.is_null() {
        XFree(pXWMHints as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWmNormalHints(
    mut pCD: *mut ClientData,
    mut firstTime: Boolean,
    mut manageFlags: ::core::ffi::c_long,
) {
    let mut pNormalHints = ::core::ptr::null_mut::<SizeHints>();
    let mut flags: ::core::ffi::c_long = 0;
    let mut diff: ::core::ffi::c_int = 0;
    let mut decoration: ::core::ffi::c_ulong = 0;
    let mut boxdim: ::core::ffi::c_uint = 0;
    let mut tmpMin: ::core::ffi::c_uint = 0;
    let mut oldWidthInc = 0 as ::core::ffi::c_uint;
    let mut oldHeightInc = 0 as ::core::ffi::c_uint;
    let mut oldBaseWidth = 0 as ::core::ffi::c_uint;
    let mut oldBaseHeight = 0 as ::core::ffi::c_uint;
    let mut incWidth = 0 as ::core::ffi::c_uint;
    let mut incHeight = 0 as ::core::ffi::c_uint;
    let mut xsi = XineramaScreenInfo {
        screen_number: 0,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    };
    let mut dispWidth: ::core::ffi::c_uint = 0;
    let mut dispHeight: ::core::ffi::c_uint = 0;
    let mut scrWidth: ::core::ffi::c_uint = 0;
    let mut scrHeight: ::core::ffi::c_uint = 0;
    dispWidth = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*pCD).pSD).screen as isize))
    .width as ::core::ffi::c_uint;
    dispHeight = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*pCD).pSD).screen as isize))
    .height as ::core::ffi::c_uint;
    pNormalHints = GetNormalHints(pCD);
    (*pCD).icccVersion = (*pNormalHints).icccVersion;
    flags = (*pNormalHints).flags;
    if flags == 0 as ::core::ffi::c_long && firstTime == 0 {
        return;
    }
    if firstTime != 0 {
        let mut x = wmGD.windowAttributes.x;
        let mut y = wmGD.windowAttributes.y;
        let mut width = wmGD.windowAttributes.width as ::core::ffi::c_uint;
        let mut height = wmGD.windowAttributes.height as ::core::ffi::c_uint;
        if !(*pCD).overrideGeometry.is_null() {
            let mut mask: ::core::ffi::c_int = 0;
            mask = XParseGeometry(
                (*pCD).overrideGeometry as *const ::core::ffi::c_char,
                &raw mut x,
                &raw mut y,
                &raw mut width,
                &raw mut height,
            );
            flags |= if mask & (XValue | YValue) != 0 {
                US_POSITION
            } else {
                0 as ::core::ffi::c_long
            };
            flags |= if mask & (WidthValue | HeightValue) != 0 {
                US_SIZE
            } else {
                0 as ::core::ffi::c_long
            };
        }
        (*pCD).sizeFlags = flags & (US_POSITION | US_SIZE | P_POSITION | P_SIZE);
        if (*pCD).clientFlags & SM_X == 0 {
            (*pCD).clientX = x;
        }
        if (*pCD).clientFlags & SM_Y == 0 {
            (*pCD).clientY = y;
        }
        if (*pCD).clientFlags & SM_WIDTH == 0 {
            (*pCD).clientWidth = width as ::core::ffi::c_int;
        }
        if (*pCD).clientFlags & SM_HEIGHT == 0 {
            (*pCD).clientHeight = height as ::core::ffi::c_int;
        }
    }
    if GetXineramaScreenFromLocation((*pCD).clientX, (*pCD).clientY, &raw mut xsi) != 0 {
        scrWidth = xsi.width as ::core::ffi::c_uint;
        scrHeight = xsi.height as ::core::ffi::c_uint;
    } else {
        scrWidth = dispWidth;
        scrHeight = dispHeight;
    }
    if flags & P_MIN_SIZE != 0 {
        (*pCD).minWidth = if (*pNormalHints).min_width < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            (*pNormalHints).min_width
        };
        (*pCD).minHeight = if (*pNormalHints).min_height < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            (*pNormalHints).min_height
        };
        if (*pCD).minWidth as ::core::ffi::c_uint > dispWidth {
            (*pCD).minWidth = dispWidth as ::core::ffi::c_int;
        }
        if (*pCD).minHeight as ::core::ffi::c_uint > dispHeight {
            (*pCD).minHeight = dispHeight as ::core::ffi::c_int;
        }
    } else if firstTime != 0 {
        (*pCD).minWidth = 0 as ::core::ffi::c_int;
        (*pCD).minHeight = 0 as ::core::ffi::c_int;
    }
    if firstTime == 0 {
        oldWidthInc = (if (*pCD).widthInc == 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            (*pCD).widthInc
        }) as ::core::ffi::c_uint;
        oldHeightInc = (if (*pCD).heightInc == 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            (*pCD).heightInc
        }) as ::core::ffi::c_uint;
    }
    if flags & P_RESIZE_INC != 0 {
        (*pCD).widthInc = if (*pNormalHints).width_inc < 1 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            (*pNormalHints).width_inc
        };
        (*pCD).heightInc = if (*pNormalHints).height_inc < 1 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            (*pNormalHints).height_inc
        };
    } else if firstTime != 0 {
        (*pCD).widthInc = 1 as ::core::ffi::c_int;
        (*pCD).heightInc = 1 as ::core::ffi::c_int;
    }
    if firstTime == 0 {
        oldBaseWidth = (*pCD).baseWidth as ::core::ffi::c_uint;
        oldBaseHeight = (*pCD).baseHeight as ::core::ffi::c_uint;
    }
    if flags & P_BASE_SIZE != 0 {
        (*pCD).baseWidth = if (*pNormalHints).base_width < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            (*pNormalHints).base_width
        };
        (*pCD).baseHeight = if (*pNormalHints).base_height < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            (*pNormalHints).base_height
        };
    } else if (*pNormalHints).icccVersion == ICCC_R2
        && (firstTime as ::core::ffi::c_int != 0 || firstTime == 0 && flags & P_MIN_SIZE != 0)
    {
        (*pCD).baseWidth = (*pCD).minWidth;
        (*pCD).baseHeight = (*pCD).minHeight;
    } else if firstTime != 0 {
        if flags & P_MIN_SIZE != 0 {
            (*pCD).baseWidth = (*pCD).minWidth;
            (*pCD).baseHeight = (*pCD).minHeight;
        } else {
            (*pCD).baseWidth = 0 as ::core::ffi::c_int;
            (*pCD).baseHeight = 0 as ::core::ffi::c_int;
        }
    }
    if firstTime != 0 {
        if (*pCD).clientFlags & SM_WIDTH != 0 {
            (*pCD).clientWidth = (*pCD).clientWidth * (*pCD).widthInc + (*pCD).baseWidth;
        }
        if (*pCD).clientFlags & SM_HEIGHT != 0 {
            (*pCD).clientHeight = (*pCD).clientHeight * (*pCD).heightInc + (*pCD).baseHeight;
        }
    }
    (*pCD).oldMaxWidth = (*pCD).maxWidth;
    if (*pCD).maximumClientSize.width != 0 {
        if (*pCD).maximumClientSize.width == BIGSIZE
            && (*pCD).maximumClientSize.height == 0 as ::core::ffi::c_int
        {
            (*pCD).maxWidth = scrWidth.wrapping_sub(
                (2 as ::core::ffi::c_int * (*pCD).clientOffset.x as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
            (*pCD).maxHeight = (*pCD).clientHeight;
        } else {
            (*pCD).maxWidth = (*pCD).maximumClientSize.width * (*pCD).widthInc + (*pCD).baseWidth;
        }
    } else {
        if flags & P_MAX_SIZE != 0 {
            if (*pNormalHints).max_width < 0 as ::core::ffi::c_int {
                (*pCD).maxWidth = scrWidth.wrapping_sub(
                    (2 as ::core::ffi::c_int * (*pCD).clientOffset.x as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                ) as ::core::ffi::c_int;
            } else {
                (*pCD).maxWidth = (*pNormalHints).max_width;
            }
        } else if !((*pCD).maximumClientSize.height == BIGSIZE
            && (*pCD).maximumClientSize.width == 0 as ::core::ffi::c_int)
        {
            (*pCD).maxWidth = scrWidth.wrapping_sub(
                (2 as ::core::ffi::c_int * (*pCD).clientOffset.x as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
        } else if firstTime == 0
            && (oldBaseWidth != (*pCD).baseWidth as ::core::ffi::c_uint
                || oldWidthInc != (*pCD).widthInc as ::core::ffi::c_uint)
        {
            incWidth = ((*pCD).maxWidth as ::core::ffi::c_uint)
                .wrapping_sub(oldBaseWidth)
                .wrapping_div(oldWidthInc);
            (*pCD).maxWidth = incWidth
                .wrapping_mul((*pCD).widthInc as ::core::ffi::c_uint)
                .wrapping_add((*pCD).baseWidth as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
        } else {
            (*pCD).maxWidth = (*pCD).maxWidthLimit;
        }
        if (*pCD).maxWidth > (*(*pCD).pSD).maximumMaximumSize.width {
            if ((*(*pCD).pSD).maximumMaximumSize.width as ::core::ffi::c_uint) < scrWidth {
                (*pCD).maxWidth = (*(*pCD).pSD).maximumMaximumSize.width;
            } else {
                (*pCD).maxWidth = scrWidth.wrapping_sub(
                    (2 as ::core::ffi::c_int * (*pCD).clientOffset.x as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                ) as ::core::ffi::c_int;
            }
        }
    }
    (*pCD).oldMaxHeight = (*pCD).maxHeight;
    if (*pCD).maximumClientSize.height != 0 {
        if (*pCD).maximumClientSize.height == BIGSIZE
            && (*pCD).maximumClientSize.width == 0 as ::core::ffi::c_int
        {
            (*pCD).maxHeight = scrHeight.wrapping_sub(
                ((*pCD).clientOffset.x as ::core::ffi::c_int
                    + (*pCD).clientOffset.y as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
            (*pCD).maxWidth = (*pCD).clientWidth;
        } else {
            (*pCD).maxHeight =
                (*pCD).maximumClientSize.height * (*pCD).heightInc + (*pCD).baseHeight;
        }
    } else {
        if flags & P_MAX_SIZE != 0 {
            if (*pNormalHints).max_height < 0 as ::core::ffi::c_int {
                (*pCD).maxHeight = scrHeight.wrapping_sub(
                    ((*pCD).clientOffset.x as ::core::ffi::c_int
                        + (*pCD).clientOffset.y as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                ) as ::core::ffi::c_int;
            } else {
                (*pCD).maxHeight = (*pNormalHints).max_height;
            }
        } else if !((*pCD).maximumClientSize.width == BIGSIZE
            && (*pCD).maximumClientSize.height == 0 as ::core::ffi::c_int)
        {
            (*pCD).maxHeight = scrHeight.wrapping_sub(
                ((*pCD).clientOffset.x as ::core::ffi::c_int
                    + (*pCD).clientOffset.y as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
        } else if firstTime == 0
            && (oldBaseHeight != (*pCD).baseHeight as ::core::ffi::c_uint
                || oldHeightInc != (*pCD).heightInc as ::core::ffi::c_uint)
        {
            incHeight = ((*pCD).maxHeight as ::core::ffi::c_uint)
                .wrapping_sub(oldBaseHeight)
                .wrapping_div(oldHeightInc);
            (*pCD).maxHeight = incHeight
                .wrapping_mul((*pCD).heightInc as ::core::ffi::c_uint)
                .wrapping_add((*pCD).baseHeight as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
        } else {
            (*pCD).maxHeight = (*pCD).maxHeightLimit;
        }
        if (*pCD).maxHeight > (*(*pCD).pSD).maximumMaximumSize.height {
            if ((*(*pCD).pSD).maximumMaximumSize.height as ::core::ffi::c_uint) < scrHeight {
                (*pCD).maxHeight = (*(*pCD).pSD).maximumMaximumSize.height;
            } else {
                (*pCD).maxHeight = scrHeight.wrapping_sub(
                    ((*pCD).clientOffset.x as ::core::ffi::c_int
                        + (*pCD).clientOffset.y as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                ) as ::core::ffi::c_int;
            }
        }
    }
    if (*pCD).maxWidth > (*(*pCD).pSD).maximumMaximumSize.width {
        (*pCD).maxWidth = (*(*pCD).pSD).maximumMaximumSize.width;
    }
    if (*pCD).maxHeight > (*(*pCD).pSD).maximumMaximumSize.height {
        (*pCD).maxHeight = (*(*pCD).pSD).maximumMaximumSize.height;
    }
    if flags & P_ASPECT != 0 {
        (*pCD).minAspect.x = (*pNormalHints).min_aspect.x;
        (*pCD).minAspect.y = (*pNormalHints).min_aspect.y;
        (*pCD).maxAspect.x = (*pNormalHints).max_aspect.x;
        (*pCD).maxAspect.y = (*pNormalHints).max_aspect.y;
        if (*pCD).minAspect.y > 0 as ::core::ffi::c_int
            && (*pCD).maxAspect.y > 0 as ::core::ffi::c_int
            && (*pCD).minAspect.x > 0 as ::core::ffi::c_int
            && (*pCD).maxAspect.x > 0 as ::core::ffi::c_int
            && (*pCD).minAspect.x * (*pCD).maxAspect.y <= (*pCD).maxAspect.x * (*pCD).minAspect.y
        {
            (*pCD).sizeFlags |= P_ASPECT;
        } else {
            (*pCD).sizeFlags &= !P_ASPECT;
        }
    }
    decoration = (*pCD).decor as ::core::ffi::c_ulong;
    if decoration & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0 {
        boxdim = (*pCD).frameInfo.titleBarHeight;
        tmpMin = boxdim
            .wrapping_add(
                (if decoration & MWM_DECOR_MENU as ::core::ffi::c_ulong != 0 {
                    boxdim
                } else {
                    0 as ::core::ffi::c_uint
                }),
            )
            .wrapping_add(
                (if decoration & MWM_DECOR_MINIMIZE as ::core::ffi::c_ulong != 0 {
                    boxdim
                } else {
                    0 as ::core::ffi::c_uint
                }),
            )
            .wrapping_add(
                (if decoration & MWM_DECOR_MAXIMIZE as ::core::ffi::c_ulong != 0 {
                    boxdim
                } else {
                    0 as ::core::ffi::c_uint
                }),
            )
            .wrapping_add(
                (if (*pCD).decor & MWM_DECOR_TITLE != 0
                    && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
                {
                    boxdim
                } else {
                    0 as ::core::ffi::c_uint
                }),
            )
            .wrapping_sub((2 as ::core::ffi::c_int * (*pCD).matteWidth) as ::core::ffi::c_uint);
    } else {
        tmpMin = 0 as ::core::ffi::c_uint;
    }
    if flags & P_WIN_GRAVITY != 0 {
        (*pCD).windowGravity = (*pNormalHints).win_gravity;
    } else if (*pNormalHints).icccVersion == ICCC_R2 {
        (*pCD).windowGravity = wmGD.windowAttributes.win_gravity;
    } else {
        (*pCD).windowGravity = NorthWestGravity;
    }
    if ((*pCD).minWidth as ::core::ffi::c_uint) < tmpMin {
        diff = tmpMin
            .wrapping_sub((*pCD).baseWidth as ::core::ffi::c_uint)
            .wrapping_rem((*pCD).widthInc as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        if diff != 0 as ::core::ffi::c_int {
            (*pCD).minWidth = tmpMin
                .wrapping_add((*pCD).widthInc as ::core::ffi::c_uint)
                .wrapping_sub(diff as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
        } else {
            (*pCD).minWidth = tmpMin as ::core::ffi::c_int;
        }
    }
    if (*pCD).minWidth < (*pCD).baseWidth {
        (*pCD).minWidth = (*pCD).baseWidth;
    }
    if (*pCD).minWidth == 0 as ::core::ffi::c_int {
        (*pCD).minWidth = (*pCD).widthInc;
    } else {
        diff = ((*pCD).minWidth - (*pCD).baseWidth) % (*pCD).widthInc;
        if diff != 0 as ::core::ffi::c_int {
            (*pCD).minWidth += (*pCD).widthInc - diff;
        }
    }
    if (*pCD).minHeight < (*pCD).baseHeight {
        (*pCD).minHeight = (*pCD).baseHeight;
    }
    if (*pCD).minHeight == 0 as ::core::ffi::c_int {
        (*pCD).minHeight = (*pCD).heightInc;
    } else {
        diff = ((*pCD).minHeight - (*pCD).baseHeight) % (*pCD).heightInc;
        if diff != 0 as ::core::ffi::c_int {
            (*pCD).minHeight += (*pCD).heightInc - diff;
        }
    }
    if (*pCD).maxWidth < (*pCD).minWidth {
        (*pCD).maxWidth = (*pCD).minWidth;
    }
    if (*pCD).maximumClientSize.height == BIGSIZE
        && (*pCD).maximumClientSize.width == 0 as ::core::ffi::c_int
    {
        (*pCD).maxWidthLimit = scrWidth.wrapping_sub(
            (2 as ::core::ffi::c_int * (*pCD).clientOffset.x as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int;
    } else if flags & P_MAX_SIZE != 0 && (*pNormalHints).max_width > 0 as ::core::ffi::c_int {
        (*pCD).maxWidthLimit = (*pNormalHints).max_width;
    } else {
        (*pCD).maxWidthLimit = (*(*pCD).pSD).maximumMaximumSize.width;
    }
    (*pCD).maxWidth -= ((*pCD).maxWidth - (*pCD).baseWidth) % (*pCD).widthInc;
    if firstTime != 0 {
        (*pCD).oldMaxWidth = (*pCD).maxWidth;
    }
    if (*pCD).maxHeight < (*pCD).minHeight {
        (*pCD).maxHeight = (*pCD).minHeight;
    }
    if (*pCD).maximumClientSize.width == BIGSIZE
        && (*pCD).maximumClientSize.height == 0 as ::core::ffi::c_int
    {
        (*pCD).maxHeightLimit = scrHeight.wrapping_sub(
            ((*pCD).clientOffset.x as ::core::ffi::c_int
                + (*pCD).clientOffset.y as ::core::ffi::c_int) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int;
    } else if flags & P_MAX_SIZE != 0 && (*pNormalHints).max_height > 0 as ::core::ffi::c_int {
        (*pCD).maxHeightLimit = (*pNormalHints).max_height;
    } else {
        (*pNormalHints).max_height = (*(*pCD).pSD).maximumMaximumSize.height;
    }
    (*pCD).maxHeight -= ((*pCD).maxHeight - (*pCD).baseHeight) % (*pCD).heightInc;
    if firstTime != 0 {
        (*pCD).oldMaxHeight = (*pCD).maxHeight;
    }
    if firstTime == 0 && (*pCD).maxConfig as ::core::ffi::c_int != 0 {
        if firstTime == 0
            && (oldBaseWidth != (*pCD).baseWidth as ::core::ffi::c_uint
                || oldBaseHeight != (*pCD).baseHeight as ::core::ffi::c_uint
                || oldWidthInc != (*pCD).widthInc as ::core::ffi::c_uint
                || oldHeightInc != (*pCD).heightInc as ::core::ffi::c_uint)
        {
            incWidth = ((*pCD).clientWidth as ::core::ffi::c_uint)
                .wrapping_sub(oldBaseWidth)
                .wrapping_div(oldWidthInc);
            incHeight = ((*pCD).clientHeight as ::core::ffi::c_uint)
                .wrapping_sub(oldBaseHeight)
                .wrapping_div(oldHeightInc);
            (*pCD).clientWidth = incWidth
                .wrapping_mul((*pCD).widthInc as ::core::ffi::c_uint)
                .wrapping_add((*pCD).baseWidth as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
            (*pCD).clientHeight = incHeight
                .wrapping_mul((*pCD).heightInc as ::core::ffi::c_uint)
                .wrapping_add((*pCD).baseHeight as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
        }
    }
    if (*pCD).sizeFlags & P_ASPECT != 0 {
        if (*pCD).minWidth * (*pCD).minAspect.y > (*pCD).minAspect.x * (*pCD).maxHeight {
            (*pCD).minAspect.x = (*pCD).minWidth;
            (*pCD).minAspect.y = (*pCD).maxHeight;
        }
        if (*pCD).maxAspect.x * (*pCD).minHeight > (*pCD).maxWidth * (*pCD).maxAspect.y {
            (*pCD).maxAspect.x = (*pCD).maxWidth;
            (*pCD).maxAspect.y = (*pCD).minHeight;
        }
        FixWindowSize(
            pCD,
            &raw mut (*pCD).maxWidth as *mut ::core::ffi::c_uint,
            &raw mut (*pCD).maxHeight as *mut ::core::ffi::c_uint,
            (*pCD).widthInc as ::core::ffi::c_uint,
            (*pCD).heightInc as ::core::ffi::c_uint,
        );
    }
    if firstTime != 0 {
        if (*pCD).maximumClientSize.width == 0 {
            if (*pCD).clientWidth > (*(*pCD).pSD).maximumMaximumSize.width {
                (*pCD).clientWidth = (*(*pCD).pSD).maximumMaximumSize.width;
            }
        }
        if (*pCD).maximumClientSize.height == 0 {
            if (*pCD).clientHeight > (*(*pCD).pSD).maximumMaximumSize.width {
                (*pCD).clientHeight = (*(*pCD).pSD).maximumMaximumSize.width;
            }
        }
        FixWindowSize(
            pCD,
            &raw mut (*pCD).clientWidth as *mut ::core::ffi::c_uint,
            &raw mut (*pCD).clientHeight as *mut ::core::ffi::c_uint,
            (*pCD).widthInc as ::core::ffi::c_uint,
            (*pCD).heightInc as ::core::ffi::c_uint,
        );
        if flags & P_MIN_SIZE != 0
            && flags & P_MAX_SIZE != 0
            && (*pNormalHints).max_height == (*pNormalHints).min_height
            && (*pNormalHints).max_width == (*pNormalHints).min_width
        {
            (*pCD).clientFunctions = ((*pCD).clientFunctions as ::core::ffi::c_long
                & !(MWM_FUNC_MAXIMIZE | MWM_FUNC_RESIZE))
                as ::core::ffi::c_int;
            (*pCD).clientDecoration = ((*pCD).clientDecoration as ::core::ffi::c_long
                & !(MWM_DECOR_MAXIMIZE | MWM_DECOR_RESIZEH))
                as ::core::ffi::c_int;
            (*pCD).decor = (*pCD).clientDecoration as ::core::ffi::c_long;
        }
    }
}
unsafe extern "C" fn WmICCCMToXmString(mut wmNameProp: *mut XTextProperty) -> XmString {
    let mut status: ::core::ffi::c_int = 0;
    let mut xms_return = ::core::ptr::null_mut::<__XmStringRec>();
    let mut xmsTable = ::core::ptr::null_mut::<XmString>();
    let mut i: ::core::ffi::c_int = 0;
    let mut nStrings = -(1 as ::core::ffi::c_int);
    let mut msg: [::core::ffi::c_char; 200] = [0; 200];
    if (*wmNameProp).value.is_null()
        || strlen((*wmNameProp).value as *mut ::core::ffi::c_char) == 0 as size_t
    {
        return NULL as XmString;
    }
    status = XmCvtTextPropertyToXmStringTable(
        wmGD.display,
        wmNameProp,
        &raw mut xmsTable,
        &raw mut nStrings,
    );
    if status != Success || nStrings <= 0 as ::core::ffi::c_int {
        match status {
            XConverterNotFound => {
                sprintf(
                    &raw mut msg as *mut ::core::ffi::c_char,
                    b"Window manager cannot convert property %.100s as clientTitle/iconTitle: XmbTextPropertyToTextList\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    XGetAtomName(wmGD.display, (*wmNameProp).encoding),
                );
                Warning(&raw mut msg as *mut ::core::ffi::c_char);
            }
            XNoMemory => {
                sprintf(
                    &raw mut msg as *mut ::core::ffi::c_char,
                    b"insufficient memory to convert property %.100s as clientTitle/iconTitle: XmbTextPropertyToTextList\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    XGetAtomName(wmGD.display, (*wmNameProp).encoding),
                );
                Warning(&raw mut msg as *mut ::core::ffi::c_char);
            }
            XLocaleNotSupported => {
                if (*wmNameProp).encoding == XA_STRING
                    || (*wmNameProp).encoding == wmGD.xa_COMPOUND_TEXT
                {
                    sprintf(
                        &raw mut msg as *mut ::core::ffi::c_char,
                        LOCALE_MSG.as_ptr(),
                        setlocale(LC_ALL, ::core::ptr::null::<::core::ffi::c_char>()),
                    );
                } else {
                    sprintf(
                        &raw mut msg as *mut ::core::ffi::c_char,
                        b"Window manager received unknown property as clientTitle/iconTitle: %.100s. Property ignored.\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        XGetAtomName(wmGD.display, (*wmNameProp).encoding),
                    );
                }
                Warning(&raw mut msg as *mut ::core::ffi::c_char);
            }
            _ => {}
        }
        return XmCvtCTToXmString((*wmNameProp).value as *mut ::core::ffi::c_char);
    }
    xms_return = *xmsTable.offset(0 as ::core::ffi::c_int as isize);
    i = 1 as ::core::ffi::c_int;
    while i < nStrings {
        XmStringFree(*xmsTable.offset(i as isize));
        i += 1;
    }
    XtFree(xmsTable as *mut ::core::ffi::c_char);
    return xms_return;
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWmWindowTitle(mut pCD: *mut ClientData, mut firstTime: Boolean) {
    let mut wmNameProp = XTextProperty {
        value: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        encoding: 0,
        format: 0,
        nitems: 0,
    };
    let mut title_xms = ::core::ptr::null_mut::<__XmStringRec>();
    if (*pCD).clientDecoration as ::core::ffi::c_long & MWM_DECOR_TITLE != 0
        && (firstTime == 0 || HasProperty(pCD, XA_WM_NAME) as ::core::ffi::c_int != 0)
        && XGetWMName(wmGD.display, (*pCD).client, &raw mut wmNameProp) != 0
    {
        title_xms = WmICCCMToXmString(&raw mut wmNameProp);
        if !wmNameProp.value.is_null() {
            XFree(wmNameProp.value as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        }
    }
    if !title_xms.is_null() {
        if firstTime == 0 && (*pCD).iconTitle == (*pCD).clientTitle {
            (*pCD).iconTitle = title_xms;
            RedisplayIconTitle(pCD);
        }
        if (*pCD).clientFlags & CLIENT_HINTS_TITLE != 0
            && (*pCD).clientTitle != wmGD.clientDefaultTitle
        {
            XmStringFree((*pCD).clientTitle);
        }
        (*pCD).clientTitle = title_xms;
        (*pCD).clientFlags |= CLIENT_HINTS_TITLE;
        if firstTime == 0 {
            DrawWindowTitle(pCD, True as Boolean);
        }
    } else if firstTime != 0 {
        if !(*pCD).clientName.is_null() {
            (*pCD).clientTitle = XmStringCreateLocalized((*pCD).clientName);
        } else {
            (*pCD).clientTitle = wmGD.clientDefaultTitle;
        }
    }
    if (*pCD).window_status & MWM_TEAROFF_WINDOW != 0 {
        let mut boxdim = (*pCD).frameInfo.titleBarHeight;
        let mut decor = (*pCD).decor as ::core::ffi::c_ulong;
        let mut renderTable = ::core::ptr::null_mut::<*mut __XmRenderTableRec>();
        let mut minWidth: ::core::ffi::c_int = 0;
        if (*(*pCD).pSD).decoupleTitleAppearance != 0 {
            renderTable = (*(*pCD).pSD).clientTitleAppearance.renderTable;
        } else {
            renderTable = (*(*pCD).pSD).clientAppearance.renderTable;
        }
        minWidth = (XmStringWidth(renderTable as XmFontList, (*pCD).clientTitle)
            as ::core::ffi::c_uint)
            .wrapping_add(
                (if decor & MWM_DECOR_MENU as ::core::ffi::c_ulong != 0 {
                    boxdim
                } else {
                    0 as ::core::ffi::c_uint
                }),
            )
            .wrapping_add(
                (if decor & MWM_DECOR_MINIMIZE as ::core::ffi::c_ulong != 0 {
                    boxdim
                } else {
                    0 as ::core::ffi::c_uint
                }),
            )
            .wrapping_add(
                (if decor & MWM_DECOR_MAXIMIZE as ::core::ffi::c_ulong != 0 {
                    boxdim
                } else {
                    0 as ::core::ffi::c_uint
                }),
            )
            .wrapping_add(
                (if (*pCD).decor & MWM_DECOR_TITLE != 0
                    && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
                {
                    boxdim
                } else {
                    0 as ::core::ffi::c_uint
                }),
            )
            .wrapping_add(WM_TOP_TITLE_SHADOW as ::core::ffi::c_uint)
            .wrapping_add(WM_BOTTOM_TITLE_SHADOW as ::core::ffi::c_uint)
            .wrapping_add(WM_TOP_TITLE_PADDING as ::core::ffi::c_uint)
            .wrapping_add(WM_BOTTOM_TITLE_PADDING as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        if minWidth > (*pCD).minWidth {
            (*pCD).minWidth = minWidth;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWmIconTitle(mut pCD: *mut ClientData, mut firstTime: Boolean) {
    let mut wmIconNameProp = XTextProperty {
        value: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        encoding: 0,
        format: 0,
        nitems: 0,
    };
    let mut icon_xms = ::core::ptr::null_mut::<__XmStringRec>();
    if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0
        && (*pCD).transientLeader.is_null()
        && (firstTime == 0 || HasProperty(pCD, XA_WM_ICON_NAME) as ::core::ffi::c_int != 0)
        && XGetWMIconName(wmGD.display, (*pCD).client, &raw mut wmIconNameProp) != 0
    {
        icon_xms = WmICCCMToXmString(&raw mut wmIconNameProp);
        if !wmIconNameProp.value.is_null() {
            XFree(wmIconNameProp.value as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        }
    }
    if !icon_xms.is_null() {
        if (*pCD).iconFlags & ICON_HINTS_TITLE != 0 && (*pCD).iconTitle != wmGD.iconDefaultTitle {
            XmStringFree((*pCD).iconTitle);
        }
        (*pCD).iconTitle = icon_xms;
        (*pCD).iconFlags |= ICON_HINTS_TITLE;
        if firstTime == 0 {
            RedisplayIconTitle(pCD);
        }
    } else if firstTime != 0 {
        if !(*pCD).clientTitle.is_null() && (*pCD).clientTitle != wmGD.clientDefaultTitle {
            (*pCD).iconTitle = (*pCD).clientTitle;
        } else {
            (*pCD).iconTitle = wmGD.iconDefaultTitle;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWmTransientFor(mut pCD: *mut ClientData) {
    let mut window: Window = 0;
    let mut leader = ::core::ptr::null_mut::<ClientData>();
    if HasProperty(pCD, XA_WM_TRANSIENT_FOR) as ::core::ffi::c_int != 0
        && XGetTransientForHint(wmGD.display, (*pCD).client, &raw mut window) != 0
    {
        (*pCD).clientFlags |= CLIENT_TRANSIENT;
        if (*pCD).client != window
            && XFindContext(
                wmGD.display,
                window as XID,
                wmGD.windowContextType,
                &raw mut leader as *mut XPointer,
            ) == 0
        {
            (*pCD).transientFor = window;
            (*pCD).transientLeader = leader as *mut _ClientData;
        }
    } else {
        (*pCD).clientFlags &= !CLIENT_TRANSIENT;
        (*pCD).transientFor = 0 as ::core::ffi::c_long as Window;
        (*pCD).transientLeader = ::core::ptr::null_mut::<_ClientData>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn MakeSystemMenu(mut pCD: *mut ClientData) {
    (*pCD).mwmMenuItems = GetMwmMenuItems(pCD);
    (*pCD).systemMenuSpec = MakeMenu(
        (*pCD).pSD,
        (*pCD).systemMenu,
        F_CONTEXT_WINDOW as Context,
        (F_CONTEXT_WINDOW | F_CONTEXT_ICON) as Context,
        (*pCD).mwmMenuItems,
        TRUE as Boolean,
    );
    if (*pCD).systemMenuSpec.is_null() {
        Warning(
            b"Retrying - using builtin window menu\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        (*pCD).systemMenuSpec = MakeMenu(
            (*pCD).pSD,
            &raw mut builtinSystemMenuName as String_0,
            F_CONTEXT_WINDOW as Context,
            (F_CONTEXT_WINDOW | F_CONTEXT_ICON) as Context,
            (*pCD).mwmMenuItems,
            TRUE as Boolean,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitCColormapData(mut pCD: *mut ClientData) {
    if wmGD.windowAttributes.colormap == None_0 as Colormap {
        (*pCD).clientColormap = (*(*pCD).pSD).workspaceColormap;
    } else {
        (*pCD).clientColormap = wmGD.windowAttributes.colormap;
    }
    ProcessWmColormapWindows(pCD);
}
#[no_mangle]
pub unsafe extern "C" fn CalculateGravityOffset(
    mut pCD: *mut ClientData,
    mut xoff: *mut ::core::ffi::c_int,
    mut yoff: *mut ::core::ffi::c_int,
) {
    let mut borderWidth = (*pCD).xBorderWidth;
    if (*pCD).windowGravity < ForgetGravity
        || (*pCD).windowGravity > StaticGravity
        || (*pCD).sizeFlags & (P_POSITION | US_POSITION) != 0
            && (*pCD).windowGravity == StaticGravity
    {
        *xoff = 0 as ::core::ffi::c_int;
        *yoff = 0 as ::core::ffi::c_int;
    } else {
        match (*pCD).windowGravity {
            NorthGravity => {
                *xoff = borderWidth;
                *yoff = (*pCD).clientOffset.y as ::core::ffi::c_int;
            }
            NorthEastGravity => {
                *xoff = -((*pCD).clientOffset.x as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int * borderWidth);
                *yoff = (*pCD).clientOffset.y as ::core::ffi::c_int;
            }
            EastGravity => {
                *xoff = -((*pCD).clientOffset.x as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int * borderWidth);
                *yoff = borderWidth
                    + ((*pCD).clientOffset.y as ::core::ffi::c_int
                        - (*pCD).clientOffset.x as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            }
            SouthEastGravity => {
                *xoff = -((*pCD).clientOffset.x as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int * borderWidth);
                *yoff = -((*pCD).clientOffset.x as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int * borderWidth);
            }
            SouthGravity => {
                *xoff = borderWidth;
                *yoff = -((*pCD).clientOffset.x as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int * borderWidth);
            }
            SouthWestGravity => {
                *xoff = (*pCD).clientOffset.x as ::core::ffi::c_int;
                *yoff = -((*pCD).clientOffset.x as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int * borderWidth);
            }
            WestGravity => {
                *xoff = (*pCD).clientOffset.x as ::core::ffi::c_int;
                *yoff = borderWidth
                    + ((*pCD).clientOffset.y as ::core::ffi::c_int
                        - (*pCD).clientOffset.x as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
            }
            CenterGravity => {
                *xoff = 0 as ::core::ffi::c_int;
                *yoff = 0 as ::core::ffi::c_int;
            }
            NorthWestGravity | _ => {
                *xoff = (*pCD).clientOffset.x as ::core::ffi::c_int;
                *yoff = (*pCD).clientOffset.y as ::core::ffi::c_int;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn InitClientPlacement(
    mut pCD: *mut ClientData,
    mut manageFlags: ::core::ffi::c_long,
) -> Boolean {
    let mut interactivelyPlaced = False as Boolean;
    let mut autoPlaced = False as Boolean;
    let mut rval = False as Boolean;
    let mut xoff: ::core::ffi::c_int = 0;
    let mut yoff: ::core::ffi::c_int = 0;
    let mut origX: ::core::ffi::c_int = 0;
    let mut origY: ::core::ffi::c_int = 0;
    let mut origWidth: ::core::ffi::c_int = 0;
    let mut origHeight: ::core::ffi::c_int = 0;
    let mut iwsc: ::core::ffi::c_int = 0;
    if (*pCD).desktopSurface != 0 {
        return False as Boolean;
    }
    origX = (*pCD).clientX;
    origY = (*pCD).clientY;
    origWidth = (*pCD).clientWidth;
    origHeight = (*pCD).clientHeight;
    if wmGD.interactivePlacement as ::core::ffi::c_int != 0
        && (*pCD).clientFlags & (SM_X | SM_Y) == 0
        && (*pCD).sizeFlags & US_POSITION == 0
        && (*pCD).clientState != MINIMIZED_STATE
        && manageFlags == MANAGEW_NORMAL
        && (*pCD).clientFlags & CLIENT_TRANSIENT == 0
        && (*pCD).inputMode != MWM_INPUT_SYSTEM_MODAL
        && ClientInWorkspace((*(*pCD).pSD).pActiveWS as *mut WmWorkspaceData, pCD)
            as ::core::ffi::c_int
            != 0
    {
        interactivelyPlaced = True as Boolean;
        PlaceWindowInteractively(pCD);
    }
    FixWindowConfiguration(
        pCD,
        &raw mut (*pCD).clientWidth as *mut ::core::ffi::c_uint,
        &raw mut (*pCD).clientHeight as *mut ::core::ffi::c_uint,
        (*pCD).widthInc as ::core::ffi::c_uint,
        (*pCD).heightInc as ::core::ffi::c_uint,
    );
    if manageFlags == MANAGEW_NORMAL
        && interactivelyPlaced == 0
        && (*pCD).clientFlags & (SM_X | SM_Y) == 0
        && (*pCD).sizeFlags & US_POSITION == 0
        && (*pCD).clientFlags & CLIENT_TRANSIENT == 0
        && (*pCD).inputMode != MWM_INPUT_SYSTEM_MODAL
        && wmGD.clientAutoPlace as ::core::ffi::c_int != 0
    {
        if !((*pCD).sizeFlags & P_POSITION != 0
            && ((*pCD).usePPosition == USE_PPOSITION_ON
                || (*pCD).usePPosition == USE_PPOSITION_NONZERO
                    && ((*pCD).clientX != 0 as ::core::ffi::c_int
                        || (*pCD).clientY != 0 as ::core::ffi::c_int)))
        {
            let mut xsi = XineramaScreenInfo {
                screen_number: 0,
                x_org: 0,
                y_org: 0,
                width: 0,
                height: 0,
            };
            FindClientPlacement(pCD);
            autoPlaced = True as Boolean;
            if GetXineramaScreenFromLocation((*pCD).clientX, (*pCD).clientY, &raw mut xsi) != 0 {
                (*pCD).maxWidth = xsi.width as ::core::ffi::c_int
                    - (*pCD).clientOffset.x as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
                (*pCD).maxHeight = xsi.height as ::core::ffi::c_int
                    - ((*pCD).clientOffset.x as ::core::ffi::c_int
                        + (*pCD).clientOffset.y as ::core::ffi::c_int);
                if (*pCD).maxWidth > (*pCD).maxWidthLimit {
                    (*pCD).maxWidth = (*pCD).maxWidthLimit;
                }
                if (*pCD).maxHeight > (*pCD).maxHeightLimit {
                    (*pCD).maxHeight = (*pCD).maxHeightLimit;
                }
                (*pCD).maxWidth -= ((*pCD).maxWidth - (*pCD).baseWidth) % (*pCD).widthInc;
                (*pCD).maxHeight -= ((*pCD).maxHeight - (*pCD).baseHeight) % (*pCD).heightInc;
            }
        }
    }
    if interactivelyPlaced == 0 && autoPlaced == 0 {
        CalculateGravityOffset(pCD, &raw mut xoff, &raw mut yoff);
        if (*pCD).clientFlags & SM_X == 0 {
            (*pCD).clientX += xoff;
        }
        if (*pCD).clientFlags & SM_Y == 0 {
            (*pCD).clientY += yoff;
        }
    }
    if wmGD.positionOnScreen as ::core::ffi::c_int != 0
        && interactivelyPlaced == 0
        && (*pCD).clientFlags & (SM_X | SM_Y) == 0
    {
        PlaceFrameOnScreen(
            pCD,
            &raw mut (*pCD).clientX,
            &raw mut (*pCD).clientY,
            (*pCD).clientWidth,
            (*pCD).clientHeight,
        );
    }
    (*pCD).maxX = (*pCD).clientX;
    (*pCD).maxY = (*pCD).clientY;
    PlaceFrameOnScreen(
        pCD,
        &raw mut (*pCD).maxX,
        &raw mut (*pCD).maxY,
        (*pCD).maxWidth,
        (*pCD).maxHeight,
    );
    if wmGD.iconAutoPlace == 0 {
        if (*pCD).iconFlags & ICON_HINTS_POSITION == 0 {
            iwsc = 0 as ::core::ffi::c_int;
            while iwsc < (*pCD).numInhabited {
                (*(*pCD).pWsList.offset(iwsc as isize)).iconX = (*pCD).clientX;
                (*(*pCD).pWsList.offset(iwsc as isize)).iconY = (*pCD).clientY;
                PlaceIconOnScreen(
                    pCD,
                    &raw mut (*(*pCD).pWsList.offset(iwsc as isize)).iconX,
                    &raw mut (*(*pCD).pWsList.offset(iwsc as isize)).iconY,
                );
                iwsc += 1;
            }
        }
    }
    if origX != (*pCD).clientX
        || origY != (*pCD).clientY
        || origWidth != (*pCD).clientWidth
        || origHeight != (*pCD).clientHeight
    {
        rval = True as Boolean;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn PlaceFrameOnScreen(
    mut pCD: *mut ClientData,
    mut pX: *mut ::core::ffi::c_int,
    mut pY: *mut ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut clientOffsetX: ::core::ffi::c_int = 0;
    let mut clientOffsetY: ::core::ffi::c_int = 0;
    let mut frameX = 0 as ::core::ffi::c_int;
    let mut frameY = 0 as ::core::ffi::c_int;
    let mut frameWidth: ::core::ffi::c_int = 0;
    let mut frameHeight: ::core::ffi::c_int = 0;
    let mut screenHeight: ::core::ffi::c_int = 0;
    let mut screenWidth: ::core::ffi::c_int = 0;
    let mut xOrg: ::core::ffi::c_int = 0;
    let mut yOrg: ::core::ffi::c_int = 0;
    let mut xsi = XineramaScreenInfo {
        screen_number: 0,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    };
    clientOffsetX = (*pCD).clientOffset.x as ::core::ffi::c_int;
    clientOffsetY = (*pCD).clientOffset.y as ::core::ffi::c_int;
    frameWidth = w + 2 as ::core::ffi::c_int * clientOffsetX;
    frameHeight = h + clientOffsetX + clientOffsetY;
    if GetXineramaScreenFromLocation((*pCD).clientX, (*pCD).clientY, &raw mut xsi) != 0 {
        screenWidth = xsi.width as ::core::ffi::c_int;
        screenHeight = xsi.height as ::core::ffi::c_int;
        xOrg = xsi.x_org as ::core::ffi::c_int;
        yOrg = xsi.y_org as ::core::ffi::c_int;
    } else {
        screenWidth = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*pCD).pSD).screen as isize))
        .width;
        screenHeight = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*pCD).pSD).screen as isize))
        .height;
        xOrg = 0 as ::core::ffi::c_int;
        yOrg = 0 as ::core::ffi::c_int;
    }
    if !pX.is_null() {
        frameX = *pX - (clientOffsetX + xOrg);
    }
    if !pY.is_null() {
        frameY = *pY - (clientOffsetY + yOrg);
    }
    if frameX + frameWidth >= screenWidth {
        frameX -= frameX + frameWidth - screenWidth;
    }
    if frameY + frameHeight >= screenHeight {
        frameY -= frameY + frameHeight - screenHeight;
    }
    if frameX < 0 as ::core::ffi::c_int {
        frameX = 0 as ::core::ffi::c_int;
    }
    if frameY < 0 as ::core::ffi::c_int {
        frameY = 0 as ::core::ffi::c_int;
    }
    if !pX.is_null() {
        *pX = frameX + clientOffsetX + xOrg;
    }
    if !pY.is_null() {
        *pY = frameY + clientOffsetY + yOrg;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PlaceIconOnScreen(
    mut pCD: *mut ClientData,
    mut pX: *mut ::core::ffi::c_int,
    mut pY: *mut ::core::ffi::c_int,
) {
    let mut screenWidth: ::core::ffi::c_int = 0;
    let mut screenHeight: ::core::ffi::c_int = 0;
    let mut iconX: ::core::ffi::c_int = 0;
    let mut iconY: ::core::ffi::c_int = 0;
    let mut xsi = XineramaScreenInfo {
        screen_number: 0,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    };
    if GetXineramaScreenFromLocation((*pCD).clientX, (*pCD).clientY, &raw mut xsi) != 0 {
        screenWidth = xsi.width as ::core::ffi::c_int;
        screenHeight = xsi.height as ::core::ffi::c_int;
    } else {
        screenWidth = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*pCD).pSD).screen as isize))
        .width;
        screenHeight = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*pCD).pSD).screen as isize))
        .height;
    }
    iconX = *pX;
    iconY = *pY;
    if iconX + (*(*pCD).pSD).iconWidth > screenWidth {
        iconX = screenWidth - (*(*pCD).pSD).iconWidth;
    } else if iconX < 0 as ::core::ffi::c_int {
        iconX = 0 as ::core::ffi::c_int;
    }
    if iconY + (*(*pCD).pSD).iconHeight > screenHeight {
        iconY = screenHeight - (*(*pCD).pSD).iconHeight;
    } else if iconY < 0 as ::core::ffi::c_int {
        iconY = 0 as ::core::ffi::c_int;
    }
    *pX = iconX;
    *pY = iconY;
}
#[no_mangle]
pub unsafe extern "C" fn FixWindowConfiguration(
    mut pCD: *mut ClientData,
    mut pWidth: *mut ::core::ffi::c_uint,
    mut pHeight: *mut ::core::ffi::c_uint,
    mut widthInc: ::core::ffi::c_uint,
    mut heightInc: ::core::ffi::c_uint,
) {
    let mut delta: ::core::ffi::c_int = 0;
    if (*pWidth as ::core::ffi::c_int) < (*pCD).minWidth {
        *pWidth = (*pCD).minWidth as ::core::ffi::c_uint;
    } else {
        delta = (*pWidth)
            .wrapping_sub((*pCD).baseWidth as ::core::ffi::c_uint)
            .wrapping_rem((*pCD).widthInc as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        if delta != 0 {
            *pWidth = (*pWidth).wrapping_sub(delta as ::core::ffi::c_uint);
        }
    }
    if (*pHeight as ::core::ffi::c_int) < (*pCD).minHeight {
        *pHeight = (*pCD).minHeight as ::core::ffi::c_uint;
    } else {
        delta = (*pHeight)
            .wrapping_sub((*pCD).baseHeight as ::core::ffi::c_uint)
            .wrapping_rem((*pCD).heightInc as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        if delta != 0 {
            *pHeight = (*pHeight).wrapping_sub(delta as ::core::ffi::c_uint);
        }
    }
    FixWindowSize(pCD, pWidth, pHeight, widthInc, heightInc);
}
#[no_mangle]
pub unsafe extern "C" fn FixWindowSize(
    mut pCD: *mut ClientData,
    mut pWidth: *mut ::core::ffi::c_uint,
    mut pHeight: *mut ::core::ffi::c_uint,
    mut widthInc: ::core::ffi::c_uint,
    mut heightInc: ::core::ffi::c_uint,
) {
    let mut deltaW: ::core::ffi::c_int = 0;
    let mut deltaH: ::core::ffi::c_int = 0;
    let mut pSD = (*pCD).pSD;
    if (*pWidth as ::core::ffi::c_int) < (*pCD).minWidth {
        *pWidth = (*pCD).minWidth as ::core::ffi::c_uint;
    } else if *pWidth > (*pCD).maxWidthLimit as ::core::ffi::c_uint
        && (*pSD).limitResize as ::core::ffi::c_int != 0
        && (*pCD).clientFlags & CLIENT_WM_CLIENTS == 0
    {
        *pWidth = (*pCD).maxWidthLimit as ::core::ffi::c_uint;
    }
    if (*pHeight as ::core::ffi::c_int) < (*pCD).minHeight {
        *pHeight = (*pCD).minHeight as ::core::ffi::c_uint;
    } else if *pHeight > (*pCD).maxHeightLimit as ::core::ffi::c_uint
        && (*pSD).limitResize as ::core::ffi::c_int != 0
        && (*pCD).clientFlags & CLIENT_WM_CLIENTS == 0
    {
        *pHeight = (*pCD).maxHeightLimit as ::core::ffi::c_uint;
    }
    if (*pCD).sizeFlags & P_ASPECT != 0
        && (*pWidth).wrapping_mul((*pCD).maxAspect.y as ::core::ffi::c_uint)
            > (*pHeight).wrapping_mul((*pCD).maxAspect.x as ::core::ffi::c_uint)
    {
        if *pHeight >= (*pCD).clientHeight as ::core::ffi::c_uint
            || *pWidth > (*pCD).clientWidth as ::core::ffi::c_uint
        {
            deltaH = (if heightInc == 1 as ::core::ffi::c_uint {
                (*pWidth)
                    .wrapping_mul((*pCD).maxAspect.y as ::core::ffi::c_uint)
                    .wrapping_div((*pCD).maxAspect.x as ::core::ffi::c_uint)
                    .wrapping_sub(*pHeight)
            } else {
                ((*pWidth)
                    .wrapping_mul((*pCD).maxAspect.y as ::core::ffi::c_uint)
                    .wrapping_div((*pCD).maxAspect.x as ::core::ffi::c_uint)
                    .wrapping_sub(*pHeight)
                    .wrapping_div(heightInc) as ::core::ffi::c_int
                    as ::core::ffi::c_uint)
                    .wrapping_mul(heightInc)
            }) as ::core::ffi::c_int;
            if (*pHeight).wrapping_add(deltaH as ::core::ffi::c_uint)
                <= (*pCD).maxHeightLimit as ::core::ffi::c_uint
                || (*pSD).limitResize == 0
                || (*pCD).clientFlags & CLIENT_WM_CLIENTS != 0
            {
                *pHeight = (*pHeight).wrapping_add(deltaH as ::core::ffi::c_uint);
            } else {
                *pHeight = (*pCD).maxHeightLimit as ::core::ffi::c_uint;
                deltaW = (if widthInc == 1 as ::core::ffi::c_uint {
                    (*pWidth).wrapping_sub(
                        (*pHeight)
                            .wrapping_mul((*pCD).maxAspect.x as ::core::ffi::c_uint)
                            .wrapping_div((*pCD).maxAspect.y as ::core::ffi::c_uint),
                    )
                } else {
                    ((*pWidth)
                        .wrapping_sub(
                            (*pHeight)
                                .wrapping_mul((*pCD).maxAspect.x as ::core::ffi::c_uint)
                                .wrapping_div((*pCD).maxAspect.y as ::core::ffi::c_uint),
                        )
                        .wrapping_div(widthInc) as ::core::ffi::c_int
                        as ::core::ffi::c_uint)
                        .wrapping_mul(widthInc)
                }) as ::core::ffi::c_int;
                if (*pWidth).wrapping_sub(deltaW as ::core::ffi::c_uint)
                    >= (*pCD).minWidth as ::core::ffi::c_uint
                {
                    *pWidth = (*pWidth).wrapping_sub(deltaW as ::core::ffi::c_uint);
                } else {
                    *pWidth = (*pCD).minWidth as ::core::ffi::c_uint;
                }
            }
        } else {
            deltaW = (if widthInc == 1 as ::core::ffi::c_uint {
                (*pWidth).wrapping_sub(
                    (*pHeight)
                        .wrapping_mul((*pCD).maxAspect.x as ::core::ffi::c_uint)
                        .wrapping_div((*pCD).maxAspect.y as ::core::ffi::c_uint),
                )
            } else {
                ((*pWidth)
                    .wrapping_sub(
                        (*pHeight)
                            .wrapping_mul((*pCD).maxAspect.x as ::core::ffi::c_uint)
                            .wrapping_div((*pCD).maxAspect.y as ::core::ffi::c_uint),
                    )
                    .wrapping_div(widthInc) as ::core::ffi::c_int
                    as ::core::ffi::c_uint)
                    .wrapping_mul(widthInc)
            }) as ::core::ffi::c_int;
            if (*pWidth).wrapping_sub(deltaW as ::core::ffi::c_uint)
                >= (*pCD).minWidth as ::core::ffi::c_uint
            {
                *pWidth = (*pWidth).wrapping_sub(deltaW as ::core::ffi::c_uint);
            } else {
                *pWidth = (*pCD).minWidth as ::core::ffi::c_uint;
                deltaH = (if heightInc == 1 as ::core::ffi::c_uint {
                    (*pWidth)
                        .wrapping_mul((*pCD).maxAspect.y as ::core::ffi::c_uint)
                        .wrapping_div((*pCD).maxAspect.x as ::core::ffi::c_uint)
                        .wrapping_sub(*pHeight)
                } else {
                    ((*pWidth)
                        .wrapping_mul((*pCD).maxAspect.y as ::core::ffi::c_uint)
                        .wrapping_div((*pCD).maxAspect.x as ::core::ffi::c_uint)
                        .wrapping_sub(*pHeight)
                        .wrapping_div(heightInc) as ::core::ffi::c_int
                        as ::core::ffi::c_uint)
                        .wrapping_mul(heightInc)
                }) as ::core::ffi::c_int;
                if (*pHeight).wrapping_add(deltaH as ::core::ffi::c_uint)
                    <= (*pCD).maxHeightLimit as ::core::ffi::c_uint
                    || (*pSD).limitResize == 0
                    || (*pCD).clientFlags & CLIENT_WM_CLIENTS != 0
                {
                    *pHeight = (*pHeight).wrapping_add(deltaH as ::core::ffi::c_uint);
                } else {
                    *pHeight = (*pCD).maxHeightLimit as ::core::ffi::c_uint;
                }
            }
        }
    } else if (*pCD).sizeFlags & P_ASPECT != 0
        && (*pHeight).wrapping_mul((*pCD).minAspect.x as ::core::ffi::c_uint)
            > (*pWidth).wrapping_mul((*pCD).minAspect.y as ::core::ffi::c_uint)
    {
        if *pWidth >= (*pCD).clientWidth as ::core::ffi::c_uint
            || *pHeight > (*pCD).clientHeight as ::core::ffi::c_uint
        {
            deltaW = (if widthInc == 1 as ::core::ffi::c_uint {
                (*pHeight)
                    .wrapping_mul((*pCD).minAspect.x as ::core::ffi::c_uint)
                    .wrapping_div((*pCD).minAspect.y as ::core::ffi::c_uint)
                    .wrapping_sub(*pWidth)
            } else {
                ((*pHeight)
                    .wrapping_mul((*pCD).minAspect.x as ::core::ffi::c_uint)
                    .wrapping_div((*pCD).minAspect.y as ::core::ffi::c_uint)
                    .wrapping_sub(*pWidth)
                    .wrapping_div(widthInc) as ::core::ffi::c_int
                    as ::core::ffi::c_uint)
                    .wrapping_mul(widthInc)
            }) as ::core::ffi::c_int;
            if (*pWidth).wrapping_add(deltaW as ::core::ffi::c_uint)
                <= (*pCD).maxWidthLimit as ::core::ffi::c_uint
                || (*pSD).limitResize == 0
                || (*pCD).clientFlags & CLIENT_WM_CLIENTS != 0
            {
                *pWidth = (*pWidth).wrapping_add(deltaW as ::core::ffi::c_uint);
            } else {
                *pWidth = (*pCD).maxWidthLimit as ::core::ffi::c_uint;
                deltaH = (if heightInc == 1 as ::core::ffi::c_uint {
                    (*pHeight).wrapping_sub(
                        (*pWidth)
                            .wrapping_mul((*pCD).minAspect.y as ::core::ffi::c_uint)
                            .wrapping_div((*pCD).minAspect.x as ::core::ffi::c_uint),
                    )
                } else {
                    ((*pHeight)
                        .wrapping_sub(
                            (*pWidth)
                                .wrapping_mul((*pCD).minAspect.y as ::core::ffi::c_uint)
                                .wrapping_div((*pCD).minAspect.x as ::core::ffi::c_uint),
                        )
                        .wrapping_div(heightInc) as ::core::ffi::c_int
                        as ::core::ffi::c_uint)
                        .wrapping_mul(heightInc)
                }) as ::core::ffi::c_int;
                if (*pHeight).wrapping_sub(deltaH as ::core::ffi::c_uint)
                    >= (*pCD).minHeight as ::core::ffi::c_uint
                {
                    *pHeight = (*pHeight).wrapping_sub(deltaH as ::core::ffi::c_uint);
                } else {
                    *pHeight = (*pCD).minHeight as ::core::ffi::c_uint;
                }
            }
        } else {
            deltaH = (if heightInc == 1 as ::core::ffi::c_uint {
                (*pHeight).wrapping_sub(
                    (*pWidth)
                        .wrapping_mul((*pCD).minAspect.y as ::core::ffi::c_uint)
                        .wrapping_div((*pCD).minAspect.x as ::core::ffi::c_uint),
                )
            } else {
                ((*pHeight)
                    .wrapping_sub(
                        (*pWidth)
                            .wrapping_mul((*pCD).minAspect.y as ::core::ffi::c_uint)
                            .wrapping_div((*pCD).minAspect.x as ::core::ffi::c_uint),
                    )
                    .wrapping_div(heightInc) as ::core::ffi::c_int
                    as ::core::ffi::c_uint)
                    .wrapping_mul(heightInc)
            }) as ::core::ffi::c_int;
            if (*pHeight).wrapping_sub(deltaH as ::core::ffi::c_uint)
                >= (*pCD).minHeight as ::core::ffi::c_uint
            {
                *pHeight = (*pHeight).wrapping_sub(deltaH as ::core::ffi::c_uint);
            } else {
                *pHeight = (*pCD).minHeight as ::core::ffi::c_uint;
                deltaW = (if widthInc == 1 as ::core::ffi::c_uint {
                    (*pHeight)
                        .wrapping_mul((*pCD).minAspect.x as ::core::ffi::c_uint)
                        .wrapping_div((*pCD).minAspect.y as ::core::ffi::c_uint)
                        .wrapping_sub(*pWidth)
                } else {
                    ((*pHeight)
                        .wrapping_mul((*pCD).minAspect.x as ::core::ffi::c_uint)
                        .wrapping_div((*pCD).minAspect.y as ::core::ffi::c_uint)
                        .wrapping_sub(*pWidth)
                        .wrapping_div(widthInc) as ::core::ffi::c_int
                        as ::core::ffi::c_uint)
                        .wrapping_mul(widthInc)
                }) as ::core::ffi::c_int;
                if (*pWidth).wrapping_add(deltaW as ::core::ffi::c_uint)
                    <= (*pCD).maxWidthLimit as ::core::ffi::c_uint
                    || (*pSD).limitResize == 0
                    || (*pCD).clientFlags & CLIENT_WM_CLIENTS != 0
                {
                    *pWidth = (*pWidth).wrapping_add(deltaW as ::core::ffi::c_uint);
                } else {
                    *pWidth = (*pCD).maxWidthLimit as ::core::ffi::c_uint;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn FindClientPlacement(mut pCD: *mut ClientData) {
    static mut clientPlacementInitialized: Boolean = False as Boolean;
    static mut clientPlacementOffset: ::core::ffi::c_int = 0;
    static mut clientPlacementX: ::core::ffi::c_int = 0;
    static mut clientPlacementY: ::core::ffi::c_int = 0;
    static mut clientPlacementOrigin: ::core::ffi::c_int = 0;
    static mut clientPlacementXOrigin: ::core::ffi::c_int = 0;
    let mut placed = False as Boolean;
    let mut frameWidth: ::core::ffi::c_int = 0;
    let mut frameHeight: ::core::ffi::c_int = 0;
    let mut screenWidth: ::core::ffi::c_int = 0;
    let mut screenHeight: ::core::ffi::c_int = 0;
    let mut borderWidth = 0 as ::core::ffi::c_int;
    let mut offScreenX: Boolean = 0;
    let mut offScreenY: Boolean = 0;
    let mut xsi = XineramaScreenInfo {
        screen_number: 0,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    };
    let mut have_xinerama = False as Boolean;
    let mut scrOriginX = 0 as ::core::ffi::c_int;
    let mut scrOriginY = 0 as ::core::ffi::c_int;
    if clientPlacementInitialized == 0 {
        if (*pCD).clientDecoration as ::core::ffi::c_long & WM_DECOR_RESIZEH != 0 {
            borderWidth = if (*(*pCD).pSD).resizeBorderWidth > (*(*pCD).pSD).frameBorderWidth {
                (*(*pCD).pSD).resizeBorderWidth
            } else {
                (*(*pCD).pSD).frameBorderWidth
            };
        } else {
            borderWidth = (*pCD).matteWidth;
        }
        clientPlacementOffset = TitleTextHeight(pCD)
            .wrapping_add(borderWidth as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        clientPlacementOrigin = clientPlacementOffset;
        clientPlacementX = clientPlacementOrigin;
        clientPlacementY = clientPlacementOrigin;
        clientPlacementXOrigin = clientPlacementX;
        clientPlacementInitialized = True as Boolean;
    }
    frameWidth =
        (*pCD).clientWidth + 2 as ::core::ffi::c_int * (*pCD).clientOffset.x as ::core::ffi::c_int;
    frameHeight = (*pCD).clientHeight
        + (*pCD).clientOffset.y as ::core::ffi::c_int
        + (*pCD).clientOffset.x as ::core::ffi::c_int;
    match wmGD.xineramaScreenFocus {
        XRS_FOCUS_POINTER => {
            have_xinerama = GetXineramaScreenFromPointer(&raw mut xsi) as Boolean;
        }
        XRS_FOCUS_KEYBOARD => {
            have_xinerama = GetActiveXineramaScreen(&raw mut xsi) as Boolean;
        }
        XRS_FOCUS_PRIMARY => {
            have_xinerama = GetPrimaryXineramaScreen(&raw mut xsi) as Boolean;
        }
        _ => {}
    }
    if have_xinerama != 0 {
        screenWidth = xsi.width as ::core::ffi::c_int;
        screenHeight = xsi.height as ::core::ffi::c_int;
        scrOriginX = xsi.x_org as ::core::ffi::c_int;
        scrOriginY = xsi.y_org as ::core::ffi::c_int;
    } else {
        screenWidth = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*pCD).pSD).screen as isize))
        .width;
        screenHeight = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*pCD).pSD).screen as isize))
        .height;
    }
    while placed == 0 {
        if clientPlacementX - (*pCD).clientOffset.x as ::core::ffi::c_int + frameWidth > screenWidth
        {
            offScreenX = True as Boolean;
        } else {
            offScreenX = False as Boolean;
        }
        if clientPlacementY - (*pCD).clientOffset.y as ::core::ffi::c_int + frameHeight
            > screenHeight
        {
            offScreenY = True as Boolean;
        } else {
            offScreenY = False as Boolean;
        }
        if offScreenX as ::core::ffi::c_int != 0 || offScreenY as ::core::ffi::c_int != 0 {
            if clientPlacementX == clientPlacementOrigin {
                placed = True as Boolean;
            } else if clientPlacementY == clientPlacementOrigin {
                if offScreenX != 0 {
                    clientPlacementXOrigin = clientPlacementOrigin;
                    clientPlacementX = clientPlacementXOrigin;
                }
                placed = True as Boolean;
            } else if offScreenX as ::core::ffi::c_int != 0 && offScreenY == 0 {
                clientPlacementX = clientPlacementOrigin;
            } else if offScreenY as ::core::ffi::c_int != 0 && offScreenX == 0 {
                clientPlacementY = clientPlacementOrigin;
            } else {
                clientPlacementXOrigin += clientPlacementOffset;
                clientPlacementX = clientPlacementXOrigin;
                clientPlacementY = clientPlacementOrigin;
            }
        } else {
            placed = True as Boolean;
        }
    }
    (*pCD).clientX = clientPlacementX + scrOriginX;
    (*pCD).clientY = clientPlacementY + scrOriginY;
    clientPlacementX += clientPlacementOffset;
    if clientPlacementX >= screenWidth {
        clientPlacementXOrigin = clientPlacementOrigin;
        clientPlacementX = clientPlacementXOrigin;
    }
    clientPlacementY += clientPlacementOffset;
    if clientPlacementY >= screenHeight / 3 as ::core::ffi::c_int {
        clientPlacementY = clientPlacementOrigin;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WmGetWindowAttributes(mut window: Window) -> Boolean {
    if wmGD.attributesWindow != window {
        if XGetWindowAttributes(wmGD.display, window, &raw mut wmGD.windowAttributes) == 0 {
            wmGD.attributesWindow = 0 as ::core::ffi::c_long as Window;
            return 0 as Boolean;
        }
        wmGD.attributesWindow = window;
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn SetupClientIconWindow(
    mut pCD: *mut ClientData,
    mut window: Window,
) -> Boolean {
    let mut pcd = ::core::ptr::null_mut::<ClientData>();
    if XFindContext(
        wmGD.display,
        window as XID,
        wmGD.windowContextType,
        &raw mut pcd as *mut XPointer,
    ) == 0
    {
        if window == (*pCD).client {
            return 0 as Boolean;
        }
        UnManageWindow(pcd);
    }
    (*pCD).iconWindow = window;
    XChangeSaveSet(wmGD.display, (*pCD).iconWindow, SetModeInsert);
    (*pCD).clientFlags |= ICON_IN_SAVE_SET;
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ProcessMwmHints(mut pCD: *mut ClientData, mut first_time: Boolean) {
    let mut pHints = ::core::ptr::null_mut::<PropMwmHints>();
    ProcessEwmhWindowType(pCD);
    if first_time != 0 {
        if (*pCD).clientFunctions as ::core::ffi::c_long & WM_FUNC_DEFAULT != 0 {
            if (*pCD).clientFlags & CLIENT_TRANSIENT != 0 {
                (*pCD).clientFunctions = (*(*pCD).pSD).transientFunctions;
            } else {
                (*pCD).clientFunctions = WM_FUNC_ALL as ::core::ffi::c_int;
            }
        }
        if (*pCD).clientDecoration as ::core::ffi::c_long & WM_DECOR_DEFAULT != 0 {
            if (*pCD).clientFlags & CLIENT_TRANSIENT != 0 {
                (*pCD).clientDecoration = (*(*pCD).pSD).transientDecoration;
            } else {
                (*pCD).clientDecoration = WM_DECOR_ALL as ::core::ffi::c_int;
            }
        }
    }
    (*pCD).inputMode = MWM_INPUT_MODELESS;
    pHints = GetMwmHints(pCD);
    if !pHints.is_null() {
        if (*pHints).flags & MWM_HINTS_FUNCTIONS as ::core::ffi::c_ulong != 0 {
            if first_time == 0 {
                (*pCD).clientFunctions = WM_FUNC_ALL as ::core::ffi::c_int;
            }
            if (*pHints).functions & MWM_FUNC_ALL as ::core::ffi::c_ulong != 0 {
                (*pCD).clientFunctions = ((*pCD).clientFunctions as ::core::ffi::c_ulong
                    & !(*pHints).functions)
                    as ::core::ffi::c_int;
            } else {
                (*pCD).clientFunctions = ((*pCD).clientFunctions as ::core::ffi::c_ulong
                    & (*pHints).functions)
                    as ::core::ffi::c_int;
            }
        }
        if (*pHints).flags & MWM_HINTS_DECORATIONS as ::core::ffi::c_ulong != 0 {
            if first_time == 0 {
                (*pCD).clientDecoration = WM_DECOR_ALL as ::core::ffi::c_int;
            }
            if (*pHints).decorations & MWM_DECOR_ALL as ::core::ffi::c_ulong != 0 {
                (*pCD).clientDecoration = ((*pCD).clientDecoration as ::core::ffi::c_ulong
                    & !(*pHints).decorations)
                    as ::core::ffi::c_int;
            } else {
                (*pCD).clientDecoration = ((*pCD).clientDecoration as ::core::ffi::c_ulong
                    & (*pHints).decorations)
                    as ::core::ffi::c_int;
            }
            if (*pCD).clientDecoration as ::core::ffi::c_long
                & (MWM_DECOR_MENU | MWM_DECOR_MINIMIZE | MWM_DECOR_MAXIMIZE)
                != 0
            {
                (*pCD).clientDecoration = ((*pCD).clientDecoration as ::core::ffi::c_long
                    | MWM_DECOR_TITLE)
                    as ::core::ffi::c_int;
            }
            if (*pCD).clientDecoration as ::core::ffi::c_long & MWM_DECOR_RESIZEH != 0 {
                (*pCD).clientDecoration = ((*pCD).clientDecoration as ::core::ffi::c_long
                    | MWM_DECOR_BORDER)
                    as ::core::ffi::c_int;
            }
        }
        if (*pHints).flags & MWM_HINTS_INPUT_MODE as ::core::ffi::c_ulong != 0 {
            if (*pHints).inputMode == MWM_INPUT_PRIMARY_APPLICATION_MODAL as ::core::ffi::c_long
                || (*pHints).inputMode == MWM_INPUT_FULL_APPLICATION_MODAL as ::core::ffi::c_long
                || (*pHints).inputMode == MWM_INPUT_SYSTEM_MODAL as ::core::ffi::c_long
                    && wmGD.systemModalActive == 0
            {
                (*pCD).inputMode = (*pHints).inputMode as ::core::ffi::c_int;
            }
            if (*pCD).inputMode == MWM_INPUT_SYSTEM_MODAL {
                (*pCD).transientLeader = ::core::ptr::null_mut::<_ClientData>();
                if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0 {
                    (*pCD).clientFunctions = ((*pCD).clientFunctions as ::core::ffi::c_long
                        & !((1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int))
                        as ::core::ffi::c_int;
                }
            }
        }
        if (*pHints).flags & MWM_HINTS_STATUS as ::core::ffi::c_ulong != 0 {
            (*pCD).window_status = (*pHints).status as ::core::ffi::c_long;
        }
        XFree(pHints as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    }
    if !(*pCD).transientLeader.is_null()
        && (*(*pCD).transientLeader).wsmFunctions & WSM_FUNCTION_OCCUPY_WS == 0
    {
        (*pCD).wsmFunctions &= !WSM_FUNCTION_OCCUPY_WS;
    }
    if (*FindTransientTreeLeader(pCD)).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE
        == 0
    {
        (*pCD).clientFunctions = ((*pCD).clientFunctions as ::core::ffi::c_long
            & !MWM_FUNC_MINIMIZE) as ::core::ffi::c_int;
    }
    if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_RESIZE == 0 {
        (*pCD).clientDecoration = ((*pCD).clientDecoration as ::core::ffi::c_long
            & !MWM_DECOR_RESIZEH) as ::core::ffi::c_int;
    }
    if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE == 0 {
        (*pCD).clientDecoration = ((*pCD).clientDecoration as ::core::ffi::c_long
            & !MWM_DECOR_MINIMIZE) as ::core::ffi::c_int;
    }
    if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MAXIMIZE == 0 {
        (*pCD).clientDecoration = ((*pCD).clientDecoration as ::core::ffi::c_long
            & !MWM_DECOR_MAXIMIZE) as ::core::ffi::c_int;
    }
    (*pCD).decor = (*pCD).clientDecoration as ::core::ffi::c_long;
    if (*pCD).desktopSurface != 0 {
        (*pCD).decor = 0 as ::core::ffi::c_long;
        (*pCD).clientDecoration = (*pCD).decor as ::core::ffi::c_int;
        (*pCD).matteWidth = 0 as ::core::ffi::c_int;
        (*pCD).internalBevel = 0 as ::core::ffi::c_int;
        (*pCD).clientFunctions = 0 as ::core::ffi::c_int;
        (*pCD).focusAutoRaise = False as Boolean;
    }
}
