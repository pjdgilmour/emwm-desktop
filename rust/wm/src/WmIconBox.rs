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
use crate::abi::_ButtonSpec_WmFeedback as _ButtonSpec;
use crate::abi::_ClickData_WmFeedback as _ClickData;
use crate::abi::_ClientData_WmFeedback as _ClientData;
use crate::abi::_ClientListEntry_WmFeedback as _ClientListEntry;
use crate::abi::_CoreClassPart_WmFeedback as _CoreClassPart;
use crate::abi::_CorePart_WmFeedback as _CorePart;
use crate::abi::_IconBoxData_WmFeedback as _IconBoxData;
use crate::abi::_IconInfo_WmFeedback as _IconInfo;
use crate::abi::_IconPlacementData_WmFeedback as _IconPlacementData;
use crate::abi::_KeySpec_WmFeedback as _KeySpec;
use crate::abi::_MenuButton_WmFeedback as _MenuButton;
use crate::abi::_MenuItem_WmFeedback as _MenuItem;
use crate::abi::_MenuSpec_WmFeedback as _MenuSpec;
use crate::abi::_WidgetClassRec_WmFeedback as _WidgetClassRec;
use crate::abi::_WidgetRec_WmFeedback as _WidgetRec;
use crate::abi::_WmGlobalData_WmFeedback as _WmGlobalData;
use crate::abi::_WmScreenData_WmFeedback as _WmScreenData;
use crate::abi::_WmTimer_WmFeedback as _WmTimer;
use crate::abi::_WmWorkspaceData_WmFeedback as _WmWorkspaceData;
use crate::abi::_WsClientData_WmFeedback as _WsClientData;
use crate::abi::_WsPresenceData_WmFeedback as _WsPresenceData;
use crate::abi::_XtActionsRec_WmFeedback as _XtActionsRec;
use crate::abi::_XtCallbackRec_WmFeedback as _XtCallbackRec;
use crate::abi::_XtTMRec_WmFeedback as _XtTMRec;
use crate::abi::ArgList_WmFeedback as ArgList;
use crate::abi::ButtonSpec_WmFeedback as ButtonSpec;
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed_0;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_1;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_10;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_2;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_3;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_4;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_5;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_6;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_7;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_8;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_9;
use crate::abi::C2RustUnnamed_WmArgb as C2RustUnnamed;
use crate::abi::ClickData_WmFeedback as ClickData;
use crate::abi::ClientData_WmFeedback as ClientData;
use crate::abi::ClientListEntry_WmFeedback as ClientListEntry;
use crate::abi::CoreClassPart_WmFeedback as CoreClassPart;
use crate::abi::CorePart_WmFeedback as CorePart;
use crate::abi::IconBoxData_WmFeedback as IconBoxData;
use crate::abi::IconInfo_WmFeedback as IconInfo;
use crate::abi::IconPlacementData_WmFeedback as IconPlacementData;
use crate::abi::KeySpec_WmFeedback as KeySpec;
use crate::abi::MenuButton_WmFeedback as MenuButton;
use crate::abi::MenuItem_WmFeedback as MenuItem;
use crate::abi::MenuSpec_WmFeedback as MenuSpec;
use crate::abi::WidgetClass_WmFeedback as WidgetClass;
use crate::abi::WidgetList_WmFeedback as WidgetList;
use crate::abi::Widget_WmFeedback as Widget;
use crate::abi::WmFunction_WmFeedback as WmFunction;
use crate::abi::WmGlobalData_WmFeedback as WmGlobalData;
use crate::abi::WmScreenData_WmFeedback as WmScreenData;
use crate::abi::WmTimer_WmFeedback as WmTimer;
use crate::abi::WsClientData_WmIDecor as WsClientData;
use crate::abi::WsPresenceData_WmFeedback as WsPresenceData;
use crate::abi::XtAcceptFocusProc_WmFeedback as XtAcceptFocusProc;
use crate::abi::XtActionList_WmFeedback as XtActionList;
use crate::abi::XtActionProc_WmFeedback as XtActionProc;
use crate::abi::XtAlmostProc_WmFeedback as XtAlmostProc;
use crate::abi::XtArgsFunc_WmFeedback as XtArgsFunc;
use crate::abi::XtArgsProc_WmFeedback as XtArgsProc;
use crate::abi::XtBoundActions_WmFeedback as XtBoundActions;
use crate::abi::XtCallbackList_WmFeedback as XtCallbackList;
use crate::abi::XtCallbackProc_WmFeedback as XtCallbackProc;
use crate::abi::XtEventHandler_WmIconBox as XtEventHandler;
use crate::abi::XtExposeProc_WmFeedback as XtExposeProc;
use crate::abi::XtGeometryHandler_WmFeedback as XtGeometryHandler;
use crate::abi::XtInitProc_WmFeedback as XtInitProc;
use crate::abi::XtRealizeProc_WmFeedback as XtRealizeProc;
use crate::abi::XtSetValuesFunc_WmFeedback as XtSetValuesFunc;
use crate::abi::XtStringProc_WmFeedback as XtStringProc;
use crate::abi::XtTMRec_WmFeedback as XtTMRec;
use crate::abi::XtWidgetClassProc_WmFeedback as XtWidgetClassProc;
use crate::abi::XtWidgetGeometry_WmFeedback as XtWidgetGeometry;
use crate::abi::XtWidgetProc_WmFeedback as XtWidgetProc;
use crate::abi::*;
extern "C" {
    fn XCreateGC(_: *mut Display, _: Drawable, _: ::core::ffi::c_ulong, _: *mut XGCValues) -> GC;
    fn XCreateBitmapFromData(
        _: *mut Display,
        _: Drawable,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> Pixmap;
    fn XCopyGC(_: *mut Display, _: GC, _: ::core::ffi::c_ulong, _: GC) -> ::core::ffi::c_int;
    fn XMapRaised(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XParseGeometry(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XRaiseWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XSetBackground(_: *mut Display, _: GC, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn XSetFillStyle(_: *mut Display, _: GC, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XSetForeground(_: *mut Display, _: GC, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn XSetStipple(_: *mut Display, _: GC, _: Pixmap) -> ::core::ffi::c_int;
    fn XSetTile(_: *mut Display, _: GC, _: Pixmap) -> ::core::ffi::c_int;
    fn XTranslateCoordinates(
        _: *mut Display,
        _: Window,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut Window,
    ) -> ::core::ffi::c_int;
    fn XUnmapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn XtUnmanageChild(_: Widget);
    fn XtTranslateCoords(_: Widget, _: Position, _: Position, _: *mut Position, _: *mut Position);
    fn XtAddEventHandler(_: Widget, _: EventMask, _: Boolean, _: XtEventHandler, _: XtPointer);
    fn XtRealizeWidget(_: Widget);
    fn XtDestroyWidget(_: Widget);
    fn XtWindowToWidget(_: *mut Display, _: Window) -> Widget;
    fn XtWindowOfObject(_: Widget) -> Window;
    fn XtAddCallback(_: Widget, _: *const ::core::ffi::c_char, _: XtCallbackProc, _: XtPointer);
    fn XtCreatePopupShell(
        _: *const ::core::ffi::c_char,
        _: WidgetClass,
        _: Widget,
        _: ArgList,
        _: Cardinal,
    ) -> Widget;
    fn XtPopup(_: Widget, _: XtGrabKind);
    fn XtCreateManagedWidget(
        _: *const ::core::ffi::c_char,
        _: WidgetClass,
        _: Widget,
        _: ArgList,
        _: Cardinal,
    ) -> Widget;
    fn XtSetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtGetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    fn XtMoveWidget(_: Widget, _: Position, _: Position);
    static XtStrings: [::core::ffi::c_char; 0];
    static XtShellStrings: [::core::ffi::c_char; 0];
    static mut topLevelShellWidgetClass: WidgetClass;
    static mut _XmStrings: [::core::ffi::c_char; 0];
    fn XmStringCreateLocalized(text: String_0) -> XmString;
    fn XmStringUnparse(
        string: XmString,
        tag: XmStringTag,
        tag_type: XmTextType,
        output_type: XmTextType,
        parse_table: XmParseTable,
        parse_count: Cardinal,
        parse_model: XmParseModel,
    ) -> XtPointer;
    static mut wmGD: WmGlobalData;
    static mut xmFrameWidgetClass: WidgetClass;
    static mut xmDrawnButtonWidgetClass: WidgetClass;
    static mut xmScrolledWindowWidgetClass: WidgetClass;
    fn XmScrollVisible(scrw: Widget, wid: Widget, hor_margin: Dimension, ver_margin: Dimension);
    static mut xmBulletinBoardWidgetClass: WidgetClass;
    fn SetFrameInfo(pcd: *mut ClientData);
    fn InitTitleBarHeight(pcd: *mut ClientData) -> ::core::ffi::c_uint;
    fn Warning(message: *mut ::core::ffi::c_char);
    fn HandleKeyPress(
        keyEvent: *mut XKeyEvent,
        keySpecs: *mut KeySpec,
        checkContext: Boolean,
        context: Context,
        onlyFirst: Boolean,
        pCD: *mut ClientData,
    ) -> Boolean;
    fn F_Restore_And_Raise(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn HideActiveIconText(pSD: *mut WmScreenData);
    fn ShowActiveIconText(pcd: *mut ClientData);
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
    fn MoveIconInfo(
        pIPD1: *mut IconPlacementData,
        p1: ::core::ffi::c_int,
        pIPD2: *mut IconPlacementData,
        p2: ::core::ffi::c_int,
    );
    fn MakeIconPixmap(
        pCD: *mut ClientData,
        bitmap: Pixmap,
        mask: Pixmap,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        depth: ::core::ffi::c_uint,
    ) -> Pixmap;
    fn ManageWindow(pSD: *mut WmScreenData, clientWindow: Window, manageFlags: ::core::ffi::c_long);
    fn UnManageWindow(pCD: *mut ClientData);
    fn ParseMwmMenuStr(pSD: *mut WmScreenData, menuStr: *mut ::core::ffi::c_uchar)
        -> *mut MenuItem;
    fn ToLower(string: *mut ::core::ffi::c_uchar);
    fn ProcessClientResources(pCD: *mut ClientData);
    fn PlaceFrameOnScreen(
        pCD: *mut ClientData,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
    );
    fn GetWorkspaceData(pSD: *mut WmScreenData, wsID: WorkspaceID) -> *mut WmWorkspaceData;
    fn GetWsClientData(pWS: *mut WmWorkspaceData, pCD: *mut ClientData) -> *mut WsClientData;
    fn SetClientWsIndex(pCD: *mut ClientData);
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const XtGrabExclusive: XtGrabKind = 2;
pub const XtGrabNonexclusive: XtGrabKind = 1;
pub const XtGrabNone: XtGrabKind = 0;
pub const XmNO_TEXT: XmTextType = 3;
pub const XmWIDECHAR_TEXT: XmTextType = 2;
pub const XmMULTIBYTE_TEXT: XmTextType = 1;
pub const XmCHARSET_TEXT: XmTextType = 0;
pub const XmOUTPUT_BOTH: XmParseModel = 4;
pub const XmOUTPUT_END: XmParseModel = 3;
pub const XmOUTPUT_BEGINNING: XmParseModel = 2;
pub const XmOUTPUT_BETWEEN: XmParseModel = 1;
pub const XmOUTPUT_ALL: XmParseModel = 0;
pub const XmPOINTER: C2RustUnnamed_1 = 1;
pub const XmEXPLICIT: C2RustUnnamed_1 = 0;
pub const XmRESIZE_ANY: C2RustUnnamed_2 = 2;
pub const XmRESIZE_GROW: C2RustUnnamed_2 = 1;
pub const XmRESIZE_NONE: C2RustUnnamed_2 = 0;
pub const XmSHADOW_OUT: C2RustUnnamed_3 = 8;
pub const XmSHADOW_IN: C2RustUnnamed_3 = 7;
pub const XmPIXMAP_AND_STRING: C2RustUnnamed_4 = 3;
pub const XmSTRING: C2RustUnnamed_4 = 2;
pub const XmPIXMAP: C2RustUnnamed_4 = 1;
pub const XmDYNAMIC: C2RustUnnamed_5 = 1;
pub const XmSTATIC: C2RustUnnamed_5 = 0;
pub const XmRESIZE_IF_POSSIBLE: C2RustUnnamed_6 = 2;
pub const XmCONSTANT: C2RustUnnamed_6 = 1;
pub const XmVARIABLE: C2RustUnnamed_6 = 0;
pub const XmAPPLICATION_DEFINED: C2RustUnnamed_7 = 1;
pub const XmAUTOMATIC: C2RustUnnamed_7 = 0;
pub const XmAS_NEEDED: C2RustUnnamed_8 = 1;
pub const XmDIALOG_SYSTEM_MODAL: C2RustUnnamed_9 = 3;
pub const XmDIALOG_FULL_APPLICATION_MODAL: C2RustUnnamed_9 = 2;
pub const XmDIALOG_PRIMARY_APPLICATION_MODAL: C2RustUnnamed_9 = 1;
pub const XmDIALOG_MODELESS: C2RustUnnamed_9 = 0;
pub const XmDIALOG_FILE_SELECTION: C2RustUnnamed_10 = 4;
pub const XmDIALOG_COMMAND: C2RustUnnamed_10 = 3;
pub const XmDIALOG_SELECTION: C2RustUnnamed_10 = 2;
pub const XmDIALOG_PROMPT: C2RustUnnamed_10 = 1;
pub const XmDIALOG_WORK_AREA: C2RustUnnamed_10 = 0;
pub const XmTRAVERSE_GLOBALLY_BACKWARD: XmTraversalDirection = 11;
pub const XmTRAVERSE_GLOBALLY_FORWARD: XmTraversalDirection = 10;
pub const XmTRAVERSE_RIGHT: XmTraversalDirection = 9;
pub const XmTRAVERSE_LEFT: XmTraversalDirection = 8;
pub const XmTRAVERSE_DOWN: XmTraversalDirection = 7;
pub const XmTRAVERSE_UP: XmTraversalDirection = 6;
pub const XmTRAVERSE_PREV_TAB_GROUP: XmTraversalDirection = 5;
pub const XmTRAVERSE_NEXT_TAB_GROUP: XmTraversalDirection = 4;
pub const XmTRAVERSE_HOME: XmTraversalDirection = 3;
pub const XmTRAVERSE_PREV: XmTraversalDirection = 2;
pub const XmTRAVERSE_NEXT: XmTraversalDirection = 1;
pub const XmTRAVERSE_CURRENT: XmTraversalDirection = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const KeyPressMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const Button1MotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int;
pub const Button2MotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const StructureNotifyMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 17 as ::core::ffi::c_int;
pub const FocusChangeMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 21 as ::core::ffi::c_int;
pub const FocusIn: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const CWWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const CWHeight: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const FillTiled: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FillStippled: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XValue: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const YValue: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WidthValue: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const HeightValue: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const XNegative: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const YNegative: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_FUNC_CLOSE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
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
pub const XmUNSPECIFIED_PIXMAP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SELECT_BUTTON_MOTION_MASK: ::core::ffi::c_long = Button1MotionMask;
pub const DMANIP_BUTTON_MOTION_MASK: ::core::ffi::c_long = Button2MotionMask;
pub const MANAGEW_ICON_BOX: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const FRAME_INTERNAL_SHADOW_WIDTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICONBOX_PART_IICON: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const ICONBOX_PART_WICON: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const F_SUBCONTEXT_IB_IICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_IICON;
pub const F_SUBCONTEXT_IB_WICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_WICON;
pub const IB_SPACING: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const IB_MARGIN_HEIGHT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const IB_MARGIN_WIDTH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const IB_HIGHLIGHT_BORDER: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAXIMIZED_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ICON_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const ICON_IMAGE_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const ICON_ACTIVE_LABEL_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ICON_PLACE_LEFT_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const ICON_PLACE_TOP_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ICON_PLACE_BOTTOM_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const NO_ICON_PLACE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const iImage_width: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const iImage_height: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
static mut iImage_bits: [::core::ffi::c_uchar; 350] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
pub const greyed75_width: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const greyed75_height: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
static mut greyed75_bits: [::core::ffi::c_uchar; 32] = [
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
pub const slant2_width: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const slant2_height: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
static mut slant2_bits: [::core::ffi::c_uchar; 32] = [
    0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
pub const WmNiconBox: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"iconbox\0") };
pub const WmCIconBox: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"Iconbox\0") };
#[no_mangle]
pub static mut select_color: Pixel = 0;
#[no_mangle]
pub static mut greyedPixmap: Pixmap = 0;
#[no_mangle]
pub static mut frameShadowThickness: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut firstTime: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut insertPosition: Cardinal = 0 as Cardinal;
pub const DEFAULT_ICON_BOX_TITLE: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Icons\0") };
#[no_mangle]
pub static mut szhorizontal: *const ::core::ffi::c_char =
    b"horizontal\0" as *const u8 as *const ::core::ffi::c_char;
#[no_mangle]
pub static mut szvertical: *const ::core::ffi::c_char =
    b"vertical\0" as *const u8 as *const ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn InitIconBox(mut pSD: *mut WmScreenData) {
    let mut iws: ::core::ffi::c_int = 0;
    iws = 0 as ::core::ffi::c_int;
    while iws < (*pSD).numWorkspaces {
        AddIconBoxForWorkspace((*pSD).pWS.offset(iws as isize) as *mut WmWorkspaceData);
        iws += 1;
    }
    if (*pSD).fadeNormalIcon != 0 {
        MakeFadeIconGC(pSD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn AddIconBoxForWorkspace(mut pWS: *mut WmWorkspaceData) {
    extern "C" {
        static mut pIconBoxInitialWS: *mut WmWorkspaceData;
    }
    pIconBoxInitialWS = pWS;
    ManageWindow((*pWS).pSD, None_0 as Window, MANAGEW_ICON_BOX);
}
#[no_mangle]
pub unsafe extern "C" fn MakeIconBox(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
) -> Boolean {
    let mut pIBD = ::core::ptr::null_mut::<IconBoxData>();
    if !pCD.is_null() {
        pIBD = XtMalloc(::core::mem::size_of::<IconBoxData>() as Cardinal) as *mut IconBoxData;
        if pIBD.is_null() {
            Warning(
                b"Insufficient memory to create icon box data\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            XtFree(pCD as *mut ::core::ffi::c_char);
            return 0 as Boolean;
        }
        InitializeIconBoxData(pWS, pIBD);
        InitializeClientData(pCD, pIBD);
        (*pIBD).IPD.placeList = XtMalloc(
            ((*pIBD).IPD.totalPlaces as usize)
                .wrapping_mul(::core::mem::size_of::<IconInfo>() as usize) as Cardinal,
        ) as *mut IconInfo;
        if (*pIBD).IPD.placeList.is_null() {
            Warning(
                b"Insufficient memory to create icon box data\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            XtFree(pIBD as *mut ::core::ffi::c_char);
            XtFree(pCD as *mut ::core::ffi::c_char);
            return 0 as Boolean;
        }
        memset(
            (*pIBD).IPD.placeList as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*pIBD).IPD.totalPlaces as size_t)
                .wrapping_mul(::core::mem::size_of::<IconInfo>() as size_t),
        );
        MakeShell(pWS, pIBD);
        MakeScrolledWindow(pWS, pIBD);
        MakeBulletinBoard(pWS, pIBD);
        RealizeIconBox(pWS, pIBD, pCD);
        AddNewBox(pWS, pIBD);
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyIconBox(mut pWS: *mut WmWorkspaceData) {
    let mut pIBD = ::core::ptr::null_mut::<IconBoxData>();
    pIBD = (*pWS).pIconBox;
    XtDestroyWidget((*pIBD).shellWidget);
    UnManageWindow((*pIBD).pCD_iconBox as *mut ClientData);
    XtFree(pIBD as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn MakeShell(mut pWS: *mut WmWorkspaceData, mut pIBD: *mut IconBoxData) {
    let mut setArgs: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut i: ::core::ffi::c_int = 0;
    let mut pchIBTitle = ::core::ptr::null_mut::<::core::ffi::c_char>();
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(280 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(124 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7466 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = XmEXPLICIT as ::core::ffi::c_int as XtArgVal;
    i += 1;
    if !(*(*pWS).pSD).iconBoxTitle.is_null() {
        pchIBTitle = WmXmStringToString((*(*pWS).pSD).iconBoxTitle) as *mut ::core::ffi::c_char;
        setArgs[i as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
            .offset(356 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = pchIBTitle as XtArgVal;
        i += 1;
        setArgs[i as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
            .offset(0 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = pchIBTitle as XtArgVal;
        i += 1;
    }
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(453 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(6047 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = XmDIALOG_MODELESS as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(163 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*pWS).pSD).screen as isize))
    .root_depth as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(584 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*pWS).pSD).screen as isize) as *mut Screen
        as XtArgVal;
    i += 1;
    (*pIBD).shellWidget = XtCreatePopupShell(
        WmNiconBox.as_ptr(),
        topLevelShellWidgetClass,
        (*pWS).workspaceTopLevelW,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
    if !pchIBTitle.is_null() {
        XtFree(pchIBTitle);
    }
}
#[no_mangle]
pub unsafe extern "C" fn MakeScrolledWindow(
    mut pWS: *mut WmWorkspaceData,
    mut pIBD: *mut IconBoxData,
) {
    let mut setArgs: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(124 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7955 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7909 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9670 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = XmSHADOW_OUT as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9654 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = frameShadowThickness as XtArgVal;
    i += 1;
    (*pIBD).frameWidget = XtCreateManagedWidget(
        b"IBframe\0" as *const u8 as *const ::core::ffi::c_char,
        xmFrameWidgetClass,
        (*pIBD).shellWidget,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9447 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = XmAUTOMATIC as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(124 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9900 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 3 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9421 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 3 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9394 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 3 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9654 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = wmGD.frameExternalShadowWidth as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9291 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = XmSTATIC as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10715 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = XmVARIABLE as ::core::ffi::c_int as XtArgVal;
    i += 1;
    (*pIBD).scrolledWidget = XtCreateManagedWidget(
        b"IBsWindow\0" as *const u8 as *const ::core::ffi::c_char,
        xmScrolledWindowWidgetClass,
        (*pIBD).frameWidget,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
    XtAddCallback(
        (*pIBD).scrolledWidget,
        (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(10421 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(Widget, XtPointer, *mut XmAnyCallbackStruct) -> ()>,
            XtCallbackProc,
        >(Some(
            IconScrollVisibleCallback
                as unsafe extern "C" fn(Widget, XtPointer, *mut XmAnyCallbackStruct) -> (),
        )),
        NULL,
    );
    XtAddEventHandler(
        (*pIBD).scrolledWidget,
        StructureNotifyMask as EventMask,
        False as Boolean,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(Widget, *mut IconBoxData, *mut XConfigureEvent) -> ()>,
            XtEventHandler,
        >(Some(
            UpdateIncrements
                as unsafe extern "C" fn(Widget, *mut IconBoxData, *mut XConfigureEvent) -> (),
        )),
        pIBD as XtPointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MakeBulletinBoard(
    mut pWS: *mut WmWorkspaceData,
    mut pIBD: *mut IconBoxData,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut setArgs: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(124 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9654 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9900 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7909 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7955 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(6047 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = XmDIALOG_WORK_AREA as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9086 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = XmRESIZE_NONE as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5955 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(326 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value =
        ::core::mem::transmute::<Option<unsafe extern "C" fn(Widget) -> Cardinal>, XtArgVal>(Some(
            InsertPosition as unsafe extern "C" fn(Widget) -> Cardinal,
        ));
    i += 1;
    (*pIBD).bBoardWidget = XtCreateManagedWidget(
        b"IBbBoard\0" as *const u8 as *const ::core::ffi::c_char,
        xmBulletinBoardWidgetClass,
        (*pIBD).scrolledWidget,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RealizeIconBox(
    mut pWS: *mut WmWorkspaceData,
    mut pIBD: *mut IconBoxData,
    mut pCD: *mut ClientData,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut getArgs: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 10];
    let mut setArgs: [Arg; 2] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 2];
    let mut clipWidget = ::core::ptr::null_mut::<_WidgetRec>();
    let mut bgPixmap: Pixmap = 0;
    let mut defaultImage: Pixmap = 0;
    XtRealizeWidget((*pIBD).shellWidget);
    (*pCD).client = XtWindowOfObject((*pIBD).shellWidget);
    SetGeometry(pWS, pCD, pIBD);
    (*pIBD).pCD_iconBox = pCD as *mut _ClientData;
    (*pCD).thisIconBox = pIBD;
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(52 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = select_color as XtArgVal;
    i += 1;
    XtSetValues(
        (*pIBD).bBoardWidget,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
    i = 0 as ::core::ffi::c_int;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(63 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut bgPixmap as XtArgVal;
    i += 1;
    XtGetValues(
        (*pIBD).bBoardWidget,
        &raw mut getArgs as ArgList,
        i as Cardinal,
    );
    i = 0 as ::core::ffi::c_int;
    getArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5505 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut clipWidget as XtArgVal;
    i += 1;
    XtGetValues(
        (*pIBD).scrolledWidget,
        &raw mut getArgs as ArgList,
        i as Cardinal,
    );
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(52 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = select_color as XtArgVal;
    i += 1;
    XtSetValues(
        clipWidget,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
    (*pIBD).clipWidget = clipWidget;
    MakeShrinkWrapIconsGC((*pWS).pSD, bgPixmap);
    if (*(*pWS).pSD).iconDecoration as ::core::ffi::c_long & ICON_IMAGE_PART != 0 {
        defaultImage = XCreateBitmapFromData(
            wmGD.display,
            (*(*pWS).pSD).rootWindow as Drawable,
            &raw mut iImage_bits as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            iImage_width as ::core::ffi::c_uint,
            iImage_height as ::core::ffi::c_uint,
        );
        (*(*pWS).pSD).defaultPixmap = MakeIconPixmap(
            pCD,
            defaultImage,
            None_0 as Pixmap,
            iImage_width as ::core::ffi::c_uint,
            iImage_height as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn AddNewBox(mut pWS: *mut WmWorkspaceData, mut pIBD: *mut IconBoxData) {
    let mut pibd = ::core::ptr::null_mut::<IconBoxData>();
    if !(*pWS).pIconBox.is_null() {
        pibd = (*pWS).pIconBox;
        while !(*pibd).pNextIconBox.is_null() {
            pibd = (*pibd).pNextIconBox as *mut IconBoxData;
        }
        (*pibd).pNextIconBox = pIBD as *mut _IconBoxData;
    } else {
        (*pWS).pIconBox = pIBD;
    };
}
#[no_mangle]
pub unsafe extern "C" fn InitializeIconBoxData(
    mut pWS: *mut WmWorkspaceData,
    mut pIBD: *mut IconBoxData,
) {
    let mut mask: ::core::ffi::c_int = 0;
    let mut X: ::core::ffi::c_int = 0;
    let mut Y: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut sW: ::core::ffi::c_int = 0;
    let mut sH: ::core::ffi::c_int = 0;
    frameShadowThickness = FRAME_INTERNAL_SHADOW_WIDTH;
    (*pIBD).numberOfIcons = 0 as ::core::ffi::c_int;
    (*pIBD).currentRow = 0 as ::core::ffi::c_int;
    (*pIBD).currentCol = 0 as ::core::ffi::c_int;
    (*pIBD).lastRow = 0 as ::core::ffi::c_int;
    (*pIBD).lastCol = 0 as ::core::ffi::c_int;
    (*pIBD).IPD.placeList = ::core::ptr::null_mut::<IconInfo>();
    (*pIBD).scrolledWidget = ::core::ptr::null_mut::<_WidgetRec>();
    (*pIBD).bBoardWidget = ::core::ptr::null_mut::<_WidgetRec>();
    (*pIBD).clipWidget = ::core::ptr::null_mut::<_WidgetRec>();
    (*pIBD).wsID = (*pWS).id;
    ToLower((*(*pWS).pSD).iconBoxSBDisplayPolicy as *mut ::core::ffi::c_uchar);
    if !(strcmp(
        (*(*pWS).pSD).iconBoxSBDisplayPolicy as *const ::core::ffi::c_char,
        b"all\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            (*(*pWS).pSD).iconBoxSBDisplayPolicy as *const ::core::ffi::c_char,
            szvertical,
        ) == 0
        || strcmp(
            (*(*pWS).pSD).iconBoxSBDisplayPolicy as *const ::core::ffi::c_char,
            szhorizontal,
        ) == 0)
    {
        strcpy(
            (*(*pWS).pSD).iconBoxSBDisplayPolicy as *mut ::core::ffi::c_char,
            b"all\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*pWS).iconBoxGeometry.is_null() {
        if (*(*pWS).pSD).iconPlacement as ::core::ffi::c_long
            & (ICON_PLACE_TOP_PRIMARY | ICON_PLACE_BOTTOM_PRIMARY)
            != 0
        {
            (*pIBD).IPD.iconPlacement = ICON_PLACE_TOP_PRIMARY as ::core::ffi::c_uint;
            (*pIBD).IPD.placementCols = 1 as ::core::ffi::c_int;
            (*pIBD).IPD.placementRows = 6 as ::core::ffi::c_int;
        } else {
            (*pIBD).IPD.iconPlacement = ICON_PLACE_LEFT_PRIMARY as ::core::ffi::c_uint;
            (*pIBD).IPD.placementCols = 6 as ::core::ffi::c_int;
            (*pIBD).IPD.placementRows = 1 as ::core::ffi::c_int;
        }
    } else {
        mask = XParseGeometry(
            (*pWS).iconBoxGeometry as *const ::core::ffi::c_char,
            &raw mut X,
            &raw mut Y,
            &raw mut width,
            &raw mut height,
        );
        if mask & WidthValue != 0 && width > 0 as ::core::ffi::c_uint {
            (*pIBD).IPD.placementCols = width as ::core::ffi::c_int;
        } else {
            (*pIBD).IPD.placementCols = 6 as ::core::ffi::c_int;
        }
        if mask & HeightValue != 0 && height > 0 as ::core::ffi::c_uint {
            (*pIBD).IPD.placementRows = height as ::core::ffi::c_int;
        } else {
            (*pIBD).IPD.placementRows = 1 as ::core::ffi::c_int;
        }
        if (*pIBD).IPD.placementRows <= (*pIBD).IPD.placementCols {
            (*pIBD).IPD.iconPlacement = ICON_PLACE_LEFT_PRIMARY as ::core::ffi::c_uint;
        } else {
            (*pIBD).IPD.iconPlacement = ICON_PLACE_TOP_PRIMARY as ::core::ffi::c_uint;
        }
    }
    if strcmp(
        (*(*pWS).pSD).iconBoxSBDisplayPolicy as *const ::core::ffi::c_char,
        szvertical,
    ) == 0
    {
        (*pIBD).IPD.iconPlacement = ICON_PLACE_LEFT_PRIMARY as ::core::ffi::c_uint;
    } else if strcmp(
        (*(*pWS).pSD).iconBoxSBDisplayPolicy as *const ::core::ffi::c_char,
        szhorizontal,
    ) == 0
    {
        (*pIBD).IPD.iconPlacement = ICON_PLACE_TOP_PRIMARY as ::core::ffi::c_uint;
    }
    sW = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*pWS).pSD).screen as isize))
    .width
        / (*(*pWS).pSD).iconWidth;
    sH = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*pWS).pSD).screen as isize))
    .height
        / (*(*pWS).pSD).iconHeight;
    (*pIBD).IPD.totalPlaces = sW * sH;
    (*pIBD).IPD.onRootWindow = False as Boolean;
    (*pIBD).pNextIconBox = ::core::ptr::null_mut::<_IconBoxData>();
}
#[no_mangle]
pub unsafe extern "C" fn SetIconBoxInfo(mut pWS: *mut WmWorkspaceData, mut pCD: *mut ClientData) {
    (*pCD).clientClass = WmCIconBox.as_ptr() as String_0;
    (*pCD).clientName = (*(*pWS).pSD).iconBoxName;
    ProcessClientResources(pCD);
}
#[no_mangle]
pub unsafe extern "C" fn InitializeClientData(
    mut pCD: *mut ClientData,
    mut pIBD: *mut IconBoxData,
) {
    (*pCD).internalBevel = if wmGD.frameStyle as ::core::ffi::c_uint
        == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        0 as ::core::ffi::c_int
    } else {
        FRAME_INTERNAL_SHADOW_WIDTH
    };
    (*pCD).clientX = 0 as ::core::ffi::c_int;
    (*pCD).clientY = 0 as ::core::ffi::c_int;
    (*pCD).clientFlags |= ICON_BOX;
    (*pIBD).IPD.iPlaceW =
        (*(*pCD).pSD).iconWidth + IB_SPACING + 2 as ::core::ffi::c_int * IB_MARGIN_WIDTH;
    (*pCD).widthInc = (*pIBD).IPD.iPlaceW;
    (*pIBD).IPD.iPlaceH =
        (*(*pCD).pSD).iconHeight + IB_SPACING + 2 as ::core::ffi::c_int * IB_MARGIN_HEIGHT;
    (*pCD).heightInc = (*pIBD).IPD.iPlaceH;
    (*pCD).clientWidth = (*pIBD).IPD.placementCols * (*pCD).widthInc;
    (*pCD).clientHeight = (*pIBD).IPD.placementRows * (*pCD).heightInc;
    if (*(*pCD).pSD).iconBoxTitle.is_null() {
        (*(*pCD).pSD).iconBoxTitle =
            XmStringCreateLocalized(DEFAULT_ICON_BOX_TITLE.as_ptr() as String_0);
    }
    (*pCD).clientTitle = (*(*pCD).pSD).iconBoxTitle;
    (*pCD).iconTitle = (*(*pCD).pSD).iconBoxTitle;
}
#[no_mangle]
pub unsafe extern "C" fn MakeShrinkWrapIconsGC(mut pSD: *mut WmScreenData, mut bgPixmap: Pixmap) {
    let mut copyMask: XtGCMask = 0;
    if (*pSD).shrinkWrapGC.is_null() {
        (*pSD).shrinkWrapGC = XCreateGC(
            wmGD.display,
            (*pSD).rootWindow as Drawable,
            0 as ::core::ffi::c_ulong,
            NULL as *mut XGCValues,
        );
        copyMask = !(0 as ::core::ffi::c_long) as XtGCMask;
        XCopyGC(
            wmGD.display,
            (*pSD).iconAppearance.inactiveGC,
            copyMask as ::core::ffi::c_ulong,
            (*pSD).shrinkWrapGC,
        );
        if bgPixmap != XmUNSPECIFIED_PIXMAP as Pixmap {
            XSetTile(wmGD.display, (*pSD).shrinkWrapGC, bgPixmap);
            XSetFillStyle(wmGD.display, (*pSD).shrinkWrapGC, FillTiled);
            XSetBackground(
                wmGD.display,
                (*pSD).shrinkWrapGC,
                select_color as ::core::ffi::c_ulong,
            );
        } else {
            XSetForeground(
                wmGD.display,
                (*pSD).shrinkWrapGC,
                select_color as ::core::ffi::c_ulong,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn MakeFadeIconGC(mut pSD: *mut WmScreenData) {
    let mut copyMask: XtGCMask = 0;
    static mut tmpFontClipMask: Pixmap = 0;
    (*pSD).fadeIconGC = XCreateGC(
        wmGD.display,
        (*pSD).rootWindow as Drawable,
        0 as ::core::ffi::c_ulong,
        NULL as *mut XGCValues,
    );
    (*pSD).fadeIconTextGC = XCreateGC(
        wmGD.display,
        (*pSD).rootWindow as Drawable,
        0 as ::core::ffi::c_ulong,
        NULL as *mut XGCValues,
    );
    copyMask = !(0 as ::core::ffi::c_long) as XtGCMask;
    XCopyGC(
        wmGD.display,
        (*pSD).iconAppearance.inactiveGC,
        copyMask as ::core::ffi::c_ulong,
        (*pSD).fadeIconGC,
    );
    XCopyGC(
        wmGD.display,
        (*pSD).iconAppearance.inactiveGC,
        copyMask as ::core::ffi::c_ulong,
        (*pSD).fadeIconTextGC,
    );
    tmpFontClipMask = XCreateBitmapFromData(
        wmGD.display,
        (*pSD).rootWindow as Drawable,
        &raw mut greyed75_bits as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        greyed75_width as ::core::ffi::c_uint,
        greyed75_height as ::core::ffi::c_uint,
    );
    greyedPixmap = XCreateBitmapFromData(
        wmGD.display,
        (*pSD).rootWindow as Drawable,
        &raw mut slant2_bits as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        slant2_width as ::core::ffi::c_uint,
        slant2_height as ::core::ffi::c_uint,
    );
    XSetStipple(wmGD.display, (*pSD).fadeIconTextGC, tmpFontClipMask);
    XSetFillStyle(wmGD.display, (*pSD).fadeIconTextGC, FillStippled);
    XSetStipple(wmGD.display, (*pSD).fadeIconGC, greyedPixmap);
    XSetFillStyle(wmGD.display, (*pSD).fadeIconGC, FillStippled);
    XSetForeground(
        wmGD.display,
        (*pSD).fadeIconGC,
        select_color as ::core::ffi::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SetGeometry(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
    mut pIBD: *mut IconBoxData,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut setArgs: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 10];
    let mut mask: ::core::ffi::c_int = 0;
    let mut X: ::core::ffi::c_int = 0;
    let mut Y: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut boxdim: ::core::ffi::c_uint = 0;
    let mut tmpMin: ::core::ffi::c_uint = 0;
    let mut diff: ::core::ffi::c_int = 0;
    let mut decoration: ::core::ffi::c_ulong = 0;
    SetGranularity(pWS, pCD, pIBD);
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*pCD).clientWidth as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*pCD).clientHeight as XtArgVal;
    i += 1;
    XtSetValues(
        (*pIBD).bBoardWidget,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
    (*pCD).clientHeight = (*pCD).clientHeight + (*pCD).baseHeight;
    (*pCD).maxHeight = (*pCD).clientHeight;
    (*pCD).oldMaxHeight = (*pCD).maxHeight;
    (*pCD).clientWidth = (*pCD).clientWidth + (*pCD).baseWidth;
    (*pCD).maxWidth = (*pCD).clientWidth;
    (*pCD).oldMaxWidth = (*pCD).maxWidth;
    decoration = (*pCD).decor as ::core::ffi::c_ulong;
    if decoration & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0 {
        boxdim = InitTitleBarHeight(pCD);
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
    if (*pCD).clientWidth < (*pCD).minWidth {
        (*pCD).clientWidth = (*pCD).minWidth;
    }
    (*pIBD).IPD.placementCols = ((*pCD).clientWidth - (*pCD).baseWidth) / (*pCD).widthInc;
    if (*pCD).maxWidth < (*pCD).minWidth {
        (*pCD).maxWidth = (*pCD).minWidth;
    }
    (*pCD).maxWidthLimit = (*pCD).maxWidth;
    (*pCD).maxWidth -= ((*pCD).maxWidth - (*pCD).baseWidth) % (*pCD).widthInc;
    (*pCD).oldMaxWidth = (*pCD).maxWidth;
    (*pCD).maxHeightLimit = (*pCD).maxHeight;
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*pCD).clientWidth as XtArgVal - (*pCD).baseWidth as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*pCD).clientHeight as XtArgVal - (*pCD).baseHeight as XtArgVal;
    i += 1;
    XtSetValues(
        (*pIBD).bBoardWidget,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value =
        ((*pCD).clientWidth - 2 as ::core::ffi::c_int * frameShadowThickness) as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value =
        ((*pCD).clientHeight - 2 as ::core::ffi::c_int * frameShadowThickness) as XtArgVal;
    i += 1;
    XtSetValues(
        (*pIBD).scrolledWidget,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
    (*pCD).xBorderWidth = 0 as ::core::ffi::c_int;
    SetFrameInfo(pCD);
    mask = XParseGeometry(
        (*pWS).iconBoxGeometry as *const ::core::ffi::c_char,
        &raw mut X,
        &raw mut Y,
        &raw mut width,
        &raw mut height,
    );
    if mask & XValue != 0 {
        if mask & XNegative != 0 {
            (*pCD).clientX = X
                + (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*(*pCD).pSD).screen as isize))
                .width
                - (*pCD).clientWidth
                - (*pCD).clientOffset.x as ::core::ffi::c_int;
        } else {
            (*pCD).clientX = X + (*pCD).clientOffset.x as ::core::ffi::c_int;
        }
    } else {
        (*pCD).clientX = (*pCD).clientOffset.x as ::core::ffi::c_int;
    }
    if mask & YValue != 0 {
        if mask & YNegative != 0 {
            (*pCD).clientY = Y
                + (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*(*pCD).pSD).screen as isize))
                .height
                - (*pCD).clientHeight
                - (*pCD).clientOffset.x as ::core::ffi::c_int;
        } else {
            (*pCD).clientY = Y + (*pCD).clientOffset.y as ::core::ffi::c_int;
        }
    } else {
        (*pCD).clientY = (*pCD).clientOffset.x as ::core::ffi::c_int
            + (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(*pCD).pSD).screen as isize))
            .height
            - (*pCD).clientHeight;
    }
    PlaceFrameOnScreen(
        pCD,
        &raw mut (*pCD).clientX,
        &raw mut (*pCD).clientY,
        (*pCD).clientWidth,
        (*pCD).clientHeight,
    );
    (*pCD).clientX -= (*pCD).clientOffset.x as ::core::ffi::c_int;
    (*pCD).clientY -= (*pCD).clientOffset.y as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(885 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*pCD).clientX as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(887 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*pCD).clientY as XtArgVal;
    i += 1;
    XtSetValues(
        (*pIBD).shellWidget,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
    (*pCD).maxX = (*pCD).clientX;
    (*pCD).maxY = (*pCD).clientY;
}
#[no_mangle]
pub unsafe extern "C" fn SetGranularity(
    mut pWS: *mut WmWorkspaceData,
    mut pCD: *mut ClientData,
    mut pIBD: *mut IconBoxData,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut hScrollBarHeight = 0 as Dimension;
    let mut hBarHeight = 0 as Dimension;
    let mut vScrollBarWidth = 0 as Dimension;
    let mut vBarWidth = 0 as Dimension;
    let mut spacing: Dimension = 0;
    let mut shadowThickness: ::core::ffi::c_short = 0;
    let mut marginWidth: ::core::ffi::c_short = 0;
    let mut marginHeight: ::core::ffi::c_short = 0;
    let mut hShighlightThickness: ::core::ffi::c_short = 0;
    let mut vShighlightThickness: ::core::ffi::c_short = 0;
    let mut setArgs: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 10];
    let mut getArgs: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 10];
    i = 0 as ::core::ffi::c_int;
    getArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9900 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut spacing as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9654 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut shadowThickness as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9421 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut marginWidth as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9394 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut marginHeight as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10649 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut (*pIBD).vScrollBar as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7186 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut (*pIBD).hScrollBar as XtArgVal;
    i += 1;
    XtGetValues(
        (*pIBD).scrolledWidget,
        &raw mut getArgs as ArgList,
        i as Cardinal,
    );
    if strcmp(
        (*(*pWS).pSD).iconBoxSBDisplayPolicy as *const ::core::ffi::c_char,
        szvertical,
    ) != 0
    {
        i = 0 as ::core::ffi::c_int;
        getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(234 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        getArgs[i as usize].value = &raw mut hBarHeight as XtArgVal;
        i += 1;
        getArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(7078 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        getArgs[i as usize].value = &raw mut hShighlightThickness as XtArgVal;
        i += 1;
        XtGetValues(
            (*pIBD).hScrollBar,
            &raw mut getArgs as ArgList,
            i as Cardinal,
        );
        i = 0 as ::core::ffi::c_int;
        setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(7258 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = (*pCD).widthInc as XtArgVal;
        i += 1;
        setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(7078 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = 3 as ::core::ffi::c_int as XtArgVal;
        i += 1;
        setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(234 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = (hBarHeight as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int * hShighlightThickness as ::core::ffi::c_int)
            as XtArgVal
            + (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as XtArgVal;
        i += 1;
        XtSetValues(
            (*pIBD).hScrollBar,
            &raw mut setArgs as *mut Arg as ArgList,
            i as Cardinal,
        );
        i = 0 as ::core::ffi::c_int;
        getArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(10461 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        getArgs[i as usize].value = &raw mut select_color as XtArgVal;
        i += 1;
        getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(234 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        getArgs[i as usize].value = &raw mut hScrollBarHeight as XtArgVal;
        i += 1;
        XtGetValues(
            (*pIBD).hScrollBar,
            &raw mut getArgs as ArgList,
            i as Cardinal,
        );
    }
    if strcmp(
        (*(*pWS).pSD).iconBoxSBDisplayPolicy as *const ::core::ffi::c_char,
        szhorizontal,
    ) != 0
    {
        i = 0 as ::core::ffi::c_int;
        getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(872 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        getArgs[i as usize].value = &raw mut vBarWidth as XtArgVal;
        i += 1;
        getArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(7078 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        getArgs[i as usize].value = &raw mut vShighlightThickness as XtArgVal;
        i += 1;
        XtGetValues(
            (*pIBD).vScrollBar,
            &raw mut getArgs as ArgList,
            i as Cardinal,
        );
        i = 0 as ::core::ffi::c_int;
        setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(7258 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = (*pCD).heightInc as XtArgVal;
        i += 1;
        setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(7078 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = 3 as ::core::ffi::c_int as XtArgVal;
        i += 1;
        setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(872 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = (vBarWidth as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int * vShighlightThickness as ::core::ffi::c_int)
            as XtArgVal
            + (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as XtArgVal;
        i += 1;
        XtSetValues(
            (*pIBD).vScrollBar,
            &raw mut setArgs as *mut Arg as ArgList,
            i as Cardinal,
        );
        i = 0 as ::core::ffi::c_int;
        getArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(10461 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        getArgs[i as usize].value = &raw mut select_color as XtArgVal;
        i += 1;
        getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(872 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        getArgs[i as usize].value = &raw mut vScrollBarWidth as XtArgVal;
        i += 1;
        XtGetValues(
            (*pIBD).vScrollBar,
            &raw mut getArgs as ArgList,
            i as Cardinal,
        );
    }
    if strcmp(
        (*(*pWS).pSD).iconBoxSBDisplayPolicy as *const ::core::ffi::c_char,
        szvertical,
    ) == 0
    {
        XtUnmanageChild((*pIBD).hScrollBar);
        hScrollBarHeight = 0 as Dimension;
        i = 0 as ::core::ffi::c_int;
        setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(9291 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = XmAS_NEEDED as ::core::ffi::c_int as XtArgVal;
        i += 1;
        XtSetValues(
            (*pIBD).scrolledWidget,
            &raw mut setArgs as *mut Arg as ArgList,
            i as Cardinal,
        );
    } else if strcmp(
        (*(*pWS).pSD).iconBoxSBDisplayPolicy as *const ::core::ffi::c_char,
        szhorizontal,
    ) == 0
    {
        XtUnmanageChild((*pIBD).vScrollBar);
        vScrollBarWidth = 0 as Dimension;
        i = 0 as ::core::ffi::c_int;
        setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(9291 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = XmAS_NEEDED as ::core::ffi::c_int as XtArgVal;
        i += 1;
        XtSetValues(
            (*pIBD).scrolledWidget,
            &raw mut setArgs as *mut Arg as ArgList,
            i as Cardinal,
        );
    }
    (*pCD).baseWidth = IB_SPACING
        + 2 as ::core::ffi::c_int * IB_HIGHLIGHT_BORDER
        + vScrollBarWidth as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int * frameShadowThickness
        + 2 as ::core::ffi::c_int * marginWidth as ::core::ffi::c_int
        + (if marginWidth as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            2 as ::core::ffi::c_int * shadowThickness as ::core::ffi::c_int
        } else {
            shadowThickness as ::core::ffi::c_int
        });
    (*pCD).baseHeight = IB_SPACING
        + 2 as ::core::ffi::c_int * IB_HIGHLIGHT_BORDER
        + spacing as ::core::ffi::c_int
        + hScrollBarHeight as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int * frameShadowThickness
        + 2 as ::core::ffi::c_int * marginHeight as ::core::ffi::c_int
        + (if marginHeight as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            2 as ::core::ffi::c_int * shadowThickness as ::core::ffi::c_int
        } else {
            shadowThickness as ::core::ffi::c_int
        });
    (*pCD).minWidth = (*pCD).baseWidth + (*pCD).widthInc;
    (*pCD).minHeight = (*pCD).baseHeight + (*pCD).heightInc;
    (*pCD).maxWidth = (*pCD).minWidth;
    (*pCD).oldMaxWidth = (*pCD).maxWidth;
    (*pCD).maxHeight = (*pCD).minHeight;
    (*pCD).oldMaxHeight = (*pCD).maxHeight;
}
#[no_mangle]
pub unsafe extern "C" fn GetIconBoxMenuItems(mut pSD: *mut WmScreenData) -> *mut MenuItem {
    return ParseMwmMenuStr(
        pSD,
        b"\"Pack Icons\" _P  Alt Shift<Key>F7 f.pack_icons\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MapIconBoxes(mut pWS: *mut WmWorkspaceData) {
    let mut pibd = ::core::ptr::null_mut::<IconBoxData>();
    if !(*pWS).pIconBox.is_null() {
        pibd = (*pWS).pIconBox;
        while !pibd.is_null() {
            XtPopup((*pibd).shellWidget, XtGrabNone);
            pibd = (*pibd).pNextIconBox as *mut IconBoxData;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn UnmapIconBoxes(mut pWS: *mut WmWorkspaceData) {
    let mut pibd = ::core::ptr::null_mut::<IconBoxData>();
    if !(*pWS).pIconBox.is_null() {
        pibd = (*pWS).pIconBox;
        while !pibd.is_null() {
            XUnmapWindow(wmGD.display, (*(*pibd).pCD_iconBox).clientFrameWin);
            pibd = (*pibd).pNextIconBox as *mut IconBoxData;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn InsertIconIntoBox(
    mut pIBD: *mut IconBoxData,
    mut pCD: *mut ClientData,
) -> Boolean {
    let mut rval = False as Boolean;
    let mut setArgs: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut i: ::core::ffi::c_int = 0;
    let mut iconWidth: ::core::ffi::c_int = 0;
    let mut iconHeight: ::core::ffi::c_int = 0;
    let mut tmpPointerToIconBox = ::core::ptr::null_mut::<IconBoxData>();
    let mut iconWidget = ::core::ptr::null_mut::<_WidgetRec>();
    let mut pIconInfo = ::core::ptr::null_mut::<IconInfo>();
    static mut dummyString: XmString = ::core::ptr::null::<__XmStringRec>() as *mut __XmStringRec;
    let mut pWsc = ::core::ptr::null_mut::<WsClientData>();
    let mut pWS = GetWorkspaceData((*pCD).pSD, (*pIBD).wsID);
    pWsc = GetWsClientData(pWS, pCD);
    tmpPointerToIconBox = pIBD;
    if (*pCD).client != 0 {
        (*pWsc).pIconBox = tmpPointerToIconBox;
        iconWidth = (*(*pCD).pSD).iconWidth + 2 as ::core::ffi::c_int * IB_MARGIN_WIDTH;
        iconHeight = (*(*pCD).pSD).iconHeight + 2 as ::core::ffi::c_int * IB_MARGIN_HEIGHT;
        pIconInfo = InsertIconInfo((*pWsc).pIconBox, pCD, NULL as Widget);
        if !pIconInfo.is_null() {
            (*(*pWsc).pIconBox).numberOfIcons += 1;
            i = 0 as ::core::ffi::c_int;
            setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(52 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = (*(*pCD).pSD).iconAppearance.background as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(214 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = (*(*pCD).pSD).iconAppearance.foreground as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(885 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = (*pWsc).iconX as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(887 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = (*pWsc).iconY as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(872 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = iconWidth as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(234 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = iconHeight as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(124 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(7078 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = 3 as ::core::ffi::c_int as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(7909 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(7955 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(7547 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = XmSTRING as ::core::ffi::c_int as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(8998 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(10395 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(8913 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
            i += 1;
            setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(9654 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
            i += 1;
            iconWidget = XtCreateManagedWidget(
                b"iconInIconBox\0" as *const u8 as *const ::core::ffi::c_char,
                xmDrawnButtonWidgetClass,
                (*(*pWsc).pIconBox).bBoardWidget,
                &raw mut setArgs as *mut Arg as ArgList,
                i as Cardinal,
            );
            if dummyString.is_null() {
                dummyString = XmStringCreateLocalized(
                    b"\0" as *const u8 as *const ::core::ffi::c_char as String_0,
                );
            }
            i = 0 as ::core::ffi::c_int;
            setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(7535 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = dummyString as XtArgVal;
            i += 1;
            XtSetValues(iconWidget, &raw mut setArgs as ArgList, i as Cardinal);
            (*pIconInfo).theWidget = iconWidget;
            (*pWsc).iconFrameWin = XtWindowOfObject(iconWidget);
            XtAddCallback(
                iconWidget,
                (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                    .offset(4700 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(Widget, XtPointer, *mut XmAnyCallbackStruct) -> ()>,
                    XtCallbackProc,
                >(Some(
                    IconActivateCallback
                        as unsafe extern "C" fn(Widget, XtPointer, *mut XmAnyCallbackStruct) -> (),
                )),
                NULL,
            );
            XtAddEventHandler(
                iconWidget,
                SELECT_BUTTON_MOTION_MASK as EventMask,
                False as Boolean,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(Widget, XtPointer, *mut XEvent) -> ()>,
                    XtEventHandler,
                >(Some(
                    HandleIconBoxButtonMotion
                        as unsafe extern "C" fn(Widget, XtPointer, *mut XEvent) -> (),
                )),
                NULL,
            );
            XtAddEventHandler(
                iconWidget,
                DMANIP_BUTTON_MOTION_MASK as EventMask,
                False as Boolean,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(Widget, XtPointer, *mut XEvent) -> ()>,
                    XtEventHandler,
                >(Some(
                    HandleIconBoxButtonMotion
                        as unsafe extern "C" fn(Widget, XtPointer, *mut XEvent) -> (),
                )),
                NULL,
            );
            XtAddEventHandler(
                iconWidget,
                KeyPressMask as EventMask,
                False as Boolean,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(Widget, XtPointer, *mut XKeyEvent) -> ()>,
                    XtEventHandler,
                >(Some(
                    HandleIconBoxIconKeyPress
                        as unsafe extern "C" fn(Widget, XtPointer, *mut XKeyEvent) -> (),
                )),
                NULL,
            );
            if (*(*pCD).pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART != 0 {
                XtAddEventHandler(
                    iconWidget,
                    FocusChangeMask as EventMask,
                    False as Boolean,
                    ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(Widget, XtPointer, *mut XFocusChangeEvent) -> (),
                        >,
                        XtEventHandler,
                    >(Some(
                        ChangeActiveIconboxIconText
                            as unsafe extern "C" fn(
                                Widget,
                                XtPointer,
                                *mut XFocusChangeEvent,
                            ) -> (),
                    )),
                    NULL,
                );
                if (*(*pCD).pSD).activeLabelParent != (*(*pCD).pSD).rootWindow {
                    XRaiseWindow(wmGD.display, (*(*pCD).pSD).activeIconTextWin);
                }
            }
            ResetIconBoxMaxSize(
                (*(*pWsc).pIconBox).pCD_iconBox as *mut ClientData,
                (*(*pWsc).pIconBox).bBoardWidget,
            );
            ResetArrowButtonIncrements((*(*pWsc).pIconBox).pCD_iconBox as *mut ClientData);
            rval = True as Boolean;
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn InsertIconInfo(
    mut pIBD: *mut IconBoxData,
    mut pCD: *mut ClientData,
    mut theWidget: Widget,
) -> *mut IconInfo {
    let mut pII = ::core::ptr::null_mut::<IconInfo>();
    let mut place: ::core::ffi::c_int = 0;
    let mut amt: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut setArgs: [Arg; 3] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 3];
    let mut getArgs: [Arg; 4] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 4];
    let mut clipWidth: Dimension = 0;
    let mut clipHeight: Dimension = 0;
    let mut pWsc = ::core::ptr::null_mut::<WsClientData>();
    place = GetNextIconPlace(&raw mut (*pIBD).IPD);
    if place == NO_ICON_PLACE {
        if (*pIBD).IPD.iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_PRIMARY != 0 {
            amt = (*pIBD).IPD.placementCols;
        } else {
            amt = (*pIBD).IPD.placementRows;
        }
        if ExtendIconList(pIBD, amt) == 0 {
            Warning(
                b"Insufficient memory to create icon box data\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<IconInfo>();
        }
        if (*pIBD).IPD.iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_PRIMARY != 0 {
            (*pIBD).IPD.placementRows += 1;
        } else {
            (*pIBD).IPD.placementCols += 1;
        }
        place = GetNextIconPlace(&raw mut (*pIBD).IPD);
    }
    insertPosition = place as Cardinal;
    pII = (*pIBD).IPD.placeList.offset(place as isize) as *mut IconInfo;
    (*pII).theWidget = theWidget;
    (*pII).pCD = pCD as *mut _ClientData;
    pWsc = GetWsClientData(GetWorkspaceData((*pCD).pSD, (*pIBD).wsID), pCD);
    (*pWsc).iconPlace = place;
    CvtIconPlaceToPosition(
        &raw mut (*pIBD).IPD,
        (*pWsc).iconPlace,
        &raw mut (*pWsc).iconX,
        &raw mut (*pWsc).iconY,
    );
    (*pIBD).currentCol = (*pWsc).iconX / (*(*pIBD).pCD_iconBox).widthInc;
    (*pIBD).currentRow = (*pWsc).iconY / (*(*pIBD).pCD_iconBox).heightInc;
    i = 0 as ::core::ffi::c_int;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut clipWidth as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut clipHeight as XtArgVal;
    i += 1;
    XtGetValues(
        (*pIBD).clipWidget,
        &raw mut getArgs as ArgList,
        i as Cardinal,
    );
    i = 0 as ::core::ffi::c_int;
    if (*pIBD).IPD.iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_PRIMARY != 0 {
        if (*pIBD).currentCol > (*pIBD).lastCol {
            (*pIBD).lastCol = (*pIBD).currentCol;
        }
        if (*pIBD).currentRow > (*pIBD).lastRow {
            (*pIBD).lastRow = (*pIBD).currentRow;
            if clipHeight as ::core::ffi::c_int
                <= ((*pWsc).iconY + (*(*pIBD).pCD_iconBox).heightInc) as Dimension
                    as ::core::ffi::c_int
            {
                setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(234 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char
                    as String_0;
                setArgs[i as usize].value =
                    (*pWsc).iconY as XtArgVal + (*(*pIBD).pCD_iconBox).heightInc as XtArgVal;
                i += 1;
            }
        }
    } else {
        if (*pIBD).currentCol > (*pIBD).lastCol {
            (*pIBD).lastCol = (*pIBD).currentCol;
            if clipWidth as ::core::ffi::c_int
                <= ((*pWsc).iconX + (*(*pIBD).pCD_iconBox).widthInc) as Dimension
                    as ::core::ffi::c_int
            {
                setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(872 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char
                    as String_0;
                setArgs[i as usize].value =
                    (*pWsc).iconX as XtArgVal + (*(*pIBD).pCD_iconBox).widthInc as XtArgVal;
                i += 1;
            }
        }
        if (*pIBD).currentRow > (*pIBD).lastRow {
            (*pIBD).lastRow = (*pIBD).currentRow;
        }
    }
    if i > 0 as ::core::ffi::c_int {
        XtSetValues(
            (*pIBD).bBoardWidget,
            &raw mut setArgs as ArgList,
            i as Cardinal,
        );
    }
    return pII;
}
#[no_mangle]
pub unsafe extern "C" fn DeleteIconFromBox(mut pIBD: *mut IconBoxData, mut pCD: *mut ClientData) {
    let mut theChild = ::core::ptr::null_mut::<_WidgetRec>();
    let mut pCD_tmp = ::core::ptr::null_mut::<ClientData>();
    let mut args: [Arg; 4] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 4];
    let mut clipWidth: Dimension = 0;
    let mut clipHeight: Dimension = 0;
    let mut oldWidth: Dimension = 0;
    let mut oldHeight: Dimension = 0;
    let mut newWidth: ::core::ffi::c_int = 0;
    let mut newHeight: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut newCols: ::core::ffi::c_int = 0;
    let mut newRows: ::core::ffi::c_int = 0;
    let mut pWS = GetWorkspaceData((*pCD).pSD, (*pIBD).wsID);
    let mut pWsc = ::core::ptr::null_mut::<WsClientData>();
    pWsc = GetWsClientData(pWS, pCD);
    i = 0 as ::core::ffi::c_int;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = &raw mut oldWidth as XtArgVal;
    i += 1;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = &raw mut oldHeight as XtArgVal;
    i += 1;
    XtGetValues(
        (*pIBD).bBoardWidget,
        &raw mut args as ArgList,
        i as Cardinal,
    );
    i = 0 as ::core::ffi::c_int;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = &raw mut clipWidth as XtArgVal;
    i += 1;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = &raw mut clipHeight as XtArgVal;
    i += 1;
    XtGetValues((*pIBD).clipWidget, &raw mut args as ArgList, i as Cardinal);
    clipHeight = (clipHeight as ::core::ffi::c_int
        / (*(*pIBD).pCD_iconBox).heightInc as Dimension as ::core::ffi::c_int)
        as Dimension;
    clipWidth = (clipWidth as ::core::ffi::c_int
        / (*(*pIBD).pCD_iconBox).widthInc as Dimension as ::core::ffi::c_int)
        as Dimension;
    if XFindContext(
        wmGD.display,
        (*(*pCD).pSD).activeIconTextWin as XID,
        wmGD.windowContextType,
        &raw mut pCD_tmp as *mut XPointer,
    ) == 0
    {
        if pCD == pCD_tmp {
            HideActiveIconText(NULL as *mut WmScreenData);
        }
    }
    DeleteIconInfo((*pWsc).pIconBox, pCD);
    (*(*pWsc).pIconBox).numberOfIcons -= 1;
    theChild = XtWindowToWidget(wmGD.display, (*pWsc).iconFrameWin);
    (*pWsc).pIconBox = ::core::ptr::null_mut::<IconBoxData>();
    (*pWsc).iconPlace = NO_ICON_PLACE;
    XtUnmanageChild(theChild);
    XtDestroyWidget(theChild);
    SetNewBounds(pIBD);
    i = 0 as ::core::ffi::c_int;
    if clipWidth as ::core::ffi::c_int
        <= ((*pIBD).lastCol + 1 as ::core::ffi::c_int) as Dimension as ::core::ffi::c_int
    {
        newWidth = ((*pIBD).lastCol + 1 as ::core::ffi::c_int) * (*(*pIBD).pCD_iconBox).widthInc;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(872 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = newWidth as XtArgVal;
        i += 1;
        newCols = newWidth / (*(*pIBD).pCD_iconBox).widthInc;
    } else {
        newWidth = clipWidth as ::core::ffi::c_int * (*(*pIBD).pCD_iconBox).widthInc;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(872 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = newWidth as XtArgVal;
        i += 1;
        newCols = newWidth / (*(*pIBD).pCD_iconBox).widthInc;
    }
    if clipHeight as ::core::ffi::c_int
        <= ((*pIBD).lastRow + 1 as ::core::ffi::c_int) as Dimension as ::core::ffi::c_int
    {
        newHeight = ((*pIBD).lastRow + 1 as ::core::ffi::c_int) * (*(*pIBD).pCD_iconBox).heightInc;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(234 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = newHeight as XtArgVal;
        i += 1;
        newRows = newHeight / (*(*pIBD).pCD_iconBox).heightInc;
    } else {
        newHeight = clipHeight as ::core::ffi::c_int * (*(*pIBD).pCD_iconBox).heightInc;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(234 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = newHeight as XtArgVal;
        i += 1;
        newRows = newHeight / (*(*pIBD).pCD_iconBox).heightInc;
    }
    if i > 0 as ::core::ffi::c_int
        && ExpandVirtualSpace(pIBD, newWidth, newHeight) as ::core::ffi::c_int != 0
    {
        XtSetValues(
            (*pIBD).bBoardWidget,
            &raw mut args as ArgList,
            i as Cardinal,
        );
        RealignIconList(pIBD, newCols, newRows);
        (*pIBD).IPD.placementCols = newCols;
        (*pIBD).IPD.placementRows = newRows;
    }
    ResetIconBoxMaxSize((*pIBD).pCD_iconBox as *mut ClientData, (*pIBD).bBoardWidget);
    ResetArrowButtonIncrements((*pIBD).pCD_iconBox as *mut ClientData);
}
#[no_mangle]
pub unsafe extern "C" fn DeleteIconInfo(mut pIBD: *mut IconBoxData, mut pCD: *mut ClientData) {
    let mut ix: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut pII = ::core::ptr::null_mut::<IconInfo>();
    pII = (*pIBD)
        .IPD
        .placeList
        .offset(0 as ::core::ffi::c_int as isize) as *mut IconInfo;
    count = (*pIBD).IPD.totalPlaces;
    ix = 0 as ::core::ffi::c_int;
    while ix < count && (*pII).pCD != pCD {
        ix += 1;
        pII = pII.offset(1);
    }
    if ix < count {
        (*pII).theWidget = ::core::ptr::null_mut::<_WidgetRec>();
        (*pII).pCD = ::core::ptr::null_mut::<_ClientData>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn ResetIconBoxMaxSize(mut pCD: *mut ClientData, mut bBoardWidget: Widget) {
    let mut i: ::core::ffi::c_int = 0;
    let mut getArgs: [Arg; 3] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 3];
    let mut newWidth: Dimension = 0;
    let mut newHeight: Dimension = 0;
    i = 0 as ::core::ffi::c_int;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut newWidth as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut newHeight as XtArgVal;
    i += 1;
    XtGetValues(bBoardWidget, &raw mut getArgs as ArgList, i as Cardinal);
    (*pCD).maxWidth = newWidth as ::core::ffi::c_int + (*pCD).baseWidth;
    (*pCD).oldMaxWidth = (*pCD).maxWidth;
    (*pCD).maxHeight = newHeight as ::core::ffi::c_int + (*pCD).baseHeight;
    (*pCD).oldMaxHeight = (*pCD).maxHeight;
    (*pCD).maxX = (*pCD).clientX;
    (*pCD).maxY = (*pCD).clientY;
    PlaceFrameOnScreen(
        pCD,
        &raw mut (*pCD).maxX,
        &raw mut (*pCD).maxY,
        (*pCD).maxWidth,
        (*pCD).maxHeight,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CheckIconBoxSize(mut pIBD: *mut IconBoxData) -> Boolean {
    let mut i: ::core::ffi::c_int = 0;
    let mut getArgs: [Arg; 3] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 3];
    let mut setArgs: [Arg; 3] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 3];
    let mut oldWidth: Dimension = 0;
    let mut oldHeight: Dimension = 0;
    let mut newWidth: Dimension = 0;
    let mut newHeight: Dimension = 0;
    let mut oldCol: ::core::ffi::c_int = 0;
    let mut oldRow: ::core::ffi::c_int = 0;
    let mut rval = True as Boolean;
    i = 0 as ::core::ffi::c_int;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut oldWidth as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut oldHeight as XtArgVal;
    i += 1;
    XtGetValues(
        (*pIBD).bBoardWidget,
        &raw mut getArgs as ArgList,
        i as Cardinal,
    );
    newWidth = oldWidth;
    newHeight = oldHeight;
    oldCol = oldWidth as ::core::ffi::c_int
        / (*(*pIBD).pCD_iconBox).widthInc as Dimension as ::core::ffi::c_int;
    oldRow = oldHeight as ::core::ffi::c_int
        / (*(*pIBD).pCD_iconBox).heightInc as Dimension as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    if (*pIBD).IPD.iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_PRIMARY != 0 {
        if oldRow < (*pIBD).lastRow + 1 as ::core::ffi::c_int {
            newHeight = ((*pIBD).lastRow * (*(*pIBD).pCD_iconBox).heightInc
                + (*(*pIBD).pCD_iconBox).heightInc) as Dimension;
            setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(234 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = newHeight as XtArgVal;
            i += 1;
        }
    } else if oldCol < (*pIBD).lastCol + 1 as ::core::ffi::c_int {
        newWidth = ((*pIBD).lastCol * (*(*pIBD).pCD_iconBox).widthInc
            + (*(*pIBD).pCD_iconBox).widthInc) as Dimension;
        setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(872 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = newWidth as XtArgVal;
        i += 1;
    }
    if i > 0 as ::core::ffi::c_int {
        if ExpandVirtualSpace(
            pIBD,
            newWidth as ::core::ffi::c_int,
            newHeight as ::core::ffi::c_int,
        ) == 0
        {
            rval = False as Boolean;
            return rval;
        }
        XtSetValues(
            (*pIBD).bBoardWidget,
            &raw mut setArgs as ArgList,
            i as Cardinal,
        );
    }
    ResetIconBoxMaxSize((*pIBD).pCD_iconBox as *mut ClientData, (*pIBD).bBoardWidget);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn CheckIconBoxResize(
    mut pCD: *mut ClientData,
    mut changedValues: ::core::ffi::c_uint,
    mut newWidth: ::core::ffi::c_int,
    mut newHeight: ::core::ffi::c_int,
) {
    let mut packVert = False as Boolean;
    let mut packHorz = False as Boolean;
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    let mut pIBD = ::core::ptr::null_mut::<IconBoxData>();
    let mut pIPD = ::core::ptr::null_mut::<IconPlacementData>();
    let mut i: ::core::ffi::c_int = 0;
    let mut newCols: ::core::ffi::c_int = 0;
    let mut newRows: ::core::ffi::c_int = 0;
    let mut getArgs: [Arg; 3] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 3];
    let mut setArgs: [Arg; 3] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 3];
    let mut oldWidth: Dimension = 0;
    let mut oldHeight: Dimension = 0;
    pIPD = &raw mut (*(*pCD).thisIconBox).IPD as *mut IconPlacementData;
    pIBD = (*pCD).thisIconBox;
    pSD = wmGD.Screens.offset((*(*pCD).pSD).screen as isize) as *mut WmScreenData;
    i = 0 as ::core::ffi::c_int;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut oldWidth as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut oldHeight as XtArgVal;
    i += 1;
    XtGetValues(
        (*pIBD).bBoardWidget,
        &raw mut getArgs as ArgList,
        i as Cardinal,
    );
    newCols = (*pIPD).placementCols;
    newRows = (*pIPD).placementRows;
    newWidth = newWidth - (*pCD).baseWidth;
    newHeight = newHeight - (*pCD).baseHeight;
    i = 0 as ::core::ffi::c_int;
    if changedValues & CWWidth as ::core::ffi::c_uint != 0 {
        if newWidth > oldWidth as ::core::ffi::c_int {
            newCols = newWidth / (*pCD).widthInc;
            setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(872 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = newWidth as XtArgVal;
            i += 1;
        }
        if newWidth < oldWidth as ::core::ffi::c_int {
            if strcmp(
                (*pSD).iconBoxSBDisplayPolicy as *const ::core::ffi::c_char,
                szvertical,
            ) == 0
                && newWidth / (*pCD).widthInc < (*pIBD).lastCol + 1 as ::core::ffi::c_int
            {
                setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(872 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char
                    as String_0;
                setArgs[i as usize].value = newWidth as XtArgVal;
                i += 1;
                newCols = newWidth / (*pCD).widthInc;
                packVert = True as Boolean;
            } else if newWidth / (*pCD).widthInc < (*pIBD).lastCol + 1 as ::core::ffi::c_int {
                newWidth = ((*pIBD).lastCol + 1 as ::core::ffi::c_int) * (*pCD).widthInc;
                setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(872 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char
                    as String_0;
                setArgs[i as usize].value = newWidth as XtArgVal;
                i += 1;
            } else {
                newCols = newWidth / (*pCD).widthInc;
                setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(872 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char
                    as String_0;
                setArgs[i as usize].value = newWidth as XtArgVal;
                i += 1;
            }
        }
    } else {
        newWidth = oldWidth as ::core::ffi::c_int;
    }
    if changedValues & CWHeight as ::core::ffi::c_uint != 0 {
        if newHeight > oldHeight as ::core::ffi::c_int {
            newRows = newHeight / (*pCD).heightInc;
            setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(234 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            setArgs[i as usize].value = newHeight as XtArgVal;
            i += 1;
        }
        if newHeight < oldHeight as ::core::ffi::c_int {
            if strcmp(
                (*pSD).iconBoxSBDisplayPolicy as *const ::core::ffi::c_char,
                szhorizontal,
            ) == 0
                && newHeight / (*pCD).heightInc < (*pIBD).lastRow + 1 as ::core::ffi::c_int
            {
                setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(234 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char
                    as String_0;
                setArgs[i as usize].value = newHeight as XtArgVal;
                i += 1;
                newRows = newHeight / (*pCD).heightInc;
                packHorz = True as Boolean;
            } else if newHeight / (*pCD).heightInc < (*pIBD).lastRow + 1 as ::core::ffi::c_int {
                newHeight = ((*pIBD).lastRow + 1 as ::core::ffi::c_int) * (*pCD).heightInc;
                setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(234 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char
                    as String_0;
                setArgs[i as usize].value = newHeight as XtArgVal;
                i += 1;
            } else {
                newRows = newHeight / (*pCD).heightInc;
                setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(234 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char
                    as String_0;
                setArgs[i as usize].value = newHeight as XtArgVal;
                i += 1;
            }
        }
    } else {
        newHeight = oldHeight as ::core::ffi::c_int;
    }
    if i > 0 as ::core::ffi::c_int
        && ExpandVirtualSpace(pIBD, newWidth, newHeight) as ::core::ffi::c_int != 0
    {
        XtSetValues(
            (*pIBD).bBoardWidget,
            &raw mut setArgs as ArgList,
            i as Cardinal,
        );
    }
    RealignIconList(pIBD, newCols, newRows);
    (*pIPD).placementCols = newCols;
    (*pIPD).placementRows = newRows;
    ResetIconBoxMaxSize(pCD, (*pIBD).bBoardWidget);
    if packVert != 0 {
        PackIconBox(
            pIBD,
            packVert,
            False as Boolean,
            newWidth,
            0 as ::core::ffi::c_int,
        );
    } else if packHorz != 0 {
        PackIconBox(
            pIBD,
            False as Boolean,
            packHorz,
            0 as ::core::ffi::c_int,
            newHeight,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExpandVirtualSpace(
    mut pIBD: *mut IconBoxData,
    mut newWidth: ::core::ffi::c_int,
    mut newHeight: ::core::ffi::c_int,
) -> Boolean {
    let mut rval = True as Boolean;
    let mut newSize: ::core::ffi::c_int = 0;
    let mut increment: ::core::ffi::c_int = 0;
    newSize =
        newWidth / (*(*pIBD).pCD_iconBox).widthInc * (newHeight / (*(*pIBD).pCD_iconBox).heightInc);
    if newSize > (*pIBD).IPD.totalPlaces {
        increment = newSize - (*pIBD).IPD.totalPlaces;
        rval = ExtendIconList(pIBD, increment);
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn ExtendIconList(
    mut pIBD: *mut IconBoxData,
    mut incr: ::core::ffi::c_int,
) -> Boolean {
    let mut rval: Boolean = 0;
    let mut newSize: ::core::ffi::c_int = 0;
    let mut pTmp = ::core::ptr::null_mut::<IconInfo>();
    newSize = (*pIBD).IPD.totalPlaces + incr;
    pTmp = XtMalloc(
        (newSize as usize).wrapping_mul(::core::mem::size_of::<IconInfo>() as usize) as Cardinal,
    ) as *mut IconInfo;
    if !pTmp.is_null() {
        memcpy(
            pTmp as *mut ::core::ffi::c_void,
            (*pIBD).IPD.placeList as *const ::core::ffi::c_void,
            ((*pIBD).IPD.totalPlaces as size_t)
                .wrapping_mul(::core::mem::size_of::<IconInfo>() as size_t),
        );
        memset(
            pTmp.offset((*pIBD).IPD.totalPlaces as isize) as *mut IconInfo
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (incr as size_t).wrapping_mul(::core::mem::size_of::<IconInfo>() as size_t),
        );
        XtFree((*pIBD).IPD.placeList as *mut ::core::ffi::c_char);
        (*pIBD).IPD.placeList = pTmp;
        (*pIBD).IPD.totalPlaces = newSize;
        rval = True as Boolean;
    } else {
        rval = False as Boolean;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn PackIconBox(
    mut pIBD: *mut IconBoxData,
    mut packVert: Boolean,
    mut packHorz: Boolean,
    mut passedInWidth: ::core::ffi::c_int,
    mut passedInHeight: ::core::ffi::c_int,
) {
    let mut pII_2 = ::core::ptr::null_mut::<IconInfo>();
    let mut pII_1 = ::core::ptr::null_mut::<IconInfo>();
    let mut ix1: ::core::ffi::c_int = 0;
    let mut ix2: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut newX: ::core::ffi::c_int = 0;
    let mut newY: ::core::ffi::c_int = 0;
    let mut pCD_tmp = ::core::ptr::null_mut::<ClientData>();
    let mut pMyCD = ::core::ptr::null_mut::<ClientData>();
    let mut hasActiveText = 1 as ::core::ffi::c_int;
    let mut args: [Arg; 4] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 4];
    let mut majorDimension: Dimension = 0;
    let mut minorDimension: Dimension = 0;
    let mut oldWidth: Dimension = 0;
    let mut oldHeight: Dimension = 0;
    let mut newWidth: ::core::ffi::c_int = 0;
    let mut newHeight: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut rippling = False as Boolean;
    let mut pWsc = ::core::ptr::null_mut::<WsClientData>();
    let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
    i = 0 as ::core::ffi::c_int;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = &raw mut oldWidth as XtArgVal;
    i += 1;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = &raw mut oldHeight as XtArgVal;
    i += 1;
    XtGetValues(
        (*pIBD).bBoardWidget,
        &raw mut args as ArgList,
        i as Cardinal,
    );
    i = 0 as ::core::ffi::c_int;
    if (*pIBD).IPD.iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_PRIMARY != 0 {
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(872 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = &raw mut majorDimension as XtArgVal;
        i += 1;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(234 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = &raw mut minorDimension as XtArgVal;
        i += 1;
        XtGetValues((*pIBD).clipWidget, &raw mut args as ArgList, i as Cardinal);
        if packVert != 0 {
            majorDimension = passedInWidth as Dimension;
        }
        minorDimension = (minorDimension as ::core::ffi::c_int
            / (*(*pIBD).pCD_iconBox).heightInc as Dimension as ::core::ffi::c_int)
            as Dimension;
        majorDimension = (majorDimension as ::core::ffi::c_int
            / (*(*pIBD).pCD_iconBox).widthInc as Dimension as ::core::ffi::c_int)
            as Dimension;
        if majorDimension as ::core::ffi::c_int != (*pIBD).IPD.placementCols {
            (*pIBD).IPD.placementCols = majorDimension as ::core::ffi::c_int;
            if (*pIBD).IPD.placementCols == 0 as ::core::ffi::c_int {
                (*pIBD).IPD.placementCols += 1;
            }
        }
    } else {
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(234 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = &raw mut majorDimension as XtArgVal;
        i += 1;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(872 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = &raw mut minorDimension as XtArgVal;
        i += 1;
        XtGetValues((*pIBD).clipWidget, &raw mut args as ArgList, i as Cardinal);
        if packHorz != 0 {
            majorDimension = passedInHeight as Dimension;
        }
        minorDimension = (minorDimension as ::core::ffi::c_int
            / (*(*pIBD).pCD_iconBox).widthInc as Dimension as ::core::ffi::c_int)
            as Dimension;
        majorDimension = (majorDimension as ::core::ffi::c_int
            / (*(*pIBD).pCD_iconBox).heightInc as Dimension as ::core::ffi::c_int)
            as Dimension;
        if majorDimension as ::core::ffi::c_int != (*pIBD).IPD.placementRows {
            (*pIBD).IPD.placementRows = majorDimension as ::core::ffi::c_int;
            if (*pIBD).IPD.placementRows == 0 as ::core::ffi::c_int {
                (*pIBD).IPD.placementRows += 1;
            }
        }
    }
    pMyCD = (*pIBD).pCD_iconBox as *mut ClientData;
    if (*(*pMyCD).pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART != 0 {
        if XFindContext(
            wmGD.display,
            (*(*pMyCD).pSD).activeIconTextWin as XID,
            wmGD.windowContextType,
            &raw mut pCD_tmp as *mut XPointer,
        ) != 0
        {
            hasActiveText = 0 as ::core::ffi::c_int;
        }
    }
    pII_1 = (*pIBD).IPD.placeList;
    pII_2 = pII_1;
    ix2 = 0 as ::core::ffi::c_int;
    ix1 = ix2;
    count = (*pIBD).IPD.totalPlaces;
    while ix1 < count {
        if rippling == 0 && !(*pII_2).pCD.is_null() {
            pWS = GetWorkspaceData((*(*pII_2).pCD).pSD, (*pIBD).wsID);
            pWsc = GetWsClientData(pWS, (*pII_2).pCD as *mut ClientData);
            CvtIconPlaceToPosition(
                &raw mut (*pIBD).IPD,
                (*pWsc).iconPlace,
                &raw mut newX,
                &raw mut newY,
            );
            rippling =
                (newX != (*pWsc).iconX || newY != (*pWsc).iconY) as ::core::ffi::c_int as Boolean;
        }
        if (*pII_2).pCD.is_null() || rippling as ::core::ffi::c_int != 0 {
            while ix1 < count && (*pII_1).pCD.is_null() {
                ix1 += 1;
                pII_1 = pII_1.offset(1);
            }
            if ix1 < count && !(*pII_1).pCD.is_null() {
                if ix1 != ix2 {
                    MoveIconInfo(&raw mut (*pIBD).IPD, ix1, &raw mut (*pIBD).IPD, ix2);
                }
                CvtIconPlaceToPosition(&raw mut (*pIBD).IPD, ix2, &raw mut newX, &raw mut newY);
                pWS = GetWorkspaceData((*(*pII_2).pCD).pSD, (*pIBD).wsID);
                pWsc = GetWsClientData(pWS, (*pII_2).pCD as *mut ClientData);
                (*pWsc).iconX = newX;
                (*pWsc).iconY = newY;
                if hasActiveText != 0 && (*pII_2).pCD == pCD_tmp {
                    HideActiveIconText(NULL as *mut WmScreenData);
                    XtMoveWidget((*pII_2).theWidget, newX as Position, newY as Position);
                    ShowActiveIconText((*pII_2).pCD as *mut ClientData);
                } else {
                    XtMoveWidget((*pII_2).theWidget, newX as Position, newY as Position);
                }
            }
        }
        if ix1 < count {
            ix2 += 1;
            pII_2 = pII_2.offset(1);
        }
        ix1 += 1;
        pII_1 = pII_1.offset(1);
    }
    SetNewBounds(pIBD);
    i = 0 as ::core::ffi::c_int;
    if (*pIBD).IPD.iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_PRIMARY != 0 {
        if majorDimension as ::core::ffi::c_int
            <= ((*pIBD).lastCol + 1 as ::core::ffi::c_int) as Dimension as ::core::ffi::c_int
        {
            newWidth =
                ((*pIBD).lastCol + 1 as ::core::ffi::c_int) * (*(*pIBD).pCD_iconBox).widthInc;
            args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(872 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            args[i as usize].value = newWidth as XtArgVal;
            i += 1;
        } else {
            newWidth = oldWidth as ::core::ffi::c_int;
        }
        if minorDimension as ::core::ffi::c_int
            <= ((*pIBD).lastRow + 1 as ::core::ffi::c_int) as Dimension as ::core::ffi::c_int
        {
            newHeight =
                ((*pIBD).lastRow + 1 as ::core::ffi::c_int) * (*(*pIBD).pCD_iconBox).heightInc;
            args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(234 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            args[i as usize].value = newHeight as XtArgVal;
            i += 1;
        } else {
            newHeight = minorDimension as ::core::ffi::c_int * (*(*pIBD).pCD_iconBox).heightInc;
            args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(234 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            args[i as usize].value = newHeight as XtArgVal;
            i += 1;
        }
    } else {
        if majorDimension as ::core::ffi::c_int
            <= ((*pIBD).lastRow + 1 as ::core::ffi::c_int) as Dimension as ::core::ffi::c_int
        {
            newHeight =
                ((*pIBD).lastRow + 1 as ::core::ffi::c_int) * (*(*pIBD).pCD_iconBox).heightInc;
            args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(234 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            args[i as usize].value = newHeight as XtArgVal;
            i += 1;
        } else {
            newHeight = oldHeight as ::core::ffi::c_int;
        }
        if minorDimension as ::core::ffi::c_int
            <= ((*pIBD).lastCol + 1 as ::core::ffi::c_int) as Dimension as ::core::ffi::c_int
        {
            newWidth =
                ((*pIBD).lastCol + 1 as ::core::ffi::c_int) * (*(*pIBD).pCD_iconBox).widthInc;
            args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(872 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            args[i as usize].value = newWidth as XtArgVal;
            i += 1;
        } else {
            newWidth = minorDimension as ::core::ffi::c_int * (*(*pIBD).pCD_iconBox).widthInc;
            args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(872 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            args[i as usize].value = newWidth as XtArgVal;
            i += 1;
        }
    }
    if i > 0 as ::core::ffi::c_int
        && ExpandVirtualSpace(pIBD, newWidth, newHeight) as ::core::ffi::c_int != 0
    {
        XtSetValues(
            (*pIBD).bBoardWidget,
            &raw mut args as ArgList,
            i as Cardinal,
        );
    }
    ResetIconBoxMaxSize((*pIBD).pCD_iconBox as *mut ClientData, (*pIBD).bBoardWidget);
    ResetArrowButtonIncrements((*pIBD).pCD_iconBox as *mut ClientData);
}
#[no_mangle]
pub unsafe extern "C" fn RealignIconList(
    mut pIBD: *mut IconBoxData,
    mut newCols: ::core::ffi::c_int,
    mut newRows: ::core::ffi::c_int,
) {
    let mut c1: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    let mut ix1: ::core::ffi::c_int = 0;
    let mut ix2: ::core::ffi::c_int = 0;
    let mut oldRows: ::core::ffi::c_int = 0;
    let mut oldCols: ::core::ffi::c_int = 0;
    let mut ipdNew = _IconPlacementData {
        onRootWindow: 0,
        iconPlacement: 0,
        placementRows: 0,
        placementCols: 0,
        totalPlaces: 0,
        iPlaceW: 0,
        iPlaceH: 0,
        placeList: ::core::ptr::null_mut::<IconInfo>(),
        placeIconX: 0,
        placeIconY: 0,
        placementRowY: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        placementColX: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        xiOrgX: 0,
        xiOrgY: 0,
        xiScreen: 0,
    };
    let mut pII = ::core::ptr::null_mut::<IconInfo>();
    ipdNew.onRootWindow = (*pIBD).IPD.onRootWindow;
    ipdNew.iconPlacement = (*pIBD).IPD.iconPlacement;
    ipdNew.placementRows = newRows;
    ipdNew.placementCols = newCols;
    ipdNew.iPlaceW = (*pIBD).IPD.iPlaceW;
    ipdNew.iPlaceH = (*pIBD).IPD.iPlaceH;
    ipdNew.placeList = (*pIBD).IPD.placeList;
    ipdNew.totalPlaces = (*pIBD).IPD.totalPlaces;
    oldRows = (*pIBD).IPD.placementRows;
    oldCols = (*pIBD).IPD.placementCols;
    if oldRows < newRows
        && (*pIBD).IPD.iconPlacement as ::core::ffi::c_long & ICON_PLACE_TOP_PRIMARY != 0
        || oldCols < newCols
            && (*pIBD).IPD.iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_PRIMARY != 0
    {
        ix1 = (*pIBD).IPD.totalPlaces - 1 as ::core::ffi::c_int;
        pII = (*pIBD).IPD.placeList.offset(ix1 as isize) as *mut IconInfo;
        while ix1 >= 0 as ::core::ffi::c_int {
            if !(*pII).pCD.is_null() {
                CvtIconPlaceToPosition(&raw mut (*pIBD).IPD, ix1, &raw mut c1, &raw mut c2);
                ix2 = CvtIconPositionToPlace(&raw mut ipdNew, c1, c2);
                if ix1 != ix2 {
                    MoveIconInfo(&raw mut (*pIBD).IPD, ix1, &raw mut (*pIBD).IPD, ix2);
                }
            }
            ix1 -= 1;
            pII = pII.offset(-1);
        }
    } else if oldRows > newRows
        && (*pIBD).IPD.iconPlacement as ::core::ffi::c_long & ICON_PLACE_TOP_PRIMARY != 0
        || oldCols > newCols
            && (*pIBD).IPD.iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_PRIMARY != 0
    {
        ix1 = 0 as ::core::ffi::c_int;
        pII = (*pIBD).IPD.placeList.offset(ix1 as isize) as *mut IconInfo;
        while ix1 < (*pIBD).IPD.totalPlaces {
            if !(*pII).pCD.is_null() {
                CvtIconPlaceToPosition(&raw mut (*pIBD).IPD, ix1, &raw mut c1, &raw mut c2);
                ix2 = CvtIconPositionToPlace(&raw mut ipdNew, c1, c2);
                if ix1 != ix2 {
                    MoveIconInfo(&raw mut (*pIBD).IPD, ix1, &raw mut (*pIBD).IPD, ix2);
                }
            }
            ix1 += 1;
            pII = pII.offset(1);
        }
    }
    (*pIBD).IPD.placementRows = newRows;
    (*pIBD).IPD.placementCols = newCols;
}
#[no_mangle]
pub unsafe extern "C" fn SetNewBounds(mut pIBD: *mut IconBoxData) {
    let mut i: ::core::ffi::c_int = 0;
    let mut X = 0 as ::core::ffi::c_int;
    let mut Y = 0 as ::core::ffi::c_int;
    let mut cw = ::core::ptr::null_mut::<_CompositeRec>();
    let mut children = ::core::ptr::null_mut::<Widget>();
    cw = (*pIBD).bBoardWidget as CompositeWidget;
    children = (*cw).composite.children;
    i = 0 as ::core::ffi::c_int;
    while (i as Cardinal) < (*cw).composite.num_children {
        if (**children.offset(i as isize)).core.x as ::core::ffi::c_int > X {
            X = (**children.offset(i as isize)).core.x as ::core::ffi::c_int;
        }
        if (**children.offset(i as isize)).core.y as ::core::ffi::c_int > Y {
            Y = (**children.offset(i as isize)).core.y as ::core::ffi::c_int;
        }
        i += 1;
    }
    (*pIBD).lastCol = X / (*(*pIBD).pCD_iconBox).widthInc;
    (*pIBD).lastRow = Y / (*(*pIBD).pCD_iconBox).heightInc;
}
#[no_mangle]
pub unsafe extern "C" fn InsertPosition(mut w: Widget) -> Cardinal {
    return insertPosition;
}
#[no_mangle]
pub unsafe extern "C" fn ShowClientIconState(
    mut pCD: *mut ClientData,
    mut newState: ::core::ffi::c_int,
) {
    if newState == MINIMIZED_STATE && (*pCD).iconWindow != 0 {
        XMapRaised(wmGD.display, (*pCD).iconWindow);
    }
    if (newState == NORMAL_STATE || newState == MAXIMIZED_STATE) && (*pCD).iconWindow != 0 {
        XUnmapWindow(wmGD.display, (*pCD).iconWindow);
    }
}
#[no_mangle]
pub unsafe extern "C" fn IconScrollVisibleCallback(
    mut w: Widget,
    mut client_data: XtPointer,
    mut call_data: *mut XmAnyCallbackStruct,
) {
    let mut vis_data = ::core::ptr::null_mut::<XmTraverseObscuredCallbackStruct>();
    vis_data = call_data as *mut XmTraverseObscuredCallbackStruct;
    XmScrollVisible(
        (*(*(*wmGD.pActiveSD).pActiveWS).pIconBox).scrolledWidget,
        (*vis_data).traversal_destination,
        0 as Dimension,
        0 as Dimension,
    );
}
#[no_mangle]
pub unsafe extern "C" fn IconActivateCallback(
    mut w: Widget,
    mut client_data: XtPointer,
    mut call_data: *mut XmAnyCallbackStruct,
) {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut theIcon: Window = 0;
    theIcon = XtWindowOfObject(w);
    if XFindContext(
        wmGD.display,
        theIcon as XID,
        wmGD.windowContextType,
        &raw mut pCD as *mut XPointer,
    ) == 0
    {
        F_Restore_And_Raise(NULL as String_0, pCD, NULL as *mut XEvent);
    }
}
#[no_mangle]
pub unsafe extern "C" fn UpdateIncrements(
    mut sWidget: Widget,
    mut pIBD: *mut IconBoxData,
    mut event: *mut XConfigureEvent,
) {
    ResetArrowButtonIncrements((*pIBD).pCD_iconBox as *mut ClientData);
}
#[no_mangle]
pub unsafe extern "C" fn ResetArrowButtonIncrements(mut pCD: *mut ClientData) {
    let mut i: ::core::ffi::c_int = 0;
    let mut setArgs: [Arg; 2] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 2];
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7258 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*pCD).heightInc as XtArgVal;
    i += 1;
    XtSetValues(
        (*(*pCD).thisIconBox).vScrollBar,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7258 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*pCD).widthInc as XtArgVal;
    i += 1;
    XtSetValues(
        (*(*pCD).thisIconBox).hScrollBar,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ChangeActiveIconboxIconText(
    mut icon: Widget,
    mut dummy: XtPointer,
    mut event: *mut XFocusChangeEvent,
) {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut theIcon: Window = 0;
    theIcon = XtWindowOfObject(icon);
    if XFindContext(
        wmGD.display,
        theIcon as XID,
        wmGD.windowContextType,
        &raw mut pCD as *mut XPointer,
    ) == 0
        && !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
            .pIconBox
            .is_null()
        && !(*(*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox)
            .pCD_iconBox
            .is_null()
        && (*(*(*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox).pCD_iconBox)
            .clientState
            != MINIMIZED_STATE
    {
        if (*event).type_0 == FocusIn {
            if (*event).send_event != 0 {
                ShowActiveIconText(pCD);
            }
        } else if (*event).send_event != 0 {
            HideActiveIconText((*pCD).pSD);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleIconBoxIconKeyPress(
    mut icon: Widget,
    mut dummy: XtPointer,
    mut keyEvent: *mut XKeyEvent,
) {
    let mut context: Context = 0;
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut theIcon: Window = 0;
    theIcon = XtWindowOfObject(icon);
    if XFindContext(
        wmGD.display,
        theIcon as XID,
        wmGD.windowContextType,
        &raw mut pCD as *mut XPointer,
    ) == 0
    {
        SetClientWsIndex(pCD);
        (*keyEvent).window = (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin;
        if (*pCD).clientState == MINIMIZED_STATE {
            context = F_SUBCONTEXT_IB_IICON as Context;
            (*pCD).grabContext = F_SUBCONTEXT_IB_IICON as Context;
        } else {
            context = F_SUBCONTEXT_IB_WICON as Context;
            (*pCD).grabContext = F_SUBCONTEXT_IB_WICON as Context;
        }
        if HandleKeyPress(
            keyEvent,
            (*wmGD.pActiveSD).keySpecs,
            True as Boolean,
            context,
            False as Boolean,
            pCD,
        ) == 0
        {
            (*keyEvent).window = 0 as Window;
            (*keyEvent).type_0 = 0 as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleIconBoxButtonMotion(
    mut icon: Widget,
    mut client_data: XtPointer,
    mut pev: *mut XEvent,
) {
}
#[no_mangle]
pub unsafe extern "C" fn GetIconBoxIconRootXY(
    mut pCD: *mut ClientData,
    mut pX: *mut ::core::ffi::c_int,
    mut pY: *mut ::core::ffi::c_int,
) {
    let mut child: Window = 0;
    let mut pWsc = GetWsClientData((*(*pCD).pSD).pActiveWS as *mut WmWorkspaceData, pCD);
    if (*(*pCD).pSD).useIconBox as ::core::ffi::c_int != 0 && !(*pWsc).pIconBox.is_null() {
        XTranslateCoordinates(
            wmGD.display,
            XtWindowOfObject((*(*pWsc).pIconBox).bBoardWidget),
            (*(*pCD).pSD).rootWindow,
            (*pWsc).iconX + IB_MARGIN_WIDTH,
            (*pWsc).iconY + IB_MARGIN_HEIGHT,
            pX,
            pY,
            &raw mut child,
        );
    } else {
        *pY = 0 as ::core::ffi::c_int;
        *pX = *pY;
    };
}
#[no_mangle]
pub unsafe extern "C" fn IconVisible(mut pCD: *mut ClientData) -> Boolean {
    let mut rval = True as Boolean;
    let mut pWsc = GetWsClientData((*(*pCD).pSD).pActiveWS as *mut WmWorkspaceData, pCD);
    let mut i: ::core::ffi::c_int = 0;
    let mut getArgs: [Arg; 5] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 5];
    let mut tmpWidth = 0 as Dimension;
    let mut tmpHeight = 0 as Dimension;
    let mut clipX = 0 as Position;
    let mut clipY = 0 as Position;
    let mut tmpX = 0 as Position;
    let mut tmpY = 0 as Position;
    let mut iconX: ::core::ffi::c_int = 0;
    let mut iconY: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut tmpWidth as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut tmpHeight as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(885 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut tmpX as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(887 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut tmpY as XtArgVal;
    i += 1;
    XtGetValues(
        (*(*pWsc).pIconBox).clipWidget,
        &raw mut getArgs as ArgList,
        i as Cardinal,
    );
    XtTranslateCoords(
        (*(*pWsc).pIconBox).scrolledWidget,
        tmpX,
        tmpY,
        &raw mut clipX,
        &raw mut clipY,
    );
    GetIconBoxIconRootXY(pCD, &raw mut iconX, &raw mut iconY);
    if iconX + 2 as ::core::ffi::c_int
        > clipX as ::core::ffi::c_int + tmpWidth as ::core::ffi::c_int
    {
        return 0 as Boolean;
    }
    if iconY + 2 as ::core::ffi::c_int
        > clipY as ::core::ffi::c_int + tmpHeight as ::core::ffi::c_int
    {
        return 0 as Boolean;
    }
    if iconX + ((*(*pCD).pSD).iconWidth - 2 as ::core::ffi::c_int) < clipX as ::core::ffi::c_int {
        return 0 as Boolean;
    }
    if iconY + ((*(*pCD).pSD).iconHeight - 2 as ::core::ffi::c_int) < clipY as ::core::ffi::c_int {
        return 0 as Boolean;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn WmXmStringToString(mut xmString: XmString) -> String_0 {
    let mut sz = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !xmString.is_null() {
        sz = XmStringUnparse(
            xmString,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            XmMULTIBYTE_TEXT,
            XmMULTIBYTE_TEXT,
            ::core::ptr::null_mut::<XmParseMapping>(),
            0 as Cardinal,
            XmOUTPUT_ALL,
        ) as *mut ::core::ffi::c_char;
    }
    return sz as String_0;
}
