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
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_6;
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
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn XWithdrawWindow(_: *mut Display, _: Window, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XSync(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XtManageChild(_: Widget);
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
    fn XtPopup(_: Widget, _: XtGrabKind);
    fn XtPopdown(_: Widget);
    fn XtSetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtGetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtRealloc(_: *mut ::core::ffi::c_char, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    static XtStrings: [::core::ffi::c_char; 0];
    static XtShellStrings: [::core::ffi::c_char; 0];
    static mut transientShellWidgetClass: WidgetClass;
    static mut _XmStrings: [::core::ffi::c_char; 0];
    fn XmStringCreateLocalized(text: String_0) -> XmString;
    fn XmStringCopy(string: XmString) -> XmString;
    fn XmStringCompare(a: XmString, b: XmString) -> Boolean;
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
    fn XmProcessTraversal(w: Widget, dir: XmTraversalDirection) -> Boolean;
    static mut wmGD: WmGlobalData;
    fn ProcessPresenceResources(pSD: *mut WmScreenData);
    fn Warning(message: *mut ::core::ffi::c_char);
    fn XmCreateForm(
        parent: Widget,
        name: *mut ::core::ffi::c_char,
        arglist: ArgList,
        argcount: Cardinal,
    ) -> Widget;
    fn XmCreatePushButton(
        parent: Widget,
        name: *mut ::core::ffi::c_char,
        arglist: ArgList,
        argcount: Cardinal,
    ) -> Widget;
    fn XmCreateLabelGadget(
        parent: Widget,
        name: *mut ::core::ffi::c_char,
        arglist: *mut Arg,
        argCount: Cardinal,
    ) -> Widget;
    fn XmListAddItem(w: Widget, item: XmString, pos: ::core::ffi::c_int);
    fn XmListDeletePos(w: Widget, pos: ::core::ffi::c_int);
    fn XmListSelectPos(w: Widget, pos: ::core::ffi::c_int, notify: Boolean);
    fn XmListDeselectAllItems(w: Widget);
    fn XmCreateScrolledList(
        parent: Widget,
        name: *mut ::core::ffi::c_char,
        args: ArgList,
        argCount: Cardinal,
    ) -> Widget;
    fn XmCreateSeparatorGadget(
        parent: Widget,
        name: *mut ::core::ffi::c_char,
        arglist: ArgList,
        argcount: Cardinal,
    ) -> Widget;
    fn XmCreateToggleButton(
        parent: Widget,
        name: *mut ::core::ffi::c_char,
        arglist: *mut Arg,
        argCount: Cardinal,
    ) -> Widget;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn mblen(__s: *const ::core::ffi::c_char, __n: size_t) -> ::core::ffi::c_int;
    fn GetSystemMenuPosition(
        pcd: *mut ClientData,
        px: *mut ::core::ffi::c_int,
        py: *mut ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        context: Context,
    );
    fn ClientInWorkspace(pWS: *mut WmWorkspaceData, pCD: *mut ClientData) -> Boolean;
    fn RemoveClientFromWorkspaces(
        pCD: *mut ClientData,
        pIDs: *mut WorkspaceID,
        numIDs: ::core::ffi::c_uint,
    );
    fn AddClientToWorkspaces(
        pCD: *mut ClientData,
        pIDs: *mut WorkspaceID,
        numIDs: ::core::ffi::c_uint,
    );
    fn F_AddToAllWorkspaces(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
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
pub const XmALIGNMENT_UNSPECIFIED: C2RustUnnamed_2 = 3;
pub const XmALIGNMENT_END: C2RustUnnamed_2 = 2;
pub const XmALIGNMENT_CENTER: C2RustUnnamed_2 = 1;
pub const XmALIGNMENT_BEGINNING: C2RustUnnamed_2 = 0;
pub const XmATTACH_SELF: C2RustUnnamed_3 = 6;
pub const XmATTACH_POSITION: C2RustUnnamed_3 = 5;
pub const XmATTACH_OPPOSITE_WIDGET: C2RustUnnamed_3 = 4;
pub const XmATTACH_WIDGET: C2RustUnnamed_3 = 3;
pub const XmATTACH_OPPOSITE_FORM: C2RustUnnamed_3 = 2;
pub const XmATTACH_FORM: C2RustUnnamed_3 = 1;
pub const XmATTACH_NONE: C2RustUnnamed_3 = 0;
pub const XmCR_PROTOCOLS: C2RustUnnamed_4 = 6666;
pub const XmCR_LEAVE_CHILD: C2RustUnnamed_4 = 86;
pub const XmCR_ENTER_CHILD: C2RustUnnamed_4 = 85;
pub const XmCR_VERIFY_TEXT_FAILED: C2RustUnnamed_4 = 84;
pub const XmCR_VERIFY_TEXT: C2RustUnnamed_4 = 83;
pub const XmCR_UPDATE_TEXT: C2RustUnnamed_4 = 82;
pub const XmCR_UPDATE_SHELL: C2RustUnnamed_4 = 81;
pub const XmCR_PDM_EXIT_ERROR: C2RustUnnamed_4 = 80;
pub const XmCR_PDM_CANCEL: C2RustUnnamed_4 = 79;
pub const XmCR_PDM_OK: C2RustUnnamed_4 = 78;
pub const XmCR_PDM_START_PXAUTH: C2RustUnnamed_4 = 77;
pub const XmCR_PDM_START_VXAUTH: C2RustUnnamed_4 = 76;
pub const XmCR_PDM_START_ERROR: C2RustUnnamed_4 = 75;
pub const XmCR_PDM_UP: C2RustUnnamed_4 = 74;
pub const XmCR_PDM_NONE: C2RustUnnamed_4 = 73;
pub const XmCR_PAGE_SETUP: C2RustUnnamed_4 = 72;
pub const XmCR_END_JOB: C2RustUnnamed_4 = 71;
pub const XmCR_START_JOB: C2RustUnnamed_4 = 70;
pub const XmCR_MINOR_TAB: C2RustUnnamed_4 = 69;
pub const XmCR_MAJOR_TAB: C2RustUnnamed_4 = 68;
pub const XmCR_PAGE_SCROLLER_DECREMENT: C2RustUnnamed_4 = 67;
pub const XmCR_PAGE_SCROLLER_INCREMENT: C2RustUnnamed_4 = 66;
pub const XmCR_SPIN_LAST: C2RustUnnamed_4 = 65;
pub const XmCR_SPIN_FIRST: C2RustUnnamed_4 = 64;
pub const XmCR_SPIN_PRIOR: C2RustUnnamed_4 = 63;
pub const XmCR_SPIN_NEXT: C2RustUnnamed_4 = 62;
pub const XmCR_POST: C2RustUnnamed_4 = 61;
pub const XmCR_NO_RENDITION: C2RustUnnamed_4 = 60;
pub const XmCR_NO_FONT: C2RustUnnamed_4 = 59;
pub const XmCR_DRAG_START: C2RustUnnamed_4 = 58;
pub const XmCR_SELECT: C2RustUnnamed_4 = 57;
pub const XmCR_EXPANDED: C2RustUnnamed_4 = 56;
pub const XmCR_COLLAPSED: C2RustUnnamed_4 = 55;
pub const XmCR_REPOST: C2RustUnnamed_4 = 54;
pub const XmCR_FOCUS_MOVED: C2RustUnnamed_4 = 47;
pub const XmCR_OBSCURED_TRAVERSAL: C2RustUnnamed_4 = 46;
pub const XmCR_TEAR_OFF_DEACTIVATE: C2RustUnnamed_4 = 45;
pub const XmCR_TEAR_OFF_ACTIVATE: C2RustUnnamed_4 = 44;
pub const XmCR_CREATE: C2RustUnnamed_4 = 43;
pub const XmCR_LOSE_PRIMARY: C2RustUnnamed_4 = 42;
pub const XmCR_GAIN_PRIMARY: C2RustUnnamed_4 = 41;
pub const XmCR_INPUT: C2RustUnnamed_4 = 40;
pub const XmCR_RESIZE: C2RustUnnamed_4 = 39;
pub const XmCR_EXPOSE: C2RustUnnamed_4 = 38;
pub const XmCR_COMMAND_CHANGED: C2RustUnnamed_4 = 37;
pub const XmCR_COMMAND_ENTERED: C2RustUnnamed_4 = 36;
pub const XmCR_NO_MATCH: C2RustUnnamed_4 = 35;
pub const XmCR_APPLY: C2RustUnnamed_4 = 34;
pub const XmCR_CANCEL: C2RustUnnamed_4 = 32;
pub const XmCR_OK: C2RustUnnamed_4 = 31;
pub const XmCR_CASCADING: C2RustUnnamed_4 = 30;
pub const XmCR_CLIPBOARD_DATA_DELETE: C2RustUnnamed_4 = 29;
pub const XmCR_CLIPBOARD_DATA_REQUEST: C2RustUnnamed_4 = 28;
pub const XmCR_DEFAULT_ACTION: C2RustUnnamed_4 = 27;
pub const XmCR_BROWSE_SELECT: C2RustUnnamed_4 = 26;
pub const XmCR_EXTENDED_SELECT: C2RustUnnamed_4 = 25;
pub const XmCR_MULTIPLE_SELECT: C2RustUnnamed_4 = 24;
pub const XmCR_SINGLE_SELECT: C2RustUnnamed_4 = 23;
pub const XmCR_EXECUTE: C2RustUnnamed_4 = 22;
pub const XmCR_MOVING_INSERT_CURSOR: C2RustUnnamed_4 = 21;
pub const XmCR_MODIFYING_TEXT_VALUE: C2RustUnnamed_4 = 20;
pub const XmCR_LOSING_FOCUS: C2RustUnnamed_4 = 19;
pub const XmCR_FOCUS: C2RustUnnamed_4 = 18;
pub const XmCR_UNMAP: C2RustUnnamed_4 = 17;
pub const XmCR_MAP: C2RustUnnamed_4 = 16;
pub const XmCR_DISARM: C2RustUnnamed_4 = 12;
pub const XmCR_ARM: C2RustUnnamed_4 = 11;
pub const XmCR_ACTIVATE: C2RustUnnamed_4 = 10;
pub const XmCR_DRAG: C2RustUnnamed_4 = 9;
pub const XmCR_TO_BOTTOM: C2RustUnnamed_4 = 8;
pub const XmCR_TO_TOP: C2RustUnnamed_4 = 7;
pub const XmCR_PAGE_DECREMENT: C2RustUnnamed_4 = 6;
pub const XmCR_PAGE_INCREMENT: C2RustUnnamed_4 = 5;
pub const XmCR_DECREMENT: C2RustUnnamed_4 = 4;
pub const XmCR_INCREMENT: C2RustUnnamed_4 = 3;
pub const XmCR_VALUE_CHANGED: C2RustUnnamed_4 = 2;
pub const XmCR_HELP: C2RustUnnamed_4 = 1;
pub const XmCR_NONE: C2RustUnnamed_4 = 0;
pub const XmBROWSE_SELECT: C2RustUnnamed_5 = 3;
pub const XmEXTENDED_SELECT: C2RustUnnamed_5 = 2;
pub const XmMULTIPLE_SELECT: C2RustUnnamed_5 = 1;
pub const XmSINGLE_SELECT: C2RustUnnamed_5 = 0;
pub const XmRESIZE_IF_POSSIBLE: C2RustUnnamed_6 = 2;
pub const XmCONSTANT: C2RustUnnamed_6 = 1;
pub const XmVARIABLE: C2RustUnnamed_6 = 0;
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
pub const WmNfeedback: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"feedback\0") };
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_CONTEXT_ICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const WINDOW_TITLE_MAX: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
static mut windowLabelString: XmString = NULL as XmString;
static mut iconLabelString: XmString = NULL as XmString;
#[no_mangle]
pub unsafe extern "C" fn MakePresenceBox(mut pSD: *mut WmScreenData) -> Boolean {
    let mut pPres: PtrWsPresenceData = &raw mut (*pSD).presence;
    let mut rval: Boolean = 0;
    (*pPres).onScreen = False as Boolean;
    if wspCreateWidgets(pSD) != 0 {
        wspLayout(pPres);
        XtRealizeWidget((*pPres).shellW);
        ProcessPresenceResources(pSD);
        rval = True as Boolean;
    } else {
        Warning(
            b"Unable to create Occupy Workspace dialog.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        rval = False as Boolean;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn ShowPresenceBox(mut pClient: *mut ClientData, mut wsContext: Context) {
    let mut pPres = ::core::ptr::null_mut::<_WsPresenceData>();
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    pPres = &raw mut (*(*pClient).pSD).presence as PtrWsPresenceData;
    pSD = (*pClient).pSD;
    (*pPres).pCDforClient = pClient as *mut _ClientData;
    (*pPres).contextForClient = wsContext;
    if (*pSD).presence.shellW.is_null() {
        (*pPres).ItemSelected = ::core::ptr::null_mut::<Boolean>();
        (*pPres).ItemStrings = ::core::ptr::null_mut::<XmString>();
        (*pPres).currentWsItem = 0 as ::core::ffi::c_int;
        MakePresenceBox(pSD);
    }
    if (*pPres).onScreen != 0 {
        HidePresenceBox(pSD, True as Boolean);
    }
    XmProcessTraversal((*pPres).workspaceListW, XmTRAVERSE_CURRENT);
    wspUpdateWorkspaceList(pPres);
    wspSetPosition(pPres);
    XtPopup((*pPres).shellW, XtGrabNone);
    (*pPres).onScreen = True as Boolean;
}
unsafe extern "C" fn wspSetWindowName(mut pPres: PtrWsPresenceData) {
    let mut title = ::core::ptr::null_mut::<__XmStringRec>();
    let mut nameArgs: [Arg; 1] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 1];
    let mut labelArgs: [Arg; 1] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 1];
    if (*pPres).contextForClient == F_CONTEXT_ICON as Context {
        title = ShortenXmString(
            if !(*(*pPres).pCDforClient).ewmhIconTitle.is_null() {
                (*(*pPres).pCDforClient).ewmhIconTitle
            } else {
                (*(*pPres).pCDforClient).iconTitle
            },
            WINDOW_TITLE_MAX as size_t,
            False as Boolean,
        );
        nameArgs[0 as ::core::ffi::c_int as usize].name =
            (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(7535 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as String_0;
        nameArgs[0 as ::core::ffi::c_int as usize].value = title as XtArgVal;
        labelArgs[0 as ::core::ffi::c_int as usize].name =
            (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(7535 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as String_0;
        labelArgs[0 as ::core::ffi::c_int as usize].value = iconLabelString as XtArgVal;
    } else {
        title = ShortenXmString(
            if !(*(*pPres).pCDforClient).ewmhClientTitle.is_null() {
                (*(*pPres).pCDforClient).ewmhClientTitle
            } else {
                (*(*pPres).pCDforClient).clientTitle
            },
            WINDOW_TITLE_MAX as size_t,
            False as Boolean,
        );
        nameArgs[0 as ::core::ffi::c_int as usize].name =
            (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(7535 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as String_0;
        nameArgs[0 as ::core::ffi::c_int as usize].value = title as XtArgVal;
        labelArgs[0 as ::core::ffi::c_int as usize].name =
            (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(7535 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as String_0;
        labelArgs[0 as ::core::ffi::c_int as usize].value = windowLabelString as XtArgVal;
    }
    XtSetValues(
        (*pPres).windowNameW,
        &raw mut nameArgs as ArgList,
        1 as Cardinal,
    );
    XtSetValues(
        (*pPres).windowLabelW,
        &raw mut labelArgs as ArgList,
        1 as Cardinal,
    );
    XmStringFree(title);
}
#[no_mangle]
pub unsafe extern "C" fn HidePresenceBox(mut pSD: *mut WmScreenData, mut userDismissed: Boolean) {
    if (*pSD).presence.onScreen != 0 {
        XtPopdown((*pSD).presence.shellW);
        XWithdrawWindow(
            wmGD.display,
            XtWindowOfObject((*pSD).presence.shellW),
            (*pSD).screen,
        );
        XSync(wmGD.display, False);
        (*pSD).presence.onScreen = False as Boolean;
        (*pSD).presence.userDismissed = userDismissed;
    }
}
unsafe extern "C" fn wspCreateWidgets(mut pSD: *mut WmScreenData) -> Boolean {
    let mut pPres: PtrWsPresenceData = &raw mut (*pSD).presence;
    let mut args: [Arg; 5] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 5];
    let mut n: ::core::ffi::c_int = 0;
    let mut rval: Boolean = 0;
    let mut tmpXmString = ::core::ptr::null_mut::<__XmStringRec>();
    (*pPres).shellW = wspCreateShell(pSD);
    rval = ((*pPres).shellW != NULL as Widget) as ::core::ffi::c_int as Boolean;
    if rval != 0 {
        (*pPres).formW = wspCreateManager((*pPres).shellW);
        rval = ((*pPres).formW != NULL as Widget) as ::core::ffi::c_int as Boolean;
    }
    if rval != 0 {
        (*pPres).windowLabelW = wspCreateLabel(
            (*pPres).formW,
            b"window\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        );
        rval = ((*pPres).windowLabelW != NULL as Widget) as ::core::ffi::c_int as Boolean;
    }
    if rval != 0 {
        (*pPres).windowNameW = wspCreateLabel(
            (*pPres).formW,
            b"windowName\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        );
        rval = ((*pPres).windowNameW != NULL as Widget) as ::core::ffi::c_int as Boolean;
    }
    if rval != 0 {
        (*pPres).workspaceLabelW = wspCreateLabel(
            (*pPres).formW,
            b"workspaces\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            b"Workspaces: \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
        );
        rval = ((*pPres).workspaceLabelW != NULL as Widget) as ::core::ffi::c_int as Boolean;
    }
    if rval != 0 {
        (*pPres).workspaceListW = wspCreateWorkspaceList((*pPres).formW, pPres, pSD);
        rval = ((*pPres).workspaceListW != NULL as Widget) as ::core::ffi::c_int as Boolean;
        (*pPres).workspaceScrolledListW = (*((*pPres).workspaceListW as Object)).object.parent;
    }
    if rval != 0 {
        (*pPres).allWsW = wspCreateToggleButton(
            (*pPres).formW,
            b"All Workspaces\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
        );
        rval = ((*pPres).allWsW != NULL as Widget) as ::core::ffi::c_int as Boolean;
        XtAddCallback(
            (*pPres).allWsW,
            (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(10591 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        Widget,
                        *mut WmScreenData,
                        *mut XmToggleButtonCallbackStruct,
                    ) -> (),
                >,
                XtCallbackProc,
            >(Some(
                wspAllWsCB
                    as unsafe extern "C" fn(
                        Widget,
                        *mut WmScreenData,
                        *mut XmToggleButtonCallbackStruct,
                    ) -> (),
            )),
            pSD as XtPointer,
        );
    }
    if rval != 0 {
        (*pPres).sepW = wspCreateSeparator((*pPres).formW);
        rval = ((*pPres).sepW != NULL as Widget) as ::core::ffi::c_int as Boolean;
    }
    if rval != 0 {
        tmpXmString =
            XmStringCreateLocalized(b"OK\0" as *const u8 as *const ::core::ffi::c_char as String_0);
        (*pPres).OkW = wspCreatePushButton(
            (*pPres).formW,
            b"ok\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            tmpXmString,
        );
        rval = ((*pPres).OkW != NULL as Widget) as ::core::ffi::c_int as Boolean;
        XmStringFree(tmpXmString);
        n = 0 as ::core::ffi::c_int;
        args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(5765 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = (*pPres).OkW as XtArgVal;
        n += 1;
        XtSetValues((*pPres).formW, &raw mut args as ArgList, n as Cardinal);
        XtAddCallback(
            (*pPres).OkW,
            (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(4700 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, *mut WmScreenData, XtPointer) -> ()>,
                XtCallbackProc,
            >(Some(
                wspOkCB as unsafe extern "C" fn(Widget, *mut WmScreenData, XtPointer) -> (),
            )),
            pSD as XtPointer,
        );
    }
    if rval != 0 {
        tmpXmString = XmStringCreateLocalized(
            b"Cancel\0" as *const u8 as *const ::core::ffi::c_char as String_0,
        );
        (*pPres).CancelW = wspCreatePushButton(
            (*pPres).formW,
            b"cancel\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            tmpXmString,
        );
        rval = ((*pPres).CancelW != NULL as Widget) as ::core::ffi::c_int as Boolean;
        XmStringFree(tmpXmString);
        XtAddCallback(
            (*pPres).CancelW,
            (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(4700 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()>,
                XtCallbackProc,
            >(Some(
                wspCancelCB as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> (),
            )),
            pSD as XtPointer,
        );
        n = 0 as ::core::ffi::c_int;
        args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(5306 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = (*pPres).CancelW as XtArgVal;
        n += 1;
        XtSetValues((*pPres).formW, &raw mut args as ArgList, n as Cardinal);
    }
    return rval;
}
unsafe extern "C" fn wspCreateShell(mut pSD: *mut WmScreenData) -> Widget {
    let mut setArgs: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut i: ::core::ffi::c_int = 0;
    let mut shellW = ::core::ptr::null_mut::<_WidgetRec>();
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10543 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
    i += 1;
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
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(453 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(8306 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(8291 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(163 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*(*(wmGD.display1 as _XPrivDisplay))
        .screens
        .offset((*pSD).screen as isize))
    .root_depth as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(584 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*(wmGD.display1 as _XPrivDisplay))
        .screens
        .offset((*pSD).screen as isize) as *mut Screen as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(154 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*(*(wmGD.display1 as _XPrivDisplay))
        .screens
        .offset((*pSD).screen as isize))
    .cmap as XtArgVal;
    i += 1;
    shellW = XtCreatePopupShell(
        WmNfeedback.as_ptr(),
        transientShellWidgetClass,
        (*pSD).screenTopLevelW1,
        &raw mut setArgs as *mut Arg as ArgList,
        i as Cardinal,
    );
    return shellW;
}
unsafe extern "C" fn wspCreateManager(mut shellW: Widget) -> Widget {
    let mut setArgs: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut formW = ::core::ptr::null_mut::<_WidgetRec>();
    formW = XmCreateForm(
        shellW,
        b"form\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut setArgs as *mut Arg as ArgList,
        0 as Cardinal,
    );
    XtManageChild(formW);
    return formW;
}
unsafe extern "C" fn wspCreateLabel(
    mut mgrW: Widget,
    mut pchName: *mut ::core::ffi::c_uchar,
    mut pchString: *mut ::core::ffi::c_uchar,
) -> Widget {
    let mut setArgs: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut i: ::core::ffi::c_int = 0;
    let mut labelW = ::core::ptr::null_mut::<_WidgetRec>();
    let mut tmpXmString = NULL as XmString;
    i = 0 as ::core::ffi::c_int;
    if strcmp(
        pchName as *mut ::core::ffi::c_char,
        b"window\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if !windowLabelString.is_null() {
            XmStringFree(windowLabelString);
        }
        if !iconLabelString.is_null() {
            XmStringFree(iconLabelString);
        }
        windowLabelString = XmStringCreateLocalized(
            b"Window: \0" as *const u8 as *const ::core::ffi::c_char as String_0,
        );
        iconLabelString = XmStringCreateLocalized(
            b"Icon: \0" as *const u8 as *const ::core::ffi::c_char as String_0,
        );
        setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(7535 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = windowLabelString as XtArgVal;
        i += 1;
    } else {
        tmpXmString = XmStringCreateLocalized(pchString as String_0);
        setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(7535 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = tmpXmString as XtArgVal;
        i += 1;
    }
    labelW = XmCreateLabelGadget(
        mgrW,
        pchName as *mut ::core::ffi::c_char,
        &raw mut setArgs as *mut Arg,
        i as Cardinal,
    );
    XtManageChild(labelW);
    if !tmpXmString.is_null() {
        XmStringFree(tmpXmString);
    }
    return labelW;
}
unsafe extern "C" fn wspCreateSeparator(mut mgrW: Widget) -> Widget {
    let mut setArgs: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 10];
    let mut i: ::core::ffi::c_int = 0;
    let mut sepW = ::core::ptr::null_mut::<_WidgetRec>();
    i = 0 as ::core::ffi::c_int;
    sepW = XmCreateSeparatorGadget(
        mgrW,
        b"separator\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut setArgs as ArgList,
        i as Cardinal,
    );
    XtManageChild(sepW);
    return sepW;
}
unsafe extern "C" fn wspCreateWorkspaceList(
    mut mgrW: Widget,
    mut pPres: PtrWsPresenceData,
    mut pSD: *mut WmScreenData,
) -> Widget {
    let mut setArgs: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut i: ::core::ffi::c_int = 0;
    let mut listW = ::core::ptr::null_mut::<_WidgetRec>();
    let mut numVisible: ::core::ffi::c_int = 0;
    (*pPres).ItemStrings = XtMalloc(
        ((*pSD).numWorkspaces as usize)
            .wrapping_mul(::core::mem::size_of::<*mut XmString>() as usize) as Cardinal,
    ) as XmStringTable;
    if (*pPres).ItemStrings.is_null() || {
        (*pPres).ItemSelected = XtMalloc(
            ((*pSD).numWorkspaces as usize).wrapping_mul(::core::mem::size_of::<Boolean>() as usize)
                as Cardinal,
        ) as *mut Boolean;
        (*pPres).ItemSelected.is_null()
    } {
        Warning(
            b"Insufficient memory to create Occupy Workspace dialog.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<_WidgetRec>();
    }
    (*pPres).numWorkspaces = (*pSD).numWorkspaces;
    i = 0 as ::core::ffi::c_int;
    while i < (*pSD).numWorkspaces {
        let ref mut fresh1 = *(*pPres).ItemStrings.offset(i as isize);
        *fresh1 = XmStringCopy((*(*pSD).pWS.offset(i as isize)).title);
        if (*(*pSD).pWS.offset(i as isize)).id == (*(*pSD).pActiveWS).id {
            (*pPres).currentWsItem = 1 as ::core::ffi::c_int + i;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7450 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*pSD).numWorkspaces as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7460 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = (*pPres).ItemStrings as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9601 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = XmEXTENDED_SELECT as ::core::ffi::c_int as XtArgVal;
    i += 1;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7705 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = XmRESIZE_IF_POSSIBLE as ::core::ffi::c_int as XtArgVal;
    i += 1;
    listW = XmCreateScrolledList(
        mgrW,
        b"list\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut setArgs as ArgList,
        i as Cardinal,
    );
    XtManageChild(listW);
    if !(*pPres).pCDforClient.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*(*(*pPres).pCDforClient).pSD).numWorkspaces {
            if ClientInWorkspace(
                (*(*(*pPres).pCDforClient).pSD).pWS.offset(i as isize) as *mut WmWorkspaceData,
                (*pPres).pCDforClient as *mut ClientData,
            ) != 0
            {
                XmListSelectPos(listW, i + 1 as ::core::ffi::c_int, TRUE as Boolean);
                *(*pPres).ItemSelected.offset(i as isize) = True as Boolean;
            } else {
                *(*pPres).ItemSelected.offset(i as isize) = False as Boolean;
            }
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10683 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = &raw mut numVisible as XtArgVal;
    i += 1;
    XtGetValues(listW, &raw mut setArgs as ArgList, i as Cardinal);
    if numVisible < MIN_VISIBLE {
        i = 0 as ::core::ffi::c_int;
        setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(10683 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        setArgs[i as usize].value = 6 as ::core::ffi::c_int as XtArgVal;
        i += 1;
        XtSetValues(listW, &raw mut setArgs as ArgList, i as Cardinal);
    }
    XtAddCallback(
        listW,
        (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(6726 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(Widget, XtPointer, *mut XmListCallbackStruct) -> ()>,
            XtCallbackProc,
        >(Some(
            wspExtendedSelectCB
                as unsafe extern "C" fn(Widget, XtPointer, *mut XmListCallbackStruct) -> (),
        )),
        pSD as XtPointer,
    );
    XtAddCallback(
        listW,
        (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(5743 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(Widget, *mut WmScreenData, XtPointer) -> ()>,
            XtCallbackProc,
        >(Some(
            wspOkCB as unsafe extern "C" fn(Widget, *mut WmScreenData, XtPointer) -> (),
        )),
        pSD as XtPointer,
    );
    return listW;
}
pub const MIN_VISIBLE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
unsafe extern "C" fn wspUpdateWorkspaceList(mut pPres: PtrWsPresenceData) {
    let mut n: ::core::ffi::c_int = 0;
    let mut xmsTmp = ::core::ptr::null_mut::<__XmStringRec>();
    let mut pxmsSelected = ::core::ptr::null_mut::<XmString>();
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    let mut args: [Arg; 5] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 5];
    let mut wsnum: ::core::ffi::c_int = 0;
    let mut numSelected: ::core::ffi::c_int = 0;
    pSD = (*(*pPres).pCDforClient).pSD;
    wsnum = 0 as ::core::ffi::c_int;
    while wsnum < (*pSD).numWorkspaces {
        xmsTmp = XmStringCopy((*(*pSD).pWS.offset(wsnum as isize)).title);
        if XmStringCompare(xmsTmp, *(*pPres).ItemStrings.offset(wsnum as isize)) == 0 {
            XmStringFree(*(*pPres).ItemStrings.offset(wsnum as isize));
            let ref mut fresh2 = *(*pPres).ItemStrings.offset(wsnum as isize);
            *fresh2 = xmsTmp;
            XmListDeletePos((*pPres).workspaceListW, 1 as ::core::ffi::c_int + wsnum);
            XmListAddItem(
                (*pPres).workspaceListW,
                xmsTmp,
                1 as ::core::ffi::c_int + wsnum,
            );
        } else {
            XmStringFree(xmsTmp);
        }
        wsnum += 1;
    }
    XmListDeselectAllItems((*pPres).workspaceListW);
    pxmsSelected = XtMalloc(
        ((*(*(*pPres).pCDforClient).pSD).numWorkspaces as usize)
            .wrapping_mul(::core::mem::size_of::<XmString>() as usize) as Cardinal,
    ) as *mut XmString;
    numSelected = 0 as ::core::ffi::c_int;
    wsnum = 0 as ::core::ffi::c_int;
    while wsnum < (*(*(*pPres).pCDforClient).pSD).numWorkspaces {
        if ClientInWorkspace(
            (*(*(*pPres).pCDforClient).pSD).pWS.offset(wsnum as isize) as *mut WmWorkspaceData,
            (*pPres).pCDforClient as *mut ClientData,
        ) != 0
        {
            let fresh3 = numSelected;
            numSelected = numSelected + 1;
            let ref mut fresh4 = *pxmsSelected.offset(fresh3 as isize);
            *fresh4 = *(*pPres).ItemStrings.offset(wsnum as isize);
            *(*pPres).ItemSelected.offset(wsnum as isize) = True as Boolean;
        } else {
            *(*pPres).ItemSelected.offset(wsnum as isize) = False as Boolean;
            (*(*pPres).pCDforClient).putInAll = False as Boolean;
        }
        if (*(*(*(*pPres).pCDforClient).pSD).pActiveWS).id
            == (*(*(*(*pPres).pCDforClient).pSD).pWS.offset(wsnum as isize)).id
        {
            (*pPres).currentWsItem = 1 as ::core::ffi::c_int + wsnum;
        }
        wsnum += 1;
    }
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9546 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = pxmsSelected as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9528 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = numSelected as XtArgVal;
    n += 1;
    XtSetValues(
        (*pPres).workspaceListW,
        &raw mut args as ArgList,
        n as Cardinal,
    );
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9643 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = (*(*pPres).pCDforClient).putInAll as XtArgVal;
    n += 1;
    XtSetValues((*pPres).allWsW, &raw mut args as ArgList, n as Cardinal);
    wspSetWindowName(pPres);
    XtFree(pxmsSelected as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn wspCreateToggleButton(
    mut mgrW: Widget,
    mut pch: *mut ::core::ffi::c_uchar,
) -> Widget {
    let mut setArgs: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut i: ::core::ffi::c_int = 0;
    let mut toggleW = ::core::ptr::null_mut::<_WidgetRec>();
    let mut labelString = ::core::ptr::null_mut::<__XmStringRec>();
    i = 0 as ::core::ffi::c_int;
    labelString = XmStringCreateLocalized(pch as String_0);
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7535 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = labelString as XtArgVal;
    i += 1;
    toggleW = XmCreateToggleButton(
        mgrW,
        pch as *mut ::core::ffi::c_char,
        &raw mut setArgs as *mut Arg,
        i as Cardinal,
    );
    XmStringFree(labelString);
    XtManageChild(toggleW);
    return toggleW;
}
unsafe extern "C" fn wspCreatePushButton(
    mut mgrW: Widget,
    mut name: *mut ::core::ffi::c_char,
    mut label: XmString,
) -> Widget {
    let mut setArgs: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut i: ::core::ffi::c_int = 0;
    let mut pushW = ::core::ptr::null_mut::<_WidgetRec>();
    i = 0 as ::core::ffi::c_int;
    setArgs[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7535 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    setArgs[i as usize].value = label as XtArgVal;
    i += 1;
    pushW = XmCreatePushButton(mgrW, name, &raw mut setArgs as ArgList, i as Cardinal);
    XtManageChild(pushW);
    return pushW;
}
unsafe extern "C" fn wspSetPosition(mut pPres: PtrWsPresenceData) {
    let mut pSD = (*(*pPres).pCDforClient).pSD;
    let mut args: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 10];
    let mut n: ::core::ffi::c_int = 0;
    let mut width: Dimension = 0;
    let mut height: Dimension = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[n as usize].value = &raw mut width as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[n as usize].value = &raw mut height as XtArgVal;
    n += 1;
    XtGetValues((*pPres).shellW, &raw mut args as ArgList, n as Cardinal);
    GetSystemMenuPosition(
        (*pPres).pCDforClient as *mut ClientData,
        &raw mut x,
        &raw mut y,
        width as ::core::ffi::c_uint,
        height as ::core::ffi::c_uint,
        (*pPres).contextForClient,
    );
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
        .offset(220 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[n as usize].value = ::core::ptr::null_mut::<::core::ffi::c_void>() as XtArgVal;
    n += 1;
    if (*pPres).contextForClient != F_CONTEXT_ICON as Context {
        args[n as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
            .offset(80 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = (*(*pPres).pCDforClient).client as XtArgVal;
        n += 1;
    } else if (*pSD).useIconBox as ::core::ffi::c_int != 0
        && !(*(*(*pPres).pCDforClient)
            .pWsList
            .offset((*(*pPres).pCDforClient).currentWsc as isize))
        .pIconBox
        .is_null()
    {
        args[n as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
            .offset(80 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = (*(*(*(*(*pPres).pCDforClient)
            .pWsList
            .offset((*(*pPres).pCDforClient).currentWsc as isize))
        .pIconBox)
            .pCD_iconBox)
            .client as XtArgVal;
        n += 1;
    } else {
        args[n as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
            .offset(80 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
        n += 1;
    }
    args[n as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(773 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[n as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    XtSetValues((*pPres).shellW, &raw mut args as ArgList, n as Cardinal);
}
unsafe extern "C" fn wspLayout(mut pPres: PtrWsPresenceData) {
    let mut args: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10199 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_POSITION as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10296 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7557 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_FORM as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7572 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9111 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_NONE as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5051 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_NONE as ::core::ffi::c_int as XtArgVal;
    n += 1;
    XtSetValues(
        (*pPres).windowLabelW,
        &raw mut args as ArgList,
        n as Cardinal,
    );
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10199 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_WIDGET as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10339 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = (*pPres).windowLabelW as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10286 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 4 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7557 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_FORM as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7572 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9111 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_NONE as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5051 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_NONE as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5068 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    XtSetValues(
        (*pPres).workspaceLabelW,
        &raw mut args as ArgList,
        n as Cardinal,
    );
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10199 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_POSITION as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10296 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7557 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_WIDGET as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7596 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = (*pPres).workspaceLabelW as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7572 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9111 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_FORM as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9127 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5051 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_NONE as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5068 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(4741 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmALIGNMENT_BEGINNING as ::core::ffi::c_int as XtArgVal;
    n += 1;
    XtSetValues(
        (*pPres).windowNameW,
        &raw mut args as ArgList,
        n as Cardinal,
    );
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10199 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_WIDGET as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10339 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = (*pPres).windowNameW as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10286 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 4 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7557 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_WIDGET as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7596 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = (*pPres).workspaceLabelW as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7572 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9111 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_FORM as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9127 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5051 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_NONE as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5068 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    XtSetValues(
        (*pPres).workspaceScrolledListW,
        &raw mut args as ArgList,
        n as Cardinal,
    );
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10199 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_WIDGET as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10339 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = (*pPres).workspaceScrolledListW as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10286 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 8 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7557 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_WIDGET as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7596 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = (*pPres).workspaceLabelW as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7572 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9111 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_FORM as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9127 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(4741 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmALIGNMENT_BEGINNING as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7933 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5051 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_NONE as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5068 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    XtSetValues((*pPres).allWsW, &raw mut args as ArgList, n as Cardinal);
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10199 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_WIDGET as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10339 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = (*pPres).allWsW as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10286 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 10 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7557 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_FORM as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7572 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9111 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_FORM as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9127 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5051 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_NONE as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5068 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    XtSetValues((*pPres).sepW, &raw mut args as ArgList, n as Cardinal);
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10199 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_WIDGET as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10339 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = (*pPres).sepW as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10286 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 10 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7557 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_FORM as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(7572 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5051 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_POSITION as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5081 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 95 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    XtSetValues((*pPres).OkW, &raw mut args as ArgList, n as Cardinal);
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10199 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_WIDGET as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10339 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = (*pPres).sepW as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10286 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 10 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9111 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_FORM as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9127 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5051 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = XmATTACH_POSITION as ::core::ffi::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(5081 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = 95 as ::core::ffi::c_int as XtArgVal;
    n += 1;
    XtSetValues((*pPres).CancelW, &raw mut args as ArgList, n as Cardinal);
}
unsafe extern "C" fn wspOkCB(
    mut buttonW: Widget,
    mut pSD: *mut WmScreenData,
    mut call_data: XtPointer,
) {
    let mut args: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut n: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut selectedItemCount: ::core::ffi::c_int = 0;
    let mut selectedItems = ::core::ptr::null_mut::<XmString>();
    let mut pPres: PtrWsPresenceData = &raw mut (*pSD).presence;
    let mut bAllSelected: Boolean = 0;
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9528 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = &raw mut selectedItemCount as XtArgVal;
    n += 1;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9546 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = &raw mut selectedItems as XtArgVal;
    n += 1;
    XtGetValues(
        (*pPres).workspaceListW,
        &raw mut args as ArgList,
        n as Cardinal,
    );
    n = 0 as ::core::ffi::c_int;
    args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(9643 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[n as usize].value = &raw mut bAllSelected as XtArgVal;
    n += 1;
    XtGetValues((*pPres).allWsW, &raw mut args as ArgList, n as Cardinal);
    if bAllSelected != 0 {
        F_AddToAllWorkspaces(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            (*pPres).pCDforClient as *mut ClientData,
            ::core::ptr::null_mut::<XEvent>(),
        );
    } else if selectedItemCount != 0 {
        n = 0 as ::core::ffi::c_int;
        while n < (*pSD).numWorkspaces {
            *(*pPres).ItemSelected.offset(n as isize) = False as Boolean;
            j = 0 as ::core::ffi::c_int;
            while j < selectedItemCount {
                if XmStringCompare(
                    *selectedItems.offset(j as isize),
                    *(*pPres).ItemStrings.offset(n as isize),
                ) != 0
                {
                    *(*pPres).ItemSelected.offset(n as isize) = True as Boolean;
                }
                j += 1;
            }
            if *(*pPres).ItemSelected.offset(n as isize) == 0
                && ClientInWorkspace(
                    (*pSD).pWS.offset(n as isize) as *mut WmWorkspaceData,
                    (*pPres).pCDforClient as *mut ClientData,
                ) as ::core::ffi::c_int
                    != 0
            {
                RemoveClientFromWorkspaces(
                    (*pPres).pCDforClient as *mut ClientData,
                    &raw mut (*(*pSD).pWS.offset(n as isize)).id,
                    1 as ::core::ffi::c_uint,
                );
            }
            if *(*pPres).ItemSelected.offset(n as isize) as ::core::ffi::c_int != 0
                && ClientInWorkspace(
                    (*pSD).pWS.offset(n as isize) as *mut WmWorkspaceData,
                    (*pPres).pCDforClient as *mut ClientData,
                ) == 0
            {
                AddClientToWorkspaces(
                    (*pPres).pCDforClient as *mut ClientData,
                    &raw mut (*(*pSD).pWS.offset(n as isize)).id,
                    1 as ::core::ffi::c_uint,
                );
            }
            n += 1;
        }
    }
    wspCancelCB(buttonW, pSD as XtPointer, call_data);
}
unsafe extern "C" fn wspAllWsCB(
    mut buttonW: Widget,
    mut pSD: *mut WmScreenData,
    mut xmTbcs: *mut XmToggleButtonCallbackStruct,
) {
    let mut pPres: PtrWsPresenceData = &raw mut (*pSD).presence;
    let mut wsnum: ::core::ffi::c_int = 0;
    let mut pxmsSelected = ::core::ptr::null_mut::<XmString>();
    let mut args: [Arg; 5] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 5];
    let mut n: ::core::ffi::c_int = 0;
    if (*xmTbcs).reason == XmCR_VALUE_CHANGED as ::core::ffi::c_int {
        if (*xmTbcs).set != 0 {
            pxmsSelected = XtMalloc(
                ((*pSD).numWorkspaces as usize)
                    .wrapping_mul(::core::mem::size_of::<XmString>() as usize)
                    as Cardinal,
            ) as *mut XmString;
            wsnum = 0 as ::core::ffi::c_int;
            while wsnum < (*pSD).numWorkspaces {
                let ref mut fresh0 = *pxmsSelected.offset(wsnum as isize);
                *fresh0 = *(*pPres).ItemStrings.offset(wsnum as isize);
                wsnum += 1;
            }
            n = 0 as ::core::ffi::c_int;
            args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(9546 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            args[n as usize].value = pxmsSelected as XtArgVal;
            n += 1;
            args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(9528 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            args[n as usize].value = (*pSD).numWorkspaces as XtArgVal;
            n += 1;
            XtSetValues(
                (*pPres).workspaceListW,
                &raw mut args as ArgList,
                n as Cardinal,
            );
            XtFree(pxmsSelected as *mut ::core::ffi::c_char);
        } else {
            XmListDeselectAllItems((*pPres).workspaceListW);
            XmListSelectPos(
                (*pPres).workspaceListW,
                (*pPres).currentWsItem,
                TRUE as Boolean,
            );
        }
    }
}
unsafe extern "C" fn wspExtendedSelectCB(
    mut w: Widget,
    mut client_data: XtPointer,
    mut cb: *mut XmListCallbackStruct,
) {
    let mut pSD = client_data as *mut WmScreenData;
    let mut pPres: PtrWsPresenceData = &raw mut (*pSD).presence;
    let mut n: ::core::ffi::c_int = 0;
    let mut args: [Arg; 5] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 5];
    if (*cb).reason == XmCR_EXTENDED_SELECT as ::core::ffi::c_int {
        if (*cb).selected_item_count < (*pSD).numWorkspaces && !(*cb).event.is_null() {
            n = 0 as ::core::ffi::c_int;
            args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(9643 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as String_0;
            args[n as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
            n += 1;
            XtSetValues((*pPres).allWsW, &raw mut args as ArgList, n as Cardinal);
        }
    }
}
unsafe extern "C" fn wspCancelCB(
    mut buttonW: Widget,
    mut client_data: XtPointer,
    mut call_data: XtPointer,
) {
    let mut pSD = client_data as *mut WmScreenData;
    XtPopdown((*pSD).presence.shellW);
    (*pSD).presence.onScreen = False as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn GetPresenceBoxMenuItems(mut pSD: *mut WmScreenData) -> *mut MenuItem {
    return ::core::ptr::null_mut::<MenuItem>();
}
#[no_mangle]
pub unsafe extern "C" fn UpdatePresenceWorkspaces(mut pSD: *mut WmScreenData) {
    let mut pPres: PtrWsPresenceData = &raw mut (*pSD).presence;
    let mut wsnum: ::core::ffi::c_int = 0;
    let mut xmsTmp = ::core::ptr::null_mut::<__XmStringRec>();
    if !(*pPres).shellW.is_null() {
        if (*pPres).numWorkspaces < (*pSD).numWorkspaces {
            (*pPres).ItemStrings = XtRealloc(
                (*pPres).ItemStrings as *mut ::core::ffi::c_char,
                ((*pSD).numWorkspaces as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut XmString>() as usize)
                    as Cardinal,
            ) as XmStringTable;
            if (*pPres).ItemStrings.is_null() || {
                (*pPres).ItemSelected = XtRealloc(
                    (*pPres).ItemSelected as *mut ::core::ffi::c_char,
                    ((*pSD).numWorkspaces as usize)
                        .wrapping_mul(::core::mem::size_of::<Boolean>() as usize)
                        as Cardinal,
                ) as *mut Boolean;
                (*pPres).ItemSelected.is_null()
            } {
                Warning(
                    b"Insufficient memory to create Occupy Workspace dialog.\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                (*pPres).shellW = ::core::ptr::null_mut::<_WidgetRec>();
                return;
            }
        }
        wsnum = 0 as ::core::ffi::c_int;
        while wsnum < (*pPres).numWorkspaces {
            if wsnum < (*pSD).numWorkspaces {
                xmsTmp = XmStringCopy((*(*pSD).pWS.offset(wsnum as isize)).title);
                if XmStringCompare(xmsTmp, *(*pPres).ItemStrings.offset(wsnum as isize)) == 0 {
                    XmStringFree(*(*pPres).ItemStrings.offset(wsnum as isize));
                    let ref mut fresh5 = *(*pPres).ItemStrings.offset(wsnum as isize);
                    *fresh5 = xmsTmp;
                    XmListDeletePos((*pPres).workspaceListW, 1 as ::core::ffi::c_int + wsnum);
                    XmListAddItem(
                        (*pPres).workspaceListW,
                        xmsTmp,
                        1 as ::core::ffi::c_int + wsnum,
                    );
                } else {
                    XmStringFree(xmsTmp);
                }
            } else {
                XmStringFree(*(*pPres).ItemStrings.offset(wsnum as isize));
                let ref mut fresh6 = *(*pPres).ItemStrings.offset(wsnum as isize);
                *fresh6 = ::core::ptr::null_mut::<__XmStringRec>();
                XmListDeletePos((*pPres).workspaceListW, 1 as ::core::ffi::c_int + wsnum);
            }
            wsnum += 1;
        }
        while wsnum < (*pSD).numWorkspaces {
            xmsTmp = XmStringCopy((*(*pSD).pWS.offset(wsnum as isize)).title);
            let ref mut fresh7 = *(*pPres).ItemStrings.offset(wsnum as isize);
            *fresh7 = xmsTmp;
            XmListAddItem(
                (*pPres).workspaceListW,
                xmsTmp,
                1 as ::core::ffi::c_int + wsnum,
            );
            if !(*pPres).pCDforClient.is_null()
                && ClientInWorkspace(
                    (*(*(*pPres).pCDforClient).pSD).pWS.offset(wsnum as isize)
                        as *mut WmWorkspaceData,
                    (*pPres).pCDforClient as *mut ClientData,
                ) as ::core::ffi::c_int
                    != 0
            {
                XmListSelectPos(
                    (*pPres).workspaceListW,
                    1 as ::core::ffi::c_int + wsnum,
                    TRUE as Boolean,
                );
                *(*pPres).ItemSelected.offset(wsnum as isize) = True as Boolean;
            } else {
                *(*pPres).ItemSelected.offset(wsnum as isize) = False as Boolean;
            }
            wsnum += 1;
        }
        (*pPres).numWorkspaces = (*pSD).numWorkspaces;
    }
}
unsafe extern "C" fn ShortenXmString(
    mut xmstr: XmString,
    mut max_chrs: size_t,
    mut ltor: Boolean,
) -> XmString {
    let mut sz = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nbytes: size_t = 0;
    let mut nchrs: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    let mut result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut new_xmstr = ::core::ptr::null_mut::<__XmStringRec>();
    sz = XmStringUnparse(
        xmstr,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        XmMULTIBYTE_TEXT,
        XmMULTIBYTE_TEXT,
        ::core::ptr::null_mut::<XmParseMapping>(),
        0 as Cardinal,
        XmOUTPUT_ALL,
    ) as *mut ::core::ffi::c_char;
    if sz.is_null() {
        return ::core::ptr::null_mut::<__XmStringRec>();
    }
    mblen(::core::ptr::null::<::core::ffi::c_char>(), 0 as size_t);
    nbytes = strlen(sz);
    while *sz.offset(i as isize) != 0 {
        let mut n = mblen(sz.offset(i as isize), nbytes.wrapping_sub(i));
        if n <= 0 as ::core::ffi::c_int {
            break;
        }
        nchrs = nchrs.wrapping_add(1);
        i = i.wrapping_add(n as size_t);
    }
    if nchrs <= max_chrs {
        return XmStringCopy(xmstr);
    }
    mblen(::core::ptr::null::<::core::ffi::c_char>(), 0 as size_t);
    if ltor != 0 {
        let mut i_0: ::core::ffi::c_int = 0;
        let mut n_0: size_t = 0;
        let mut nskip: size_t = nchrs.wrapping_sub(max_chrs.wrapping_sub(3 as size_t));
        n_0 = 0 as size_t;
        i_0 = 0 as ::core::ffi::c_int;
        while n_0 < nskip {
            i_0 += mblen(sz.offset(i_0 as isize), nbytes.wrapping_sub(i_0 as size_t));
            n_0 = n_0.wrapping_add(1);
        }
        nbytes = strlen(sz.offset(i_0 as isize));
        result = malloc(nbytes.wrapping_add(4 as size_t)) as *mut ::core::ffi::c_char;
        if result.is_null() {
            return ::core::ptr::null_mut::<__XmStringRec>();
        }
        sprintf(
            result,
            b"...%s\0" as *const u8 as *const ::core::ffi::c_char,
            sz.offset(i_0 as isize),
        );
    } else {
        let mut i_1: ::core::ffi::c_int = 0;
        let mut n_1: size_t = 0;
        let mut nskip_0: size_t =
            nchrs.wrapping_sub(nchrs.wrapping_sub(max_chrs.wrapping_sub(3 as size_t)));
        n_1 = 0 as size_t;
        i_1 = 0 as ::core::ffi::c_int;
        while n_1 < nskip_0 {
            i_1 += mblen(sz.offset(i_1 as isize), nbytes.wrapping_sub(i_1 as size_t));
            n_1 = n_1.wrapping_add(1);
        }
        result = malloc((i_1 + 4 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
        if result.is_null() {
            return ::core::ptr::null_mut::<__XmStringRec>();
        }
        strncpy(result, sz, i_1 as size_t);
        *result.offset(i_1 as isize) = '\0' as i32 as ::core::ffi::c_char;
        strcat(result, b"...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    new_xmstr = XmStringGenerate(
        result as XtPointer,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        XmMULTIBYTE_TEXT,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    free(result as *mut ::core::ffi::c_void);
    return new_xmstr;
}
