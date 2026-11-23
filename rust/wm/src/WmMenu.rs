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
use crate::abi::C2RustUnnamed_1_WmMenu as C2RustUnnamed_1;
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
    fn XDisplayHeight(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XDisplayWidth(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XUngrabPointer(_: *mut Display, _: Time) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XtManageChildren(_: WidgetList, _: Cardinal);
    fn XtManageChild(_: Widget);
    fn XtUnmanageChild(_: Widget);
    fn XtDispatchEvent(_: *mut XEvent) -> Boolean;
    fn XtIsManaged(_: Widget) -> Boolean;
    fn XtDestroyWidget(_: Widget);
    fn XtSetSensitive(_: Widget, _: Boolean);
    fn XtDisplayOfObject(_: Widget) -> *mut Display;
    fn XtScreenOfObject(_: Widget) -> *mut Screen;
    fn XtWindowOfObject(_: Widget) -> Window;
    fn XtAddCallback(_: Widget, _: *const ::core::ffi::c_char, _: XtCallbackProc, _: XtPointer);
    fn XtCreatePopupShell(
        _: *const ::core::ffi::c_char,
        _: WidgetClass,
        _: Widget,
        _: ArgList,
        _: Cardinal,
    ) -> Widget;
    fn XtCreateWidget(
        _: *const ::core::ffi::c_char,
        _: WidgetClass,
        _: Widget,
        _: ArgList,
        _: Cardinal,
    ) -> Widget;
    fn XtSetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtGetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtRealloc(_: *mut ::core::ffi::c_char, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    static XtStrings: [::core::ffi::c_char; 0];
    static XtShellStrings: [::core::ffi::c_char; 0];
    static mut _XmStrings: [::core::ffi::c_char; 0];
    fn XmStringCreateLocalized(text: String_0) -> XmString;
    fn XmStringFree(string: XmString);
    static mut XmQmotif: XrmQuark;
    static mut _Xm_fastPtr: *mut XmBaseClassExt;
    fn _XmGetClassExtensionPtr(
        listHeadPtr: *mut XmGenericClassExt,
        owner: XrmQuark,
    ) -> *mut XmGenericClassExt;
    static mut wmGD: WmGlobalData;
    fn GetScreenForWindow(win: Window) -> *mut WmScreenData;
    fn FreeMenuItem(menuItem: *mut MenuItem);
    fn XmCreateCascadeButtonGadget(
        parent: Widget,
        name: *mut ::core::ffi::c_char,
        al: ArgList,
        ac: Cardinal,
    ) -> Widget;
    fn XmCreateLabelGadget(
        parent: Widget,
        name: *mut ::core::ffi::c_char,
        arglist: *mut Arg,
        argCount: Cardinal,
    ) -> Widget;
    static mut xmMenuShellWidgetClass: WidgetClass;
    fn XmCreatePushButtonGadget(
        parent: Widget,
        name: *mut ::core::ffi::c_char,
        arglist: ArgList,
        argcount: Cardinal,
    ) -> Widget;
    static mut xmRowColumnWidgetClass: WidgetClass;
    fn XmMenuPosition(p: Widget, event: *mut XButtonPressedEvent);
    static mut xmSeparatorGadgetClass: WidgetClass;
    fn XmCreateSeparatorGadget(
        parent: Widget,
        name: *mut ::core::ffi::c_char,
        arglist: ArgList,
        argcount: Cardinal,
    ) -> Widget;
    fn GetSystemMenuPosition(
        pcd: *mut ClientData,
        px: *mut ::core::ffi::c_int,
        py: *mut ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        context: Context,
    );
    fn PopGadgetOut(pcd: *mut ClientData, gadget: ::core::ffi::c_int);
    fn ForceColormapFocus(pSD: *mut WmScreenData, pCD: *mut ClientData);
    fn PullExposureEvents();
    fn F_Circle_Down(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Circle_Up(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Menu(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Send_Msg(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Set_Context(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Separator(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Title(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn IconVisible(pCD: *mut ClientData) -> Boolean;
    fn MakeCachedLabelPixmap(
        pSD: *mut WmScreenData,
        menuW: Widget,
        bitmapIndex: ::core::ffi::c_int,
    ) -> Pixmap;
    fn Warning(message: *mut ::core::ffi::c_char);
    fn GetXineramaScreenFromLocation(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
    fn F_Remove(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const XmSYNTHETIC_LOAD: XmImportOperator = 1;
pub const XmSYNTHETIC_NONE: XmImportOperator = 0;
pub const XmHORIZONTAL: C2RustUnnamed_2 = 2;
pub const XmVERTICAL: C2RustUnnamed_2 = 1;
pub const XmNO_ORIENTATION: C2RustUnnamed_2 = 0;
pub const XmMENU_OPTION: C2RustUnnamed_3 = 4;
pub const XmMENU_POPUP: C2RustUnnamed_3 = 3;
pub const XmMENU_PULLDOWN: C2RustUnnamed_3 = 2;
pub const XmMENU_BAR: C2RustUnnamed_3 = 1;
pub const XmWORK_AREA: C2RustUnnamed_3 = 0;
pub const XmPACK_NONE: C2RustUnnamed_4 = 3;
pub const XmPACK_COLUMN: C2RustUnnamed_4 = 2;
pub const XmPACK_TIGHT: C2RustUnnamed_4 = 1;
pub const XmNO_PACKING: C2RustUnnamed_4 = 0;
pub const XmALIGNMENT_UNSPECIFIED: C2RustUnnamed_5 = 3;
pub const XmALIGNMENT_END: C2RustUnnamed_5 = 2;
pub const XmALIGNMENT_CENTER: C2RustUnnamed_5 = 1;
pub const XmALIGNMENT_BEGINNING: C2RustUnnamed_5 = 0;
pub const XmINVALID_SEPARATOR_TYPE: C2RustUnnamed_6 = 9;
pub const XmSHADOW_ETCHED_OUT_DASH: C2RustUnnamed_6 = 8;
pub const XmSHADOW_ETCHED_IN_DASH: C2RustUnnamed_6 = 7;
pub const XmSHADOW_ETCHED_OUT: C2RustUnnamed_6 = 6;
pub const XmSHADOW_ETCHED_IN: C2RustUnnamed_6 = 5;
pub const XmDOUBLE_DASHED_LINE: C2RustUnnamed_6 = 4;
pub const XmSINGLE_DASHED_LINE: C2RustUnnamed_6 = 3;
pub const XmDOUBLE_LINE: C2RustUnnamed_6 = 2;
pub const XmSINGLE_LINE: C2RustUnnamed_6 = 1;
pub const XmNO_LINE: C2RustUnnamed_6 = 0;
pub const XmPIXMAP_AND_STRING: C2RustUnnamed_7 = 3;
pub const XmSTRING: C2RustUnnamed_7 = 2;
pub const XmPIXMAP: C2RustUnnamed_7 = 1;
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
pub const XmMENU_TEAR_OFF_ARM: C2RustUnnamed_8 = 18;
pub const XmMENU_GET_LAST_SELECT_TOPLEVEL: C2RustUnnamed_8 = 17;
pub const XmMENU_RESTORE_TEAROFF_TO_MENUSHELL: C2RustUnnamed_8 = 16;
pub const XmMENU_RESTORE_TEAROFF_TO_TOPLEVEL_SHELL: C2RustUnnamed_8 = 15;
pub const XmMENU_RESTORE_EXCLUDED_TEAROFF_TO_TOPLEVEL_SHELL: C2RustUnnamed_8 = 14;
pub const XmMENU_BUTTON_POPDOWN: C2RustUnnamed_8 = 13;
pub const XmMENU_MEMWIDGET_UPDATE: C2RustUnnamed_8 = 12;
pub const XmMENU_STATUS: C2RustUnnamed_8 = 11;
pub const XmMENU_BAR_CLEANUP: C2RustUnnamed_8 = 10;
pub const XmMENU_DISARM: C2RustUnnamed_8 = 9;
pub const XmMENU_ARM: C2RustUnnamed_8 = 8;
pub const XmMENU_SUBMENU: C2RustUnnamed_8 = 7;
pub const XmMENU_CASCADING: C2RustUnnamed_8 = 6;
pub const XmMENU_BUTTON: C2RustUnnamed_8 = 5;
pub const XmMENU_CALLBACK: C2RustUnnamed_8 = 4;
pub const XmMENU_SHELL_POPDOWN: C2RustUnnamed_8 = 3;
pub const XmMENU_TRAVERSAL: C2RustUnnamed_8 = 2;
pub const XmMENU_PROCESS_TREE: C2RustUnnamed_8 = 1;
pub const XmMENU_POPDOWN: C2RustUnnamed_8 = 0;
pub const XmLEAVE: XmFocusChange = 3;
pub const XmENTER: XmFocusChange = 2;
pub const XmFOCUS_OUT: XmFocusChange = 1;
pub const XmFOCUS_IN: XmFocusChange = 0;
pub const XmDESCENDANTS_TAB_NAVIGABLE: XmNavigability = 4;
pub const XmDESCENDANTS_NAVIGABLE: XmNavigability = 3;
pub const XmTAB_NAVIGABLE: XmNavigability = 2;
pub const XmCONTROL_NAVIGABLE: XmNavigability = 1;
pub const XmNOT_NAVIGABLE: XmNavigability = 0;
pub const XmFAST_SUBCLASS_TAIL_BIT: C2RustUnnamed_9 = 70;
pub const XmDROP_DOWN_BIT: C2RustUnnamed_9 = 69;
pub const XmCOMBINATION_BOX_2_BIT: C2RustUnnamed_9 = 69;
pub const XmFONTSELECTOR_BIT: C2RustUnnamed_9 = 68;
pub const XmCOLUMN_BIT: C2RustUnnamed_9 = 67;
pub const XmTABBOX_BIT: C2RustUnnamed_9 = 66;
pub const XmTREE_BIT: C2RustUnnamed_9 = 65;
pub const XmTABSTACK_BIT: C2RustUnnamed_9 = 64;
pub const XmTABLE_BIT: C2RustUnnamed_9 = 63;
pub const XmICONBUTTON_BIT: C2RustUnnamed_9 = 62;
pub const XmICONBOX_BIT: C2RustUnnamed_9 = 61;
pub const XmCOLORSELECTOR_BIT: C2RustUnnamed_9 = 60;
pub const XmMULTI_LIST_BIT: C2RustUnnamed_9 = 59;
pub const XmEXT18LIST_BIT: C2RustUnnamed_9 = 59;
pub const XmI18LIST_BIT: C2RustUnnamed_9 = 58;
pub const XmDATAFIELD_BIT: C2RustUnnamed_9 = 57;
pub const XmBUTTONBOX_BIT: C2RustUnnamed_9 = 56;
pub const XmICONHEADER_BIT: C2RustUnnamed_9 = 55;
pub const XmSPINBOX_BIT: C2RustUnnamed_9 = 54;
pub const XmCOMBO_BOX_BIT: C2RustUnnamed_9 = 53;
pub const XmGRAB_SHELL_BIT: C2RustUnnamed_9 = 52;
pub const XmCSTEXT_BIT: C2RustUnnamed_9 = 51;
pub const XmNOTEBOOK_BIT: C2RustUnnamed_9 = 50;
pub const XmICONGADGET_BIT: C2RustUnnamed_9 = 49;
pub const XmCONTAINER_BIT: C2RustUnnamed_9 = 48;
pub const XmDRAG_CONTEXT_BIT: C2RustUnnamed_9 = 47;
pub const XmDRAG_OVER_SHELL_BIT: C2RustUnnamed_9 = 46;
pub const XmTEAROFF_BUTTON_BIT: C2RustUnnamed_9 = 45;
pub const XmDRAG_ICON_BIT: C2RustUnnamed_9 = 44;
pub const XmCLIP_WINDOW_BIT: C2RustUnnamed_9 = 43;
pub const XmVENDOR_SHELL_BIT: C2RustUnnamed_9 = 42;
pub const XmPANED_WINDOW_BIT: C2RustUnnamed_9 = 41;
pub const XmSCALE_BIT: C2RustUnnamed_9 = 40;
pub const XmSASH_BIT: C2RustUnnamed_9 = 39;
pub const XmMESSAGE_BOX_BIT: C2RustUnnamed_9 = 38;
pub const XmFRAME_BIT: C2RustUnnamed_9 = 37;
pub const XmFILE_SELECTION_BOX_BIT: C2RustUnnamed_9 = 36;
pub const XmDRAWING_AREA_BIT: C2RustUnnamed_9 = 35;
pub const XmBULLETIN_BOARD_BIT: C2RustUnnamed_9 = 34;
pub const XmARROW_BUTTON_GADGET_BIT: C2RustUnnamed_9 = 33;
pub const XmARROW_BUTTON_BIT: C2RustUnnamed_9 = 32;
pub const XmPRINT_SHELL_BIT: C2RustUnnamed_9 = 31;
pub const XmSCREEN_BIT: C2RustUnnamed_9 = 30;
pub const XmDISPLAY_BIT: C2RustUnnamed_9 = 29;
pub const XmDROP_SITE_MANAGER_BIT: C2RustUnnamed_9 = 28;
pub const XmDROP_TRANSFER_BIT: C2RustUnnamed_9 = 27;
pub const XmTOGGLE_BUTTON_GADGET_BIT: C2RustUnnamed_9 = 26;
pub const XmTOGGLE_BUTTON_BIT: C2RustUnnamed_9 = 25;
pub const XmTEXT_BIT: C2RustUnnamed_9 = 24;
pub const XmSEPARATOR_GADGET_BIT: C2RustUnnamed_9 = 23;
pub const XmSEPARATOR_BIT: C2RustUnnamed_9 = 22;
pub const XmSELECTION_BOX_BIT: C2RustUnnamed_9 = 21;
pub const XmSCROLLED_WINDOW_BIT: C2RustUnnamed_9 = 20;
pub const XmSCROLL_BAR_BIT: C2RustUnnamed_9 = 19;
pub const XmROW_COLUMN_BIT: C2RustUnnamed_9 = 18;
pub const XmPUSH_BUTTON_GADGET_BIT: C2RustUnnamed_9 = 17;
pub const XmPUSH_BUTTON_BIT: C2RustUnnamed_9 = 16;
pub const XmPRIMITIVE_BIT: C2RustUnnamed_9 = 15;
pub const XmDRAWN_BUTTON_BIT: C2RustUnnamed_9 = 14;
pub const XmMENU_SHELL_BIT: C2RustUnnamed_9 = 13;
pub const XmMANAGER_BIT: C2RustUnnamed_9 = 12;
pub const XmMAIN_WINDOW_BIT: C2RustUnnamed_9 = 11;
pub const XmLABEL_GADGET_BIT: C2RustUnnamed_9 = 10;
pub const XmLABEL_BIT: C2RustUnnamed_9 = 9;
pub const XmGADGET_BIT: C2RustUnnamed_9 = 8;
pub const XmTEXT_FIELD_BIT: C2RustUnnamed_9 = 7;
pub const XmFORM_BIT: C2RustUnnamed_9 = 6;
pub const XmLIST_BIT: C2RustUnnamed_9 = 5;
pub const XmDIALOG_SHELL_BIT: C2RustUnnamed_9 = 4;
pub const XmCOMMAND_BOX_BIT: C2RustUnnamed_9 = 3;
pub const XmCASCADE_BUTTON_GADGET_BIT: C2RustUnnamed_9 = 2;
pub const XmCASCADE_BUTTON_BIT: C2RustUnnamed_9 = 1;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const ButtonPress: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ButtonRelease: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const Button1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SELECT_BUTTON: ::core::ffi::c_int = Button1;
pub const NoButton: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POST_AT_XY: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const POST_TRAVERSAL_ON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
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
pub const CR_MENU_MARK: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const FRAME_SYSTEM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICONBOX_PART_IICON: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const ICONBOX_PART_WICON: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const F_SUBCONTEXT_IB_IICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_IICON;
pub const F_SUBCONTEXT_IB_WICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_WICON;
pub const MWM_MGT_MASK: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;
pub const WSM_MGT_MASK: ::core::ffi::c_uint = 0xffff0000 as ::core::ffi::c_uint;
pub const ICON_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const MAXWMPATH: ::core::ffi::c_int = 1023 as ::core::ffi::c_int;
pub const SHELL_NAME: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"menu\0") };
pub const SEPARATOR_NAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"separator\0") };
pub const TITLE_NAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"menuTitle\0") };
pub const CASCADE_BTN_NAME: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"cascadebutton\0") };
pub const PUSH_BTN_NAME: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"pushbutton\0") };
pub const CHILDREN_CACHE: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const MENU_BUTTON_INC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn MakeMenu(
    mut pSD: *mut WmScreenData,
    mut menuName: String_0,
    mut initialContext: Context,
    mut accelContext: Context,
    mut moreMenuItems: *mut MenuItem,
    mut fSystemMenu: Boolean,
) -> *mut MenuSpec {
    let mut n: ::core::ffi::c_uint = 0;
    let mut menuSpec = ::core::ptr::null_mut::<MenuSpec>();
    let mut newMenuSpec = ::core::ptr::null_mut::<MenuSpec>();
    let mut menuItem = ::core::ptr::null_mut::<MenuItem>();
    let mut accelKeySpec = ::core::ptr::null_mut::<KeySpec>();
    if menuName.is_null() || pSD.is_null() {
        return ::core::ptr::null_mut::<MenuSpec>();
    }
    menuSpec = (*pSD).menuSpecs;
    while !menuSpec.is_null() {
        if !(*menuSpec).name.is_null()
            && strcmp(
                (*menuSpec).name as *const ::core::ffi::c_char,
                menuName as *const ::core::ffi::c_char,
            ) == 0
        {
            break;
        }
        menuSpec = (*menuSpec).nextMenuSpec as *mut MenuSpec;
    }
    if menuSpec.is_null() {
        MWarning(
            b"Menu specification %s not found\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            menuName as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<MenuSpec>();
    }
    if fSystemMenu != 0 {
        accelContext = 0 as Context;
    } else if accelContext & F_CONTEXT_ROOT as Context != 0 {
        accelContext = F_CONTEXT_ALL as Context;
    }
    if !moreMenuItems.is_null() {
        newMenuSpec = XtMalloc(::core::mem::size_of::<MenuSpec>() as Cardinal) as *mut MenuSpec;
        if newMenuSpec.is_null() {
            MWarning(
                b"Insufficient memory for menu %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                menuName as *mut ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<MenuSpec>();
        }
        (*newMenuSpec).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*newMenuSpec).whichButton = SELECT_BUTTON as ::core::ffi::c_uint;
        (*newMenuSpec).height = 0 as ::core::ffi::c_uint;
        (*newMenuSpec).width = 0 as ::core::ffi::c_uint;
        (*newMenuSpec).menuItems = (*menuSpec).menuItems;
        (*newMenuSpec).accelContext = (*menuSpec).accelContext;
        (*newMenuSpec).accelKeySpecs = ::core::ptr::null_mut::<KeySpec>();
        (*newMenuSpec).nextMenuSpec = ::core::ptr::null_mut::<_MenuSpec>();
        menuSpec = newMenuSpec;
    } else if !(*menuSpec).menuWidget.is_null() {
        if accelContext != (*menuSpec).accelContext & accelContext {
            (*menuSpec).accelContext |= accelContext;
            accelKeySpec = (*menuSpec).accelKeySpecs;
            while !accelKeySpec.is_null() {
                (*accelKeySpec).context |= accelContext;
                accelKeySpec = (*accelKeySpec).nextKeySpec as *mut KeySpec;
            }
        }
        return menuSpec;
    }
    (*menuSpec).currentContext = initialContext;
    (*menuSpec).accelContext = accelContext;
    n = 0 as ::core::ffi::c_uint;
    menuItem = (*menuSpec).menuItems;
    while !menuItem.is_null() {
        if (*menuItem).greyedContext != 0 || (*menuItem).mgtMask != 0 {
            n = n.wrapping_add(1);
        }
        menuItem = (*menuItem).nextMenuItem as *mut MenuItem;
    }
    menuItem = moreMenuItems;
    while !menuItem.is_null() {
        if (*menuItem).greyedContext != 0 || (*menuItem).mgtMask != 0 {
            n = n.wrapping_add(1);
        }
        menuItem = (*menuItem).nextMenuItem as *mut MenuItem;
    }
    if n != 0 {
        (*menuSpec).menuButtons = XtMalloc(
            (n as usize).wrapping_mul(::core::mem::size_of::<MenuButton>() as usize) as Cardinal,
        ) as *mut MenuButton;
        if (*menuSpec).menuButtons.is_null() {
            MWarning(
                b"Insufficient memory for menu %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                menuName as *mut ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<MenuSpec>();
        }
        (*menuSpec).menuButtonSize = n;
    } else {
        (*menuSpec).menuButtons = ::core::ptr::null_mut::<MenuButton>();
        (*menuSpec).menuButtonSize = 0 as ::core::ffi::c_uint;
    }
    (*menuSpec).menuButtonCount = 0 as ::core::ffi::c_uint;
    (*menuSpec).menuWidget = CreateMenuWidget(
        pSD,
        menuName,
        (*pSD).screenTopLevelW,
        TRUE as Boolean,
        menuSpec,
        moreMenuItems,
    );
    if (*menuSpec).menuWidget.is_null() {
        return ::core::ptr::null_mut::<MenuSpec>();
    }
    return menuSpec;
}
unsafe extern "C" fn CheckTerminalSeparator(
    mut menuSpec: *mut MenuSpec,
    mut buttonWidget: Widget,
    mut manage: Boolean,
) {
    let mut cw = ::core::ptr::null_mut::<_CompositeRec>();
    let mut children = ::core::ptr::null_mut::<Widget>();
    let mut wPos: Cardinal = 0;
    if menuSpec.is_null() || (*menuSpec).menuWidget.is_null() {
        return;
    }
    cw = (*menuSpec).menuWidget as CompositeWidget;
    children = (*cw).composite.children;
    wPos = 0 as Cardinal;
    while wPos < (*cw).composite.num_children {
        if *children.offset(wPos as isize) == buttonWidget {
            break;
        }
        wPos = wPos.wrapping_add(1);
    }
    if wPos > 0 as Cardinal
        && (*(*children.offset(wPos.wrapping_sub(1 as Cardinal) as isize) as Object))
            .object
            .widget_class
            == xmSeparatorGadgetClass
    {
        if manage != 0 {
            if XtIsManaged(*children.offset(wPos.wrapping_sub(1 as Cardinal) as isize)) == 0 {
                XtManageChild(*children.offset(wPos.wrapping_sub(1 as Cardinal) as isize));
            }
        } else if XtIsManaged(*children.offset(wPos.wrapping_sub(1 as Cardinal) as isize)) != 0 {
            XtUnmanageChild(*children.offset(wPos.wrapping_sub(1 as Cardinal) as isize));
        }
    }
}
unsafe extern "C" fn DuplicateMenuItems(mut menuItems: *mut MenuItem) -> *mut MenuItem {
    let mut newMenuItem = NULL as *mut MenuItem;
    let mut returnMenuItem = ::core::ptr::null_mut::<MenuItem>();
    let mut curMenuItem = ::core::ptr::null_mut::<MenuItem>();
    curMenuItem = menuItems;
    while !curMenuItem.is_null() {
        if newMenuItem.is_null() {
            newMenuItem = XtMalloc(::core::mem::size_of::<MenuItem>() as Cardinal) as *mut MenuItem;
            returnMenuItem = newMenuItem;
        } else {
            (*newMenuItem).nextMenuItem = XtMalloc(::core::mem::size_of::<MenuItem>() as Cardinal)
                as *mut MenuItem as *mut _MenuItem;
            newMenuItem = (*newMenuItem).nextMenuItem as *mut MenuItem;
        }
        (*newMenuItem).labelType = (*curMenuItem).labelType;
        if !(*curMenuItem).label.is_null() {
            (*newMenuItem).label = (if !(*curMenuItem).label.is_null() {
                strcpy(
                    XtMalloc(
                        (strlen((*curMenuItem).label as *const ::core::ffi::c_char) as Cardinal)
                            .wrapping_add(1 as Cardinal),
                    ),
                    (*curMenuItem).label as *const ::core::ffi::c_char,
                )
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            }) as String_0;
        } else {
            (*newMenuItem).label = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        (*newMenuItem).labelBitmapIndex = (*curMenuItem).labelBitmapIndex;
        (*newMenuItem).mnemonic = (*curMenuItem).mnemonic;
        (*newMenuItem).accelState = (*curMenuItem).accelState;
        (*newMenuItem).accelKeyCode = (*curMenuItem).accelKeyCode;
        if !(*curMenuItem).accelText.is_null() {
            (*newMenuItem).accelText = (if !(*curMenuItem).accelText.is_null() {
                strcpy(
                    XtMalloc(
                        (strlen((*curMenuItem).accelText as *const ::core::ffi::c_char)
                            as Cardinal)
                            .wrapping_add(1 as Cardinal),
                    ),
                    (*curMenuItem).accelText as *const ::core::ffi::c_char,
                )
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            }) as String_0;
        } else {
            (*newMenuItem).accelText = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        (*newMenuItem).wmFunction = (*curMenuItem).wmFunction;
        if (*curMenuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Send_Msg
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) || (*curMenuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Circle_Up
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) || (*curMenuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Circle_Down
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) || (*curMenuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Set_Context
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) {
            (*newMenuItem).wmFuncArgs = (*curMenuItem).wmFuncArgs;
        } else if !(*curMenuItem).wmFuncArgs.is_null() {
            (*newMenuItem).wmFuncArgs = (if !(*curMenuItem).wmFuncArgs.is_null() {
                strcpy(
                    XtMalloc(
                        (strlen((*curMenuItem).wmFuncArgs as *const ::core::ffi::c_char)
                            as Cardinal)
                            .wrapping_add(1 as Cardinal),
                    ),
                    (*curMenuItem).wmFuncArgs as *const ::core::ffi::c_char,
                )
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            }) as String_0;
        } else {
            (*newMenuItem).wmFuncArgs = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        (*newMenuItem).greyedContext = (*curMenuItem).greyedContext;
        (*newMenuItem).mgtMask = (*curMenuItem).mgtMask;
        (*newMenuItem).nextMenuItem = NULL as *mut MenuItem as *mut _MenuItem;
        curMenuItem = (*curMenuItem).nextMenuItem as *mut MenuItem;
    }
    return returnMenuItem;
}
#[no_mangle]
pub unsafe extern "C" fn DuplicateMenuSpec(mut menuSpec: *mut MenuSpec) -> *mut MenuSpec {
    let mut newMenuSpec = ::core::ptr::null_mut::<MenuSpec>();
    newMenuSpec = XtMalloc(::core::mem::size_of::<MenuSpec>() as Cardinal) as *mut MenuSpec;
    if newMenuSpec.is_null() {
        Warning(
            b"Insufficient memory for menu specification\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<MenuSpec>();
    }
    (*newMenuSpec).name = (if !(*menuSpec).name.is_null() {
        strcpy(
            XtMalloc(
                (strlen((*menuSpec).name as *const ::core::ffi::c_char) as Cardinal)
                    .wrapping_add(1 as Cardinal),
            ),
            (*menuSpec).name as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }) as String_0;
    (*newMenuSpec).currentContext = (*menuSpec).currentContext;
    (*newMenuSpec).menuWidget = NULL as Widget;
    (*newMenuSpec).whichButton = (*menuSpec).whichButton;
    (*newMenuSpec).width = (*menuSpec).width;
    (*newMenuSpec).height = (*menuSpec).height;
    (*newMenuSpec).menuItems = DuplicateMenuItems((*menuSpec).menuItems);
    (*newMenuSpec).menuButtons = NULL as *mut MenuButton;
    (*newMenuSpec).menuButtonSize = 0 as ::core::ffi::c_uint;
    (*newMenuSpec).menuButtonCount = 0 as ::core::ffi::c_uint;
    (*newMenuSpec).accelContext = (*menuSpec).accelContext;
    (*newMenuSpec).accelKeySpecs = NULL as *mut KeySpec;
    (*newMenuSpec).nextMenuSpec = NULL as *mut MenuSpec as *mut _MenuSpec;
    return newMenuSpec;
}
unsafe extern "C" fn AdjustPBs(
    mut menuSpec: *mut MenuSpec,
    mut pCD: *mut ClientData,
    mut newContext: Context,
) -> Boolean {
    let mut menuButton = ::core::ptr::null_mut::<MenuButton>();
    let mut menuItem = ::core::ptr::null_mut::<MenuItem>();
    let mut msgc: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut pMsg = ::core::ptr::null_mut::<::core::ffi::c_long>();
    let mut fSupported: Boolean = 0;
    let mut fChangeManaged = FALSE as Boolean;
    if menuSpec.is_null() {
        return 0 as Boolean;
    }
    n = 0 as ::core::ffi::c_uint;
    menuButton = (*menuSpec).menuButtons;
    while n < (*menuSpec).menuButtonCount {
        menuItem = (*menuButton).menuItem;
        if (*menuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Send_Msg
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) {
            if newContext & (*menuItem).greyedContext != 0
                || !(!pCD.is_null()
                    && (*pCD).mwmMessagesCount != 0
                    && !(*pCD).mwmMessages.is_null())
            {
                XtSetSensitive((*menuButton).buttonWidget, FALSE as Boolean);
            } else {
                msgc = (*pCD).mwmMessagesCount;
                pMsg = (*pCD).mwmMessages;
                fSupported = FALSE as Boolean;
                loop {
                    let fresh0 = msgc;
                    msgc = msgc - 1;
                    if !(fresh0 != 0) {
                        break;
                    }
                    if *pMsg == (*menuItem).wmFuncArgs as ::core::ffi::c_long {
                        fSupported = TRUE as Boolean;
                        break;
                    } else {
                        pMsg = pMsg.offset(1);
                    }
                }
                XtSetSensitive((*menuButton).buttonWidget, fSupported);
            }
        } else {
            if (*menuSpec).currentContext & (*menuItem).greyedContext != 0 {
                if newContext & (*menuItem).greyedContext == 0 {
                    XtSetSensitive((*menuButton).buttonWidget, TRUE as Boolean);
                }
            } else if newContext & (*menuItem).greyedContext != 0 {
                XtSetSensitive((*menuButton).buttonWidget, FALSE as Boolean);
            }
            if (*menuItem).wmFunction.is_some_and(|action| {
                std::ptr::fn_addr_eq(
                    action,
                    F_Remove
                        as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
                )
            }) {
                fSupported = (!pCD.is_null() && (*pCD).numInhabited > 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as Boolean;
                XtSetSensitive((*menuButton).buttonWidget, fSupported);
            }
            if (*menuItem).mgtMask != 0 && !pCD.is_null() {
                if (*pCD).clientFunctions as ::core::ffi::c_long
                    & (*menuItem).mgtMask
                    & MWM_MGT_MASK as ::core::ffi::c_long
                    != 0
                    || (*pCD).wsmFunctions
                        & (*menuItem).mgtMask
                        & WSM_MGT_MASK as ::core::ffi::c_long
                        != 0
                {
                    if (*menuButton).managed == 0 {
                        XtManageChild((*menuButton).buttonWidget);
                        (*menuButton).managed = TRUE as Boolean;
                        fChangeManaged = TRUE as Boolean;
                        if n == (*menuSpec)
                            .menuButtonCount
                            .wrapping_sub(1 as ::core::ffi::c_uint)
                        {
                            CheckTerminalSeparator(
                                menuSpec,
                                (*menuButton).buttonWidget,
                                True as Boolean,
                            );
                        }
                    }
                } else if (*menuButton).managed != 0 {
                    XtUnmanageChild((*menuButton).buttonWidget);
                    (*menuButton).managed = FALSE as Boolean;
                    fChangeManaged = TRUE as Boolean;
                    if n == (*menuSpec)
                        .menuButtonCount
                        .wrapping_sub(1 as ::core::ffi::c_uint)
                    {
                        CheckTerminalSeparator(
                            menuSpec,
                            (*menuButton).buttonWidget,
                            False as Boolean,
                        );
                    }
                }
            } else if (*menuButton).managed == 0 {
                XtManageChild((*menuButton).buttonWidget);
                (*menuButton).managed = TRUE as Boolean;
                fChangeManaged = TRUE as Boolean;
            }
        }
        n = n.wrapping_add(1);
        menuButton = menuButton.offset(1);
    }
    return fChangeManaged;
}
unsafe extern "C" fn SavePBInfo(
    mut topMenuSpec: *mut MenuSpec,
    mut menuItem: *mut MenuItem,
    mut itemW: Widget,
) -> Boolean {
    let mut menuButton = ::core::ptr::null_mut::<MenuButton>();
    if topMenuSpec.is_null() {
        return 0 as Boolean;
    }
    if (*topMenuSpec).menuButtonSize == 0 as ::core::ffi::c_uint {
        (*topMenuSpec).menuButtonSize = MENU_BUTTON_INC as ::core::ffi::c_uint;
        (*topMenuSpec).menuButtons = XtMalloc(
            (MENU_BUTTON_INC as usize).wrapping_mul(::core::mem::size_of::<MenuButton>() as usize)
                as Cardinal,
        ) as *mut MenuButton;
    } else if (*topMenuSpec).menuButtonCount == (*topMenuSpec).menuButtonSize {
        (*topMenuSpec).menuButtonSize = (*topMenuSpec)
            .menuButtonSize
            .wrapping_add(MENU_BUTTON_INC as ::core::ffi::c_uint);
        (*topMenuSpec).menuButtons = XtRealloc(
            (*topMenuSpec).menuButtons as *mut ::core::ffi::c_char,
            ((*topMenuSpec).menuButtonSize as usize)
                .wrapping_mul(::core::mem::size_of::<MenuButton>() as usize)
                as Cardinal,
        ) as *mut MenuButton;
    }
    if (*topMenuSpec).menuButtons.is_null() {
        (*topMenuSpec).menuButtonSize = 0 as ::core::ffi::c_uint;
        (*topMenuSpec).menuButtonCount = 0 as ::core::ffi::c_uint;
        return 0 as Boolean;
    }
    menuButton = (*topMenuSpec)
        .menuButtons
        .offset((*topMenuSpec).menuButtonCount as isize) as *mut MenuButton;
    (*topMenuSpec).menuButtonCount = (*topMenuSpec).menuButtonCount.wrapping_add(1);
    (*menuButton).menuItem = menuItem;
    (*menuButton).buttonWidget = itemW;
    (*menuButton).managed = TRUE as Boolean;
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn CreateMenuWidget(
    mut pSD: *mut WmScreenData,
    mut menuName: String_0,
    mut parent: Widget,
    mut fTopLevelPane: Boolean,
    mut topMenuSpec: *mut MenuSpec,
    mut moreMenuItems: *mut MenuItem,
) -> Widget {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut sepArgs: [Arg; 1] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 1];
    let mut args: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 10];
    let mut menuSpec = NULL as *mut MenuSpec;
    let mut menuItem = ::core::ptr::null_mut::<MenuItem>();
    let mut menuShellW = ::core::ptr::null_mut::<_WidgetRec>();
    let mut menuW = ::core::ptr::null_mut::<_WidgetRec>();
    let mut subMenuW = ::core::ptr::null_mut::<_WidgetRec>();
    let mut children: [Widget; 22] = [::core::ptr::null_mut::<_WidgetRec>(); 22];
    let mut labelPixmap: Pixmap = 0;
    let mut accelKeySpec = ::core::ptr::null_mut::<KeySpec>();
    let mut menuHeight: Dimension = 0;
    let mut menuWidth: Dimension = 0;
    let mut fUseTitleSep = False as Boolean;
    let mut stringsToFree = ::core::ptr::null_mut::<StrList>();
    let mut sPtr = ::core::ptr::null_mut::<StrList>();
    let mut tmpstr = ::core::ptr::null_mut::<__XmStringRec>();
    let mut scr = ::core::ptr::null_mut::<Screen>();
    if menuName.is_null() || pSD.is_null() {
        return ::core::ptr::null_mut::<_WidgetRec>();
    }
    if fTopLevelPane != 0 {
        menuSpec = topMenuSpec;
    } else {
        menuSpec = (*pSD).menuSpecs;
        while !menuSpec.is_null() {
            if !(*menuSpec).name.is_null()
                && strcmp(
                    (*menuSpec).name as *const ::core::ffi::c_char,
                    menuName as *const ::core::ffi::c_char,
                ) == 0
            {
                break;
            }
            menuSpec = (*menuSpec).nextMenuSpec as *mut MenuSpec;
        }
    }
    if menuSpec.is_null() {
        MWarning(
            b"Menu specification %s not found\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            menuName as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<_WidgetRec>();
    }
    if (*menuSpec).currentContext & CR_MENU_MARK as Context != 0 {
        MWarning(
            b"Menu recursion detected for %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            menuName as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<_WidgetRec>();
    }
    (*menuSpec).currentContext |= CR_MENU_MARK as Context;
    i = 0 as ::core::ffi::c_int;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    args[i as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(280 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    args[i as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(186 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(163 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = (*(*(XtDisplayOfObject(parent) as _XPrivDisplay))
        .screens
        .offset((*pSD).screen as isize))
    .root_depth as XtArgVal;
    i += 1;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(584 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value =
        (*((XtDisplayOfObject as unsafe extern "C" fn(Widget) -> *mut Display)(parent)
            as _XPrivDisplay))
            .screens
            .offset((*pSD).screen as isize) as *mut Screen as XtArgVal;
    i += 1;
    if !(*(parent as Object)).object.parent.is_null() && {
        _Xm_fastPtr = (if !((*(*((*(parent as Object)).object.parent as Object))
            .object
            .widget_class)
            .core_class
            .extension as XmBaseClassExt)
            .is_null()
            && (*((*(*((*(parent as Object)).object.parent as Object))
                .object
                .widget_class)
                .core_class
                .extension as XmBaseClassExt))
                .record_type
                == XmQmotif
        {
            &raw mut (*(*((*(parent as Object)).object.parent as Object))
                .object
                .widget_class)
                .core_class
                .extension as *mut XmBaseClassExt
        } else {
            _XmGetClassExtensionPtr(
                &raw mut (*(*((*(parent as Object)).object.parent as Object))
                    .object
                    .widget_class)
                    .core_class
                    .extension as *mut XmBaseClassExt as *mut XmGenericClassExt,
                XmQmotif,
            ) as *mut XmBaseClassExt
        });
        (if !_Xm_fastPtr.is_null() && !(*_Xm_fastPtr).is_null() {
            (if (**_Xm_fastPtr).flags
                [(XmMENU_SHELL_BIT as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int)
                    << (XmMENU_SHELL_BIT as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int)
                != 0
            {
                TRUE
            } else {
                FALSE
            })
        } else {
            FALSE
        }) != 0
    } {
        parent = (*(parent as Object)).object.parent;
    }
    menuShellW = XtCreatePopupShell(
        SHELL_NAME.as_ptr(),
        xmMenuShellWidgetClass,
        parent,
        &raw mut args as *mut Arg as ArgList,
        i as Cardinal,
    );
    i = 0 as ::core::ffi::c_int;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(124 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10728 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[i as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(4728 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[i as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
    i += 1;
    if fTopLevelPane != 0 {
        args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(9165 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[i as usize].value = XmMENU_POPUP as ::core::ffi::c_int as XtArgVal;
        i += 1;
        args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(8782 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[i as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
        i += 1;
        menuW = XtCreateWidget(
            menuName as *const ::core::ffi::c_char,
            xmRowColumnWidgetClass,
            menuShellW,
            &raw mut args as *mut Arg as ArgList,
            i as Cardinal,
        );
        XtAddCallback(
            menuW,
            (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(10482 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            Some(UnmapCallback as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()),
            menuSpec as XtPointer,
        );
    } else {
        args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(9165 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[i as usize].value = XmMENU_PULLDOWN as ::core::ffi::c_int as XtArgVal;
        i += 1;
        menuW = XtCreateWidget(
            menuName as *const ::core::ffi::c_char,
            xmRowColumnWidgetClass,
            menuShellW,
            &raw mut args as *mut Arg as ArgList,
            i as Cardinal,
        );
    }
    sepArgs[0 as ::core::ffi::c_int as usize].name =
        (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(9629 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
            as String_0;
    sepArgs[0 as ::core::ffi::c_int as usize].value =
        XmDOUBLE_LINE as ::core::ffi::c_int as XtArgVal;
    n = 0 as ::core::ffi::c_int;
    menuItem = (*menuSpec).menuItems;
    if menuItem.is_null() && !moreMenuItems.is_null() {
        menuItem = moreMenuItems;
        (*menuSpec).menuItems = menuItem;
        moreMenuItems = ::core::ptr::null_mut::<MenuItem>();
    }
    while !menuItem.is_null() {
        i = 0 as ::core::ffi::c_int;
        if (*menuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Separator
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) {
            children[n as usize] = XmCreateSeparatorGadget(
                menuW,
                SEPARATOR_NAME.as_ptr() as *mut ::core::ffi::c_char,
                NULL as ArgList,
                0 as Cardinal,
            );
            fUseTitleSep = FALSE as Boolean;
        } else {
            if (*menuItem).labelType == XmPIXMAP as ::core::ffi::c_int && {
                labelPixmap = MakeCachedLabelPixmap(pSD, menuW, (*menuItem).labelBitmapIndex);
                labelPixmap != 0
            } {
                args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                    .offset(7547 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char as String_0;
                args[i as usize].value = XmPIXMAP as ::core::ffi::c_int as XtArgVal;
                i += 1;
                args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                    .offset(7523 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char as String_0;
                args[i as usize].value = labelPixmap as XtArgVal;
                i += 1;
                args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                    .offset(7500 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char as String_0;
                args[i as usize].value = labelPixmap as XtArgVal;
                i += 1;
            } else {
                args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                    .offset(7547 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char as String_0;
                args[i as usize].value = XmSTRING as ::core::ffi::c_int as XtArgVal;
                i += 1;
                args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                    .offset(7535 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char as String_0;
                tmpstr = XmStringCreateLocalized((*menuItem).label);
                args[i as usize].value = tmpstr as XtArgVal;
                i += 1;
                sPtr = XtMalloc(::core::mem::size_of::<StrList>() as Cardinal) as *mut StrList;
                if sPtr.is_null() {
                    MWarning(
                        b"Insufficient memory for menu %s\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        menuName as *mut ::core::ffi::c_char,
                    );
                    return ::core::ptr::null_mut::<_WidgetRec>();
                } else {
                    (*sPtr).string = tmpstr;
                    (*sPtr).next = stringsToFree as *mut _StrList;
                    stringsToFree = sPtr;
                }
            }
            if (*menuItem).wmFunction.is_some_and(|action| {
                std::ptr::fn_addr_eq(
                    action,
                    F_Title
                        as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
                )
            }) {
                if fUseTitleSep != 0 {
                    children[n as usize] = XmCreateSeparatorGadget(
                        menuW,
                        SEPARATOR_NAME.as_ptr() as *mut ::core::ffi::c_char,
                        &raw mut sepArgs as ArgList,
                        1 as Cardinal,
                    );
                    n += 1;
                }
                args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                    .offset(4741 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char as String_0;
                args[i as usize].value = XmALIGNMENT_CENTER as ::core::ffi::c_int as XtArgVal;
                i += 1;
                children[n as usize] = XmCreateLabelGadget(
                    menuW,
                    TITLE_NAME.as_ptr() as *mut ::core::ffi::c_char,
                    &raw mut args as *mut Arg,
                    i as Cardinal,
                );
                n += 1;
                children[n as usize] = XmCreateSeparatorGadget(
                    menuW,
                    SEPARATOR_NAME.as_ptr() as *mut ::core::ffi::c_char,
                    &raw mut sepArgs as ArgList,
                    1 as Cardinal,
                );
                fUseTitleSep = TRUE as Boolean;
            } else {
                args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                    .offset(4741 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char as String_0;
                args[i as usize].value = XmALIGNMENT_BEGINNING as ::core::ffi::c_int as XtArgVal;
                i += 1;
                if (*menuItem).mnemonic != 0 {
                    args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(8142 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char
                        as String_0;
                    args[i as usize].value = (*menuItem).mnemonic as XtArgVal;
                    i += 1;
                }
                if (*menuItem).wmFunction.is_some_and(|action| {
                    std::ptr::fn_addr_eq(
                        action,
                        F_Menu
                            as unsafe extern "C" fn(
                                String_0,
                                *mut ClientData,
                                *mut XEvent,
                            ) -> Boolean,
                    )
                }) {
                    subMenuW = CreateMenuWidget(
                        pSD,
                        (*menuItem).wmFuncArgs,
                        menuW,
                        FALSE as Boolean,
                        topMenuSpec,
                        NULL as *mut MenuItem,
                    );
                    if !subMenuW.is_null() {
                        args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                            .offset(9963 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char
                            as String_0;
                        args[i as usize].value = subMenuW as XtArgVal;
                        i += 1;
                        children[n as usize] = XmCreateCascadeButtonGadget(
                            menuW,
                            CASCADE_BTN_NAME.as_ptr() as *mut ::core::ffi::c_char,
                            &raw mut args as *mut Arg as ArgList,
                            i as Cardinal,
                        );
                    } else {
                        children[n as usize] = XmCreateCascadeButtonGadget(
                            menuW,
                            CASCADE_BTN_NAME.as_ptr() as *mut ::core::ffi::c_char,
                            &raw mut args as *mut Arg as ArgList,
                            i as Cardinal,
                        );
                        XtSetSensitive(children[n as usize], FALSE as Boolean);
                    }
                    fUseTitleSep = TRUE as Boolean;
                } else {
                    if !(*menuItem).accelText.is_null() {
                        args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                            .offset(4684 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char
                            as String_0;
                        tmpstr = XmStringCreateLocalized((*menuItem).accelText);
                        args[i as usize].value = tmpstr as XtArgVal;
                        i += 1;
                        sPtr =
                            XtMalloc(::core::mem::size_of::<StrList>() as Cardinal) as *mut StrList;
                        if sPtr.is_null() {
                            MWarning(
                                b"Insufficient memory for menu %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                menuName as *mut ::core::ffi::c_char,
                            );
                            return ::core::ptr::null_mut::<_WidgetRec>();
                        } else {
                            (*sPtr).string = tmpstr;
                            (*sPtr).next = stringsToFree as *mut _StrList;
                            stringsToFree = sPtr;
                        }
                        accelKeySpec =
                            XtMalloc(::core::mem::size_of::<KeySpec>() as Cardinal) as *mut KeySpec;
                        if accelKeySpec.is_null() {
                            MWarning(
                                b"Insufficient memory for menu %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                menuName as *mut ::core::ffi::c_char,
                            );
                            (*menuSpec).currentContext &= !CR_MENU_MARK as Context;
                            return ::core::ptr::null_mut::<_WidgetRec>();
                        }
                        (*accelKeySpec).state = (*menuItem).accelState;
                        (*accelKeySpec).keycode = (*menuItem).accelKeyCode;
                        (*accelKeySpec).context = (*topMenuSpec).accelContext;
                        (*accelKeySpec).subContext = 0 as Context;
                        (*accelKeySpec).wmFunction = (*menuItem).wmFunction;
                        (*accelKeySpec).wmFuncArgs = (*menuItem).wmFuncArgs;
                        (*accelKeySpec).nextKeySpec = (*topMenuSpec).accelKeySpecs as *mut _KeySpec;
                        (*topMenuSpec).accelKeySpecs = accelKeySpec;
                    }
                    children[n as usize] = XmCreatePushButtonGadget(
                        menuW,
                        PUSH_BTN_NAME.as_ptr() as *mut ::core::ffi::c_char,
                        &raw mut args as *mut Arg as ArgList,
                        i as Cardinal,
                    );
                    if (*menuItem).greyedContext & (*topMenuSpec).currentContext != 0 {
                        XtSetSensitive(children[n as usize], FALSE as Boolean);
                    } else {
                        XtSetSensitive(children[n as usize], TRUE as Boolean);
                    }
                    if (*menuItem).greyedContext != 0 || (*menuItem).mgtMask != 0 {
                        if SavePBInfo(topMenuSpec, menuItem, children[n as usize]) == 0 {
                            MWarning(
                                b"Insufficient memory for menu %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                menuName as *mut ::core::ffi::c_char,
                            );
                            (*menuSpec).currentContext &= !CR_MENU_MARK as Context;
                            return ::core::ptr::null_mut::<_WidgetRec>();
                        }
                    }
                    XtAddCallback(
                        children[n as usize],
                        (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                            .offset(4700 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char,
                        ::core::mem::transmute::<
                            Option<unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()>,
                            XtCallbackProc,
                        >(Some(
                            ActivateCallback
                                as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> (),
                        )),
                        menuItem as XtPointer,
                    );
                    fUseTitleSep = TRUE as Boolean;
                }
            }
        }
        n += 1;
        menuItem = (*menuItem).nextMenuItem as *mut MenuItem;
        if menuItem.is_null() && !moreMenuItems.is_null() {
            menuItem = moreMenuItems;
            (*menuSpec).menuItems = menuItem;
            moreMenuItems = ::core::ptr::null_mut::<MenuItem>();
        }
        if n >= CHILDREN_CACHE - 2 as ::core::ffi::c_int {
            XtManageChildren(&raw mut children as WidgetList, n as Cardinal);
            n = 0 as ::core::ffi::c_int;
        }
    }
    if n > 0 as ::core::ffi::c_int {
        XtManageChildren(&raw mut children as WidgetList, n as Cardinal);
    }
    if fTopLevelPane != 0 {
        i = 0 as ::core::ffi::c_int;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(872 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = &raw mut menuWidth as XtArgVal;
        i += 1;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(234 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = &raw mut menuHeight as XtArgVal;
        i += 1;
        XtGetValues(menuW, &raw mut args as *mut Arg as ArgList, i as Cardinal);
        (*topMenuSpec).height = menuHeight as ::core::ffi::c_uint;
        (*topMenuSpec).width = menuWidth as ::core::ffi::c_uint;
    }
    i = 0 as ::core::ffi::c_int;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = &raw mut menuHeight as XtArgVal;
    i += 1;
    XtGetValues(menuW, &raw mut args as *mut Arg as ArgList, i as Cardinal);
    scr = XtScreenOfObject(menuW);
    if menuHeight as ::core::ffi::c_int > (*scr).height as Dimension as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(9073 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[i as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
        i += 1;
        args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(8670 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[i as usize].value = XmPACK_TIGHT as ::core::ffi::c_int as XtArgVal;
        i += 1;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(505 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = XmVERTICAL as ::core::ffi::c_int as XtArgVal;
        i += 1;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(234 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = (*scr).height as XtArgVal;
        i += 1;
        XtSetValues(menuW, &raw mut args as *mut Arg as ArgList, i as Cardinal);
    }
    sPtr = stringsToFree;
    while !sPtr.is_null() {
        stringsToFree = (*stringsToFree).next as *mut StrList;
        XmStringFree((*sPtr).string);
        XtFree(sPtr as *mut ::core::ffi::c_char);
        sPtr = stringsToFree;
    }
    (*menuSpec).currentContext &= !CR_MENU_MARK as Context;
    return menuW;
}
#[no_mangle]
pub unsafe extern "C" fn PostMenu(
    mut menuSpec: *mut MenuSpec,
    mut pCD: *mut ClientData,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut button: ::core::ffi::c_uint,
    mut newContext: Context,
    mut flags: ::core::ffi::c_long,
    mut passedInEvent: *mut XEvent,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut args: [Arg; 3] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 3];
    let mut whichButton: ::core::ffi::c_uint = 0;
    let mut menuWidth: Dimension = 0;
    let mut menuHeight: Dimension = 0;
    let mut event = XButtonEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: ::core::ptr::null::<Display>() as *mut Display,
        window: 0,
        root: 0,
        subwindow: 0,
        time: 0,
        x: 0,
        y: 0,
        x_root: 0,
        y_root: 0,
        state: 0,
        button: 0,
        same_screen: 0,
    };
    let mut saveWindow: Window = 0;
    let mut saveDisplay = ::core::ptr::null_mut::<Display>();
    if menuSpec.is_null() || (*menuSpec).menuWidget.is_null() {
        return;
    }
    if (newContext == F_SUBCONTEXT_IB_WICON as Context
        || newContext == F_SUBCONTEXT_IB_IICON as Context)
        && IconVisible(pCD) == 0
    {
        return;
    }
    wmGD.grabContext = newContext;
    if !pCD.is_null() && (*pCD).clientFlags & ICON_BOX != 0 {
        newContext |= F_CONTEXT_ICONBOX as Context;
    }
    if AdjustPBs(menuSpec, pCD, newContext) != 0 {
        i = 0 as ::core::ffi::c_int;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(872 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = &raw mut menuWidth as XtArgVal;
        i += 1;
        args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(234 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[i as usize].value = &raw mut menuHeight as XtArgVal;
        i += 1;
        XtGetValues(
            (*menuSpec).menuWidget,
            &raw mut args as *mut Arg as ArgList,
            i as Cardinal,
        );
        (*menuSpec).height = menuHeight as ::core::ffi::c_uint;
        (*menuSpec).width = menuWidth as ::core::ffi::c_uint;
    }
    (*menuSpec).currentContext = newContext;
    whichButton = if button == NoButton as ::core::ffi::c_uint {
        SELECT_BUTTON as ::core::ffi::c_uint
    } else {
        button
    };
    if whichButton != (*menuSpec).whichButton {
        i = 0 as ::core::ffi::c_int;
        args[i as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(10728 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[i as usize].value = whichButton as XtArgVal;
        i += 1;
        XtSetValues(
            (*menuSpec).menuWidget,
            &raw mut args as ArgList,
            i as Cardinal,
        );
        (*menuSpec).whichButton = whichButton;
    }
    if flags & POST_AT_XY != 0 {
        AdjustMenuPosition(
            &raw mut x,
            &raw mut y,
            (*menuSpec).width,
            (*menuSpec).height,
        );
    } else {
        GetSystemMenuPosition(
            pCD,
            &raw mut x,
            &raw mut y,
            (*menuSpec).width,
            (*menuSpec).height,
            newContext,
        );
    }
    event.x_root = x;
    event.y_root = y;
    XmMenuPosition((*menuSpec).menuWidget, &raw mut event);
    wmGD.menuClient = pCD;
    wmGD.menuActive = menuSpec;
    if !passedInEvent.is_null() {
        saveWindow = (*passedInEvent).xany.window;
        saveDisplay = (*passedInEvent).xany.display;
        (*passedInEvent).xany.window = 0 as Window;
        (*passedInEvent).xany.display = XtDisplayOfObject((*menuSpec).menuWidget);
        XtDispatchEvent(passedInEvent);
        (*passedInEvent).xany.window = saveWindow;
        (*passedInEvent).xany.display = saveDisplay;
        if (*passedInEvent).type_0 == ButtonPress || (*passedInEvent).type_0 == ButtonRelease {
            XUngrabPointer((*passedInEvent).xany.display, (*passedInEvent).xbutton.time);
        }
    }
    XtManageChild((*menuSpec).menuWidget);
    if button == NoButton as ::core::ffi::c_uint || flags & POST_TRAVERSAL_ON != 0 {
        TraversalOn(menuSpec);
    } else {
        TraversalOff(menuSpec);
    };
}
#[no_mangle]
pub unsafe extern "C" fn UnpostMenu(mut menuSpec: *mut MenuSpec) {
    if !menuSpec.is_null() && !(*menuSpec).menuWidget.is_null() {
        XtUnmanageChild((*menuSpec).menuWidget);
        ForceColormapFocus(
            wmGD.pActiveSD,
            (*wmGD.pActiveSD).colormapFocus as *mut ClientData,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ActivateCallback(
    mut w: Widget,
    mut client_data: XtPointer,
    mut call_data: XtPointer,
) {
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    pSD = GetScreenForWindow(XtWindowOfObject(w));
    if !pSD.is_null() {
        wmGD.pActiveSD = pSD;
        wmGD.queryScreen = False as Boolean;
    }
    (*(client_data as *mut MenuItem))
        .wmFunction
        .expect("non-null function pointer")(
        (*(client_data as *mut MenuItem)).wmFuncArgs,
        wmGD.menuClient as *mut ClientData,
        ::core::ptr::null_mut::<XEvent>(),
    );
}
unsafe extern "C" fn UnmapCallback(
    mut w: Widget,
    mut client_data: XtPointer,
    mut call_data: XtPointer,
) {
    wmGD.menuActive = ::core::ptr::null_mut::<MenuSpec>();
    wmGD.menuUnpostKeySpec = ::core::ptr::null_mut::<KeySpec>();
    wmGD.checkHotspot = FALSE as Boolean;
    if !wmGD.gadgetClient.is_null() {
        PopGadgetOut(wmGD.gadgetClient, FRAME_SYSTEM);
    }
    ForceColormapFocus(
        wmGD.pActiveSD,
        (*wmGD.pActiveSD).colormapFocus as *mut ClientData,
    );
    PullExposureEvents();
}
#[no_mangle]
pub unsafe extern "C" fn MWarning(
    mut format: *mut ::core::ffi::c_char,
    mut message: *mut ::core::ffi::c_char,
) {
    if strlen(format).wrapping_add(strlen(message)) < MAXWMPATH as size_t {
        let mut pch: [::core::ffi::c_char; 1024] = [0; 1024];
        sprintf(&raw mut pch as *mut ::core::ffi::c_char, format, message);
        Warning(&raw mut pch as *mut ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn TraversalOff(mut menuSpec: *mut MenuSpec) {
    if !menuSpec.is_null() && !(*menuSpec).menuWidget.is_null() {
        Some(
            (*((*((*menuSpec).menuWidget as Object)).object.widget_class
                as XmRowColumnWidgetClass))
                .row_column_class
                .menuProcedures
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            XmMENU_TRAVERSAL as ::core::ffi::c_int,
            (*menuSpec).menuWidget,
            False,
            NULL,
            NULL,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn TraversalOn(mut menuSpec: *mut MenuSpec) {
    if !menuSpec.is_null() && !(*menuSpec).menuWidget.is_null() {
        Some(
            (*((*((*menuSpec).menuWidget as Object)).object.widget_class
                as XmRowColumnWidgetClass))
                .row_column_class
                .menuProcedures
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            XmMENU_TRAVERSAL as ::core::ffi::c_int,
            (*menuSpec).menuWidget,
            True,
            NULL,
            NULL,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn FreeCustomMenuSpec(mut menuSpec: *mut MenuSpec) {
    let mut menuItem = ::core::ptr::null_mut::<MenuItem>();
    let mut nextMenuItem = ::core::ptr::null_mut::<MenuItem>();
    let mut accelKeySpec = ::core::ptr::null_mut::<KeySpec>();
    let mut nextAccelKeySpec = ::core::ptr::null_mut::<KeySpec>();
    if menuSpec.is_null() || !(*menuSpec).name.is_null() {
        return;
    }
    if wmGD.menuActive == menuSpec {
        UnmapCallback(NULL as Widget, NULL, NULL);
    }
    if !(*menuSpec).name.is_null() {
        XtFree((*menuSpec).name as *mut ::core::ffi::c_char);
    }
    menuItem = (*menuSpec).menuItems;
    while !menuItem.is_null() {
        nextMenuItem = (*menuItem).nextMenuItem as *mut MenuItem;
        FreeMenuItem(menuItem);
        menuItem = nextMenuItem;
    }
    if !(*menuSpec).menuButtons.is_null() {
        XtFree((*menuSpec).menuButtons as *mut ::core::ffi::c_char);
    }
    accelKeySpec = (*menuSpec).accelKeySpecs;
    while !accelKeySpec.is_null() {
        nextAccelKeySpec = (*accelKeySpec).nextKeySpec as *mut KeySpec;
        XtFree(accelKeySpec as *mut ::core::ffi::c_char);
        accelKeySpec = nextAccelKeySpec;
    }
    if !(*menuSpec).menuWidget.is_null() {
        XtDestroyWidget((*((*menuSpec).menuWidget as Object)).object.parent);
    }
    XtFree(menuSpec as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn AdjustMenuPosition(
    mut x: *mut ::core::ffi::c_int,
    mut y: *mut ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
) {
    let mut xsi = XineramaScreenInfo {
        screen_number: 0,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    };
    if GetXineramaScreenFromLocation(*x, *y, &raw mut xsi) == 0 {
        xsi.x_org = 0 as ::core::ffi::c_short;
        xsi.y_org = 0 as ::core::ffi::c_short;
        xsi.width = XDisplayWidth(wmGD.display, (*wmGD.pActiveSD).screen) as ::core::ffi::c_short;
        xsi.height = XDisplayHeight(wmGD.display, (*wmGD.pActiveSD).screen) as ::core::ffi::c_short;
    }
    if (*x as ::core::ffi::c_uint).wrapping_add(width)
        >= (xsi.x_org as ::core::ffi::c_int + xsi.width as ::core::ffi::c_int)
            as ::core::ffi::c_uint
    {
        *x = (*x as ::core::ffi::c_uint).wrapping_sub(
            (*x as ::core::ffi::c_uint)
                .wrapping_add(width)
                .wrapping_sub(
                    (xsi.x_org as ::core::ffi::c_int + xsi.width as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                ),
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    if (*y as ::core::ffi::c_uint).wrapping_add(height)
        >= (xsi.y_org as ::core::ffi::c_int + xsi.height as ::core::ffi::c_int)
            as ::core::ffi::c_uint
    {
        *y = (*y as ::core::ffi::c_uint).wrapping_sub(
            (*y as ::core::ffi::c_uint)
                .wrapping_add(height)
                .wrapping_sub(
                    (xsi.y_org as ::core::ffi::c_int + xsi.height as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                ),
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
}
