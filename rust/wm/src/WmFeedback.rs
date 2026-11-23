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
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_2;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_3;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_4;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_5;
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
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    fn XClearArea(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XConfigureWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_uint,
        _: *mut XWindowChanges,
    ) -> ::core::ffi::c_int;
    fn XFillRectangles(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XRectangle,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XFlush(_: *mut Display) -> ::core::ffi::c_int;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> ::core::ffi::c_int;
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
    fn XMapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XParseGeometry(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XQueryBestCursor(
        _: *mut Display,
        _: Drawable,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XQueryColors(
        _: *mut Display,
        _: Colormap,
        _: *mut XColor,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XRaiseWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XUngrabKeyboard(_: *mut Display, _: Time) -> ::core::ffi::c_int;
    fn XUngrabPointer(_: *mut Display, _: Time) -> ::core::ffi::c_int;
    fn XUnmapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XtUnmanageChild(_: Widget);
    fn XtRealizeWidget(_: Widget);
    fn XtWindowOfObject(_: Widget) -> Window;
    fn XtAddCallback(_: Widget, _: *const ::core::ffi::c_char, _: XtCallbackProc, _: XtPointer);
    fn XtCreatePopupShell(
        _: *const ::core::ffi::c_char,
        _: WidgetClass,
        _: Widget,
        _: ArgList,
        _: Cardinal,
    ) -> Widget;
    fn XtCreateManagedWidget(
        _: *const ::core::ffi::c_char,
        _: WidgetClass,
        _: Widget,
        _: ArgList,
        _: Cardinal,
    ) -> Widget;
    fn XtSetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtGetValues(_: Widget, _: ArgList, _: Cardinal);
    static XtStrings: [::core::ffi::c_char; 0];
    static XtShellStrings: [::core::ffi::c_char; 0];
    static mut transientShellWidgetClass: WidgetClass;
    static mut _XmStrings: [::core::ffi::c_char; 0];
    fn XmStringCreateLocalized(text: String_0) -> XmString;
    fn XmStringCompare(a: XmString, b: XmString) -> Boolean;
    fn XmStringFree(string: XmString);
    fn XmStringExtent(
        fontlist: XmFontList,
        string: XmString,
        width: *mut Dimension,
        height: *mut Dimension,
    );
    fn XmStringDraw(
        d: *mut Display,
        w: Window,
        fontlist: XmFontList,
        string: XmString,
        gc: GC,
        x: Position,
        y: Position,
        width: Dimension,
        align: ::core::ffi::c_uchar,
        lay_dir: ::core::ffi::c_uchar,
        clip: *mut XRectangle,
    );
    fn XmRenderTableGetDefaultFontExtents(
        rendertable: XmRenderTable,
        height: *mut ::core::ffi::c_int,
        ascent: *mut ::core::ffi::c_int,
        descent: *mut ::core::ffi::c_int,
    );
    static mut wmGD: WmGlobalData;
    static mut xmMessageBoxWidgetClass: WidgetClass;
    fn XmMessageBoxGetChild(widget: Widget, child: ::core::ffi::c_uchar) -> Widget;
    fn Do_Quit_Mwm(diedOnRestart: Boolean);
    fn Do_Restart(dummy: Boolean);
    fn Do_Set_Behavior(dummy: Boolean);
    fn AllocateRList(amt: ::core::ffi::c_uint) -> *mut RList;
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
    fn ManageWindow(pSD: *mut WmScreenData, clientWindow: Window, manageFlags: ::core::ffi::c_long);
    fn WithdrawDialog(dialogboxW: Widget);
    fn ReManageDialog(pSD: *mut WmScreenData, dialogboxW: Widget);
    fn ForceColormapFocus(pSD: *mut WmScreenData, pCD: *mut ClientData);
    fn GetXineramaScreenFromLocation(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
    fn GetPrimaryXineramaScreen(xsi: *mut XineramaScreenInfo) -> ::core::ffi::c_int;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const XmSTRING_DIRECTION_DEFAULT: C2RustUnnamed_1 = 255;
pub const XmSTRING_DIRECTION_UNSET: C2RustUnnamed_1 = 3;
pub const XmSTRING_DIRECTION_R_TO_L: C2RustUnnamed_1 = 1;
pub const XmSTRING_DIRECTION_L_TO_R: C2RustUnnamed_1 = 0;
pub const XmPOINTER: C2RustUnnamed_2 = 1;
pub const XmEXPLICIT: C2RustUnnamed_2 = 0;
pub const XmALIGNMENT_UNSPECIFIED: C2RustUnnamed_3 = 3;
pub const XmALIGNMENT_END: C2RustUnnamed_3 = 2;
pub const XmALIGNMENT_CENTER: C2RustUnnamed_3 = 1;
pub const XmALIGNMENT_BEGINNING: C2RustUnnamed_3 = 0;
pub const XmDIALOG_DIR_LIST_LABEL: C2RustUnnamed_4 = 16;
pub const XmDIALOG_DIR_LIST: C2RustUnnamed_4 = 15;
pub const XmDIALOG_SEPARATOR: C2RustUnnamed_4 = 14;
pub const XmDIALOG_TEXT: C2RustUnnamed_4 = 13;
pub const XmDIALOG_SYMBOL_LABEL: C2RustUnnamed_4 = 12;
pub const XmDIALOG_SELECTION_LABEL: C2RustUnnamed_4 = 11;
pub const XmDIALOG_MESSAGE_LABEL: C2RustUnnamed_4 = 10;
pub const XmDIALOG_LIST_LABEL: C2RustUnnamed_4 = 9;
pub const XmDIALOG_LIST: C2RustUnnamed_4 = 8;
pub const XmDIALOG_HELP_BUTTON: C2RustUnnamed_4 = 7;
pub const XmDIALOG_FILTER_TEXT: C2RustUnnamed_4 = 6;
pub const XmDIALOG_FILTER_LABEL: C2RustUnnamed_4 = 5;
pub const XmDIALOG_OK_BUTTON: C2RustUnnamed_4 = 4;
pub const XmDIALOG_DEFAULT_BUTTON: C2RustUnnamed_4 = 3;
pub const XmDIALOG_CANCEL_BUTTON: C2RustUnnamed_4 = 2;
pub const XmDIALOG_APPLY_BUTTON: C2RustUnnamed_4 = 1;
pub const XmDIALOG_NONE: C2RustUnnamed_4 = 0;
pub const XmDIALOG_WORKING: C2RustUnnamed_5 = 6;
pub const XmDIALOG_WARNING: C2RustUnnamed_5 = 5;
pub const XmDIALOG_QUESTION: C2RustUnnamed_5 = 4;
pub const XmDIALOG_MESSAGE: C2RustUnnamed_5 = 3;
pub const XmDIALOG_INFORMATION: C2RustUnnamed_5 = 2;
pub const XmDIALOG_ERROR: C2RustUnnamed_5 = 1;
pub const XmDIALOG_TEMPLATE: C2RustUnnamed_5 = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CopyFromParent: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CurrentTime: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ExposureMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 15 as ::core::ffi::c_int;
pub const GrabModeAsync: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const InputOutput: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CWBackPixmap: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const CWBackPixel: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const CWOverrideRedirect: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const CWSaveUnder: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 10 as ::core::ffi::c_int;
pub const CWEventMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CWX: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const CWY: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const CWWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const CWHeight: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const CWStackMode: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const Above: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XValue: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const YValue: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const XNegative: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const YNegative: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MANAGEW_CONFIRM_BOX: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const FB_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FB_SIZE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const FB_POSITION: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const WmNfeedback: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"feedback\0") };
pub const time32_width: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const time32_height: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const time32_x_hot: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const time32_y_hot: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
static mut time32_bits: [::core::ffi::c_uchar; 128] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut time32m_bits: [::core::ffi::c_uchar; 128] = [
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
    0xcf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xce as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x78 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xce as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x77 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xcf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
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
];
pub const time16_x_hot: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const time16_y_hot: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const time16_width: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const time16_height: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
static mut time16_bits: [::core::ffi::c_uchar; 32] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x44 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x84 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x84 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x44 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2b as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut time16m_bits: [::core::ffi::c_uchar; 32] = [
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
pub const FEEDBACK_BEVEL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DEFAULT_POSITION_STRING: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"(0000x0000)\0") };
static mut waitCursor: Cursor = 0 as ::core::ffi::c_long as Cursor;
static mut confirm_mesg: [*mut ::core::ffi::c_char; 4] = [
    b"Toggle to Default Behavior?\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Toggle to Custom Behavior?\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Restart Window Manager?\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"QUIT Window Manager?\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
];
static mut confirm_widget: [*mut ::core::ffi::c_char; 4] = [
    b"confirmDefaultBehavior\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"confirmCustomBehavior\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"confirmRestart\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"confirmQuit\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
static mut confirm_func: [ConfirmFunc; 4] = unsafe {
    [
        Some(Do_Set_Behavior as unsafe extern "C" fn(Boolean) -> ()),
        Some(Do_Set_Behavior as unsafe extern "C" fn(Boolean) -> ()),
        Some(Do_Restart as unsafe extern "C" fn(Boolean) -> ()),
        Some(Do_Quit_Mwm as unsafe extern "C" fn(Boolean) -> ()),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn ShowFeedbackWindow(
    mut pSD: *mut WmScreenData,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
    mut style: ::core::ffi::c_ulong,
) {
    let mut mask = 0 as ::core::ffi::c_ulong;
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
    let mut win_changes = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    let mut winX: ::core::ffi::c_int = 0;
    let mut winY: ::core::ffi::c_int = 0;
    let mut tmpX: ::core::ffi::c_int = 0;
    let mut tmpY: ::core::ffi::c_int = 0;
    static mut defaultPositionString: XmString =
        ::core::ptr::null::<__XmStringRec>() as *mut __XmStringRec;
    static mut ascent: ::core::ffi::c_int = 0;
    static mut descent: ::core::ffi::c_int = 0;
    static mut fheight: ::core::ffi::c_int = 0;
    static mut swidth: Dimension = 0;
    static mut sheight: Dimension = 0;
    (*pSD).fbStyle = style;
    if (*pSD).fbStyle == FB_OFF as ::core::ffi::c_ulong {
        return;
    }
    (*pSD).fbLastX = x;
    (*pSD).fbLastY = y;
    (*pSD).fbLastWidth = width;
    (*pSD).fbLastHeight = height;
    if defaultPositionString.is_null() {
        defaultPositionString =
            XmStringCreateLocalized(DEFAULT_POSITION_STRING.as_ptr() as String_0);
        XmRenderTableGetDefaultFontExtents(
            (*pSD).feedbackAppearance.renderTable,
            &raw mut fheight,
            &raw mut ascent,
            &raw mut descent,
        );
        XmStringExtent(
            (*pSD).feedbackAppearance.renderTable as XmFontList,
            defaultPositionString,
            &raw mut swidth,
            &raw mut sheight,
        );
    }
    (*pSD).fbWinWidth = (swidth as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FEEDBACK_BEVEL)
        as ::core::ffi::c_uint;
    match (*pSD).fbStyle {
        1 => {
            (*pSD).fbSizeY = 2 as ::core::ffi::c_int * FEEDBACK_BEVEL;
            (*pSD).fbWinHeight = (sheight as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * FEEDBACK_BEVEL)
                as ::core::ffi::c_uint;
        }
        2 => {
            (*pSD).fbLocY = 2 as ::core::ffi::c_int * FEEDBACK_BEVEL;
            (*pSD).fbWinHeight = (sheight as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * FEEDBACK_BEVEL)
                as ::core::ffi::c_uint;
        }
        3 | _ => {
            (*pSD).fbLocY = 2 as ::core::ffi::c_int * FEEDBACK_BEVEL;
            (*pSD).fbSizeY = (*pSD).fbLocY + sheight as ::core::ffi::c_int;
            (*pSD).fbWinHeight = (2 as ::core::ffi::c_int * sheight as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * FEEDBACK_BEVEL)
                as ::core::ffi::c_uint;
        }
    }
    if !(*pSD).feedbackGeometry.is_null() {
        let mut junkWidth: ::core::ffi::c_uint = 0;
        let mut junkHeight: ::core::ffi::c_uint = 0;
        mask = XParseGeometry(
            (*pSD).feedbackGeometry as *const ::core::ffi::c_char,
            &raw mut tmpX,
            &raw mut tmpY,
            &raw mut junkWidth,
            &raw mut junkHeight,
        ) as ::core::ffi::c_ulong;
    }
    if mask & (XValue | YValue) as ::core::ffi::c_ulong != 0 {
        winX = (if mask & XNegative as ::core::ffi::c_ulong != 0 {
            (((*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .width
                + tmpX) as ::core::ffi::c_uint)
                .wrapping_sub((*pSD).fbWinWidth)
        } else {
            tmpX as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
        winY = (if mask & YNegative as ::core::ffi::c_ulong != 0 {
            (((*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .height
                + tmpY) as ::core::ffi::c_uint)
                .wrapping_sub((*pSD).fbWinHeight)
        } else {
            tmpY as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
    } else {
        let mut xsi = XineramaScreenInfo {
            screen_number: 0,
            x_org: 0,
            y_org: 0,
            width: 0,
            height: 0,
        };
        if GetXineramaScreenFromLocation(x, y, &raw mut xsi) != 0 {
            winX = (xsi.x_org as ::core::ffi::c_uint).wrapping_add(
                (xsi.width as ::core::ffi::c_uint)
                    .wrapping_sub((*pSD).fbWinWidth)
                    .wrapping_div(2 as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int;
            winY = (xsi.y_org as ::core::ffi::c_uint).wrapping_add(
                (xsi.height as ::core::ffi::c_uint)
                    .wrapping_sub((*pSD).fbWinHeight)
                    .wrapping_div(2 as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int;
        } else {
            winX = ((*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .width as ::core::ffi::c_uint)
                .wrapping_sub((*pSD).fbWinWidth)
                .wrapping_div(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            winY = ((*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .height as ::core::ffi::c_uint)
                .wrapping_sub((*pSD).fbWinHeight)
                .wrapping_div(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        }
    }
    UpdateFeedbackText(pSD, x, y, width, height);
    if !(*pSD).fbTop.is_null() && !(*pSD).fbBottom.is_null() || {
        (*pSD).fbTop = AllocateRList(
            (2 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(FEEDBACK_BEVEL as ::core::ffi::c_uint),
        );
        !(*pSD).fbTop.is_null() && {
            (*pSD).fbBottom = AllocateRList(
                (2 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(FEEDBACK_BEVEL as ::core::ffi::c_uint),
            );
            !(*pSD).fbBottom.is_null()
        }
    } {
        (*(*pSD).fbTop).used = 0 as ::core::ffi::c_int;
        (*(*pSD).fbBottom).used = 0 as ::core::ffi::c_int;
        BevelRectangle(
            (*pSD).fbTop,
            (*pSD).fbBottom,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*pSD).fbWinWidth,
            (*pSD).fbWinHeight,
            FEEDBACK_BEVEL as ::core::ffi::c_uint,
            FEEDBACK_BEVEL as ::core::ffi::c_uint,
            FEEDBACK_BEVEL as ::core::ffi::c_uint,
            FEEDBACK_BEVEL as ::core::ffi::c_uint,
        );
    }
    if (*pSD).feedbackWin == 0 {
        mask = (CWEventMask | CWOverrideRedirect | CWSaveUnder) as ::core::ffi::c_ulong;
        win_attribs.event_mask = ExposureMask;
        win_attribs.override_redirect = TRUE;
        win_attribs.save_under = TRUE;
        if (*pSD).feedbackAppearance.backgroundPixmap != 0 {
            mask |= CWBackPixmap as ::core::ffi::c_ulong;
            win_attribs.background_pixmap = (*pSD).feedbackAppearance.backgroundPixmap;
        } else {
            mask |= CWBackPixel as ::core::ffi::c_ulong;
            win_attribs.background_pixel =
                (*pSD).feedbackAppearance.background as ::core::ffi::c_ulong;
        }
        (*pSD).feedbackWin = XCreateWindow(
            wmGD.display,
            (*pSD).rootWindow,
            winX,
            winY,
            (*pSD).fbWinWidth,
            (*pSD).fbWinHeight,
            0 as ::core::ffi::c_uint,
            CopyFromParent as ::core::ffi::c_int,
            InputOutput as ::core::ffi::c_uint,
            ::core::ptr::null_mut::<Visual>(),
            mask,
            &raw mut win_attribs,
        );
    } else {
        win_changes.x = winX;
        win_changes.y = winY;
        win_changes.width = (*pSD).fbWinWidth as ::core::ffi::c_int;
        win_changes.height = (*pSD).fbWinHeight as ::core::ffi::c_int;
        win_changes.stack_mode = Above;
        mask = (CWX | CWY | CWWidth | CWHeight | CWStackMode) as ::core::ffi::c_ulong;
        XConfigureWindow(
            wmGD.display,
            (*pSD).feedbackWin,
            mask as ::core::ffi::c_uint,
            &raw mut win_changes,
        );
    }
    if (*pSD).feedbackWin != 0 {
        XRaiseWindow(wmGD.display, (*pSD).feedbackWin);
        XMapWindow(wmGD.display, (*pSD).feedbackWin);
        PaintFeedbackWindow(pSD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PaintFeedbackWindow(mut pSD: *mut WmScreenData) {
    if (*pSD).feedbackWin != 0 {
        if (*(*pSD).fbTop).used > 0 as ::core::ffi::c_int {
            XFillRectangles(
                wmGD.display,
                (*pSD).feedbackWin as Drawable,
                (*pSD).feedbackAppearance.inactiveTopShadowGC,
                (*(*pSD).fbTop).prect,
                (*(*pSD).fbTop).used,
            );
        }
        if (*(*pSD).fbBottom).used > 0 as ::core::ffi::c_int {
            XFillRectangles(
                wmGD.display,
                (*pSD).feedbackWin as Drawable,
                (*pSD).feedbackAppearance.inactiveBottomShadowGC,
                (*(*pSD).fbBottom).prect,
                (*(*pSD).fbBottom).used,
            );
        }
        XClearArea(
            wmGD.display,
            (*pSD).feedbackWin,
            FEEDBACK_BEVEL,
            FEEDBACK_BEVEL,
            (*pSD)
                .fbWinWidth
                .wrapping_sub((2 as ::core::ffi::c_int * FEEDBACK_BEVEL) as ::core::ffi::c_uint),
            (*pSD)
                .fbWinHeight
                .wrapping_sub((2 as ::core::ffi::c_int * FEEDBACK_BEVEL) as ::core::ffi::c_uint),
            FALSE,
        );
        if (*pSD).fbStyle & FB_POSITION as ::core::ffi::c_ulong != 0 {
            XmStringDraw(
                wmGD.display,
                (*pSD).feedbackWin,
                (*pSD).feedbackAppearance.renderTable as XmFontList,
                (*pSD).fbLocation,
                (*pSD).feedbackAppearance.inactiveGC,
                (*pSD).fbLocX as Position,
                (*pSD).fbLocY as Position,
                (*pSD).fbWinWidth as Dimension,
                XmALIGNMENT_CENTER as ::core::ffi::c_int as ::core::ffi::c_uchar,
                XmSTRING_DIRECTION_L_TO_R as ::core::ffi::c_int as ::core::ffi::c_uchar,
                ::core::ptr::null_mut::<XRectangle>(),
            );
        }
        if (*pSD).fbStyle & FB_SIZE as ::core::ffi::c_ulong != 0 {
            XmStringDraw(
                wmGD.display,
                (*pSD).feedbackWin,
                (*pSD).feedbackAppearance.renderTable as XmFontList,
                (*pSD).fbSize,
                (*pSD).feedbackAppearance.inactiveGC,
                (*pSD).fbSizeX as Position,
                (*pSD).fbSizeY as Position,
                (*pSD).fbWinWidth as Dimension,
                XmALIGNMENT_CENTER as ::core::ffi::c_int as ::core::ffi::c_uchar,
                XmSTRING_DIRECTION_L_TO_R as ::core::ffi::c_int as ::core::ffi::c_uchar,
                ::core::ptr::null_mut::<XRectangle>(),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HideFeedbackWindow(mut pSD: *mut WmScreenData) {
    if (*pSD).feedbackWin != 0 {
        XUnmapWindow(wmGD.display, (*pSD).feedbackWin);
        ForceColormapFocus(
            wmGD.pActiveSD,
            (*wmGD.pActiveSD).colormapFocus as *mut ClientData,
        );
    }
    (*pSD).fbStyle = FB_OFF as ::core::ffi::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn UpdateFeedbackInfo(
    mut pSD: *mut WmScreenData,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
) {
    (*pSD).fbLastX = x;
    (*pSD).fbLastY = y;
    (*pSD).fbLastWidth = width;
    (*pSD).fbLastHeight = height;
    UpdateFeedbackText(pSD, x, y, width, height);
    PaintFeedbackWindow(pSD);
}
#[no_mangle]
pub unsafe extern "C" fn UpdateFeedbackText(
    mut pSD: *mut WmScreenData,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
) {
    let mut swidth: Dimension = 0;
    let mut sheight: Dimension = 0;
    let mut buffer: [::core::ffi::c_char; 20] = [0; 20];
    if (*pSD).fbStyle & FB_POSITION as ::core::ffi::c_ulong != 0 {
        snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            20 as size_t,
            b"(%4d,%-4d)\0" as *const u8 as *const ::core::ffi::c_char,
            x,
            y,
        );
        if !(*pSD).fbLocation.is_null() {
            XmStringFree((*pSD).fbLocation);
        }
        (*pSD).fbLocation = XmStringCreateLocalized(&raw mut buffer as String_0);
        XmStringExtent(
            (*pSD).feedbackAppearance.renderTable as XmFontList,
            (*pSD).fbLocation,
            &raw mut swidth,
            &raw mut sheight,
        );
    }
    if (*pSD).fbStyle & FB_SIZE as ::core::ffi::c_ulong != 0 {
        snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            20 as size_t,
            b"%4dx%-4d\0" as *const u8 as *const ::core::ffi::c_char,
            width,
            height,
        );
        if !(*pSD).fbSize.is_null() {
            XmStringFree((*pSD).fbSize);
        }
        (*pSD).fbSize = XmStringCreateLocalized(&raw mut buffer as String_0);
        XmStringExtent(
            (*pSD).feedbackAppearance.renderTable as XmFontList,
            (*pSD).fbSize,
            &raw mut swidth,
            &raw mut sheight,
        );
    }
}
unsafe extern "C" fn OkCB(mut w: Widget, mut client_data: XtPointer, mut call_data: XtPointer) {
    WithdrawDialog(w);
    confirm_func[(*(client_data as *mut WmScreenData)).actionNbr as usize]
        .expect("non-null function pointer")(False as Boolean);
    wmGD.confirmDialogMapped = False as Boolean;
}
unsafe extern "C" fn CancelCB(mut w: Widget, mut client_data: XtPointer, mut call_data: XtPointer) {
    WithdrawDialog(w);
    wmGD.confirmDialogMapped = False as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ConfirmAction(mut pSD: *mut WmScreenData, mut nbr: ::core::ffi::c_int) {
    let mut args: [Arg; 8] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 8];
    let mut n: ::core::ffi::c_int = 0;
    let mut x: Position = 0;
    let mut y: Position = 0;
    let mut width: Dimension = 0;
    let mut height: Dimension = 0;
    let mut dialogShellW = ::core::ptr::null_mut::<_WidgetRec>();
    let mut messageString = ::core::ptr::null_mut::<__XmStringRec>();
    static mut defaultMessageString: XmString =
        ::core::ptr::null::<__XmStringRec>() as *mut __XmStringRec;
    if wmGD.systemModalActive != 0 {
        return;
    }
    if (*pSD).confirmboxW[nbr as usize].is_null() {
        let mut have_xinerama: ::core::ffi::c_int = 0;
        let mut xsi = XineramaScreenInfo {
            screen_number: 0,
            x_org: 0,
            y_org: 0,
            width: 0,
            height: 0,
        };
        have_xinerama = GetPrimaryXineramaScreen(&raw mut xsi);
        if have_xinerama != 0 {
            x = (xsi.x_org as ::core::ffi::c_int
                + xsi.width as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
                as Position;
            y = (xsi.y_org as ::core::ffi::c_int
                + xsi.height as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
                as Position;
        } else {
            x = ((*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .width
                / 2 as ::core::ffi::c_int) as Position;
            y = ((*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .height
                / 2 as ::core::ffi::c_int) as Position;
        }
        n = 0 as ::core::ffi::c_int;
        args[n as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(885 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = x as XtArgVal;
        n += 1;
        args[n as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(887 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = y as XtArgVal;
        n += 1;
        args[n as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
            .offset(280 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
        n += 1;
        args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(7466 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = XmEXPLICIT as ::core::ffi::c_int as XtArgVal;
        n += 1;
        args[n as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(163 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*pSD).screen as isize))
        .root_depth as XtArgVal;
        n += 1;
        args[n as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(584 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = (*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*pSD).screen as isize) as *mut Screen
            as XtArgVal;
        n += 1;
        dialogShellW = XtCreatePopupShell(
            WmNfeedback.as_ptr() as String_0 as *const ::core::ffi::c_char,
            transientShellWidgetClass,
            (*pSD).screenTopLevelW,
            &raw mut args as ArgList,
            n as Cardinal,
        );
        n = 0 as ::core::ffi::c_int;
        args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(6071 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = XmDIALOG_QUESTION as ::core::ffi::c_int as XtArgVal;
        n += 1;
        args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(8061 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = XmALIGNMENT_CENTER as ::core::ffi::c_int as XtArgVal;
        n += 1;
        args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(10395 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
        n += 1;
        (*pSD).confirmboxW[nbr as usize] = XtCreateManagedWidget(
            confirm_widget[nbr as usize],
            xmMessageBoxWidgetClass,
            dialogShellW,
            &raw mut args as ArgList,
            n as Cardinal,
        );
        n = 0 as ::core::ffi::c_int;
        args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(8090 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = &raw mut messageString as XtArgVal;
        n += 1;
        XtGetValues(
            (*pSD).confirmboxW[nbr as usize],
            &raw mut args as *mut Arg as ArgList,
            n as Cardinal,
        );
        if defaultMessageString.is_null() {
            defaultMessageString = XmStringCreateLocalized(
                b"\0" as *const u8 as *const ::core::ffi::c_char as String_0,
            );
        }
        n = 0 as ::core::ffi::c_int;
        if XmStringCompare(messageString, defaultMessageString) != 0 {
            messageString = XmStringCreateLocalized(confirm_mesg[nbr as usize]);
            args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(8090 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            args[n as usize].value = messageString as XtArgVal;
            n += 1;
            XtSetValues(
                (*pSD).confirmboxW[nbr as usize],
                &raw mut args as *mut Arg as ArgList,
                n as Cardinal,
            );
            XmStringFree(messageString);
        }
        n = 0 as ::core::ffi::c_int;
        args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(10395 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
        n += 1;
        args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(7078 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = 3 as ::core::ffi::c_int as XtArgVal;
        n += 1;
        XtSetValues(
            XmMessageBoxGetChild(
                (*pSD).confirmboxW[nbr as usize],
                XmDIALOG_OK_BUTTON as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ),
            &raw mut args as ArgList,
            n as Cardinal,
        );
        XtSetValues(
            XmMessageBoxGetChild(
                (*pSD).confirmboxW[nbr as usize],
                XmDIALOG_CANCEL_BUTTON as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ),
            &raw mut args as ArgList,
            n as Cardinal,
        );
        XtAddCallback(
            (*pSD).confirmboxW[nbr as usize],
            (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(8560 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()>,
                XtCallbackProc,
            >(Some(
                OkCB as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> (),
            )),
            pSD as XtPointer,
        );
        XtAddCallback(
            (*pSD).confirmboxW[nbr as usize],
            (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(5319 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()>,
                XtCallbackProc,
            >(Some(
                CancelCB as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> (),
            )),
            NULL,
        );
        XtUnmanageChild(XmMessageBoxGetChild(
            (*pSD).confirmboxW[nbr as usize],
            XmDIALOG_HELP_BUTTON as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ));
        XtRealizeWidget(dialogShellW);
        n = 0 as ::core::ffi::c_int;
        args[n as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(234 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = &raw mut height as XtArgVal;
        n += 1;
        args[n as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(872 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = &raw mut width as XtArgVal;
        n += 1;
        XtGetValues(
            dialogShellW,
            &raw mut args as *mut Arg as ArgList,
            n as Cardinal,
        );
        if have_xinerama != 0 {
            x = (xsi.x_org as ::core::ffi::c_int
                + (xsi.width as ::core::ffi::c_int - width as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int) as Position;
            y = (xsi.y_org as ::core::ffi::c_int
                + (xsi.height as ::core::ffi::c_int - height as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int) as Position;
        } else {
            x = (((*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .width
                - width as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int) as Position;
            y = (((*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .height
                - height as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int) as Position;
        }
        n = 0 as ::core::ffi::c_int;
        args[n as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(885 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = x as XtArgVal;
        n += 1;
        args[n as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(887 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = y as XtArgVal;
        n += 1;
        XtSetValues(
            dialogShellW,
            &raw mut args as *mut Arg as ArgList,
            n as Cardinal,
        );
        ManageWindow(pSD, XtWindowOfObject(dialogShellW), MANAGEW_CONFIRM_BOX);
    } else {
        ReManageDialog(pSD, (*pSD).confirmboxW[nbr as usize]);
    }
    (*pSD).actionNbr = nbr;
    XFlush(wmGD.display);
    wmGD.confirmDialogMapped = True as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ShowWaitState(mut flag: Boolean) {
    let mut bits = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut maskBits = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut xHotspot: ::core::ffi::c_uint = 0;
    let mut yHotspot: ::core::ffi::c_uint = 0;
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
    if waitCursor == 0 {
        if wmGD.useLargeCursors != 0 {
            width = time32_width as ::core::ffi::c_uint;
            height = time32_height as ::core::ffi::c_uint;
            bits = &raw mut time32_bits as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char;
            maskBits =
                &raw mut time32m_bits as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char;
            xHotspot = time32_x_hot as ::core::ffi::c_uint;
            yHotspot = time32_y_hot as ::core::ffi::c_uint;
        } else {
            width = time16_width as ::core::ffi::c_uint;
            height = time16_height as ::core::ffi::c_uint;
            bits = &raw mut time16_bits as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char;
            maskBits =
                &raw mut time16m_bits as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char;
            xHotspot = time16_x_hot as ::core::ffi::c_uint;
            yHotspot = time16_y_hot as ::core::ffi::c_uint;
        }
        pixmap = XCreateBitmapFromData(
            wmGD.display,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
            .root as Drawable,
            bits,
            width,
            height,
        );
        maskPixmap = XCreateBitmapFromData(
            wmGD.display,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
            .root as Drawable,
            maskBits,
            width,
            height,
        );
        xcolors[0 as ::core::ffi::c_int as usize].pixel = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
        .black_pixel;
        xcolors[1 as ::core::ffi::c_int as usize].pixel = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
        .white_pixel;
        XQueryColors(
            wmGD.display,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
            .cmap,
            &raw mut xcolors as *mut XColor,
            2 as ::core::ffi::c_int,
        );
        waitCursor = XCreatePixmapCursor(
            wmGD.display,
            pixmap,
            maskPixmap,
            (&raw mut xcolors as *mut XColor).offset(0 as ::core::ffi::c_int as isize)
                as *mut XColor,
            (&raw mut xcolors as *mut XColor).offset(1 as ::core::ffi::c_int as isize)
                as *mut XColor,
            xHotspot,
            yHotspot,
        );
        XFreePixmap(wmGD.display, pixmap);
        XFreePixmap(wmGD.display, maskPixmap);
    }
    if flag != 0 {
        XGrabPointer(
            wmGD.display,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
            .root,
            FALSE,
            0 as ::core::ffi::c_uint,
            GrabModeAsync,
            GrabModeAsync,
            None_0 as Window,
            waitCursor,
            CurrentTime as Time,
        );
        XGrabKeyboard(
            wmGD.display,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
            .root,
            FALSE,
            GrabModeAsync,
            GrabModeAsync,
            CurrentTime as Time,
        );
    } else {
        XUngrabPointer(wmGD.display, CurrentTime as Time);
        XUngrabKeyboard(wmGD.display, CurrentTime as Time);
    };
}
#[no_mangle]
pub unsafe extern "C" fn InitCursorInfo() {
    let mut cWidth: ::core::ffi::c_uint = 0;
    let mut cHeight: ::core::ffi::c_uint = 0;
    wmGD.useLargeCursors = False as Boolean;
    if XQueryBestCursor(
        wmGD.display,
        (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
        .root as Drawable,
        32 as ::core::ffi::c_uint,
        32 as ::core::ffi::c_uint,
        &raw mut cWidth,
        &raw mut cHeight,
    ) != 0
    {
        if cWidth >= 32 as ::core::ffi::c_uint && cHeight >= 32 as ::core::ffi::c_uint {
            wmGD.useLargeCursors = True as Boolean;
        }
    }
}
