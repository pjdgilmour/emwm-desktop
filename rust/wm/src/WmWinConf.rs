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
    fn XCheckIfEvent(
        _: *mut Display,
        _: *mut XEvent,
        _: Option<unsafe extern "C" fn(*mut Display, *mut XEvent, XPointer) -> ::core::ffi::c_int>,
        _: XPointer,
    ) -> ::core::ffi::c_int;
    fn XCheckWindowEvent(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_long,
        _: *mut XEvent,
    ) -> ::core::ffi::c_int;
    fn XDefaultDepth(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XDrawSegments(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XSegment,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XFlush(_: *mut Display) -> ::core::ffi::c_int;
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
    fn XGrabServer(_: *mut Display) -> ::core::ffi::c_int;
    fn XMapRaised(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XMaskEvent(_: *mut Display, _: ::core::ffi::c_long, _: *mut XEvent) -> ::core::ffi::c_int;
    fn XMoveResizeWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XMoveWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
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
    fn XResizeWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XSync(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn XUngrabKeyboard(_: *mut Display, _: Time) -> ::core::ffi::c_int;
    fn XUngrabPointer(_: *mut Display, _: Time) -> ::core::ffi::c_int;
    fn XUngrabServer(_: *mut Display) -> ::core::ffi::c_int;
    fn XUnmapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XWarpPointer(
        _: *mut Display,
        _: Window,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XWindowEvent(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_long,
        _: *mut XEvent,
    ) -> ::core::ffi::c_int;
    fn XtTranslateCoords(_: Widget, _: Position, _: Position, _: *mut Position, _: *mut Position);
    fn XtScreenOfObject(_: Widget) -> *mut Screen;
    fn XtWindowOfObject(_: Widget) -> Window;
    fn XtGetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtMoveWidget(_: Widget, _: Position, _: Position);
    static XtStrings: [::core::ffi::c_char; 0];
    fn XmGetPixmap(
        screen: *mut Screen,
        image_name: *mut ::core::ffi::c_char,
        foreground: Pixel,
        background: Pixel,
    ) -> Pixmap;
    static mut wmGD: WmGlobalData;
    fn XkbKeycodeToKeysym(
        _: *mut Display,
        _: KeyCode,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> KeySym;
    fn FrameToClient(
        pcd: *mut ClientData,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
        pWidth: *mut ::core::ffi::c_uint,
        pHeight: *mut ::core::ffi::c_uint,
    );
    fn ClientToFrame(
        pcd: *mut ClientData,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
        pWidth: *mut ::core::ffi::c_uint,
        pHeight: *mut ::core::ffi::c_uint,
    );
    fn SetFrameInfo(pcd: *mut ClientData);
    fn FrameExposureProc(pcd: *mut ClientData);
    fn RegenerateClientFrame(pcd: *mut ClientData);
    fn PopGadgetOut(pcd: *mut ClientData, gadget: ::core::ffi::c_int);
    fn IsRepeatedKeyEvent(
        dpy: *mut Display,
        pEvent: *mut XEvent,
        pOldEvent: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn PullExposureEvents();
    fn HideFeedbackWindow(pSD: *mut WmScreenData);
    fn ShowFeedbackWindow(
        pSD: *mut WmScreenData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        style: ::core::ffi::c_ulong,
    );
    fn UpdateFeedbackInfo(
        pSD: *mut WmScreenData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
    );
    fn F_Beep(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn HideActiveIconText(pSD: *mut WmScreenData);
    fn MoveActiveIconText(pcd: *mut ClientData);
    fn ShowActiveIconText(pcd: *mut ClientData);
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
    fn MoveIconInfo(
        pIPD1: *mut IconPlacementData,
        p1: ::core::ffi::c_int,
        pIPD2: *mut IconPlacementData,
        p2: ::core::ffi::c_int,
    );
    fn PositionToPlacementData(
        pWS: *mut WmWorkspaceData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    ) -> *mut IconPlacementData;
    fn CheckIconBoxResize(
        pCD: *mut ClientData,
        changedValues: ::core::ffi::c_uint,
        newWidth: ::core::ffi::c_int,
        newHeight: ::core::ffi::c_int,
    );
    fn CheckIconBoxSize(pIBD: *mut IconBoxData) -> Boolean;
    fn SetNewBounds(pIBD: *mut IconBoxData);
    fn RepairFocus();
    fn SendConfigureNotify(pCD: *mut ClientData);
    fn FixWindowConfiguration(
        pCD: *mut ClientData,
        pWidth: *mut ::core::ffi::c_uint,
        pHeight: *mut ::core::ffi::c_uint,
        widthInc: ::core::ffi::c_uint,
        heightInc: ::core::ffi::c_uint,
    );
    fn FixWindowSize(
        pCD: *mut ClientData,
        pWidth: *mut ::core::ffi::c_uint,
        pHeight: *mut ::core::ffi::c_uint,
        widthInc: ::core::ffi::c_uint,
        heightInc: ::core::ffi::c_uint,
    );
    fn PlaceFrameOnScreen(
        pCD: *mut ClientData,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
    );
    fn GetWorkspaceData(pSD: *mut WmScreenData, wsID: WorkspaceID) -> *mut WmWorkspaceData;
    fn GetXineramaScreenFromLocation(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
    fn GetXineramaScreenCount(count: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CopyFromParent: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CurrentTime: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const KeyPressMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const ButtonPressMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ButtonReleaseMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const PointerMotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const PointerMotionHintMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
pub const KeyPress: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const KeyRelease: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ButtonPress: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ButtonRelease: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MotionNotify: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ControlMask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const Button1Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const Button2Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
pub const Button3Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
pub const Button4Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int;
pub const Button5Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int;
pub const Button1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Button2: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const Button3: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const Button4: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const Button5: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const NotifyHint: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GrabModeAsync: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GrabSuccess: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CWBackPixmap: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const CWBackingStore: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const CWOverrideRedirect: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const CWSaveUnder: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 10 as ::core::ffi::c_int;
pub const CWX: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const CWY: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const CWWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const CWHeight: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const NotUseful: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XK_Return: KeySym = 65293 as KeySym;
pub const XK_Escape: KeySym = 65307 as KeySym;
pub const XK_Left: KeySym = 65361 as KeySym;
pub const XK_Up: KeySym = 65362 as KeySym;
pub const XK_Right: KeySym = 65363 as KeySym;
pub const XK_Down: KeySym = 65364 as KeySym;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FB_SIZE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const FB_POSITION: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const STRETCH_NORTH_WEST: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STRETCH_NORTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STRETCH_NORTH_EAST: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STRETCH_EAST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STRETCH_SOUTH_EAST: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const STRETCH_SOUTH: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const STRETCH_SOUTH_WEST: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const STRETCH_WEST: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const WM_SHOW_FB_MOVE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const WM_SHOW_FB_RESIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const WINDOW_PART_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FRAME_NONE: ::core::ffi::c_int = WINDOW_PART_NONE;
pub const FRAME_TITLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FRAME_RESIZE_NW: ::core::ffi::c_int = 6;
pub const FRAME_RESIZE_N: ::core::ffi::c_int = 7;
pub const FRAME_RESIZE_NE: ::core::ffi::c_int = 8;
pub const FRAME_RESIZE_E: ::core::ffi::c_int = 9;
pub const FRAME_RESIZE_SE: ::core::ffi::c_int = 10;
pub const FRAME_RESIZE_S: ::core::ffi::c_int = 11;
pub const FRAME_RESIZE_SW: ::core::ffi::c_int = 12;
pub const FRAME_RESIZE_W: ::core::ffi::c_int = 13;
pub const ICONBOX_PART_WICON: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const F_SUBCONTEXT_IB_WICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_WICON;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICON_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const ICON_ACTIVE_LABEL_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const NO_ICON_PLACE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NO_ACTION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MOVE_CLIENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RESIZE_CLIENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MARQUEE_SELECT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BIGSIZE: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
pub const CONFIG_MASK: ::core::ffi::c_long =
    KeyPressMask | ButtonPressMask | ButtonReleaseMask | PointerMotionMask;
pub const PGRAB_MASK: ::core::ffi::c_long =
    ButtonPressMask | ButtonReleaseMask | PointerMotionMask | PointerMotionHintMask;
pub const ANCHOR_NW: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ANCHOR_NE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ANCHOR_SE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ANCHOR_SW: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ButtonMask: ::core::ffi::c_int =
    Button1Mask | Button2Mask | Button3Mask | Button4Mask | Button5Mask;
static mut pointerX: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut pointerY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut offsetX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut offsetY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut resizeX: ::core::ffi::c_int = 0;
static mut resizeY: ::core::ffi::c_int = 0;
static mut resizeWidth: ::core::ffi::c_uint = 0;
static mut resizeHeight: ::core::ffi::c_uint = 0;
static mut resizeBigWidthInc: ::core::ffi::c_uint = 0;
static mut resizeBigHeightInc: ::core::ffi::c_uint = 0;
static mut startY: ::core::ffi::c_int = 0;
static mut startX: ::core::ffi::c_int = 0;
static mut startWidth: ::core::ffi::c_uint = 0;
static mut startHeight: ::core::ffi::c_uint = 0;
static mut maxWidth: ::core::ffi::c_uint = 0;
static mut maxHeight: ::core::ffi::c_uint = 0;
static mut minWidth: ::core::ffi::c_uint = 0;
static mut minHeight: ::core::ffi::c_uint = 0;
static mut marqueeX: ::core::ffi::c_int = 0;
static mut marqueeY: ::core::ffi::c_int = 0;
static mut marqueeHeight: ::core::ffi::c_long = 0;
static mut marqueeWidth: ::core::ffi::c_long = 0;
static mut marqueeAnchor: ::core::ffi::c_uint = 0;
static mut marqueeWidth0: ::core::ffi::c_long = 0;
static mut marqueeHeight0: ::core::ffi::c_long = 0;
static mut opaqueMoveX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut opaqueMoveY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut moveX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut moveY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut moveIBbbX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut moveIBbbY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut moveWidth: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
static mut moveHeight: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
static mut moveLastPointerX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut moveLastPointerY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut anyMotion: Boolean = FALSE as Boolean;
static mut configGrab: Boolean = FALSE as Boolean;
#[no_mangle]
pub static mut clipWidth: Dimension = 0 as Dimension;
#[no_mangle]
pub static mut clipHeight: Dimension = 0 as Dimension;
#[no_mangle]
pub static mut clipX: Position = 0 as Position;
#[no_mangle]
pub static mut clipY: Position = 0 as Position;
#[no_mangle]
pub unsafe extern "C" fn GetClipDimensions(mut pCD: *mut ClientData, mut fromRoot: Boolean) {
    let mut i: ::core::ffi::c_int = 0;
    let mut getArgs: [Arg; 5] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 5];
    let mut tmpX: Position = 0;
    let mut tmpY: Position = 0;
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
        (*(*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox).clipWidget,
        &raw mut getArgs as ArgList,
        i as Cardinal,
    );
    if fromRoot != 0 {
        XtTranslateCoords(
            (*(*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox).scrolledWidget,
            tmpX,
            tmpY,
            &raw mut clipX,
            &raw mut clipY,
        );
    } else {
        clipX = tmpX;
        clipY = tmpY;
    };
}
#[no_mangle]
pub unsafe extern "C" fn HandleClientFrameMove(mut pcd: *mut ClientData, mut pev: *mut XEvent) {
    let mut tmpX: ::core::ffi::c_int = 0;
    let mut tmpY: ::core::ffi::c_int = 0;
    let mut warpX: ::core::ffi::c_int = 0;
    let mut warpY: ::core::ffi::c_int = 0;
    let mut grab_win: Window = 0;
    let mut keysym: KeySym = 0;
    let mut control: Boolean = 0;
    let mut moveDone: Boolean = 0;
    let mut firstTime: Boolean = 0;
    let mut big_inc: ::core::ffi::c_int = 0;
    let mut keyMultiplier: ::core::ffi::c_int = 0;
    let mut newX: ::core::ffi::c_int = 0;
    let mut newY: ::core::ffi::c_int = 0;
    let mut event = _XEvent { type_0: 0 };
    let mut KeyEvent = _XEvent { type_0: 0 };
    if !pev.is_null() {
        firstTime = True as Boolean;
    } else {
        firstTime = False as Boolean;
    }
    big_inc = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*pcd).pSD).screen as isize))
    .width
        / 20 as ::core::ffi::c_int;
    if configGrab == 0 {
        if StartClientMove(pcd, pev) == 0 {
            return;
        }
    }
    grab_win = GrabWin(pcd, pev);
    if (*(*pcd).pSD).useIconBox as ::core::ffi::c_int != 0
        && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
            .pIconBox
            .is_null()
    {
        GetClipDimensions(pcd, True as Boolean);
    }
    moveDone = False as Boolean;
    while moveDone == 0 {
        tmpY = 0 as ::core::ffi::c_int;
        tmpX = tmpY;
        if firstTime != 0 {
            firstTime = False as Boolean;
        } else {
            pev = &raw mut event;
            GetConfigEvent(
                wmGD.display,
                grab_win,
                CONFIG_MASK as ::core::ffi::c_ulong,
                moveLastPointerX,
                moveLastPointerY,
                moveX,
                moveY,
                moveWidth,
                moveHeight,
                &raw mut event,
            );
        }
        if (*pev).type_0 == KeyPress {
            keyMultiplier = 1 as ::core::ffi::c_int;
            while keyMultiplier <= big_inc
                && XCheckIfEvent(
                    wmGD.display,
                    &raw mut KeyEvent,
                    Some(
                        IsRepeatedKeyEvent
                            as unsafe extern "C" fn(
                                *mut Display,
                                *mut XEvent,
                                *mut ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    pev as XPointer,
                ) != 0
            {
                keyMultiplier += 1;
            }
            keysym = XkbKeycodeToKeysym(
                wmGD.display,
                (*pev).xkey.keycode as KeyCode,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            control = ((*pev).xkey.state & ControlMask as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as Boolean;
            tmpY = 0 as ::core::ffi::c_int;
            tmpX = tmpY;
            match keysym {
                65361 => {
                    tmpX = keyMultiplier
                        * (if control as ::core::ffi::c_int != 0 {
                            -big_inc
                        } else {
                            -(1 as ::core::ffi::c_int)
                        });
                }
                65362 => {
                    tmpY = keyMultiplier
                        * (if control as ::core::ffi::c_int != 0 {
                            -big_inc
                        } else {
                            -(1 as ::core::ffi::c_int)
                        });
                }
                65363 => {
                    tmpX = keyMultiplier
                        * (if control as ::core::ffi::c_int != 0 {
                            big_inc
                        } else {
                            1 as ::core::ffi::c_int
                        });
                }
                65364 => {
                    tmpY = keyMultiplier
                        * (if control as ::core::ffi::c_int != 0 {
                            big_inc
                        } else {
                            1 as ::core::ffi::c_int
                        });
                }
                65293 => {
                    CompleteFrameConfig(pcd, pev);
                    return;
                }
                65307 => {
                    CancelFrameConfig(pcd);
                    CheckEatButtonRelease(pcd, pev);
                    return;
                }
                _ => {}
            }
            if tmpX != 0 || tmpY != 0 {
                warpX = moveLastPointerX + tmpX;
                warpY = moveLastPointerY + tmpY;
                ForceOnScreen((*(*pcd).pSD).screen, &raw mut warpX, &raw mut warpY);
                if warpX != moveLastPointerX || warpY != moveLastPointerY {
                    SetPointerPosition(warpX, warpY, &raw mut newX, &raw mut newY);
                    tmpX = newX - moveLastPointerX;
                    tmpY = newY - moveLastPointerY;
                    moveLastPointerX = newX;
                    moveLastPointerY = newY;
                    moveX += tmpX;
                    moveY += tmpY;
                } else {
                    moveX += tmpX;
                    moveY += tmpY;
                }
            }
        } else if (*pev).type_0 == ButtonRelease {
            moveX += (*pev).xbutton.x_root - moveLastPointerX;
            moveY += (*pev).xbutton.y_root - moveLastPointerY;
            CompleteFrameConfig(pcd, pev);
            moveDone = True as Boolean;
        } else if (*pev).type_0 == MotionNotify {
            tmpX = (*pev).xmotion.x_root - moveLastPointerX;
            tmpY = (*pev).xmotion.y_root - moveLastPointerY;
            moveLastPointerX = (*pev).xmotion.x_root;
            moveLastPointerY = (*pev).xmotion.y_root;
            moveX += tmpX;
            moveY += tmpY;
            anyMotion = True as Boolean;
        }
        if tmpX != 0 || tmpY != 0 {
            FixFrameValues(
                pcd,
                &raw mut moveX,
                &raw mut moveY,
                &raw mut moveWidth,
                &raw mut moveHeight,
                FALSE as Boolean,
            );
            if (*(*pcd).pSD).moveOpaque != 0 {
                MoveOpaque(pcd, moveX, moveY, moveWidth, moveHeight);
            } else {
                MoveOutline(moveX, moveY, moveWidth, moveHeight);
            }
            if wmGD.movingIcon == 0
                && wmGD.showFeedback as ::core::ffi::c_long & WM_SHOW_FB_MOVE != 0
            {
                DoFeedback(
                    pcd,
                    moveX,
                    moveY,
                    moveWidth,
                    moveHeight,
                    0 as ::core::ffi::c_int as ::core::ffi::c_ulong,
                    FALSE as Boolean,
                );
            }
        }
    }
}
unsafe extern "C" fn UpdateAndDrawResize(mut pcd: *mut ClientData) {
    let mut tmpHeight: ::core::ffi::c_int = 0;
    let mut tmpWidth: ::core::ffi::c_int = 0;
    match wmGD.configPart {
        6 | 7 | 8 => {
            tmpHeight = startHeight as ::core::ffi::c_int + (startY - pointerY);
            if tmpHeight < minHeight as ::core::ffi::c_int {
                resizeHeight = minHeight;
                resizeY = (startY as ::core::ffi::c_uint)
                    .wrapping_add(startHeight)
                    .wrapping_sub(minHeight) as ::core::ffi::c_int;
            } else if (*(*pcd).pSD).limitResize as ::core::ffi::c_int != 0
                && tmpHeight > maxHeight as ::core::ffi::c_int
                && (*pcd).clientFlags & ICON_BOX == 0
            {
                resizeHeight = maxHeight;
                resizeY = (startY as ::core::ffi::c_uint)
                    .wrapping_add(startHeight)
                    .wrapping_sub(maxHeight) as ::core::ffi::c_int;
            } else {
                resizeHeight = tmpHeight as ::core::ffi::c_uint;
                resizeY = pointerY;
            }
        }
        12 | 11 | 10 => {
            resizeY = startY;
            tmpHeight = pointerY - startY + 1 as ::core::ffi::c_int;
            if tmpHeight < minHeight as ::core::ffi::c_int {
                resizeHeight = minHeight;
            } else if (*(*pcd).pSD).limitResize as ::core::ffi::c_int != 0
                && tmpHeight > maxHeight as ::core::ffi::c_int
                && (*pcd).clientFlags & ICON_BOX == 0
            {
                resizeHeight = maxHeight;
            } else {
                resizeHeight = tmpHeight as ::core::ffi::c_uint;
            }
        }
        _ => {
            resizeY = startY;
            resizeHeight = startHeight;
        }
    }
    match wmGD.configPart {
        6 | 13 | 12 => {
            tmpWidth = startWidth as ::core::ffi::c_int + (startX - pointerX);
            if tmpWidth < minWidth as ::core::ffi::c_int {
                resizeWidth = minWidth;
                resizeX = (startX as ::core::ffi::c_uint)
                    .wrapping_add(startWidth)
                    .wrapping_sub(minWidth) as ::core::ffi::c_int;
            } else if (*(*pcd).pSD).limitResize as ::core::ffi::c_int != 0
                && tmpWidth > maxWidth as ::core::ffi::c_int
                && (*pcd).clientFlags & ICON_BOX == 0
            {
                resizeWidth = maxWidth;
                resizeX = (startX as ::core::ffi::c_uint)
                    .wrapping_add(startWidth)
                    .wrapping_sub(maxWidth) as ::core::ffi::c_int;
            } else {
                resizeWidth = tmpWidth as ::core::ffi::c_uint;
                resizeX = pointerX;
            }
        }
        8 | 9 | 10 => {
            resizeX = startX;
            tmpWidth = pointerX - startX + 1 as ::core::ffi::c_int;
            if tmpWidth < minWidth as ::core::ffi::c_int {
                resizeWidth = minWidth;
            } else if (*(*pcd).pSD).limitResize as ::core::ffi::c_int != 0
                && tmpWidth > maxWidth as ::core::ffi::c_int
                && (*pcd).clientFlags & ICON_BOX == 0
            {
                resizeWidth = maxWidth;
            } else {
                resizeWidth = tmpWidth as ::core::ffi::c_uint;
            }
        }
        _ => {
            resizeX = startX;
            resizeWidth = startWidth;
        }
    }
    FixFrameValues(
        pcd,
        &raw mut resizeX,
        &raw mut resizeY,
        &raw mut resizeWidth,
        &raw mut resizeHeight,
        TRUE as Boolean,
    );
    MoveOutline(resizeX, resizeY, resizeWidth, resizeHeight);
    if wmGD.showFeedback as ::core::ffi::c_long & WM_SHOW_FB_RESIZE != 0 {
        DoFeedback(
            pcd,
            resizeX,
            resizeY,
            resizeWidth,
            resizeHeight,
            0 as ::core::ffi::c_int as ::core::ffi::c_ulong,
            TRUE as Boolean,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleClientFrameResize(mut pcd: *mut ClientData, mut pev: *mut XEvent) {
    let mut grab_win: Window = 0;
    let mut resizeDone: Boolean = 0;
    let mut event = _XEvent { type_0: 0 };
    if configGrab == 0 {
        if StartResizeConfig(pcd, pev) != 0 {
            configGrab = TRUE as Boolean;
        } else {
            return;
        }
    }
    grab_win = GrabWin(pcd, pev);
    resizeDone = False as Boolean;
    while resizeDone == 0 {
        if pev.is_null() {
            pev = &raw mut event;
            GetConfigEvent(
                wmGD.display,
                grab_win,
                CONFIG_MASK as ::core::ffi::c_ulong,
                pointerX,
                pointerY,
                resizeX,
                resizeY,
                resizeWidth,
                resizeHeight,
                &raw mut event,
            );
        }
        if (*pev).type_0 == MotionNotify {
            pointerX = (*pev).xmotion.x_root;
            pointerY = (*pev).xmotion.y_root;
            anyMotion = TRUE as Boolean;
            if wmGD.configSet == 0 && SetPointerResizePart(pcd, pev) == 0 {
                pev = ::core::ptr::null_mut::<XEvent>();
                continue;
            }
        } else if (*pev).type_0 == KeyPress {
            resizeDone = HandleResizeKeyPress(pcd, pev);
        } else if (*pev).type_0 == ButtonRelease {
            pointerX = (*pev).xbutton.x_root;
            pointerY = (*pev).xbutton.y_root;
            UpdateAndDrawResize(pcd);
            CompleteFrameConfig(pcd, pev);
            resizeDone = True as Boolean;
        } else {
            pev = ::core::ptr::null_mut::<XEvent>();
            continue;
        }
        if resizeDone == 0 {
            UpdateAndDrawResize(pcd);
        }
        pev = ::core::ptr::null_mut::<XEvent>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleResizeKeyPress(
    mut pcd: *mut ClientData,
    mut pev: *mut XEvent,
) -> Boolean {
    let mut keysym: KeySym = 0;
    let mut control: Boolean = 0;
    let mut warpX: ::core::ffi::c_int = 0;
    let mut warpY: ::core::ffi::c_int = 0;
    let mut currentX: ::core::ffi::c_int = 0;
    let mut currentY: ::core::ffi::c_int = 0;
    let mut newX: ::core::ffi::c_int = 0;
    let mut newY: ::core::ffi::c_int = 0;
    let mut junk: ::core::ffi::c_int = 0;
    let mut keyMult: ::core::ffi::c_int = 0;
    let mut junk_win: Window = 0;
    let mut KeyEvent = _XEvent { type_0: 0 };
    keyMult = 1 as ::core::ffi::c_int;
    while keyMult <= 10 as ::core::ffi::c_int
        && XCheckIfEvent(
            wmGD.display,
            &raw mut KeyEvent,
            Some(
                IsRepeatedKeyEvent
                    as unsafe extern "C" fn(
                        *mut Display,
                        *mut XEvent,
                        *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            pev as XPointer,
        ) != 0
    {
        keyMult += 1;
    }
    keysym = XkbKeycodeToKeysym(
        wmGD.display,
        (*pev).xkey.keycode as KeyCode,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    control = ((*pev).xkey.state & ControlMask as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint)
        as ::core::ffi::c_int as Boolean;
    match keysym {
        65361 => match wmGD.configPart {
            0 => {
                wmGD.configPart = FRAME_RESIZE_W as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
                warpY = (resizeY as ::core::ffi::c_uint)
                    .wrapping_add(resizeHeight.wrapping_div(2 as ::core::ffi::c_uint))
                    as ::core::ffi::c_int;
                warpX = (resizeX as ::core::ffi::c_uint).wrapping_add(
                    (if control as ::core::ffi::c_int != 0 {
                        resizeBigWidthInc.wrapping_neg()
                    } else {
                        -(*pcd).widthInc as ::core::ffi::c_uint
                    }),
                ) as ::core::ffi::c_int;
            }
            7 => {
                wmGD.configPart = FRAME_RESIZE_NW as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
                warpX = (resizeX as ::core::ffi::c_uint).wrapping_add(
                    (if control as ::core::ffi::c_int != 0 {
                        resizeBigWidthInc.wrapping_neg()
                    } else {
                        -(*pcd).widthInc as ::core::ffi::c_uint
                    }),
                ) as ::core::ffi::c_int;
                warpY = pointerY;
            }
            11 => {
                wmGD.configPart = FRAME_RESIZE_SW as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
                warpX = (resizeX as ::core::ffi::c_uint).wrapping_add(
                    (if control as ::core::ffi::c_int != 0 {
                        resizeBigWidthInc.wrapping_neg()
                    } else {
                        -(*pcd).widthInc as ::core::ffi::c_uint
                    }),
                ) as ::core::ffi::c_int;
                warpY = pointerY;
            }
            _ => {
                warpX = (pointerX as ::core::ffi::c_uint).wrapping_add(
                    (if control as ::core::ffi::c_int != 0 {
                        resizeBigWidthInc
                            .wrapping_neg()
                            .wrapping_mul(keyMult as ::core::ffi::c_uint)
                    } else {
                        (-(*pcd).widthInc * keyMult) as ::core::ffi::c_uint
                    }),
                ) as ::core::ffi::c_int;
                warpY = pointerY;
            }
        },
        65362 => match wmGD.configPart {
            0 => {
                wmGD.configPart = FRAME_RESIZE_N as ::core::ffi::c_uint;
                warpX = (resizeX as ::core::ffi::c_uint)
                    .wrapping_add(resizeWidth.wrapping_div(2 as ::core::ffi::c_uint))
                    as ::core::ffi::c_int;
                warpY = (resizeY as ::core::ffi::c_uint).wrapping_add(
                    (if control as ::core::ffi::c_int != 0 {
                        resizeBigHeightInc.wrapping_neg()
                    } else {
                        -(*pcd).heightInc as ::core::ffi::c_uint
                    }),
                ) as ::core::ffi::c_int;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
            }
            13 => {
                wmGD.configPart = FRAME_RESIZE_NW as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
                warpX = pointerX;
                warpY = (resizeY as ::core::ffi::c_uint).wrapping_add(
                    (if control as ::core::ffi::c_int != 0 {
                        resizeBigHeightInc.wrapping_neg()
                    } else {
                        -(*pcd).heightInc as ::core::ffi::c_uint
                    }),
                ) as ::core::ffi::c_int;
            }
            9 => {
                wmGD.configPart = FRAME_RESIZE_NE as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
                warpX = pointerX;
                warpY = (resizeY as ::core::ffi::c_uint).wrapping_add(
                    (if control as ::core::ffi::c_int != 0 {
                        resizeBigHeightInc.wrapping_neg()
                    } else {
                        -(*pcd).heightInc as ::core::ffi::c_uint
                    }),
                ) as ::core::ffi::c_int;
            }
            _ => {
                warpX = pointerX;
                warpY = (pointerY as ::core::ffi::c_uint).wrapping_add(
                    (if control as ::core::ffi::c_int != 0 {
                        resizeBigHeightInc
                            .wrapping_neg()
                            .wrapping_mul(keyMult as ::core::ffi::c_uint)
                    } else {
                        (-(*pcd).heightInc * keyMult) as ::core::ffi::c_uint
                    }),
                ) as ::core::ffi::c_int;
            }
        },
        65363 => match wmGD.configPart {
            0 => {
                wmGD.configPart = FRAME_RESIZE_E as ::core::ffi::c_uint;
                warpY = (resizeY as ::core::ffi::c_uint)
                    .wrapping_add(resizeHeight.wrapping_div(2 as ::core::ffi::c_uint))
                    as ::core::ffi::c_int;
                warpX = (resizeX as ::core::ffi::c_uint)
                    .wrapping_add(resizeWidth)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (if control as ::core::ffi::c_int != 0 {
                            resizeBigWidthInc
                        } else {
                            (*pcd).widthInc as ::core::ffi::c_uint
                        }),
                    ) as ::core::ffi::c_int;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
            }
            7 => {
                wmGD.configPart = FRAME_RESIZE_NE as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
                warpX = (resizeX as ::core::ffi::c_uint)
                    .wrapping_add(resizeWidth)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (if control as ::core::ffi::c_int != 0 {
                            resizeBigWidthInc
                        } else {
                            (*pcd).widthInc as ::core::ffi::c_uint
                        }),
                    ) as ::core::ffi::c_int;
                warpY = pointerY;
            }
            11 => {
                wmGD.configPart = FRAME_RESIZE_SE as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
                warpX = (resizeX as ::core::ffi::c_uint)
                    .wrapping_add(resizeWidth)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (if control as ::core::ffi::c_int != 0 {
                            resizeBigWidthInc
                        } else {
                            (*pcd).widthInc as ::core::ffi::c_uint
                        }),
                    ) as ::core::ffi::c_int;
                warpY = pointerY;
            }
            _ => {
                warpX = (pointerX as ::core::ffi::c_uint).wrapping_add(
                    (if control as ::core::ffi::c_int != 0 {
                        resizeBigWidthInc.wrapping_mul(keyMult as ::core::ffi::c_uint)
                    } else {
                        ((*pcd).widthInc * keyMult) as ::core::ffi::c_uint
                    }),
                ) as ::core::ffi::c_int;
                warpY = pointerY;
            }
        },
        65364 => match wmGD.configPart {
            0 => {
                wmGD.configPart = FRAME_RESIZE_S as ::core::ffi::c_uint;
                warpX = (resizeX as ::core::ffi::c_uint)
                    .wrapping_add(resizeWidth.wrapping_div(2 as ::core::ffi::c_uint))
                    as ::core::ffi::c_int;
                warpY = (resizeY as ::core::ffi::c_uint)
                    .wrapping_add(resizeHeight)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (if control as ::core::ffi::c_int != 0 {
                            resizeBigHeightInc
                        } else {
                            (*pcd).heightInc as ::core::ffi::c_uint
                        }),
                    ) as ::core::ffi::c_int;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
            }
            9 => {
                wmGD.configPart = FRAME_RESIZE_SE as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
                warpX = pointerX;
                warpY = (resizeY as ::core::ffi::c_uint)
                    .wrapping_add(resizeHeight)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (if control as ::core::ffi::c_int != 0 {
                            resizeBigHeightInc
                        } else {
                            (*pcd).heightInc as ::core::ffi::c_uint
                        }),
                    ) as ::core::ffi::c_int;
            }
            13 => {
                wmGD.configPart = FRAME_RESIZE_SW as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, (*pev).xkey.time);
                warpX = pointerX;
                warpY = (resizeY as ::core::ffi::c_uint)
                    .wrapping_add(resizeHeight)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (if control as ::core::ffi::c_int != 0 {
                            resizeBigHeightInc
                        } else {
                            (*pcd).heightInc as ::core::ffi::c_uint
                        }),
                    ) as ::core::ffi::c_int;
            }
            _ => {
                warpX = pointerX;
                warpY = (pointerY as ::core::ffi::c_uint).wrapping_add(
                    (if control as ::core::ffi::c_int != 0 {
                        resizeBigHeightInc.wrapping_mul(keyMult as ::core::ffi::c_uint)
                    } else {
                        ((*pcd).heightInc * keyMult) as ::core::ffi::c_uint
                    }),
                ) as ::core::ffi::c_int;
            }
        },
        65293 => {
            CompleteFrameConfig(pcd, pev);
            return 1 as Boolean;
        }
        65307 => {
            CancelFrameConfig(pcd);
            CheckEatButtonRelease(pcd, pev);
            return 1 as Boolean;
        }
        _ => return 0 as Boolean,
    }
    pointerX = warpX;
    pointerY = warpY;
    ForceOnScreen((*(*pcd).pSD).screen, &raw mut warpX, &raw mut warpY);
    if wmGD.enableWarp as ::core::ffi::c_int != 0
        && XQueryPointer(
            wmGD.display,
            (*(*pcd).pSD).rootWindow,
            &raw mut junk_win,
            &raw mut junk_win,
            &raw mut currentX,
            &raw mut currentY,
            &raw mut junk,
            &raw mut junk,
            &raw mut junk as *mut ::core::ffi::c_uint,
        ) != 0
    {
        if warpX != currentX || warpY != currentY {
            SetPointerPosition(warpX, warpY, &raw mut newX, &raw mut newY);
            return 0 as Boolean;
        }
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn DoFeedback(
    mut pcd: *mut ClientData,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
    mut newStyle: ::core::ffi::c_ulong,
    mut resizing: Boolean,
) {
    let mut cwidth: ::core::ffi::c_uint = 0;
    let mut cheight: ::core::ffi::c_uint = 0;
    let mut decor_x = ((*pcd).clientOffset.x as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    let mut decor_y = ((*pcd).clientOffset.y as ::core::ffi::c_int
        + (*pcd).clientOffset.x as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    if resizing != 0 {
        FixWindowConfiguration(
            pcd,
            &raw mut width,
            &raw mut height,
            (*pcd).widthInc as ::core::ffi::c_uint,
            (*pcd).heightInc as ::core::ffi::c_uint,
        );
    }
    if (*pcd).widthInc > 1 as ::core::ffi::c_int && width >= decor_x {
        width = width.wrapping_sub(decor_x);
    }
    if (*pcd).heightInc > 1 as ::core::ffi::c_int && height >= decor_y {
        height = height.wrapping_sub(decor_y);
    }
    cwidth = width
        .wrapping_sub(
            (if (*pcd).widthInc == 1 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else {
                (*pcd).baseWidth
            }) as ::core::ffi::c_uint,
        )
        .wrapping_div((*pcd).widthInc as ::core::ffi::c_uint);
    cheight = height
        .wrapping_sub(
            (if (*pcd).heightInc == 1 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else {
                (*pcd).baseHeight
            }) as ::core::ffi::c_uint,
        )
        .wrapping_div((*pcd).heightInc as ::core::ffi::c_uint);
    if newStyle != 0 {
        ShowFeedbackWindow((*pcd).pSD, x, y, cwidth, cheight, newStyle);
    } else {
        UpdateFeedbackInfo((*pcd).pSD, x, y, cwidth, cheight);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CheckVisualPlace(
    mut pCD: *mut ClientData,
    mut tmpX: ::core::ffi::c_int,
    mut tmpY: ::core::ffi::c_int,
) -> Boolean {
    let mut rval = True as Boolean;
    let mut child: Window = 0;
    let mut newX: ::core::ffi::c_int = 0;
    let mut newY: ::core::ffi::c_int = 0;
    GetClipDimensions(pCD, True as Boolean);
    XTranslateCoordinates(
        wmGD.display,
        XtWindowOfObject(
            (*(*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox).bBoardWidget,
        ),
        (*(*pCD).pSD).rootWindow,
        tmpX,
        tmpY,
        &raw mut newX,
        &raw mut newY,
        &raw mut child,
    );
    if newX < clipX as ::core::ffi::c_int {
        return 0 as Boolean;
    }
    if newY < clipY as ::core::ffi::c_int {
        return 0 as Boolean;
    }
    if newX
        > clipX as ::core::ffi::c_int + clipWidth as ::core::ffi::c_int - (*(*pCD).pSD).iconWidth
    {
        return 0 as Boolean;
    }
    if newY
        > clipY as ::core::ffi::c_int + clipHeight as ::core::ffi::c_int - (*(*pCD).pSD).iconHeight
    {
        return 0 as Boolean;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn CompleteFrameConfig(mut pcd: *mut ClientData, mut pev: *mut XEvent) {
    let mut tmpWidth: ::core::ffi::c_uint = 0;
    let mut tmpHeight: ::core::ffi::c_uint = 0;
    let mut tmpX: ::core::ffi::c_int = 0;
    let mut tmpY: ::core::ffi::c_int = 0;
    let mut inIconBox: Boolean = 0;
    if wmGD.configAction == RESIZE_CLIENT {
        UndoGrabs();
        tmpX = resizeX;
        tmpY = resizeY;
        FrameToClient(
            pcd,
            &raw mut tmpX,
            &raw mut tmpY,
            &raw mut resizeWidth,
            &raw mut resizeHeight,
        );
        tmpWidth = resizeWidth;
        tmpHeight = resizeHeight;
        FixWindowConfiguration(
            pcd,
            &raw mut tmpWidth,
            &raw mut tmpHeight,
            (*pcd).widthInc as ::core::ffi::c_uint,
            (*pcd).heightInc as ::core::ffi::c_uint,
        );
        AdjustPos(
            &raw mut resizeX,
            &raw mut resizeY,
            resizeWidth,
            resizeHeight,
            tmpWidth,
            tmpHeight,
        );
        ProcessNewConfiguration(
            pcd,
            resizeX,
            resizeY,
            resizeWidth,
            resizeHeight,
            FALSE as Boolean,
        );
    } else if wmGD.configAction == MOVE_CLIENT {
        UndoGrabs();
        if wmGD.configAction == MOVE_CLIENT
            && wmGD.gadgetClient == pcd
            && wmGD.gadgetDepressed == FRAME_TITLE
        {
            PopGadgetOut(pcd, FRAME_TITLE);
            FrameExposureProc(pcd);
        }
        if wmGD.movingIcon != 0 {
            inIconBox = ((*(*pcd).pSD).useIconBox as ::core::ffi::c_int != 0
                && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                    .pIconBox
                    .is_null()) as ::core::ffi::c_int as Boolean;
            if wmGD.iconAutoPlace as ::core::ffi::c_int != 0 || inIconBox as ::core::ffi::c_int != 0
            {
                let mut centerX: ::core::ffi::c_int = 0;
                let mut centerY: ::core::ffi::c_int = 0;
                let mut place: ::core::ffi::c_int = 0;
                let mut pcdIPD = ::core::ptr::null_mut::<IconPlacementData>();
                let mut pIPD = ::core::ptr::null_mut::<IconPlacementData>();
                let mut pIPDnew = ::core::ptr::null_mut::<IconPlacementData>();
                pcdIPD = (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).IPData;
                if inIconBox != 0 {
                    pIPD = &raw mut (*(*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).pIconBox)
                        .IPD as *mut IconPlacementData;
                    pIPDnew = pIPD;
                    moveX -= moveIBbbX;
                    moveY -= moveIBbbY;
                } else {
                    let mut cwsd = GetWorkspaceData(
                        (*pcd).pSD,
                        (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).wsID,
                    );
                    pIPDnew = PositionToPlacementData(cwsd, moveX, moveY);
                    pIPD = if !pcdIPD.is_null() { pcdIPD } else { pIPDnew };
                }
                centerX = moveX + (*(*pcd).pSD).iconWidth / 2 as ::core::ffi::c_int;
                centerY = moveY + (*(*pcd).pSD).iconHeight / 2 as ::core::ffi::c_int;
                place = CvtIconPositionToPlace(pIPDnew, centerX, centerY);
                if pcdIPD != pIPDnew
                    || place != (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconPlace
                {
                    if !(*(*pIPDnew).placeList.offset(place as isize)).pCD.is_null() {
                        place = FindIconPlace(pcd, pIPDnew, centerX, centerY);
                        if place == NO_ICON_PLACE {
                            F_Beep(
                                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                pcd,
                                NULL as *mut XEvent,
                            );
                            if (*(*pcd).pSD).moveOpaque as ::core::ffi::c_int != 0 && inIconBox == 0
                            {
                                XMoveWindow(
                                    wmGD.display,
                                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                                        .iconFrameWin,
                                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconX,
                                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconY,
                                );
                                if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long
                                    & ICON_ACTIVE_LABEL_PART
                                    != 0
                                    && wmGD.keyboardFocus == pcd
                                {
                                    MoveActiveIconText(pcd);
                                    ShowActiveIconText(pcd);
                                }
                            }
                        }
                    }
                    if place != NO_ICON_PLACE
                        && (pcdIPD != pIPDnew
                            || place
                                != (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconPlace)
                    {
                        if inIconBox != 0 {
                            CvtIconPlaceToPosition(pIPD, place, &raw mut tmpX, &raw mut tmpY);
                            if CheckIconBoxSize(
                                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).pIconBox,
                            ) as ::core::ffi::c_int
                                != 0
                                && CheckVisualPlace(pcd, tmpX, tmpY) as ::core::ffi::c_int != 0
                            {
                                MoveIconInfo(
                                    pIPD,
                                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconPlace,
                                    pIPD,
                                    place,
                                );
                                CvtIconPlaceToPosition(
                                    pIPD,
                                    place,
                                    &raw mut (*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                                        .iconX,
                                    &raw mut (*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                                        .iconY,
                                );
                                XtMoveWidget(
                                    (*(*pIPD).placeList.offset(
                                        (*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                                            .iconPlace
                                            as isize,
                                    ))
                                    .theWidget,
                                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconX
                                        as Position,
                                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconY
                                        as Position,
                                );
                                SetNewBounds(
                                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).pIconBox,
                                );
                                if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long
                                    & ICON_ACTIVE_LABEL_PART
                                    != 0
                                {
                                    MoveActiveIconText(pcd);
                                }
                            } else {
                                F_Beep(
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    pcd,
                                    NULL as *mut XEvent,
                                );
                            }
                        } else {
                            MoveIconInfo(
                                pIPD,
                                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconPlace,
                                pIPDnew,
                                place,
                            );
                            CvtIconPlaceToPosition(
                                pIPDnew,
                                place,
                                &raw mut (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconX,
                                &raw mut (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconY,
                            );
                            XMoveWindow(
                                wmGD.display,
                                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconX,
                                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconY,
                            );
                            if (*(*pcd).pSD).moveOpaque as ::core::ffi::c_int != 0
                                && (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long
                                    & ICON_ACTIVE_LABEL_PART
                                    != 0
                                && wmGD.keyboardFocus == pcd
                            {
                                MoveActiveIconText(pcd);
                                ShowActiveIconText(pcd);
                            }
                            let ref mut fresh4 =
                                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).IPData;
                            *fresh4 = pIPDnew;
                        }
                    }
                } else if (*(*pcd).pSD).moveOpaque as ::core::ffi::c_int != 0 && inIconBox == 0 {
                    XMoveWindow(
                        wmGD.display,
                        (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                        (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconX,
                        (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconY,
                    );
                    if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART
                        != 0
                        && wmGD.keyboardFocus == pcd
                    {
                        MoveActiveIconText(pcd);
                        ShowActiveIconText(pcd);
                    }
                }
            } else {
                XMoveWindow(
                    wmGD.display,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                    moveX,
                    moveY,
                );
                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconX = moveX;
                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconY = moveY;
            }
            if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART != 0
                && wmGD.keyboardFocus == pcd
            {
                MoveActiveIconText(pcd);
            }
        } else {
            ProcessNewConfiguration(
                pcd,
                moveX,
                moveY,
                moveWidth.wrapping_sub((2 as ::core::ffi::c_int * offsetX) as ::core::ffi::c_uint),
                moveHeight
                    .wrapping_sub(offsetX as ::core::ffi::c_uint)
                    .wrapping_sub(offsetY as ::core::ffi::c_uint),
                FALSE as Boolean,
            );
        }
    } else if wmGD.configAction == MARQUEE_SELECT {
        UndoGrabs();
    }
    wmGD.configAction = NO_ACTION;
    wmGD.configPart = FRAME_NONE as ::core::ffi::c_uint;
    wmGD.configSet = False as Boolean;
    configGrab = FALSE as Boolean;
    anyMotion = FALSE as Boolean;
    wmGD.movingIcon = FALSE as Boolean;
    if !pcd.is_null() {
        HideFeedbackWindow((*pcd).pSD);
        RepairFocus();
    }
}
#[no_mangle]
pub unsafe extern "C" fn MoveOpaque(
    mut pcd: *mut ClientData,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
) {
    if wmGD.movingIcon != 0 {
        if (*(*pcd).pSD).useIconBox as ::core::ffi::c_int != 0
            && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                .pIconBox
                .is_null()
        {
            MoveOutline(x, y, width, height);
        } else {
            XMoveWindow(
                wmGD.display,
                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                x,
                y,
            );
        }
    } else {
        XMoveWindow(wmGD.display, (*pcd).clientFrameWin, x, y);
    }
    PullExposureEvents();
}
#[no_mangle]
pub unsafe extern "C" fn MoveOutline(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
) {
    if wmGD.useWindowOutline != 0 {
        WindowOutline(x, y, width, height);
    } else {
        DrawOutline(x, y, width, height);
    };
}
unsafe extern "C" fn CreateOutlineWindows(mut pSD: *mut WmScreenData) {
    let mut xswa = XSetWindowAttributes {
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
    let mut xswamask: ::core::ffi::c_uint = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    x = -(10 as ::core::ffi::c_int);
    y = -(10 as ::core::ffi::c_int);
    width = wmGD.outlineWidth as ::core::ffi::c_int;
    height = wmGD.outlineWidth as ::core::ffi::c_int;
    xswa.override_redirect = True;
    xswa.backing_store = NotUseful;
    xswa.save_under = True;
    xswa.background_pixmap = XmGetPixmap(
        XtScreenOfObject((*pSD).screenTopLevelW),
        b"50_foreground\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*pSD).clientAppearance.activeTopShadowColor,
        (*pSD).clientAppearance.activeBackground,
    );
    xswamask =
        (CWOverrideRedirect | CWBackingStore | CWBackPixmap | CWSaveUnder) as ::core::ffi::c_uint;
    (*pSD).woN = XCreateWindow(
        wmGD.display,
        (*pSD).rootWindow,
        x,
        y,
        width as ::core::ffi::c_uint,
        height as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        XDefaultDepth(wmGD.display, (*pSD).screen),
        CopyFromParent as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<Visual>(),
        xswamask as ::core::ffi::c_ulong,
        &raw mut xswa,
    );
    (*pSD).woS = XCreateWindow(
        wmGD.display,
        (*pSD).rootWindow,
        x,
        y,
        width as ::core::ffi::c_uint,
        height as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        XDefaultDepth(wmGD.display, (*pSD).screen),
        CopyFromParent as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<Visual>(),
        xswamask as ::core::ffi::c_ulong,
        &raw mut xswa,
    );
    (*pSD).woE = XCreateWindow(
        wmGD.display,
        (*pSD).rootWindow,
        x,
        y,
        width as ::core::ffi::c_uint,
        height as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        XDefaultDepth(wmGD.display, (*pSD).screen),
        CopyFromParent as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<Visual>(),
        xswamask as ::core::ffi::c_ulong,
        &raw mut xswa,
    );
    (*pSD).woW = XCreateWindow(
        wmGD.display,
        (*pSD).rootWindow,
        x,
        y,
        width as ::core::ffi::c_uint,
        height as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        XDefaultDepth(wmGD.display, (*pSD).screen),
        CopyFromParent as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<Visual>(),
        xswamask as ::core::ffi::c_ulong,
        &raw mut xswa,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WindowOutline(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
) {
    static mut lastOutlineX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut lastOutlineY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut lastOutlineWidth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut lastOutlineHeight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pSD = wmGD.pActiveSD;
    let mut iX: ::core::ffi::c_int = 0;
    let mut iY: ::core::ffi::c_int = 0;
    let mut iW: ::core::ffi::c_int = 0;
    let mut iH: ::core::ffi::c_int = 0;
    if (*pSD).woN == 0 as ::core::ffi::c_long as Window {
        CreateOutlineWindows(pSD);
    }
    if x == lastOutlineX
        && y == lastOutlineY
        && width == lastOutlineWidth as ::core::ffi::c_uint
        && height == lastOutlineHeight as ::core::ffi::c_uint
    {
        return;
    }
    XUnmapWindow(wmGD.display, (*pSD).woN);
    XUnmapWindow(wmGD.display, (*pSD).woS);
    XUnmapWindow(wmGD.display, (*pSD).woE);
    XUnmapWindow(wmGD.display, (*pSD).woW);
    if width == 0 as ::core::ffi::c_uint && height == 0 as ::core::ffi::c_uint {
        lastOutlineHeight = 0 as ::core::ffi::c_int;
        lastOutlineWidth = lastOutlineHeight;
        lastOutlineY = 0 as ::core::ffi::c_int;
        lastOutlineX = lastOutlineY;
    } else {
        iX = x;
        iY = y;
        iW = width as ::core::ffi::c_int;
        iH = wmGD.outlineWidth as ::core::ffi::c_int;
        if iW < 0 as ::core::ffi::c_int {
            iW = 1 as ::core::ffi::c_int;
        }
        XMoveResizeWindow(
            wmGD.display,
            (*pSD).woN,
            iX,
            iY,
            iW as ::core::ffi::c_uint,
            iH as ::core::ffi::c_uint,
        );
        iX = x;
        iY = y + wmGD.outlineWidth as ::core::ffi::c_int;
        iW = wmGD.outlineWidth as ::core::ffi::c_int;
        iH = height as ::core::ffi::c_int - wmGD.outlineWidth as ::core::ffi::c_int;
        if iH < 0 as ::core::ffi::c_int {
            iH = 1 as ::core::ffi::c_int;
        }
        XMoveResizeWindow(
            wmGD.display,
            (*pSD).woW,
            iX,
            iY,
            iW as ::core::ffi::c_uint,
            iH as ::core::ffi::c_uint,
        );
        iX = x + width as ::core::ffi::c_int - wmGD.outlineWidth as ::core::ffi::c_int;
        iY = y + wmGD.outlineWidth as ::core::ffi::c_int;
        iW = wmGD.outlineWidth as ::core::ffi::c_int;
        iH = height as ::core::ffi::c_int - wmGD.outlineWidth as ::core::ffi::c_int;
        if iH < 0 as ::core::ffi::c_int {
            iH = 1 as ::core::ffi::c_int;
        }
        XMoveResizeWindow(
            wmGD.display,
            (*pSD).woE,
            iX,
            iY,
            iW as ::core::ffi::c_uint,
            iH as ::core::ffi::c_uint,
        );
        iX = x + wmGD.outlineWidth as ::core::ffi::c_int;
        iY = y + height as ::core::ffi::c_int - wmGD.outlineWidth as ::core::ffi::c_int;
        iW = width as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int * wmGD.outlineWidth as ::core::ffi::c_int;
        iH = wmGD.outlineWidth as ::core::ffi::c_int;
        if iW < 0 as ::core::ffi::c_int {
            iW = 1 as ::core::ffi::c_int;
        }
        XMoveResizeWindow(
            wmGD.display,
            (*pSD).woS,
            iX,
            iY,
            iW as ::core::ffi::c_uint,
            iH as ::core::ffi::c_uint,
        );
        lastOutlineX = x;
        lastOutlineY = y;
        lastOutlineWidth = width as ::core::ffi::c_int;
        lastOutlineHeight = height as ::core::ffi::c_int;
        XMapRaised(wmGD.display, (*pSD).woN);
        XMapRaised(wmGD.display, (*pSD).woS);
        XMapRaised(wmGD.display, (*pSD).woE);
        XMapRaised(wmGD.display, (*pSD).woW);
        PullExposureEvents();
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawOutline(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
) {
    static mut lastOutlineX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut lastOutlineY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut lastOutlineWidth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut lastOutlineHeight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let vla = (4 as ::core::ffi::c_int * wmGD.outlineWidth as ::core::ffi::c_int) as usize;
    let mut outline: Vec<XSegment> = ::std::vec::from_elem(
        XSegment {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        },
        vla,
    );
    if x == lastOutlineX
        && y == lastOutlineY
        && width == lastOutlineWidth as ::core::ffi::c_uint
        && height == lastOutlineHeight as ::core::ffi::c_uint
    {
        return;
    }
    if lastOutlineWidth != 0 || lastOutlineHeight != 0 {
        SetOutline(
            outline.as_mut_ptr(),
            lastOutlineX,
            lastOutlineY,
            lastOutlineWidth as ::core::ffi::c_uint,
            lastOutlineHeight as ::core::ffi::c_uint,
            wmGD.outlineWidth as ::core::ffi::c_int,
        );
        XDrawSegments(
            wmGD.display,
            (*wmGD.pActiveSD).rootWindow as Drawable,
            (*wmGD.pActiveSD).xorGC,
            outline.as_mut_ptr(),
            4 as ::core::ffi::c_int * wmGD.outlineWidth as ::core::ffi::c_int,
        );
    }
    lastOutlineX = x;
    lastOutlineY = y;
    lastOutlineWidth = width as ::core::ffi::c_int;
    lastOutlineHeight = height as ::core::ffi::c_int;
    if lastOutlineWidth != 0 || lastOutlineHeight != 0 {
        SetOutline(
            outline.as_mut_ptr(),
            lastOutlineX,
            lastOutlineY,
            lastOutlineWidth as ::core::ffi::c_uint,
            lastOutlineHeight as ::core::ffi::c_uint,
            wmGD.outlineWidth as ::core::ffi::c_int,
        );
        XDrawSegments(
            wmGD.display,
            (*wmGD.pActiveSD).rootWindow as Drawable,
            (*wmGD.pActiveSD).xorGC,
            outline.as_mut_ptr(),
            4 as ::core::ffi::c_int * wmGD.outlineWidth as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn WindowIsOnScreen(
    mut pCD: *mut ClientData,
    mut dx: *mut ::core::ffi::c_int,
    mut dy: *mut ::core::ffi::c_int,
) -> Boolean {
    let mut x1 = (*pCD).clientX;
    let mut x2 = (*pCD).clientX + (*pCD).clientWidth;
    let mut y1 = (*pCD).clientY;
    let mut y2 = (*pCD).clientY + (*pCD).clientHeight;
    let mut screenW = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*pCD).pSD).screen as isize))
    .width;
    let mut screenH = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*pCD).pSD).screen as isize))
    .height;
    *dy = 0 as ::core::ffi::c_int;
    *dx = *dy;
    if x2 < 0 as ::core::ffi::c_int {
        *dx = -x2;
    } else if x1 > screenW {
        *dx = screenW - x1;
    }
    if y2 < 0 as ::core::ffi::c_int {
        *dy = -y2;
    } else if y1 > screenH {
        *dy = screenH - y1;
    }
    return (*dx == 0 as ::core::ffi::c_int && *dy == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn RecomputeMaxConfig(mut pCD: *mut ClientData) {
    let mut si = XineramaScreenInfo {
        screen_number: 0,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    };
    let mut frmWidth = (*pCD).clientOffset.x as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
    let mut frmHeight =
        (*pCD).clientOffset.x as ::core::ffi::c_int + (*pCD).clientOffset.y as ::core::ffi::c_int;
    if GetXineramaScreenFromLocation((*pCD).clientX, (*pCD).clientY, &raw mut si) == 0 {
        si.width = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*pCD).pSD).screen as isize))
        .width as ::core::ffi::c_short;
        si.height = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*pCD).pSD).screen as isize))
        .height as ::core::ffi::c_short;
    }
    (*pCD).oldMaxWidth = (*pCD).maxWidth;
    (*pCD).maxWidth = si.width as ::core::ffi::c_int - frmWidth;
    (*pCD).oldMaxHeight = (*pCD).maxHeight;
    (*pCD).maxHeight = si.height as ::core::ffi::c_int - frmHeight;
    if (*pCD).maxWidth > (*pCD).maxWidthLimit {
        (*pCD).maxWidth = (*pCD).maxWidthLimit;
    }
    if (*pCD).maxHeight > (*pCD).maxHeightLimit {
        (*pCD).maxHeight = (*pCD).maxHeightLimit;
    }
    (*pCD).maxWidth -= ((*pCD).maxWidth - (*pCD).baseWidth) % (*pCD).widthInc;
    (*pCD).maxHeight -= ((*pCD).maxHeight - (*pCD).baseHeight) % (*pCD).heightInc;
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
pub unsafe extern "C" fn ProcessNewConfiguration(
    mut pCD: *mut ClientData,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
    mut clientRequest: Boolean,
) {
    let mut changedValues = 0 as ::core::ffi::c_uint;
    let mut xoff = 0 as ::core::ffi::c_int;
    let mut yoff = 0 as ::core::ffi::c_int;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut oldX = (*pCD).clientX - (*pCD).clientOffset.x as ::core::ffi::c_int;
    let mut oldY = (*pCD).clientY - (*pCD).clientOffset.y as ::core::ffi::c_int;
    let mut originallyOnScreen = WindowIsOnScreen(pCD, &raw mut dx, &raw mut dy);
    let mut newMax = False as Boolean;
    let mut toNewMax = False as Boolean;
    let mut nxs: ::core::ffi::c_int = 0;
    GetXineramaScreenCount(&raw mut nxs);
    FixWindowConfiguration(
        pCD,
        &raw mut width,
        &raw mut height,
        (*pCD).widthInc as ::core::ffi::c_uint,
        (*pCD).heightInc as ::core::ffi::c_uint,
    );
    if (*pCD).maxWidth != (*pCD).oldMaxWidth || (*pCD).maxHeight != (*pCD).oldMaxHeight {
        newMax = True as Boolean;
    }
    if (*pCD).maxConfig != 0 {
        if newMax as ::core::ffi::c_int != 0
            && (*pCD).maxWidth as ::core::ffi::c_uint == width
            && (*pCD).maxHeight as ::core::ffi::c_uint == height
        {
            toNewMax = True as Boolean;
        }
        changedValues |= (if width != (*pCD).oldMaxWidth as ::core::ffi::c_uint {
            CWWidth
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint;
        changedValues |= (if height != (*pCD).oldMaxHeight as ::core::ffi::c_uint {
            CWHeight
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint;
        if toNewMax == 0 && changedValues & CWWidth as ::core::ffi::c_uint != 0 {
            if (*pCD).maximumClientSize.height == BIGSIZE
                && (*pCD).maximumClientSize.width == 0 as ::core::ffi::c_int
            {
                (*pCD).maxWidth = width as ::core::ffi::c_int;
            }
            (*pCD).clientWidth = width as ::core::ffi::c_int;
            if changedValues & CWHeight as ::core::ffi::c_uint != 0 {
                if (*pCD).maximumClientSize.width == BIGSIZE
                    && (*pCD).maximumClientSize.height == 0 as ::core::ffi::c_int
                {
                    (*pCD).maxHeight = height as ::core::ffi::c_int;
                }
                (*pCD).clientHeight = height as ::core::ffi::c_int;
            } else {
                (*pCD).clientHeight = (*pCD).oldMaxHeight;
            }
        } else if toNewMax == 0 && changedValues & CWHeight as ::core::ffi::c_uint != 0 {
            if (*pCD).maximumClientSize.width == BIGSIZE
                && (*pCD).maximumClientSize.height == 0 as ::core::ffi::c_int
            {
                (*pCD).maxHeight = height as ::core::ffi::c_int;
            }
            (*pCD).clientHeight = height as ::core::ffi::c_int;
            (*pCD).clientWidth = (*pCD).oldMaxWidth;
        }
    } else {
        if width != (*pCD).clientWidth as ::core::ffi::c_uint {
            if (*pCD).maximumClientSize.height == BIGSIZE
                && (*pCD).maximumClientSize.width == 0 as ::core::ffi::c_int
            {
                (*pCD).maxWidth = width as ::core::ffi::c_int;
            }
            changedValues |= CWWidth as ::core::ffi::c_uint;
            (*pCD).clientWidth = width as ::core::ffi::c_int;
        }
        if height != (*pCD).clientHeight as ::core::ffi::c_uint {
            if (*pCD).maximumClientSize.width == BIGSIZE
                && (*pCD).maximumClientSize.height == 0 as ::core::ffi::c_int
            {
                (*pCD).maxHeight = height as ::core::ffi::c_int;
            }
            changedValues |= CWHeight as ::core::ffi::c_uint;
            (*pCD).clientHeight = height as ::core::ffi::c_int;
        }
    }
    if clientRequest == 0 {
        xoff = (*pCD).clientOffset.x as ::core::ffi::c_int;
        yoff = (*pCD).clientOffset.y as ::core::ffi::c_int;
    }
    if (*pCD).maxConfig != 0 {
        if x != (*pCD).maxX {
            changedValues |= CWX as ::core::ffi::c_uint;
            if toNewMax == 0 && changedValues & (CWWidth | CWHeight) as ::core::ffi::c_uint != 0 {
                (*pCD).clientX = x + xoff;
            } else {
                (*pCD).maxX = x + xoff;
            }
        } else if toNewMax == 0 && changedValues & (CWWidth | CWHeight) as ::core::ffi::c_uint != 0
        {
            (*pCD).clientX = (*pCD).maxX;
        }
        if y != (*pCD).maxY {
            changedValues |= CWY as ::core::ffi::c_uint;
            if toNewMax == 0 && changedValues & (CWWidth | CWHeight) as ::core::ffi::c_uint != 0 {
                (*pCD).clientY = y + yoff;
            } else {
                (*pCD).maxY = y + yoff;
            }
        } else if toNewMax == 0 && changedValues & (CWWidth | CWHeight) as ::core::ffi::c_uint != 0
        {
            (*pCD).clientY = (*pCD).maxY;
        }
    } else {
        if x + xoff != (*pCD).clientX {
            changedValues |= CWX as ::core::ffi::c_uint;
            (*pCD).clientX = x + xoff;
        }
        if y + yoff != (*pCD).clientY {
            changedValues |= CWY as ::core::ffi::c_uint;
            (*pCD).clientY = y + yoff;
        }
        if nxs > 1 as ::core::ffi::c_int
            && toNewMax == 0
            && changedValues & (CWX | CWY) as ::core::ffi::c_uint != 0
        {
            let mut prev_xs = XineramaScreenInfo {
                screen_number: 0,
                x_org: 0,
                y_org: 0,
                width: 0,
                height: 0,
            };
            let mut cur_xs = XineramaScreenInfo {
                screen_number: 0,
                x_org: 0,
                y_org: 0,
                width: 0,
                height: 0,
            };
            if GetXineramaScreenFromLocation(oldX, oldY, &raw mut prev_xs) != 0
                && GetXineramaScreenFromLocation(x, y, &raw mut cur_xs) != 0
                && prev_xs.screen_number != cur_xs.screen_number
            {
                RecomputeMaxConfig(pCD);
            }
        }
    }
    if originallyOnScreen as ::core::ffi::c_int != 0
        && WindowIsOnScreen(pCD, &raw mut dx, &raw mut dy) == 0
    {
        if dx != 0 as ::core::ffi::c_int {
            changedValues |= CWX as ::core::ffi::c_uint;
            (*pCD).clientX += dx;
        }
        if dy != 0 as ::core::ffi::c_int {
            changedValues |= CWY as ::core::ffi::c_uint;
            (*pCD).clientY += dy;
        }
    }
    if changedValues & (CWWidth | CWHeight) as ::core::ffi::c_uint != 0 {
        if (*pCD).maxConfig != 0 {
            if toNewMax == 0 {
                (*pCD).maxConfig = FALSE as Boolean;
                (*pCD).clientState = NORMAL_STATE;
            }
        }
        XResizeWindow(wmGD.display, (*pCD).client, width, height);
        RegenerateClientFrame(pCD);
    }
    if changedValues & (CWX | CWY) as ::core::ffi::c_uint != 0 {
        if (*pCD).maxConfig != 0 {
            if clientRequest != 0 {
                XMoveWindow(
                    wmGD.display,
                    (*pCD).clientFrameWin,
                    (*pCD).maxX - (*pCD).clientOffset.x as ::core::ffi::c_int,
                    (*pCD).maxY - (*pCD).clientOffset.y as ::core::ffi::c_int,
                );
            } else {
                XMoveWindow(
                    wmGD.display,
                    (*pCD).clientFrameWin,
                    (*pCD).maxX - offsetX,
                    (*pCD).maxY - offsetY,
                );
            }
        } else if clientRequest != 0 {
            XMoveWindow(
                wmGD.display,
                (*pCD).clientFrameWin,
                (*pCD).clientX - (*pCD).clientOffset.x as ::core::ffi::c_int,
                (*pCD).clientY - (*pCD).clientOffset.y as ::core::ffi::c_int,
            );
        } else {
            XMoveWindow(
                wmGD.display,
                (*pCD).clientFrameWin,
                (*pCD).clientX - offsetX,
                (*pCD).clientY - offsetY,
            );
        }
        SetFrameInfo(pCD);
    }
    if changedValues == 0 && clientRequest as ::core::ffi::c_int != 0
        || changedValues != 0 && changedValues & (CWWidth | CWHeight) as ::core::ffi::c_uint == 0
    {
        SendConfigureNotify(pCD);
    }
    if (*pCD).clientFlags & ICON_BOX != 0
        && changedValues & (CWWidth | CWHeight) as ::core::ffi::c_uint != 0
    {
        CheckIconBoxResize(
            pCD,
            changedValues,
            width as ::core::ffi::c_int,
            height as ::core::ffi::c_int,
        );
    }
    if changedValues != 0 && (*pCD).tileApplying == 0 {
        (*pCD).tileState = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn StartResizeConfig(
    mut pcd: *mut ClientData,
    mut pev: *mut XEvent,
) -> Boolean {
    let mut grab_win: Window = 0;
    let mut junk_win: Window = 0;
    let mut grabbed: Boolean = 0;
    let mut big_inc: ::core::ffi::c_int = 0;
    let mut tmp_inc: ::core::ffi::c_int = 0;
    let mut junk: ::core::ffi::c_int = 0;
    let mut junkX: ::core::ffi::c_int = 0;
    let mut junkY: ::core::ffi::c_int = 0;
    if configGrab == 0 {
        grab_win = GrabWin(pcd, pev);
        if !pev.is_null() {
            grabbed = DoGrabs(
                grab_win,
                ConfigCursor(wmGD.configPart as ::core::ffi::c_int),
                PGRAB_MASK as ::core::ffi::c_uint,
                (*pev).xbutton.time,
                pcd,
                True as Boolean,
            );
        } else {
            grabbed = DoGrabs(
                grab_win,
                ConfigCursor(wmGD.configPart as ::core::ffi::c_int),
                PGRAB_MASK as ::core::ffi::c_uint,
                CurrentTime as Time,
                pcd,
                True as Boolean,
            );
        }
        if grabbed == 0 {
            return 0 as Boolean;
        }
        configGrab = TRUE as Boolean;
    } else {
        return 1 as Boolean;
    }
    if XQueryPointer(
        wmGD.display,
        (*(*pcd).pSD).rootWindow,
        &raw mut junk_win,
        &raw mut junk_win,
        &raw mut pointerX,
        &raw mut pointerY,
        &raw mut junk,
        &raw mut junk,
        &raw mut junk as *mut ::core::ffi::c_uint,
    ) == 0
    {
        CancelFrameConfig(pcd);
        return 0 as Boolean;
    }
    wmGD.preMoveX = pointerX;
    wmGD.preMoveY = pointerY;
    anyMotion = FALSE as Boolean;
    offsetX = (*pcd).clientOffset.x as ::core::ffi::c_int;
    offsetY = (*pcd).clientOffset.y as ::core::ffi::c_int;
    if (*pcd).maxConfig != 0 {
        resizeX = (*pcd).maxX;
        resizeY = (*pcd).maxY;
        resizeWidth = (*pcd).maxWidth as ::core::ffi::c_uint;
        resizeHeight = (*pcd).maxHeight as ::core::ffi::c_uint;
    } else {
        resizeX = (*pcd).clientX;
        resizeY = (*pcd).clientY;
        resizeWidth = (*pcd).clientWidth as ::core::ffi::c_uint;
        resizeHeight = (*pcd).clientHeight as ::core::ffi::c_uint;
    }
    ClientToFrame(
        pcd,
        &raw mut resizeX,
        &raw mut resizeY,
        &raw mut resizeWidth,
        &raw mut resizeHeight,
    );
    startX = resizeX;
    startY = resizeY;
    startWidth = resizeWidth;
    startHeight = resizeHeight;
    minWidth = (*pcd).minWidth as ::core::ffi::c_uint;
    minHeight = (*pcd).minHeight as ::core::ffi::c_uint;
    junkY = 0 as ::core::ffi::c_int;
    junkX = junkY;
    ClientToFrame(
        pcd,
        &raw mut junkX,
        &raw mut junkY,
        &raw mut minWidth,
        &raw mut minHeight,
    );
    maxWidth = (*pcd).maxWidthLimit as ::core::ffi::c_uint;
    maxHeight = (*pcd).maxHeightLimit as ::core::ffi::c_uint;
    junkY = 0 as ::core::ffi::c_int;
    junkX = junkY;
    ClientToFrame(
        pcd,
        &raw mut junkX,
        &raw mut junkY,
        &raw mut maxWidth,
        &raw mut maxHeight,
    );
    big_inc = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*pcd).pSD).screen as isize))
    .width
        / 20 as ::core::ffi::c_int;
    tmp_inc = big_inc - big_inc % (*pcd).widthInc;
    if tmp_inc > 5 as ::core::ffi::c_int * (*pcd).widthInc {
        resizeBigWidthInc = tmp_inc as ::core::ffi::c_uint;
    } else {
        resizeBigWidthInc = (5 as ::core::ffi::c_int * (*pcd).widthInc) as ::core::ffi::c_uint;
    }
    tmp_inc = big_inc - big_inc % (*pcd).heightInc;
    if tmp_inc > 5 as ::core::ffi::c_int * (*pcd).heightInc {
        resizeBigHeightInc = tmp_inc as ::core::ffi::c_uint;
    } else {
        resizeBigHeightInc = (5 as ::core::ffi::c_int * (*pcd).heightInc) as ::core::ffi::c_uint;
    }
    if wmGD.showFeedback as ::core::ffi::c_long & WM_SHOW_FB_RESIZE != 0 {
        DoFeedback(
            pcd,
            resizeX,
            resizeY,
            resizeWidth,
            resizeHeight,
            FB_SIZE as ::core::ffi::c_ulong,
            TRUE as Boolean,
        );
    }
    wmGD.configAction = RESIZE_CLIENT;
    wmGD.configButton = if !pev.is_null() {
        (*pev).xbutton.button
    } else {
        0 as ::core::ffi::c_uint
    };
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn StartClientResize(mut pcd: *mut ClientData, mut pev: *mut XEvent) {
    wmGD.configPart = FRAME_NONE as ::core::ffi::c_uint;
    wmGD.configSet = False as Boolean;
    if StartResizeConfig(pcd, pev) == 0 {
        return;
    }
    if pev.is_null() || (*pev).type_0 == KeyPress {
        pointerX = (resizeX as ::core::ffi::c_uint)
            .wrapping_add(resizeWidth.wrapping_div(2 as ::core::ffi::c_uint))
            as ::core::ffi::c_int;
        pointerY = (resizeY as ::core::ffi::c_uint)
            .wrapping_add(resizeHeight.wrapping_div(2 as ::core::ffi::c_uint))
            as ::core::ffi::c_int;
        ForceOnScreen((*(*pcd).pSD).screen, &raw mut pointerX, &raw mut pointerY);
        if wmGD.enableWarp != 0 {
            XWarpPointer(
                wmGD.display,
                None_0 as Window,
                (*(*pcd).pSD).rootWindow,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_uint,
                0 as ::core::ffi::c_uint,
                pointerX,
                pointerY,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn StartClientMove(
    mut pcd: *mut ClientData,
    mut pev: *mut XEvent,
) -> Boolean {
    let mut grab_win: Window = 0;
    let mut junk_win: Window = 0;
    let mut grabbed: Boolean = 0;
    let mut junk: ::core::ffi::c_int = 0;
    let mut child: Window = 0;
    if configGrab == 0 {
        grab_win = GrabWin(pcd, pev);
        if grab_win == (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin {
            wmGD.movingIcon = True as Boolean;
        }
        if !pev.is_null() {
            grabbed = DoGrabs(
                grab_win,
                wmGD.configCursor,
                PGRAB_MASK as ::core::ffi::c_uint,
                (*pev).xbutton.time,
                pcd,
                False as Boolean,
            );
        } else {
            grabbed = DoGrabs(
                grab_win,
                wmGD.configCursor,
                PGRAB_MASK as ::core::ffi::c_uint,
                CurrentTime as Time,
                pcd,
                False as Boolean,
            );
        }
        if grabbed == 0 {
            wmGD.movingIcon = False as Boolean;
            return 0 as Boolean;
        }
        configGrab = TRUE as Boolean;
    }
    if !pev.is_null() && ((*pev).type_0 == ButtonPress || (*pev).type_0 == ButtonRelease) {
        wmGD.preMoveX = (*pev).xbutton.x_root;
        wmGD.preMoveY = (*pev).xbutton.y_root;
    } else if !pev.is_null() && (*pev).type_0 != MotionNotify || pev.is_null() {
        if XQueryPointer(
            wmGD.display,
            (*(*pcd).pSD).rootWindow,
            &raw mut junk_win,
            &raw mut junk_win,
            &raw mut wmGD.preMoveX,
            &raw mut wmGD.preMoveY,
            &raw mut junk,
            &raw mut junk,
            &raw mut junk as *mut ::core::ffi::c_uint,
        ) == 0
        {
            CancelFrameConfig(pcd);
            return 0 as Boolean;
        }
    }
    offsetX = (*pcd).clientOffset.x as ::core::ffi::c_int;
    offsetY = (*pcd).clientOffset.y as ::core::ffi::c_int;
    anyMotion = FALSE as Boolean;
    moveLastPointerX = wmGD.preMoveX;
    moveLastPointerY = wmGD.preMoveY;
    if wmGD.movingIcon != 0 {
        moveWidth = (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint;
        moveHeight = (*(*pcd).pSD).iconHeight as ::core::ffi::c_uint;
        moveX = (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconX;
        moveY = (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconY;
        if (*(*pcd).pSD).useIconBox as ::core::ffi::c_int != 0
            && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                .pIconBox
                .is_null()
        {
            XTranslateCoordinates(
                wmGD.display,
                XtWindowOfObject(
                    (*(*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).pIconBox).bBoardWidget,
                ),
                (*(*pcd).pSD).rootWindow,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                &raw mut moveIBbbX,
                &raw mut moveIBbbY,
                &raw mut child,
            );
            moveX += moveIBbbX;
            moveY += moveIBbbY;
        } else if (*(*pcd).pSD).moveOpaque as ::core::ffi::c_int != 0
            && (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART != 0
            && wmGD.keyboardFocus == pcd
        {
            HideActiveIconText(NULL as *mut WmScreenData);
        }
    } else {
        if (*pcd).maxConfig != 0 {
            moveWidth = (*pcd).maxWidth as ::core::ffi::c_uint;
            moveHeight = (*pcd).maxHeight as ::core::ffi::c_uint;
            moveX = (*pcd).maxX;
            moveY = (*pcd).maxY;
        } else {
            moveWidth = (*pcd).clientWidth as ::core::ffi::c_uint;
            moveHeight = (*pcd).clientHeight as ::core::ffi::c_uint;
            moveX = (*pcd).clientX;
            moveY = (*pcd).clientY;
        }
        ClientToFrame(
            pcd,
            &raw mut moveX,
            &raw mut moveY,
            &raw mut moveWidth,
            &raw mut moveHeight,
        );
    }
    if (*(*pcd).pSD).moveOpaque != 0 {
        opaqueMoveX = moveX;
        opaqueMoveY = moveY;
    }
    if pev.is_null() || (*pev).type_0 == KeyPress {
        moveLastPointerX = (moveX as ::core::ffi::c_uint)
            .wrapping_add(moveWidth.wrapping_div(2 as ::core::ffi::c_uint))
            as ::core::ffi::c_int;
        moveLastPointerY = (moveY as ::core::ffi::c_uint)
            .wrapping_add(moveHeight.wrapping_div(2 as ::core::ffi::c_uint))
            as ::core::ffi::c_int;
        ForceOnScreen(
            (*(*pcd).pSD).screen,
            &raw mut moveLastPointerX,
            &raw mut moveLastPointerY,
        );
        if wmGD.enableWarp != 0 {
            XWarpPointer(
                wmGD.display,
                None_0 as Window,
                (*(*pcd).pSD).rootWindow,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_uint,
                0 as ::core::ffi::c_uint,
                moveLastPointerX,
                moveLastPointerY,
            );
        }
    }
    if wmGD.movingIcon == 0 && wmGD.showFeedback as ::core::ffi::c_long & WM_SHOW_FB_MOVE != 0 {
        DoFeedback(
            pcd,
            moveX,
            moveY,
            moveWidth,
            moveHeight,
            FB_POSITION as ::core::ffi::c_ulong,
            FALSE as Boolean,
        );
    }
    wmGD.configAction = MOVE_CLIENT;
    if !pev.is_null() && (*pev).type_0 != KeyPress {
        wmGD.configButton = (*pev).xbutton.button;
    } else {
        wmGD.configButton = 0 as ::core::ffi::c_uint;
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn DoGrabs(
    mut grab_win: Window,
    mut cursor: Cursor,
    mut pmask: ::core::ffi::c_uint,
    mut grabTime: Time,
    mut pCD: *mut ClientData,
    mut alwaysGrab: Boolean,
) -> Boolean {
    let mut root: Window = 0;
    if !pCD.is_null() {
        root = (*(*pCD).pSD).rootWindow;
    } else {
        root = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*wmGD.pActiveSD).screen as isize))
        .root;
    }
    if !pCD.is_null()
        && (*(*pCD).pSD).useIconBox as ::core::ffi::c_int != 0
        && wmGD.movingIcon as ::core::ffi::c_int != 0
        && !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
            .pIconBox
            .is_null()
    {
        if XGrabPointer(
            wmGD.display,
            grab_win,
            FALSE,
            pmask,
            GrabModeAsync,
            GrabModeAsync,
            XtWindowOfObject(
                (*(*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox).clipWidget,
            ),
            cursor,
            grabTime,
        ) != GrabSuccess
        {
            return 0 as Boolean;
        }
    } else if XGrabPointer(
        wmGD.display,
        grab_win,
        FALSE,
        pmask,
        GrabModeAsync,
        GrabModeAsync,
        root,
        cursor,
        grabTime,
    ) != GrabSuccess
    {
        return 0 as Boolean;
    }
    if wmGD.menuActive.is_null() {
        if XGrabKeyboard(
            wmGD.display,
            grab_win,
            FALSE,
            GrabModeAsync,
            GrabModeAsync,
            grabTime,
        ) != GrabSuccess
        {
            XUngrabPointer(wmGD.display, CurrentTime as Time);
            return 0 as Boolean;
        }
    }
    if wmGD.useWindowOutline == 0 {
        if pCD.is_null()
            || ((*(*pCD).pSD).moveOpaque as ::core::ffi::c_int != 0
                && alwaysGrab as ::core::ffi::c_int != 0
                || (*(*pCD).pSD).moveOpaque == 0)
        {
            XGrabServer(wmGD.display);
        }
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn UndoGrabs() {
    MoveOutline(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
    );
    XSync(wmGD.display, FALSE);
    if wmGD.useWindowOutline == 0 {
        XUngrabServer(wmGD.display);
    }
    if wmGD.menuActive.is_null() {
        XUngrabKeyboard(wmGD.display, CurrentTime as Time);
    }
    XUngrabPointer(wmGD.display, CurrentTime as Time);
    XFlush(wmGD.display);
}
#[no_mangle]
pub unsafe extern "C" fn CancelFrameConfig(mut pcd: *mut ClientData) {
    UndoGrabs();
    if !pcd.is_null() {
        HideFeedbackWindow((*pcd).pSD);
        if wmGD.configAction == MOVE_CLIENT
            && wmGD.gadgetClient == pcd
            && wmGD.gadgetDepressed == FRAME_TITLE
        {
            PopGadgetOut(pcd, FRAME_TITLE);
            FrameExposureProc(pcd);
        }
        if (*(*pcd).pSD).moveOpaque as ::core::ffi::c_int != 0 && wmGD.configAction == MOVE_CLIENT {
            if (*pcd).clientState == MINIMIZED_STATE
                && !((*(*pcd).pSD).useIconBox as ::core::ffi::c_int != 0
                    && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                        .pIconBox
                        .is_null())
            {
                XMoveWindow(
                    wmGD.display,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconX,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconY,
                );
                if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART != 0
                {
                    ShowActiveIconText(pcd);
                }
            } else if wmGD.movingIcon == 0 {
                XMoveWindow(
                    wmGD.display,
                    (*pcd).clientFrameWin,
                    opaqueMoveX,
                    opaqueMoveY,
                );
            }
        }
    }
    wmGD.configAction == MARQUEE_SELECT;
    if !pcd.is_null() && anyMotion == 0 && wmGD.enableWarp as ::core::ffi::c_int != 0 {
        XWarpPointer(
            wmGD.display,
            None_0 as Window,
            (*(*pcd).pSD).rootWindow,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            wmGD.preMoveX,
            wmGD.preMoveY,
        );
    }
    anyMotion = FALSE as Boolean;
    wmGD.configAction = NO_ACTION;
    wmGD.configPart = FRAME_NONE as ::core::ffi::c_uint;
    wmGD.configSet = False as Boolean;
    configGrab = FALSE as Boolean;
    wmGD.movingIcon = FALSE as Boolean;
    RepairFocus();
}
#[no_mangle]
pub unsafe extern "C" fn CheckEatButtonRelease(mut pcd: *mut ClientData, mut pev: *mut XEvent) {
    let mut grab_win: Window = 0;
    let mut root: Window = 0;
    if !pcd.is_null() {
        root = (*(*pcd).pSD).rootWindow;
    } else {
        root = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*wmGD.pActiveSD).screen as isize))
        .root;
    }
    if pcd.is_null() {
        grab_win = root;
    } else {
        grab_win = GrabWin(pcd, pev);
    }
    if ((*pev).type_0 == KeyPress || (*pev).type_0 == KeyRelease)
        && (*pev).xbutton.state & ButtonMask as ::core::ffi::c_uint != 0
    {
        if XGrabPointer(
            wmGD.display,
            grab_win,
            False,
            ButtonReleaseMask as ::core::ffi::c_uint,
            GrabModeAsync,
            GrabModeAsync,
            root,
            wmGD.configCursor,
            (*pev).xbutton.time,
        ) == GrabSuccess
        {
            EatButtonRelease((*pev).xbutton.state & ButtonMask as ::core::ffi::c_uint);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn EatButtonRelease(mut releaseButtons: ::core::ffi::c_uint) {
    let mut new_state: ::core::ffi::c_uint = 0;
    let mut event = _XEvent { type_0: 0 };
    while releaseButtons != 0 {
        PullExposureEvents();
        XMaskEvent(wmGD.display, ButtonReleaseMask, &raw mut event);
        if event.type_0 == ButtonRelease {
            new_state = event.xbutton.state & !ButtonStateBit(event.xbutton.button);
            if new_state & releaseButtons == 0 {
                XUngrabPointer(wmGD.display, event.xbutton.time);
                releaseButtons = 0 as ::core::ffi::c_uint;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ButtonStateBit(mut button: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    static mut bmap: [ButtonAssoc; 5] = [
        ButtonAssoc {
            button: Button1 as ::core::ffi::c_uint,
            maskbit: Button1Mask as ::core::ffi::c_uint,
        },
        ButtonAssoc {
            button: Button2 as ::core::ffi::c_uint,
            maskbit: Button2Mask as ::core::ffi::c_uint,
        },
        ButtonAssoc {
            button: Button3 as ::core::ffi::c_uint,
            maskbit: Button3Mask as ::core::ffi::c_uint,
        },
        ButtonAssoc {
            button: Button4 as ::core::ffi::c_uint,
            maskbit: Button4Mask as ::core::ffi::c_uint,
        },
        ButtonAssoc {
            button: Button5 as ::core::ffi::c_uint,
            maskbit: Button5Mask as ::core::ffi::c_uint,
        },
    ];
    let mut i: ::core::ffi::c_int = 0;
    let mut rval = 0 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    while i < MAX_BUTTON {
        if bmap[i as usize].button == button {
            rval = bmap[i as usize].maskbit;
            break;
        } else {
            i += 1;
        }
    }
    return rval;
}
pub const MAX_BUTTON: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ConfigCursor(mut frame_part: ::core::ffi::c_int) -> Cursor {
    let mut cursor: Cursor = 0;
    match frame_part {
        FRAME_RESIZE_NW => {
            cursor = wmGD.stretchCursors[STRETCH_NORTH_WEST as usize];
        }
        FRAME_RESIZE_N => {
            cursor = wmGD.stretchCursors[STRETCH_NORTH as usize];
        }
        FRAME_RESIZE_NE => {
            cursor = wmGD.stretchCursors[STRETCH_NORTH_EAST as usize];
        }
        FRAME_RESIZE_E => {
            cursor = wmGD.stretchCursors[STRETCH_EAST as usize];
        }
        FRAME_RESIZE_SE => {
            cursor = wmGD.stretchCursors[STRETCH_SOUTH_EAST as usize];
        }
        FRAME_RESIZE_S => {
            cursor = wmGD.stretchCursors[STRETCH_SOUTH as usize];
        }
        FRAME_RESIZE_SW => {
            cursor = wmGD.stretchCursors[STRETCH_SOUTH_WEST as usize];
        }
        FRAME_RESIZE_W => {
            cursor = wmGD.stretchCursors[STRETCH_WEST as usize];
        }
        _ => {
            cursor = wmGD.configCursor;
        }
    }
    return cursor;
}
#[no_mangle]
pub unsafe extern "C" fn ReGrabPointer(mut grab_win: Window, mut grabTime: Time) {
    XGrabPointer(
        wmGD.display,
        grab_win,
        FALSE,
        PGRAB_MASK as ::core::ffi::c_uint,
        GrabModeAsync,
        GrabModeAsync,
        (*wmGD.pActiveSD).rootWindow,
        ConfigCursor(wmGD.configPart as ::core::ffi::c_int),
        grabTime,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SetPointerResizePart(
    mut pcd: *mut ClientData,
    mut pev: *mut XEvent,
) -> Boolean {
    let mut newPart: ::core::ffi::c_int = 0;
    let mut grabTime: Time = 0;
    newPart = ResizeType(pcd, pev);
    grabTime = if !pev.is_null() {
        (*pev).xmotion.time
    } else {
        CurrentTime as Time
    };
    match wmGD.configPart {
        0 => {
            if newPart == FRAME_NONE {
                return 0 as Boolean;
            }
            wmGD.configPart = newPart as ::core::ffi::c_uint;
            ReGrabPointer((*pcd).clientFrameWin, grabTime);
            return 1 as Boolean;
        }
        7 => match newPart {
            FRAME_RESIZE_W | FRAME_RESIZE_NW => {
                wmGD.configPart = FRAME_RESIZE_NW as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, grabTime);
            }
            FRAME_RESIZE_E | FRAME_RESIZE_NE => {
                wmGD.configPart = FRAME_RESIZE_NE as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, grabTime);
            }
            _ => {}
        },
        9 => match newPart {
            FRAME_RESIZE_N | FRAME_RESIZE_NE => {
                wmGD.configPart = FRAME_RESIZE_NE as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, grabTime);
            }
            FRAME_RESIZE_S | FRAME_RESIZE_SE => {
                wmGD.configPart = FRAME_RESIZE_SE as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, grabTime);
            }
            _ => {}
        },
        11 => match newPart {
            FRAME_RESIZE_E | FRAME_RESIZE_SE => {
                wmGD.configPart = FRAME_RESIZE_SE as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, grabTime);
            }
            FRAME_RESIZE_W | FRAME_RESIZE_SW => {
                wmGD.configPart = FRAME_RESIZE_SW as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, grabTime);
            }
            _ => {}
        },
        13 => match newPart {
            FRAME_RESIZE_N | FRAME_RESIZE_NW => {
                wmGD.configPart = FRAME_RESIZE_NW as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, grabTime);
            }
            FRAME_RESIZE_S | FRAME_RESIZE_SW => {
                wmGD.configPart = FRAME_RESIZE_SW as ::core::ffi::c_uint;
                ReGrabPointer((*pcd).clientFrameWin, grabTime);
            }
            _ => {}
        },
        6 | 8 | 12 | 10 => {}
        _ => return 0 as Boolean,
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ResizeType(
    mut pcd: *mut ClientData,
    mut pev: *mut XEvent,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    if pev.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    x = (*pev).xmotion.x_root;
    y = (*pev).xmotion.y_root;
    if x > resizeX
        && y > resizeY
        && (x as ::core::ffi::c_uint)
            < (resizeX as ::core::ffi::c_uint)
                .wrapping_add(resizeWidth)
                .wrapping_sub(1 as ::core::ffi::c_uint)
        && (y as ::core::ffi::c_uint)
            < (resizeY as ::core::ffi::c_uint)
                .wrapping_add(resizeHeight)
                .wrapping_sub(1 as ::core::ffi::c_uint)
    {
        return 0 as ::core::ffi::c_int;
    }
    if x <= resizeX {
        if y < resizeY + (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int {
            return 6 as ::core::ffi::c_int;
        } else if y as ::core::ffi::c_uint
            >= (resizeY as ::core::ffi::c_uint)
                .wrapping_add(resizeHeight)
                .wrapping_sub(
                    (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int as ::core::ffi::c_uint,
                )
        {
            return 12 as ::core::ffi::c_int;
        } else {
            return 13 as ::core::ffi::c_int;
        }
    }
    if x as ::core::ffi::c_uint
        >= (resizeX as ::core::ffi::c_uint)
            .wrapping_add(resizeWidth)
            .wrapping_sub(1 as ::core::ffi::c_uint)
    {
        if y < resizeY + (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int {
            return 8 as ::core::ffi::c_int;
        } else if y as ::core::ffi::c_uint
            >= (resizeY as ::core::ffi::c_uint)
                .wrapping_add(resizeHeight)
                .wrapping_sub(
                    (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int as ::core::ffi::c_uint,
                )
        {
            return 10 as ::core::ffi::c_int;
        } else {
            return 9 as ::core::ffi::c_int;
        }
    }
    if y <= resizeY {
        if x < resizeX + (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int {
            return 6 as ::core::ffi::c_int;
        } else if x as ::core::ffi::c_uint
            >= (resizeX as ::core::ffi::c_uint)
                .wrapping_add(resizeWidth)
                .wrapping_sub(
                    (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int as ::core::ffi::c_uint,
                )
        {
            return 8 as ::core::ffi::c_int;
        } else {
            return 7 as ::core::ffi::c_int;
        }
    }
    if y as ::core::ffi::c_uint
        >= (resizeY as ::core::ffi::c_uint)
            .wrapping_add(resizeHeight)
            .wrapping_sub(1 as ::core::ffi::c_uint)
    {
        if x < resizeX + (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int {
            return 12 as ::core::ffi::c_int;
        } else if x as ::core::ffi::c_uint
            >= (resizeX as ::core::ffi::c_uint)
                .wrapping_add(resizeWidth)
                .wrapping_sub(
                    (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int as ::core::ffi::c_uint,
                )
        {
            return 10 as ::core::ffi::c_int;
        } else {
            return 11 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FixFrameValues(
    mut pcd: *mut ClientData,
    mut pfX: *mut ::core::ffi::c_int,
    mut pfY: *mut ::core::ffi::c_int,
    mut pfWidth: *mut ::core::ffi::c_uint,
    mut pfHeight: *mut ::core::ffi::c_uint,
    mut resizing: Boolean,
) {
    let mut lswidth: ::core::ffi::c_uint = 0;
    let mut oWidth: ::core::ffi::c_uint = 0;
    let mut oHeight: ::core::ffi::c_uint = 0;
    if resizing as ::core::ffi::c_int != 0 && wmGD.movingIcon == 0 {
        FrameToClient(pcd, pfX, pfY, pfWidth, pfHeight);
        oWidth = *pfWidth;
        oHeight = *pfHeight;
        FixWindowSize(
            pcd,
            pfWidth,
            pfHeight,
            1 as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
        );
        AdjustPos(pfX, pfY, oWidth, oHeight, *pfWidth, *pfHeight);
        ClientToFrame(pcd, pfX, pfY, pfWidth, pfHeight);
    }
    if wmGD.movingIcon != 0 {
        lswidth = (*(*pcd).pSD).frameBorderWidth as ::core::ffi::c_uint;
    } else {
        lswidth = (*pcd).frameInfo.lowerBorderWidth;
    }
    if lswidth < 5 as ::core::ffi::c_uint {
        lswidth = 5 as ::core::ffi::c_uint;
    }
    if wmGD.movingIcon as ::core::ffi::c_int != 0
        && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
            .pIconBox
            .is_null()
    {
        if *pfX < clipX as ::core::ffi::c_int {
            *pfX = clipX as ::core::ffi::c_int;
        }
        if *pfY < clipY as ::core::ffi::c_int {
            *pfY = clipY as ::core::ffi::c_int;
        }
        if *pfX
            > clipX as ::core::ffi::c_int + clipWidth as ::core::ffi::c_int
                - *pfWidth as ::core::ffi::c_int
        {
            *pfX = ((clipX as ::core::ffi::c_int + clipWidth as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_sub(*pfWidth) as ::core::ffi::c_int;
        }
        if *pfY
            > clipY as ::core::ffi::c_int + clipHeight as ::core::ffi::c_int
                - *pfHeight as ::core::ffi::c_int
        {
            *pfY = ((clipY as ::core::ffi::c_int + clipHeight as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_sub(*pfHeight) as ::core::ffi::c_int;
        }
    } else {
        if *pfX < lswidth as ::core::ffi::c_int - *pfWidth as ::core::ffi::c_int {
            *pfX = lswidth as ::core::ffi::c_int - *pfWidth as ::core::ffi::c_int;
        }
        if *pfY < lswidth as ::core::ffi::c_int - *pfHeight as ::core::ffi::c_int {
            *pfY = lswidth as ::core::ffi::c_int - *pfHeight as ::core::ffi::c_int;
        }
        if *pfX
            > (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(*pcd).pSD).screen as isize))
            .width
                - lswidth as ::core::ffi::c_int
        {
            *pfX = (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(*pcd).pSD).screen as isize))
            .width
                - lswidth as ::core::ffi::c_int;
        }
        if *pfY
            > (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(*pcd).pSD).screen as isize))
            .height
                - lswidth as ::core::ffi::c_int
        {
            *pfY = (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(*pcd).pSD).screen as isize))
            .height
                - lswidth as ::core::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ForceOnScreen(
    mut screen: ::core::ffi::c_int,
    mut pX: *mut ::core::ffi::c_int,
    mut pY: *mut ::core::ffi::c_int,
) {
    if *pX
        >= (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset(screen as isize))
        .width
    {
        *pX = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset(screen as isize))
        .width
            - 1 as ::core::ffi::c_int;
    } else if *pX < 0 as ::core::ffi::c_int {
        *pX = 0 as ::core::ffi::c_int;
    }
    if *pY
        >= (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset(screen as isize))
        .height
    {
        *pY = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset(screen as isize))
        .height
            - 1 as ::core::ffi::c_int;
    } else if *pY < 0 as ::core::ffi::c_int {
        *pY = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetPointerPosition(
    mut newX: ::core::ffi::c_int,
    mut newY: ::core::ffi::c_int,
    mut actualX: *mut ::core::ffi::c_int,
    mut actualY: *mut ::core::ffi::c_int,
) {
    let mut junk: ::core::ffi::c_int = 0;
    let mut junk_win: Window = 0;
    if wmGD.enableWarp != 0 {
        XWarpPointer(
            wmGD.display,
            None_0 as Window,
            (*wmGD.pActiveSD).rootWindow,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            newX,
            newY,
        );
    }
    if wmGD.enableWarp == 0
        || XQueryPointer(
            wmGD.display,
            (*wmGD.pActiveSD).rootWindow,
            &raw mut junk_win,
            &raw mut junk_win,
            actualX,
            actualY,
            &raw mut junk,
            &raw mut junk,
            &raw mut junk as *mut ::core::ffi::c_uint,
        ) == 0
    {
        *actualX = newX;
        *actualY = newY;
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetConfigEvent(
    mut display: *mut Display,
    mut window: Window,
    mut mask: ::core::ffi::c_ulong,
    mut curX: ::core::ffi::c_int,
    mut curY: ::core::ffi::c_int,
    mut oX: ::core::ffi::c_int,
    mut oY: ::core::ffi::c_int,
    mut oWidth: ::core::ffi::c_uint,
    mut oHeight: ::core::ffi::c_uint,
    mut pev: *mut XEvent,
) {
    let mut root_ret: Window = 0;
    let mut child_ret: Window = 0;
    let mut root_x: ::core::ffi::c_int = 0;
    let mut root_y: ::core::ffi::c_int = 0;
    let mut win_x: ::core::ffi::c_int = 0;
    let mut win_y: ::core::ffi::c_int = 0;
    let mut mask_ret: ::core::ffi::c_uint = 0;
    let mut gotEvent: Boolean = 0;
    let mut eventToReturn = False as Boolean;
    while eventToReturn == 0 {
        gotEvent = False as Boolean;
        while XCheckWindowEvent(display, window, mask as ::core::ffi::c_long, pev) != 0 {
            gotEvent = True as Boolean;
            if (*pev).type_0 != MotionNotify {
                break;
            }
        }
        if gotEvent == 0 {
            XWindowEvent(display, window, mask as ::core::ffi::c_long, pev);
            gotEvent = True as Boolean;
        }
        if gotEvent != 0 {
            eventToReturn = True as Boolean;
            if (*pev).type_0 == MotionNotify
                && (*pev).xmotion.is_hint as ::core::ffi::c_int == NotifyHint
            {
                if XQueryPointer(
                    display,
                    window,
                    &raw mut root_ret,
                    &raw mut child_ret,
                    &raw mut root_x,
                    &raw mut root_y,
                    &raw mut win_x,
                    &raw mut win_y,
                    &raw mut mask_ret,
                ) != 0
                    && (root_x != curX || root_y != curY)
                {
                    (*pev).xmotion.window = root_ret;
                    (*pev).xmotion.subwindow = child_ret;
                    (*pev).xmotion.x = root_x;
                    (*pev).xmotion.y = root_y;
                    (*pev).xmotion.x_root = root_x;
                    (*pev).xmotion.y_root = root_y;
                } else {
                    eventToReturn = False as Boolean;
                    curX += 1;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetOutline(
    mut pOutline: *mut XSegment,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
    mut fatness: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < fatness {
        (*pOutline).x1 = x as ::core::ffi::c_short;
        (*pOutline).y1 = y as ::core::ffi::c_short;
        (*pOutline).x2 = (x as ::core::ffi::c_uint)
            .wrapping_add(width)
            .wrapping_sub(1 as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
        let fresh0 = pOutline;
        pOutline = pOutline.offset(1);
        (*fresh0).y2 = y as ::core::ffi::c_short;
        (*pOutline).x1 = (x as ::core::ffi::c_uint)
            .wrapping_add(width)
            .wrapping_sub(1 as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
        (*pOutline).y1 = y as ::core::ffi::c_short;
        (*pOutline).x2 = (x as ::core::ffi::c_uint)
            .wrapping_add(width)
            .wrapping_sub(1 as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
        let fresh1 = pOutline;
        pOutline = pOutline.offset(1);
        (*fresh1).y2 = (y as ::core::ffi::c_uint)
            .wrapping_add(height)
            .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_short;
        (*pOutline).x1 = (x as ::core::ffi::c_uint)
            .wrapping_add(width)
            .wrapping_sub(1 as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
        (*pOutline).y1 = (y as ::core::ffi::c_uint)
            .wrapping_add(height)
            .wrapping_sub(1 as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
        (*pOutline).x2 = x as ::core::ffi::c_short;
        let fresh2 = pOutline;
        pOutline = pOutline.offset(1);
        (*fresh2).y2 = (y as ::core::ffi::c_uint)
            .wrapping_add(height)
            .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_short;
        (*pOutline).x1 = x as ::core::ffi::c_short;
        (*pOutline).y1 = (y as ::core::ffi::c_uint)
            .wrapping_add(height)
            .wrapping_sub(1 as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
        (*pOutline).x2 = x as ::core::ffi::c_short;
        let fresh3 = pOutline;
        pOutline = pOutline.offset(1);
        (*fresh3).y2 = y as ::core::ffi::c_short;
        x += 1 as ::core::ffi::c_int;
        y += 1 as ::core::ffi::c_int;
        width = width.wrapping_sub(2 as ::core::ffi::c_uint);
        height = height.wrapping_sub(2 as ::core::ffi::c_uint);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn AdjustPos(
    mut pX: *mut ::core::ffi::c_int,
    mut pY: *mut ::core::ffi::c_int,
    mut oWidth: ::core::ffi::c_uint,
    mut oHeight: ::core::ffi::c_uint,
    mut nWidth: ::core::ffi::c_uint,
    mut nHeight: ::core::ffi::c_uint,
) {
    match wmGD.configPart {
        6 => {
            *pX = (*pX as ::core::ffi::c_uint).wrapping_add(oWidth.wrapping_sub(nWidth))
                as ::core::ffi::c_int as ::core::ffi::c_int;
            *pY = (*pY as ::core::ffi::c_uint).wrapping_add(oHeight.wrapping_sub(nHeight))
                as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        7 => {
            *pY = (*pY as ::core::ffi::c_uint).wrapping_add(oHeight.wrapping_sub(nHeight))
                as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        8 => {
            *pY = (*pY as ::core::ffi::c_uint).wrapping_add(oHeight.wrapping_sub(nHeight))
                as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        9 => {}
        10 => {}
        12 => {
            *pX = (*pX as ::core::ffi::c_uint).wrapping_add(oWidth.wrapping_sub(nWidth))
                as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        13 => {
            *pX = (*pX as ::core::ffi::c_uint).wrapping_add(oWidth.wrapping_sub(nWidth))
                as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        11 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn GrabWin(mut pcd: *mut ClientData, mut pev: *mut XEvent) -> Window {
    let mut grab_win: Window = 0;
    if (*pcd).clientState == MINIMIZED_STATE
        || (*(*pcd).pSD).useIconBox as ::core::ffi::c_int != 0
            && !pev.is_null()
            && ((*pev).xany.window
                == (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin
                || (*pev).xany.window == (*wmGD.pActiveSD).activeIconTextWin)
    {
        grab_win = (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin;
    } else if !pev.is_null()
        && ((*pev).xany.window == (*pcd).clientFrameWin
            || (*pev).xany.window == (*pcd).clientBaseWin)
    {
        grab_win = (*pcd).clientFrameWin;
    } else if (*(*pcd).pSD).useIconBox as ::core::ffi::c_int != 0
        && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
            .pIconBox
            .is_null()
        && wmGD.grabContext == F_SUBCONTEXT_IB_WICON as Context
    {
        grab_win = (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin;
    } else {
        grab_win = (*pcd).clientFrameWin;
    }
    return grab_win;
}
#[no_mangle]
pub unsafe extern "C" fn HandleMarqueeSelect(mut pSD: *mut WmScreenData, mut pev: *mut XEvent) {
    let mut grab_win: Window = 0;
    let mut bDone: Boolean = 0;
    let mut event = _XEvent { type_0: 0 };
    grab_win = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*pSD).screen as isize))
    .root;
    bDone = False as Boolean;
    while bDone == 0 && wmGD.configAction == MARQUEE_SELECT {
        if pev.is_null() {
            pev = &raw mut event;
            GetConfigEvent(
                wmGD.display,
                grab_win,
                CONFIG_MASK as ::core::ffi::c_ulong,
                pointerX,
                pointerY,
                resizeX,
                resizeY,
                resizeWidth,
                resizeHeight,
                &raw mut event,
            );
        }
        if (*pev).type_0 == MotionNotify {
            pointerX = (*pev).xmotion.x_root;
            pointerY = (*pev).xmotion.y_root;
            UpdateMarqueeSelectData(pSD);
        } else if (*pev).type_0 == KeyPress {
            bDone = HandleMarqueeKeyPress(pSD, pev);
        } else if (*pev).type_0 == ButtonRelease {
            pointerX = (*pev).xbutton.x_root;
            pointerY = (*pev).xbutton.y_root;
            UpdateMarqueeSelectData(pSD);
            CompleteFrameConfig(NULL as *mut ClientData, pev);
            bDone = True as Boolean;
        } else {
            pev = ::core::ptr::null_mut::<XEvent>();
            continue;
        }
        if bDone == 0 {
            MoveOutline(
                marqueeX,
                marqueeY,
                marqueeWidth as ::core::ffi::c_uint,
                marqueeHeight as ::core::ffi::c_uint,
            );
        }
        pev = ::core::ptr::null_mut::<XEvent>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn StartMarqueeSelect(mut pSD: *mut WmScreenData, mut pev: *mut XEvent) {
    let mut grab_win: Window = 0;
    let mut junk_win: Window = 0;
    let mut grabbed: Boolean = 0;
    let mut junk: ::core::ffi::c_int = 0;
    if (*pSD).bMarqueeSelectionInitialized == 0 {
        (*pSD).bMarqueeSelectionInitialized = True as Boolean;
    }
    if configGrab == 0 {
        grab_win = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*pSD).screen as isize))
        .root;
        if !pev.is_null() {
            grabbed = DoGrabs(
                grab_win,
                wmGD.configCursor,
                PGRAB_MASK as ::core::ffi::c_uint,
                (*pev).xbutton.time,
                ::core::ptr::null_mut::<ClientData>(),
                True as Boolean,
            );
        } else {
            grabbed = DoGrabs(
                grab_win,
                wmGD.configCursor,
                PGRAB_MASK as ::core::ffi::c_uint,
                CurrentTime as Time,
                ::core::ptr::null_mut::<ClientData>(),
                True as Boolean,
            );
        }
        if grabbed == 0 {
            return;
        }
        configGrab = TRUE as Boolean;
    } else {
        return;
    }
    if (*pev).type_0 == ButtonPress || (*pev).type_0 == ButtonRelease {
        pointerX = (*pev).xbutton.x_root;
        pointerY = (*pev).xbutton.y_root;
    } else if XQueryPointer(
        wmGD.display,
        (*pSD).rootWindow,
        &raw mut junk_win,
        &raw mut junk_win,
        &raw mut pointerX,
        &raw mut pointerY,
        &raw mut junk,
        &raw mut junk,
        &raw mut junk as *mut ::core::ffi::c_uint,
    ) == 0
    {
        CancelFrameConfig(NULL as *mut ClientData);
        return;
    }
    startX = pointerX;
    marqueeX = startX;
    startY = pointerY;
    marqueeY = startY;
    marqueeWidth = 0 as ::core::ffi::c_long;
    marqueeWidth0 = marqueeWidth;
    marqueeHeight = 0 as ::core::ffi::c_long;
    marqueeHeight0 = marqueeHeight;
    marqueeAnchor = ANCHOR_NW as ::core::ffi::c_uint;
    wmGD.configAction = MARQUEE_SELECT;
    wmGD.configButton = if !pev.is_null() {
        (*pev).xbutton.button
    } else {
        0 as ::core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn UpdateMarqueeSelectData(mut pSD: *mut WmScreenData) {
    match marqueeAnchor {
        1 => {
            marqueeWidth = (pointerX - marqueeX) as ::core::ffi::c_long;
            marqueeHeight = (pointerY - marqueeY) as ::core::ffi::c_long;
            if marqueeWidth < 0 as ::core::ffi::c_long {
                marqueeWidth = -marqueeWidth;
                marqueeX = pointerX;
                if marqueeHeight < 0 as ::core::ffi::c_long {
                    marqueeHeight = -marqueeHeight;
                    marqueeY = pointerY;
                    marqueeAnchor = ANCHOR_SE as ::core::ffi::c_uint;
                } else {
                    marqueeAnchor = ANCHOR_NE as ::core::ffi::c_uint;
                }
            } else if marqueeHeight < 0 as ::core::ffi::c_long {
                marqueeHeight = -marqueeHeight;
                marqueeY = pointerY;
                marqueeAnchor = ANCHOR_SW as ::core::ffi::c_uint;
            }
        }
        2 => {
            marqueeWidth += (marqueeX - pointerX) as ::core::ffi::c_long;
            marqueeHeight = (pointerY - marqueeY) as ::core::ffi::c_long;
            marqueeX = pointerX;
            if marqueeWidth < 0 as ::core::ffi::c_long {
                marqueeWidth = -marqueeWidth;
                marqueeX = (pointerX as ::core::ffi::c_long - marqueeWidth) as ::core::ffi::c_int;
                if marqueeHeight < 0 as ::core::ffi::c_long {
                    marqueeHeight = -marqueeHeight;
                    marqueeY = pointerY;
                    marqueeAnchor = ANCHOR_SW as ::core::ffi::c_uint;
                } else {
                    marqueeAnchor = ANCHOR_NW as ::core::ffi::c_uint;
                }
            } else if marqueeHeight < 0 as ::core::ffi::c_long {
                marqueeHeight = -marqueeHeight;
                marqueeY = pointerY;
                marqueeAnchor = ANCHOR_SE as ::core::ffi::c_uint;
            }
        }
        3 => {
            marqueeWidth += (marqueeX - pointerX) as ::core::ffi::c_long;
            marqueeHeight += (marqueeY - pointerY) as ::core::ffi::c_long;
            marqueeX = pointerX;
            marqueeY = pointerY;
            if marqueeWidth < 0 as ::core::ffi::c_long {
                marqueeWidth = -marqueeWidth;
                marqueeX = (pointerX as ::core::ffi::c_long - marqueeWidth) as ::core::ffi::c_int;
                if marqueeHeight < 0 as ::core::ffi::c_long {
                    marqueeHeight = -marqueeHeight;
                    marqueeY =
                        (pointerY as ::core::ffi::c_long - marqueeHeight) as ::core::ffi::c_int;
                    marqueeAnchor = ANCHOR_NW as ::core::ffi::c_uint;
                } else {
                    marqueeAnchor = ANCHOR_SW as ::core::ffi::c_uint;
                }
            } else if marqueeHeight < 0 as ::core::ffi::c_long {
                marqueeHeight = -marqueeHeight;
                marqueeY = (pointerY as ::core::ffi::c_long - marqueeHeight) as ::core::ffi::c_int;
                marqueeAnchor = ANCHOR_NE as ::core::ffi::c_uint;
            }
        }
        4 => {
            marqueeWidth = (pointerX - marqueeX) as ::core::ffi::c_long;
            marqueeHeight += (marqueeY - pointerY) as ::core::ffi::c_long;
            marqueeY = pointerY;
            if marqueeWidth < 0 as ::core::ffi::c_long {
                marqueeWidth = -marqueeWidth;
                marqueeX = pointerX;
                if marqueeHeight < 0 as ::core::ffi::c_long {
                    marqueeHeight = -marqueeHeight;
                    marqueeY =
                        (pointerY as ::core::ffi::c_long - marqueeHeight) as ::core::ffi::c_int;
                    marqueeAnchor = ANCHOR_NE as ::core::ffi::c_uint;
                } else {
                    marqueeAnchor = ANCHOR_SE as ::core::ffi::c_uint;
                }
            } else if marqueeHeight < 0 as ::core::ffi::c_long {
                marqueeHeight = -marqueeHeight;
                marqueeY = (pointerY as ::core::ffi::c_long - marqueeHeight) as ::core::ffi::c_int;
                marqueeAnchor = ANCHOR_NW as ::core::ffi::c_uint;
            }
        }
        _ => {}
    }
    if wmGD.marqueeSelectGranularity > 0 as ::core::ffi::c_int
        && ((if marqueeWidth - marqueeWidth0 > 0 as ::core::ffi::c_long {
            marqueeWidth - marqueeWidth0
        } else {
            -(marqueeWidth - marqueeWidth0)
        }) > wmGD.marqueeSelectGranularity as ::core::ffi::c_long
            || (if marqueeHeight - marqueeHeight0 > 0 as ::core::ffi::c_long {
                marqueeHeight - marqueeHeight0
            } else {
                -(marqueeHeight - marqueeHeight0)
            }) > wmGD.marqueeSelectGranularity as ::core::ffi::c_long)
    {
        marqueeWidth0 = marqueeWidth;
        marqueeHeight0 = marqueeHeight;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleMarqueeKeyPress(
    mut pSD: *mut WmScreenData,
    mut pev: *mut XEvent,
) -> Boolean {
    let mut keysym: KeySym = 0;
    let mut keyMult: ::core::ffi::c_int = 0;
    let mut KeyEvent = _XEvent { type_0: 0 };
    keyMult = 1 as ::core::ffi::c_int;
    while keyMult <= 10 as ::core::ffi::c_int
        && XCheckIfEvent(
            wmGD.display,
            &raw mut KeyEvent,
            Some(
                IsRepeatedKeyEvent
                    as unsafe extern "C" fn(
                        *mut Display,
                        *mut XEvent,
                        *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            pev as XPointer,
        ) != 0
    {
        keyMult += 1;
    }
    keysym = XkbKeycodeToKeysym(
        wmGD.display,
        (*pev).xkey.keycode as KeyCode,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    match keysym {
        65293 => {
            CompleteFrameConfig(NULL as *mut ClientData, pev);
            return 1 as Boolean;
        }
        65307 => {
            CancelFrameConfig(NULL as *mut ClientData);
            CheckEatButtonRelease(NULL as *mut ClientData, pev);
            return 1 as Boolean;
        }
        _ => return 0 as Boolean,
    };
}
