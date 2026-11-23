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
use crate::abi::_AppearanceData_WmInitWs as _AppearanceData;
use crate::abi::_ButtonSpec_WmInitWs as _ButtonSpec;
use crate::abi::_ClickData_WmInitWs as _ClickData;
use crate::abi::_ClientData_WmInitWs as _ClientData;
use crate::abi::_ClientListEntry_WmInitWs as _ClientListEntry;
use crate::abi::_CoreClassPart_WmInitWs as _CoreClassPart;
use crate::abi::_CorePart_WmInitWs as _CorePart;
use crate::abi::_IconBoxData_WmInitWs as _IconBoxData;
use crate::abi::_IconInfo_WmInitWs as _IconInfo;
use crate::abi::_IconPlacementData_WmInitWs as _IconPlacementData;
use crate::abi::_KeySpec_WmInitWs as _KeySpec;
use crate::abi::_MenuButton_WmInitWs as _MenuButton;
use crate::abi::_MenuItem_WmInitWs as _MenuItem;
use crate::abi::_MenuSpec_WmInitWs as _MenuSpec;
use crate::abi::_WidgetClassRec_WmInitWs as _WidgetClassRec;
use crate::abi::_WidgetRec_WmInitWs as _WidgetRec;
use crate::abi::_WmGlobalData_WmInitWs as _WmGlobalData;
use crate::abi::_WmScreenData_WmInitWs as _WmScreenData;
use crate::abi::_WmTimer_WmInitWs as _WmTimer;
use crate::abi::_WmWorkspaceData_WmInitWs as _WmWorkspaceData;
use crate::abi::_WsClientData_WmInitWs as _WsClientData;
use crate::abi::_WsPresenceData_WmInitWs as _WsPresenceData;
use crate::abi::_XDisplay_WmInitWs as _XDisplay;
use crate::abi::_XEvent_WmInitWs as _XEvent;
use crate::abi::_XGC_WmInitWs as _XGC;
use crate::abi::_XPrivDisplay_WmInitWs as _XPrivDisplay;
use crate::abi::_XtActionsRec_WmInitWs as _XtActionsRec;
use crate::abi::_XtCallbackRec_WmInitWs as _XtCallbackRec;
use crate::abi::_XtTMRec_WmInitWs as _XtTMRec;
use crate::abi::AppearanceData_WmInitWs as AppearanceData;
use crate::abi::ArgList_WmFeedback as ArgList;
use crate::abi::ButtonSpec_WmInitWs as ButtonSpec;
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed_30;
use crate::abi::C2RustUnnamed_0_WmInitWs as C2RustUnnamed_0;
use crate::abi::C2RustUnnamed_10_WmInitWs as C2RustUnnamed_10;
use crate::abi::C2RustUnnamed_11_WmInitWs as C2RustUnnamed_11;
use crate::abi::C2RustUnnamed_12_WmInitWs as C2RustUnnamed_12;
use crate::abi::C2RustUnnamed_13_WmInitWs as C2RustUnnamed_13;
use crate::abi::C2RustUnnamed_14_WmInitWs as C2RustUnnamed_14;
use crate::abi::C2RustUnnamed_15_WmInitWs as C2RustUnnamed_15;
use crate::abi::C2RustUnnamed_16_WmInitWs as C2RustUnnamed_16;
use crate::abi::C2RustUnnamed_17_WmInitWs as C2RustUnnamed_17;
use crate::abi::C2RustUnnamed_18_WmInitWs as C2RustUnnamed_18;
use crate::abi::C2RustUnnamed_19_WmInitWs as C2RustUnnamed_19;
use crate::abi::C2RustUnnamed_1_WmInitWs as C2RustUnnamed_1;
use crate::abi::C2RustUnnamed_20_WmInitWs as C2RustUnnamed_20;
use crate::abi::C2RustUnnamed_21_WmInitWs as C2RustUnnamed_21;
use crate::abi::C2RustUnnamed_22_WmInitWs as C2RustUnnamed_22;
use crate::abi::C2RustUnnamed_23_WmInitWs as C2RustUnnamed_23;
use crate::abi::C2RustUnnamed_24_WmInitWs as C2RustUnnamed_24;
use crate::abi::C2RustUnnamed_25_WmInitWs as C2RustUnnamed_25;
use crate::abi::C2RustUnnamed_26_WmInitWs as C2RustUnnamed_26;
use crate::abi::C2RustUnnamed_27_WmInitWs as C2RustUnnamed_27;
use crate::abi::C2RustUnnamed_28_WmInitWs as C2RustUnnamed_28;
use crate::abi::C2RustUnnamed_29_WmInitWs as C2RustUnnamed_29;
use crate::abi::C2RustUnnamed_2_WmInitWs as C2RustUnnamed_2;
use crate::abi::C2RustUnnamed_31_WmInitWs as C2RustUnnamed_31;
use crate::abi::C2RustUnnamed_32_WmInitWs as C2RustUnnamed_32;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_33;
use crate::abi::C2RustUnnamed_3_WmInitWs as C2RustUnnamed_3;
use crate::abi::C2RustUnnamed_4_WmInitWs as C2RustUnnamed_4;
use crate::abi::C2RustUnnamed_5_WmInitWs as C2RustUnnamed_5;
use crate::abi::C2RustUnnamed_6_WmInitWs as C2RustUnnamed_6;
use crate::abi::C2RustUnnamed_7_WmInitWs as C2RustUnnamed_7;
use crate::abi::C2RustUnnamed_8_WmInitWs as C2RustUnnamed_8;
use crate::abi::C2RustUnnamed_9_WmInitWs as C2RustUnnamed_9;
use crate::abi::C2RustUnnamed_WmInitWs as C2RustUnnamed;
use crate::abi::ClickData_WmInitWs as ClickData;
use crate::abi::ClientData_WmInitWs as ClientData;
use crate::abi::ClientListEntry_WmInitWs as ClientListEntry;
use crate::abi::CoreClassPart_WmInitWs as CoreClassPart;
use crate::abi::CorePart_WmInitWs as CorePart;
use crate::abi::Display_WmInitWs as Display;
use crate::abi::GC_WmInitWs as GC;
use crate::abi::IconBoxData_WmInitWs as IconBoxData;
use crate::abi::IconInfo_WmInitWs as IconInfo;
use crate::abi::IconPlacementData_WmInitWs as IconPlacementData;
use crate::abi::KeySpec_WmInitWs as KeySpec;
use crate::abi::MenuButton_WmInitWs as MenuButton;
use crate::abi::MenuItem_WmInitWs as MenuItem;
use crate::abi::MenuSpec_WmInitWs as MenuSpec;
use crate::abi::Screen_WmInitWs as Screen;
use crate::abi::WidgetClass_WmInitWs as WidgetClass;
use crate::abi::WidgetList_WmInitWs as WidgetList;
use crate::abi::Widget_WmInitWs as Widget;
use crate::abi::WmFunction_WmInitWs as WmFunction;
use crate::abi::WmGlobalData_WmInitWs as WmGlobalData;
use crate::abi::WmScreenData_WmInitWs as WmScreenData;
use crate::abi::WmTimer_WmInitWs as WmTimer;
use crate::abi::WmWorkspaceData_WmInitWs as WmWorkspaceData;
use crate::abi::WsPresenceData_WmInitWs as WsPresenceData;
use crate::abi::XAnyEvent_WmInitWs as XAnyEvent;
use crate::abi::XButtonEvent_WmInitWs as XButtonEvent;
use crate::abi::XCirculateEvent_WmInitWs as XCirculateEvent;
use crate::abi::XCirculateRequestEvent_WmInitWs as XCirculateRequestEvent;
use crate::abi::XClientMessageEvent_WmInitWs as XClientMessageEvent;
use crate::abi::XColormapEvent_WmInitWs as XColormapEvent;
use crate::abi::XConfigureEvent_WmInitWs as XConfigureEvent;
use crate::abi::XConfigureRequestEvent_WmInitWs as XConfigureRequestEvent;
use crate::abi::XCreateWindowEvent_WmInitWs as XCreateWindowEvent;
use crate::abi::XCrossingEvent_WmInitWs as XCrossingEvent;
use crate::abi::XDestroyWindowEvent_WmInitWs as XDestroyWindowEvent;
use crate::abi::XEnterWindowEvent_WmInitWs as XEnterWindowEvent;
use crate::abi::XErrorEvent_WmInitWs as XErrorEvent;
use crate::abi::XEvent_WmInitWs as XEvent;
use crate::abi::XExposeEvent_WmInitWs as XExposeEvent;
use crate::abi::XFocusChangeEvent_WmInitWs as XFocusChangeEvent;
use crate::abi::XGenericEventCookie_WmInitWs as XGenericEventCookie;
use crate::abi::XGenericEvent_WmInitWs as XGenericEvent;
use crate::abi::XGraphicsExposeEvent_WmInitWs as XGraphicsExposeEvent;
use crate::abi::XGravityEvent_WmInitWs as XGravityEvent;
use crate::abi::XKeyEvent_WmInitWs as XKeyEvent;
use crate::abi::XKeymapEvent_WmInitWs as XKeymapEvent;
use crate::abi::XMapEvent_WmInitWs as XMapEvent;
use crate::abi::XMapRequestEvent_WmInitWs as XMapRequestEvent;
use crate::abi::XMappingEvent_WmInitWs as XMappingEvent;
use crate::abi::XMotionEvent_WmInitWs as XMotionEvent;
use crate::abi::XNoExposeEvent_WmInitWs as XNoExposeEvent;
use crate::abi::XPropertyEvent_WmInitWs as XPropertyEvent;
use crate::abi::XReparentEvent_WmInitWs as XReparentEvent;
use crate::abi::XResizeRequestEvent_WmInitWs as XResizeRequestEvent;
use crate::abi::XSelectionClearEvent_WmInitWs as XSelectionClearEvent;
use crate::abi::XSelectionEvent_WmInitWs as XSelectionEvent;
use crate::abi::XSelectionRequestEvent_WmInitWs as XSelectionRequestEvent;
use crate::abi::XUnmapEvent_WmInitWs as XUnmapEvent;
use crate::abi::XVisibilityEvent_WmInitWs as XVisibilityEvent;
use crate::abi::XWindowAttributes_WmInitWs as XWindowAttributes;
use crate::abi::XtAcceptFocusProc_WmInitWs as XtAcceptFocusProc;
use crate::abi::XtActionList_WmInitWs as XtActionList;
use crate::abi::XtActionProc_WmInitWs as XtActionProc;
use crate::abi::XtAlmostProc_WmInitWs as XtAlmostProc;
use crate::abi::XtArgsFunc_WmInitWs as XtArgsFunc;
use crate::abi::XtArgsProc_WmInitWs as XtArgsProc;
use crate::abi::XtBoundActions_WmInitWs as XtBoundActions;
use crate::abi::XtCallbackList_WmInitWs as XtCallbackList;
use crate::abi::XtCallbackProc_WmInitWs as XtCallbackProc;
use crate::abi::XtExposeProc_WmInitWs as XtExposeProc;
use crate::abi::XtGeometryHandler_WmInitWs as XtGeometryHandler;
use crate::abi::XtInitProc_WmInitWs as XtInitProc;
use crate::abi::XtRealizeProc_WmInitWs as XtRealizeProc;
use crate::abi::XtSetValuesFunc_WmInitWs as XtSetValuesFunc;
use crate::abi::XtStringProc_WmInitWs as XtStringProc;
use crate::abi::XtTMRec_WmInitWs as XtTMRec;
use crate::abi::XtWidgetClassProc_WmInitWs as XtWidgetClassProc;
use crate::abi::XtWidgetGeometry_WmInitWs as XtWidgetGeometry;
use crate::abi::XtWidgetProc_WmInitWs as XtWidgetProc;
use crate::abi::*;
extern "C" {
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn XGetModifierMapping(_: *mut Display) -> *mut XModifierKeymap;
    fn XInternAtom(_: *mut Display, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int) -> Atom;
    fn XInternAtoms(
        _: *mut Display,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut Atom,
    ) -> ::core::ffi::c_int;
    fn XCreateFontCursor(_: *mut Display, _: ::core::ffi::c_uint) -> Cursor;
    fn XCreateGC(_: *mut Display, _: Drawable, _: ::core::ffi::c_ulong, _: *mut XGCValues) -> GC;
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
    fn XChangeWindowAttributes(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> ::core::ffi::c_int;
    fn XDefaultScreen(_: *mut Display) -> ::core::ffi::c_int;
    fn XDefineCursor(_: *mut Display, _: Window, _: Cursor) -> ::core::ffi::c_int;
    fn XFlush(_: *mut Display) -> ::core::ffi::c_int;
    fn XFree(_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn XFreeModifiermap(_: *mut XModifierKeymap) -> ::core::ffi::c_int;
    fn XGetPointerMapping(
        _: *mut Display,
        _: *mut ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XKeysymToKeycode(_: *mut Display, _: KeySym) -> KeyCode;
    fn XMapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XSync(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XSaveContext(
        _: *mut Display,
        _: XID,
        _: XContext,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn XShapeQueryExtension(
        _: *mut Display,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XrmUniqueQuark() -> XrmQuark;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XtAddEventHandler(_: Widget, _: EventMask, _: Boolean, _: XtEventHandler, _: XtPointer);
    fn XtRealizeWidget(_: Widget);
    fn XtWindowOfObject(_: Widget) -> Window;
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
    fn XtAppCreateShell(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: WidgetClass,
        _: *mut Display,
        _: ArgList,
        _: Cardinal,
    ) -> Widget;
    fn XtToolkitInitialize();
    fn XtOpenDisplay(
        _: XtAppContext,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *mut XrmOptionDescRec,
        _: Cardinal,
        _: *mut ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut Display;
    fn XtCreateApplicationContext() -> XtAppContext;
    fn XtAppSetFallbackResources(_: XtAppContext, _: *mut String_0);
    fn XtSetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtCalloc(_: Cardinal, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtRealloc(_: *mut ::core::ffi::c_char, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    static XtStrings: [::core::ffi::c_char; 0];
    static XtShellStrings: [::core::ffi::c_char; 0];
    static mut applicationShellWidgetClass: WidgetClass;
    static mut sessionShellWidgetClass: WidgetClass;
    static mut _XmStrings: [::core::ffi::c_char; 0];
    static mut xmPrimitiveWidgetClass: WidgetClass;
    fn XmStringCreateLocalized(text: String_0) -> XmString;
    static mut vendorShellWidgetClass: WidgetClass;
    fn XmeGetHomeDirName() -> String_0;
    fn XmeGetIconControlInfo(
        screen: *mut Screen,
        useMaskRtn: *mut Boolean,
        useMultiColorIconsRtn: *mut Boolean,
        useIconFileCacheRtn: *mut Boolean,
    ) -> Boolean;
    static mut wmGD: WmGlobalData;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn XmInternAtom(display: *mut Display, name: String_0, only_if_exists: Boolean) -> Atom;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn putenv(__string: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn mblen(__s: *const ::core::ffi::c_char, __n: size_t) -> ::core::ffi::c_int;
    fn XRRQueryExtension(
        dpy: *mut Display,
        event_base_return: *mut ::core::ffi::c_int,
        error_base_return: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XRRSelectInput(dpy: *mut Display, window: Window, mask: ::core::ffi::c_int);
    fn ChangeBackdrop(pWS: *mut WmWorkspaceData);
    fn InitWorkspaceColormap(pSD: *mut WmScreenData);
    fn InitColormapFocus(pSD: *mut WmScreenData);
    fn WmInitErrorHandler(display: *mut Display);
    fn Warning(message: *mut ::core::ffi::c_char);
    fn InitEventHandling();
    fn InitCursorInfo();
    fn ShowWaitState(flag: Boolean);
    fn F_Menu(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Nop(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn CreateActiveIconTextWindow(pSD: *mut WmScreenData);
    fn InitIconSize(pSD: *mut WmScreenData);
    fn InitIconPlacement(pWS: *mut WmWorkspaceData);
    fn InitIconBox(pSD: *mut WmScreenData);
    fn MapIconBoxes(pWS: *mut WmWorkspaceData);
    fn InitKeyboardFocus();
    fn AdoptInitialClients(pSD: *mut WmScreenData);
    fn MakeMenu(
        pSD: *mut WmScreenData,
        menuName: String_0,
        initialContext: Context,
        accelContext: Context,
        moreMenuItems: *mut MenuItem,
        fSystemMenu: Boolean,
    ) -> *mut MenuSpec;
    fn SetMwmInfo(propWindow: Window, flags: ::core::ffi::c_long, wmWindow: Window);
    fn GetMwmInfo(rootWindowOfScreen: Window) -> *mut PropMwmInfo;
    fn SetWorkspaceListProperty(pSD: *mut WmScreenData);
    fn SetCurrentWorkspaceProperty(pSD: *mut WmScreenData);
    fn SetWorkspaceInfoProperty(pWS: *mut WmWorkspaceData);
    fn AddWmResourceConverters();
    fn ProcessWmColors(pSD: *mut WmScreenData);
    fn ProcessWmResources();
    fn ProcessGlobalScreenResources();
    fn ProcessScreenResources(pSD: *mut WmScreenData, screenName: *mut ::core::ffi::c_uchar);
    fn ProcessWorkspaceResources(pWS: *mut WmWorkspaceData);
    fn SetupDefaultResources(pSD: *mut WmScreenData);
    fn SetupWmSignalHandlers(_: ::core::ffi::c_int);
    fn SetupWmICCC();
    fn InitClientDecoration(pSD: *mut WmScreenData);
    fn ProcessWmFile(pSD: *mut WmScreenData);
    fn ProcessCommandLine(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char);
    fn ProcessMotifBindings();
    fn SaveMenuAccelerators(pSD: *mut WmScreenData, newMenuSpec: *mut MenuSpec);
    fn SyncModifierStrings();
    fn AddSMCallbacks();
    fn ExitWM(exitCode: ::core::ffi::c_int) -> !;
    fn LoadClientResourceDB() -> XrmDatabase;
    fn SetupXinerama();
    fn SetupWmEwmh();
    fn UpdateEwmhWorkspaceProperties(_: *mut WmScreenData);
    fn UpdateEwmhActiveWorkspace(_: *mut WmScreenData, _: WorkspaceID);
    static mut firstTime: ::core::ffi::c_int;
}
pub const XrmoptionSkipNArgs: XrmOptionKind = 7;
pub const XrmoptionSkipLine: XrmOptionKind = 6;
pub const XrmoptionSkipArg: XrmOptionKind = 5;
pub const XrmoptionResArg: XrmOptionKind = 4;
pub const XrmoptionSepArg: XrmOptionKind = 3;
pub const XrmoptionStickyArg: XrmOptionKind = 2;
pub const XrmoptionIsArg: XrmOptionKind = 1;
pub const XrmoptionNoArg: XrmOptionKind = 0;
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const XA_MWM_WM_REQUEST: C2RustUnnamed_33 = 4;
pub const XA_MWM_WORKSPACE_CURRENT: C2RustUnnamed_33 = 6;
pub const XA_MWM_WORKSPACE_LIST: C2RustUnnamed_33 = 5;
pub const XA_WmNall: C2RustUnnamed_33 = 3;
pub const XA_MWM_WORKSPACE_INFO: C2RustUnnamed_33 = 2;
pub const XA_MWM_WORKSPACE_PRESENCE: C2RustUnnamed_33 = 1;
pub const XA_MWM_WORKSPACE_HINTS: C2RustUnnamed_33 = 0;
pub const NUM_ATOMS: C2RustUnnamed_33 = 7;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CopyFromParent: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NoEventMask: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const SubstructureRedirectMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 20 as ::core::ffi::c_int;
pub const FocusChangeMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 21 as ::core::ffi::c_int;
pub const PropertyChangeMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 22 as ::core::ffi::c_int;
pub const MappingNotify: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const LockMask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const Mod1Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const Mod2Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const Mod3Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const Mod4Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const Mod5Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
pub const Mod1MapIndex: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const Button2: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const Button3: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const InputOnly: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CWOverrideRedirect: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const CWEventMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const GXinvert: ::core::ffi::c_int = 0xa as ::core::ffi::c_int;
pub const CapNotLast: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const IncludeInferiors: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GCFunction: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const GCPlaneMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const GCLineWidth: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const GCCapStyle: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const GCSubwindowMode: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 15 as ::core::ffi::c_int;
pub const MappingModifier: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MappingPointer: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_INFO_STARTUP_STANDARD: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const MWM_INFO_STARTUP_CUSTOM: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const _XA_MOTIF_WM_INFO: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"_MOTIF_WM_INFO\0") };
pub const _XA_MWM_INFO: [::core::ffi::c_char; 15] = _XA_MOTIF_WM_INFO;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WM_RESOURCE_CLASS: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Emwm\0") };
pub const CSERVE_NOT_AVAILABLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XGA_RES_DISPLAY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const _XA_MWM_WM_REQUEST: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"_MWM_WM_REQUEST\0") };
pub const _XA_MWM_WORKSPACE_HINTS: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"_MWM_WORKSPACE_HINTS\0")
};
pub const _XA_MWM_WORKSPACE_PRESENCE: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"_MWM_WORKSPACE_PRESENCE\0")
};
pub const _XA_MWM_WORKSPACE_INFO: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"_MWM_WORKSPACE_INFO\0")
};
pub const _XA_MWM_WORKSPACE_LIST: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"_MWM_WORKSPACE_LIST\0")
};
pub const _XA_MWM_WORKSPACE_CURRENT: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"_MWM_WORKSPACE_CURRENT\0")
};
pub const WM_ERROR_EXIT_VALUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FB_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DEFAULT_BEHAVIOR_ACTION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CUSTOM_BEHAVIOR_ACTION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RESTART_ACTION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const QUIT_MWM_ACTION: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STRETCH_NORTH_WEST: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STRETCH_NORTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STRETCH_NORTH_EAST: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STRETCH_EAST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STRETCH_SOUTH_EAST: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const STRETCH_SOUTH: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const STRETCH_SOUTH_WEST: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const STRETCH_WEST: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const F_CONTEXT_ROOT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const F_CONTEXT_ICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const F_CONTEXT_NORMAL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const F_CONTEXT_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const F_CONTEXT_WINDOW: ::core::ffi::c_long = F_CONTEXT_NORMAL | F_CONTEXT_MAXIMIZE;
pub const WINDOW_PART_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FRAME_NONE: ::core::ffi::c_int = WINDOW_PART_NONE;
pub const DEFAULT_BACKDROP_DIR: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"/usr/share/backdrops\0")
};
pub const ICON_ACTIVE_LABEL_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const DEFAULT_CLIENT_TITLE: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"*****\0") };
pub const DEFAULT_ICON_TITLE: [::core::ffi::c_char; 6] = DEFAULT_CLIENT_TITLE;
pub const NO_ACTION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GLOBAL_DATA_TYPE: ::core::ffi::c_int = 1001 as ::core::ffi::c_int;
pub const SCREEN_DATA_TYPE: ::core::ffi::c_int = 1003 as ::core::ffi::c_int;
pub const WORKSPACE_DATA_TYPE: ::core::ffi::c_int = 1004 as ::core::ffi::c_int;
pub const WmNall: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"all\0") };
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XC_bottom_left_corner: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const XC_bottom_right_corner: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const XC_bottom_side: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const XC_fleur: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const XC_left_ptr: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const XC_left_side: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
pub const XC_lr_angle: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const XC_right_side: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const XC_top_left_corner: ::core::ffi::c_int = 134 as ::core::ffi::c_int;
pub const XC_top_right_corner: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const XC_top_side: ::core::ffi::c_int = 138 as ::core::ffi::c_int;
pub const XC_ul_angle: ::core::ffi::c_int = 144 as ::core::ffi::c_int;
pub const XK_Scroll_Lock: ::core::ffi::c_int = 0xff14 as ::core::ffi::c_int;
pub const XK_Kana_Lock: ::core::ffi::c_int = 0xff2d as ::core::ffi::c_int;
pub const XK_Mode_switch: ::core::ffi::c_int = 0xff7e as ::core::ffi::c_int;
pub const XK_Num_Lock: ::core::ffi::c_int = 0xff7f as ::core::ffi::c_int;
pub const X_ChangeWindowAttributes: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const RRScreenChangeNotifyMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const _XA_MWM_SM_CLIENT_ID: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"SM_CLIENT_ID\0") };
pub const _XA_MWM_WMSAVE_HINT: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"_MWM_WMSAVE_HINT\0")
};
pub const UNSPECIFIED_SCREEN_NAME: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"fbk\0") };
#[no_mangle]
pub static mut dpy2Argv: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut dpy2Argc: ::core::ffi::c_int = 0;
unsafe extern "C" fn InitMouseBinding() {
    wmGD.numMouseButtons = XGetPointerMapping(
        wmGD.display,
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        0 as ::core::ffi::c_int,
    );
    if wmGD.numMouseButtons < 3 as ::core::ffi::c_int {
        wmGD.bMenuButton = Button2 as ::core::ffi::c_uint;
    } else {
        wmGD.bMenuButton = Button3 as ::core::ffi::c_uint;
    };
}
unsafe extern "C" fn BuildLockMaskSequence() {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut mask: ::core::ffi::c_uint = 0;
    let mut thisbit: ::core::ffi::c_uint = 0;
    let mut bit_on: Boolean = 0;
    let mut num_masks: ::core::ffi::c_int = 0;
    let mut num_bits: ::core::ffi::c_int = 0;
    let mut bit: ::core::ffi::c_int = 0;
    let mut run: ::core::ffi::c_int = 0;
    mask = wmGD.lockingModMask;
    num_bits = 0 as ::core::ffi::c_int;
    while mask != 0 {
        if mask & 0x1 as ::core::ffi::c_uint != 0 {
            num_bits += 1;
        }
        mask = mask >> 1 as ::core::ffi::c_int;
    }
    num_masks = ((0x1 as ::core::ffi::c_int) << num_bits) - 1 as ::core::ffi::c_int;
    wmGD.pLockMaskSequence = XtCalloc(
        (num_masks + 1 as ::core::ffi::c_int) as Cardinal,
        ::core::mem::size_of::<::core::ffi::c_uint>() as Cardinal,
    ) as *mut ::core::ffi::c_uint;
    mask = wmGD.lockingModMask;
    thisbit = 0x1 as ::core::ffi::c_uint;
    bit = 0 as ::core::ffi::c_int;
    while mask != 0 && thisbit != 0 {
        while thisbit & mask == 0 {
            thisbit = thisbit << 1 as ::core::ffi::c_int;
        }
        mask &= !thisbit;
        bit += 1;
        run = (0x1 as ::core::ffi::c_int) << bit - 1 as ::core::ffi::c_int;
        bit_on = False as Boolean;
        j = 0 as ::core::ffi::c_int;
        k = run - 1 as ::core::ffi::c_int;
        while j < num_masks {
            if k < 1 as ::core::ffi::c_int {
                if bit_on != 0 {
                    bit_on = False as Boolean;
                } else {
                    bit_on = True as Boolean;
                }
                k = run;
            }
            if bit_on != 0 {
                *wmGD.pLockMaskSequence.offset(j as isize) |= thisbit;
            }
            j += 1;
            k -= 1;
        }
    }
}
static mut pksLockingMods: [KeySym; 4] = [
    XK_Scroll_Lock as KeySym,
    XK_Kana_Lock as KeySym,
    XK_Num_Lock as KeySym,
    XK_Mode_switch as KeySym,
];
pub const NUM_LOCKING_MODS: usize = (::core::mem::size_of::<[KeySym; 4]>() as usize)
    .wrapping_div(::core::mem::size_of::<KeySym>() as usize);
unsafe extern "C" fn SetupLockingModifierMask() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut start_index: ::core::ffi::c_int = 0;
    let mut modifier_map = ::core::ptr::null_mut::<XModifierKeymap>();
    static mut mod_masks: [Modifiers; 6] = [
        None_0 as Modifiers,
        Mod1Mask as Modifiers,
        Mod2Mask as Modifiers,
        Mod3Mask as Modifiers,
        Mod4Mask as Modifiers,
        Mod5Mask as Modifiers,
    ];
    let mut dpy = wmGD.display;
    let mut pkcLockingMods: [::core::ffi::c_int; 4] = [0; 4];
    let mut kc: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as usize) < NUM_LOCKING_MODS {
        pkcLockingMods[i as usize] =
            XKeysymToKeycode(dpy, pksLockingMods[i as usize]) as ::core::ffi::c_int;
        i += 1;
    }
    wmGD.lockingModMask = LockMask as ::core::ffi::c_uint;
    modifier_map = XGetModifierMapping(dpy);
    start_index = (*modifier_map).max_keypermod * Mod1MapIndex;
    i = start_index;
    while i < (*modifier_map).max_keypermod * 8 as ::core::ffi::c_int {
        let mut this_mod =
            (i - start_index) / (*modifier_map).max_keypermod + 1 as ::core::ffi::c_int;
        kc = *(*modifier_map).modifiermap.offset(i as isize) as ::core::ffi::c_int;
        if kc != 0 {
            j = 0 as ::core::ffi::c_int;
            while (j as usize) < NUM_LOCKING_MODS {
                if pkcLockingMods[j as usize] == kc {
                    wmGD.lockingModMask |= mod_masks[this_mod as usize] as ::core::ffi::c_uint;
                    break;
                } else {
                    j += 1;
                }
            }
        }
        i += 1;
    }
    BuildLockMaskSequence();
    if !modifier_map.is_null() {
        XFreeModifiermap(modifier_map);
    }
}
unsafe extern "C" fn MappingEventHandler(
    mut w: Widget,
    mut client_data: XtPointer,
    mut event: *mut XEvent,
    mut cont_to_dispatch: *mut Boolean,
) {
    if (*event).xany.type_0 != MappingNotify || (*event).xmapping.request == MappingPointer {
        return;
    }
    if (*event).xmapping.request == MappingModifier {
        SetupLockingModifierMask();
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitWmGlobal(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut environ: *mut *mut ::core::ffi::c_char,
) {
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
    let mut scr: ::core::ffi::c_int = 0;
    let mut managed = 0 as ::core::ffi::c_int;
    let mut pch: [::core::ffi::c_char; 80] = [0; 80];
    let mut activeSet = False as Boolean;
    let mut processedGlobalResources = False as Boolean;
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    let mut args: [Arg; 20] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 20];
    let mut argnum: ::core::ffi::c_int = 0;
    let mut res_class = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut savedArgc: ::core::ffi::c_int = 0;
    wmGD.errorFlag = False as Boolean;
    SetupWmSignalHandlers(0 as ::core::ffi::c_int);
    wmGD.windowContextType = XrmUniqueQuark();
    wmGD.screenContextType = XrmUniqueQuark();
    wmGD.cmapWindowContextType = XrmUniqueQuark();
    wmGD.mwmWindowContextType = XrmUniqueQuark();
    savedArgc = argc;
    CopyArgv(argc, argv);
    wmGD.environ = environ as *mut *mut ::core::ffi::c_char;
    wmGD.mwmName = strrchr(
        *wmGD.argv.offset(0 as ::core::ffi::c_int as isize),
        '/' as i32,
    );
    if !wmGD.mwmName.is_null() {
        wmGD.mwmName = wmGD.mwmName.offset(1);
    } else {
        wmGD.mwmName = *wmGD.argv.offset(0 as ::core::ffi::c_int as isize);
    }
    res_class = WM_RESOURCE_CLASS.as_ptr() as *mut ::core::ffi::c_char;
    wmGD.statusColorServer = CSERVE_NOT_AVAILABLE;
    wmGD.display = NULL as *mut Display;
    wmGD.topLevelW = NULL as Widget;
    XtToolkitInitialize();
    wmGD.mwmAppContext = XtCreateApplicationContext();
    AddWmResourceConverters();
    XtAppSetFallbackResources(wmGD.mwmAppContext, ::core::ptr::null_mut::<String_0>());
    wmGD.display = XtOpenDisplay(
        wmGD.mwmAppContext,
        ::core::ptr::null::<::core::ffi::c_char>(),
        wmGD.mwmName,
        res_class,
        ::core::ptr::null_mut::<XrmOptionDescRec>(),
        0 as Cardinal,
        &raw mut argc,
        argv as *mut *mut ::core::ffi::c_char,
    );
    if wmGD.display.is_null() {
        Warning(
            b"Could not open display.\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        ExitWM(WM_ERROR_EXIT_VALUE);
    }
    WmInitErrorHandler(wmGD.display);
    InitCursorInfo();
    InitWmDisplayEnv();
    ShowWaitState(TRUE as Boolean);
    InitMouseBinding();
    ProcessMotifBindings();
    argnum = 0 as ::core::ffi::c_int;
    args[argnum as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(469 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = ::core::ptr::null_mut::<::core::ffi::c_void>() as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(885 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 10000 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(887 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 10 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 10 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(453 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(1057 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(1163 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 3 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(373 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = savedArgc as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(383 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = wmGD.argv as XtArgVal;
    argnum += 1;
    wmGD.topLevelW = XtAppCreateShell(
        ::core::ptr::null::<::core::ffi::c_char>(),
        res_class,
        sessionShellWidgetClass,
        wmGD.display,
        &raw mut args as ArgList,
        argnum as Cardinal,
    );
    SetupXinerama();
    XtAddEventHandler(
        wmGD.topLevelW,
        NoEventMask as EventMask,
        True as Boolean,
        Some(
            MappingEventHandler
                as unsafe extern "C" fn(Widget, XtPointer, *mut XEvent, *mut Boolean) -> (),
        ),
        NULL,
    );
    AddSMCallbacks();
    InitScreenNames();
    ProcessGlobalScreenResources();
    ProcessCommandLine(argc, argv);
    let mut sNum: ::core::ffi::c_int = 0;
    let mut useMaskRtn: Boolean = 0;
    let mut useMultiColorIcons: Boolean = 0;
    let mut useIconFileCacheRtn: Boolean = 0;
    let mut sBdPath = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sNum = if wmGD.numScreens == 1 as ::core::ffi::c_int {
        (*(wmGD.display as _XPrivDisplay)).default_screen
    } else {
        0 as ::core::ffi::c_int
    };
    XmeGetIconControlInfo(
        (*(wmGD.display as _XPrivDisplay))
            .screens
            .offset(sNum as isize) as *mut Screen,
        &raw mut useMaskRtn,
        &raw mut useMultiColorIcons,
        &raw mut useIconFileCacheRtn,
    );
    sBdPath = wmGD.backdropDirs;
    InsureDefaultBackdropDir(&raw mut sBdPath as *mut *mut ::core::ffi::c_char);
    XtFree(sBdPath as *mut ::core::ffi::c_char);
    wmGD.Screens = XtCalloc(
        wmGD.numScreens as Cardinal,
        ::core::mem::size_of::<WmScreenData>() as Cardinal,
    ) as *mut WmScreenData;
    if wmGD.Screens.is_null() {
        ShowWaitState(FALSE as Boolean);
        Warning(
            b"Insufficient memory for Screen data\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        ExitWM(WM_ERROR_EXIT_VALUE);
    } else {
        sAttributes.event_mask = SubstructureRedirectMask;
        scr = 0 as ::core::ffi::c_int;
        while scr < wmGD.numScreens {
            let mut sNum_0: ::core::ffi::c_int = 0;
            sNum_0 = if wmGD.numScreens == 1 as ::core::ffi::c_int {
                (*(wmGD.display as _XPrivDisplay)).default_screen
            } else {
                scr
            };
            wmGD.errorFlag = False as Boolean;
            XChangeWindowAttributes(
                wmGD.display,
                (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset(sNum_0 as isize))
                .root,
                CWEventMask as ::core::ffi::c_ulong,
                &raw mut sAttributes,
            );
            XSync(wmGD.display, False);
            if wmGD.errorFlag as ::core::ffi::c_int != 0
                && (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset(sNum_0 as isize))
                .root
                    == wmGD.errorResource
                && wmGD.errorRequestCode as ::core::ffi::c_int == X_ChangeWindowAttributes
            {
                sprintf(
                    &raw mut pch as *mut ::core::ffi::c_char,
                    b"Another window manager is running on screen %d\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    sNum_0,
                );
                Warning(
                    (&raw mut pch as *mut ::core::ffi::c_char)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                );
                (*wmGD.Screens.offset(scr as isize)).managed = False as Boolean;
            } else {
                if processedGlobalResources == 0 {
                    wmGD.hasShape = XShapeQueryExtension(
                        wmGD.display,
                        &raw mut wmGD.shapeEventBase,
                        &raw mut wmGD.shapeErrorBase,
                    ) as Boolean;
                    wmGD.replayEnterEvent = False;
                    wmGD.menuActive = ::core::ptr::null_mut::<MenuSpec>();
                    wmGD.menuUnpostKeySpec = ::core::ptr::null_mut::<KeySpec>();
                    wmGD.F_NextKeySpec = ::core::ptr::null_mut::<KeySpec>();
                    wmGD.F_PrevKeySpec = ::core::ptr::null_mut::<KeySpec>();
                    wmGD.passKeysActive = False as Boolean;
                    wmGD.passKeysKeySpec = ::core::ptr::null_mut::<KeySpec>();
                    wmGD.checkHotspot = False as Boolean;
                    wmGD.configAction = NO_ACTION;
                    wmGD.configPart = FRAME_NONE as ::core::ffi::c_uint;
                    wmGD.configSet = False as Boolean;
                    wmGD.preMove = False as Boolean;
                    wmGD.gadgetClient = ::core::ptr::null_mut::<ClientData>();
                    wmGD.wmTimers = ::core::ptr::null_mut::<WmTimer>();
                    wmGD.clientDefaultTitle =
                        XmStringCreateLocalized(DEFAULT_CLIENT_TITLE.as_ptr() as String_0);
                    wmGD.iconDefaultTitle =
                        XmStringCreateLocalized(DEFAULT_ICON_TITLE.as_ptr() as String_0);
                    wmGD.attributesWindow = NULL as Window;
                    wmGD.clickData.pCD = ::core::ptr::null_mut::<_ClientData>();
                    wmGD.clickData.clickPending = False as Boolean;
                    wmGD.clickData.doubleClickPending = False as Boolean;
                    wmGD.systemModalActive = False as Boolean;
                    wmGD.activeIconTextDisplayed = False as Boolean;
                    wmGD.movingIcon = False as Boolean;
                    wmGD.queryScreen = True as Boolean;
                    wmGD.dataType = GLOBAL_DATA_TYPE;
                    wmGD.pLockMaskSequence = ::core::ptr::null_mut::<::core::ffi::c_uint>();
                    SetupLockingModifierMask();
                    wmGD.cppCommand = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    wmGD.evLastButton.button = 0 as ::core::ffi::c_uint;
                    wmGD.bReplayedButton = False as Boolean;
                    wmGD.bSuspendSecondaryRestack = False as Boolean;
                    wmGD.display1 = XtOpenDisplay(
                        wmGD.mwmAppContext,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        wmGD.mwmName,
                        res_class,
                        ::core::ptr::null_mut::<XrmOptionDescRec>(),
                        0 as Cardinal,
                        &raw mut dpy2Argc,
                        dpy2Argv,
                    );
                    if wmGD.display1.is_null() {
                        ShowWaitState(FALSE as Boolean);
                        Warning(
                            b"Could not open second display connection.\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        ExitWM(WM_ERROR_EXIT_VALUE);
                    }
                    argnum = 0 as ::core::ffi::c_int;
                    args[argnum as usize].name =
                        (&raw const XtShellStrings as *const ::core::ffi::c_char)
                            .offset(469 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_char as String_0;
                    args[argnum as usize].value =
                        ::core::ptr::null_mut::<::core::ffi::c_void>() as XtArgVal;
                    argnum += 1;
                    args[argnum as usize].name =
                        (&raw const XtStrings as *const ::core::ffi::c_char)
                            .offset(885 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_char as String_0;
                    args[argnum as usize].value = 10000 as ::core::ffi::c_int as XtArgVal;
                    argnum += 1;
                    args[argnum as usize].name =
                        (&raw const XtStrings as *const ::core::ffi::c_char)
                            .offset(887 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_char as String_0;
                    args[argnum as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
                    argnum += 1;
                    args[argnum as usize].name =
                        (&raw const XtStrings as *const ::core::ffi::c_char)
                            .offset(872 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_char as String_0;
                    args[argnum as usize].value = 10 as ::core::ffi::c_int as XtArgVal;
                    argnum += 1;
                    args[argnum as usize].name =
                        (&raw const XtStrings as *const ::core::ffi::c_char)
                            .offset(234 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_char as String_0;
                    args[argnum as usize].value = 10 as ::core::ffi::c_int as XtArgVal;
                    argnum += 1;
                    args[argnum as usize].name =
                        (&raw const XtStrings as *const ::core::ffi::c_char)
                            .offset(453 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_char as String_0;
                    args[argnum as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
                    argnum += 1;
                    wmGD.topLevelW1 = XtAppCreateShell(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        res_class,
                        applicationShellWidgetClass,
                        wmGD.display1,
                        &raw mut args as ArgList,
                        argnum as Cardinal,
                    );
                    processedGlobalResources = True as Boolean;
                    ProcessMotifWmInfo(
                        (*(*(wmGD.display as _XPrivDisplay))
                            .screens
                            .offset(sNum_0 as isize))
                        .root,
                    );
                    ProcessWmResources();
                }
                InitWmScreen(
                    wmGD.Screens.offset(scr as isize) as *mut WmScreenData,
                    sNum_0,
                );
                (*wmGD.Screens.offset(scr as isize)).managed = True as Boolean;
                managed += 1;
                if activeSet == 0 {
                    activeSet = True as Boolean;
                    wmGD.pActiveSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
                }
            }
            scr += 1;
        }
        if managed == 0 as ::core::ffi::c_int {
            ShowWaitState(FALSE as Boolean);
            Warning(
                b"Unable to manage any screens on display.\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            ExitWM(WM_ERROR_EXIT_VALUE);
        }
    }
    argnum = 0 as ::core::ffi::c_int;
    args[argnum as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(10543 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    XtSetValues(wmGD.topLevelW, &raw mut args as ArgList, argnum as Cardinal);
    XtSetValues(
        wmGD.topLevelW1,
        &raw mut args as ArgList,
        argnum as Cardinal,
    );
    if fcntl(
        (*(wmGD.display as _XPrivDisplay)).fd,
        F_SETFD,
        1 as ::core::ffi::c_int,
    ) == -(1 as ::core::ffi::c_int)
    {
        ShowWaitState(FALSE as Boolean);
        Warning(
            b"Cannot configure X connection\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        ExitWM(WM_ERROR_EXIT_VALUE);
    }
    static mut atom_names: [*mut ::core::ffi::c_char; 7] = [
        _XA_MWM_WORKSPACE_HINTS.as_ptr() as *mut ::core::ffi::c_char,
        _XA_MWM_WORKSPACE_PRESENCE.as_ptr() as *mut ::core::ffi::c_char,
        _XA_MWM_WORKSPACE_INFO.as_ptr() as *mut ::core::ffi::c_char,
        WmNall.as_ptr() as *mut ::core::ffi::c_char,
        _XA_MWM_WM_REQUEST.as_ptr() as *mut ::core::ffi::c_char,
        _XA_MWM_WORKSPACE_LIST.as_ptr() as *mut ::core::ffi::c_char,
        _XA_MWM_WORKSPACE_CURRENT.as_ptr() as *mut ::core::ffi::c_char,
    ];
    let mut atoms: [Atom; 7] = [0; 7];
    XInternAtoms(
        wmGD.display,
        &raw mut atom_names as *mut *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[*mut ::core::ffi::c_char; 7]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as Cardinal as ::core::ffi::c_int,
        False,
        &raw mut atoms as *mut Atom,
    );
    wmGD.xa_MWM_WORKSPACE_HINTS = atoms[XA_MWM_WORKSPACE_HINTS as ::core::ffi::c_int as usize];
    wmGD.xa_MWM_WORKSPACE_PRESENCE =
        atoms[XA_MWM_WORKSPACE_PRESENCE as ::core::ffi::c_int as usize];
    wmGD.xa_MWM_WORKSPACE_INFO = atoms[XA_MWM_WORKSPACE_INFO as ::core::ffi::c_int as usize];
    wmGD.xa_ALL_WORKSPACES = atoms[XA_WmNall as ::core::ffi::c_int as usize];
    wmGD.xa_MWM_WORKSPACE_LIST = atoms[XA_MWM_WORKSPACE_LIST as ::core::ffi::c_int as usize];
    wmGD.xa_MWM_WORKSPACE_CURRENT = atoms[XA_MWM_WORKSPACE_CURRENT as ::core::ffi::c_int as usize];
    wmGD.xa_MWM_WM_REQUEST = atoms[XA_MWM_WM_REQUEST as ::core::ffi::c_int as usize];
    wmGD.xa_SM_CLIENT_ID = XmInternAtom(
        wmGD.display,
        _XA_MWM_SM_CLIENT_ID.as_ptr() as String_0,
        False as Boolean,
    );
    wmGD.xa_WMSAVE_HINT = XmInternAtom(
        wmGD.display,
        _XA_MWM_WMSAVE_HINT.as_ptr() as String_0,
        False as Boolean,
    );
    wmGD.clientResourceDB = LoadClientResourceDB();
    SetupWmWorkspaceWindows();
    MakeWorkspaceCursors();
    SyncModifierStrings();
    scr = 0 as ::core::ffi::c_int;
    while scr < wmGD.numScreens {
        pSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
        if (*pSD).managed != 0 {
            if XDefaultScreen(wmGD.display) == (*pSD).screen {
                wmGD.commandWindow = (*wmGD.Screens.offset(scr as isize)).wmWorkspaceWin;
            }
            InitWorkspaceColormap(pSD);
            ProcessWmFile(pSD);
            SetupDefaultResources(pSD);
            if (*pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART != 0 {
                CreateActiveIconTextWindow(pSD);
            }
            MakeWmFunctionResources(pSD);
        }
        XDefineCursor(
            wmGD.display,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset(scr as isize))
            .root,
            wmGD.workspaceCursor,
        );
        scr += 1;
    }
    (*wmGD.display1).db = (*wmGD.display).db;
    XtRealizeWidget(wmGD.topLevelW);
    XtRealizeWidget(wmGD.topLevelW1);
    sAttributes.override_redirect = True;
    XChangeWindowAttributes(
        wmGD.display,
        XtWindowOfObject(wmGD.topLevelW),
        CWOverrideRedirect as ::core::ffi::c_ulong,
        &raw mut sAttributes,
    );
    if XRRQueryExtension(
        wmGD.display,
        &raw mut wmGD.xrandr_base_evt,
        &raw mut wmGD.xrandr_base_err,
    ) != 0
    {
        wmGD.xrandr_present = True as Boolean;
        XRRSelectInput(
            wmGD.display,
            XtWindowOfObject(wmGD.topLevelW),
            RRScreenChangeNotifyMask as ::core::ffi::c_int,
        );
    } else {
        wmGD.xrandr_present = False as Boolean;
    }
    SetupWmICCC();
    SetupWmEwmh();
    InitEventHandling();
    scr = 0 as ::core::ffi::c_int;
    while scr < wmGD.numScreens {
        pSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
        if (*pSD).managed != 0 {
            InitClientDecoration(pSD);
            if (*pSD).useIconBox != 0 {
                InitIconBox(pSD);
            }
            AdoptInitialClients(pSD);
            InitColormapFocus(pSD);
        }
        scr += 1;
    }
    scr = 0 as ::core::ffi::c_int;
    while scr < wmGD.numScreens {
        let mut iws: ::core::ffi::c_int = 0;
        pSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
        if (*pSD).managed != 0 {
            wmGD.pActiveSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
            MapIconBoxes((*pSD).pActiveWS as *mut WmWorkspaceData);
            ChangeBackdrop((*pSD).pActiveWS as *mut WmWorkspaceData);
            SetCurrentWorkspaceProperty(pSD);
            SetWorkspaceListProperty(pSD);
            iws = 0 as ::core::ffi::c_int;
            while iws < (*pSD).numWorkspaces {
                SetWorkspaceInfoProperty((*pSD).pWS.offset(iws as isize) as *mut WmWorkspaceData);
                iws += 1;
            }
            XFlush(wmGD.display);
        }
        UpdateEwmhWorkspaceProperties(pSD);
        UpdateEwmhActiveWorkspace(pSD, (*(*pSD).pActiveWS).id);
        scr += 1;
    }
    InitKeyboardFocus();
    InitWmDisplayEnv();
    ShowWaitState(FALSE as Boolean);
    firstTime = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn InitWmScreen(mut pSD: *mut WmScreenData, mut sNum: ::core::ffi::c_int) {
    let mut args: [Arg; 12] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 12];
    let mut argnum: ::core::ffi::c_int = 0;
    let mut wsnum: ::core::ffi::c_int = 0;
    let mut pwsI = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut buf_size: ::core::ffi::c_uint = 0;
    let mut iwsx: ::core::ffi::c_int = 0;
    let mut pDisplayName = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buffer: [::core::ffi::c_char; 256] = [0; 256];
    let mut displayName: [::core::ffi::c_char; 256] = [0; 256];
    let mut token1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pSD).rootWindow = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset(sNum as isize))
    .root;
    (*pSD).clientCounter = 0 as ::core::ffi::c_ulong;
    (*pSD).defaultSystemMenuUseBuiltin = TRUE as Boolean;
    (*pSD).displayString = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pSD).acceleratorMenuCount = 0 as ::core::ffi::c_uint;
    (*pSD).activeIconTextWin = NULL as Window;
    (*pSD).focusPriority = 0 as ::core::ffi::c_long;
    (*pSD).inputScreenWindow = NULL as Window;
    (*pSD).colormapFocus = ::core::ptr::null_mut::<_ClientData>();
    (*pSD).keySpecs = ::core::ptr::null_mut::<KeySpec>();
    (*pSD).screen = sNum;
    (*pSD).confirmboxW[DEFAULT_BEHAVIOR_ACTION as usize] = ::core::ptr::null_mut::<_WidgetRec>();
    (*pSD).confirmboxW[CUSTOM_BEHAVIOR_ACTION as usize] = ::core::ptr::null_mut::<_WidgetRec>();
    (*pSD).confirmboxW[RESTART_ACTION as usize] = ::core::ptr::null_mut::<_WidgetRec>();
    (*pSD).confirmboxW[QUIT_MWM_ACTION as usize] = ::core::ptr::null_mut::<_WidgetRec>();
    (*pSD).feedbackWin = NULL as Window;
    (*pSD).fbStyle = FB_OFF as ::core::ffi::c_ulong;
    (*pSD).fbWinWidth = 0 as ::core::ffi::c_uint;
    (*pSD).fbWinHeight = 0 as ::core::ffi::c_uint;
    (*pSD).fbLocation = ::core::ptr::null_mut::<__XmStringRec>();
    (*pSD).fbSize = ::core::ptr::null_mut::<__XmStringRec>();
    (*pSD).fbLocX = 0 as ::core::ffi::c_int;
    (*pSD).fbLocY = 0 as ::core::ffi::c_int;
    (*pSD).fbSizeX = 0 as ::core::ffi::c_int;
    (*pSD).fbSizeY = 0 as ::core::ffi::c_int;
    (*pSD).fbLastX = -(1 as ::core::ffi::c_int);
    (*pSD).fbLastY = -(1 as ::core::ffi::c_int);
    (*pSD).fbLastWidth = -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    (*pSD).fbLastHeight = -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    (*pSD).fbTop = ::core::ptr::null_mut::<RList>();
    (*pSD).fbBottom = ::core::ptr::null_mut::<RList>();
    (*pSD).actionNbr = -(1 as ::core::ffi::c_int);
    (*pSD).clientList = ::core::ptr::null_mut::<ClientListEntry>();
    (*pSD).lastClient = ::core::ptr::null_mut::<ClientListEntry>();
    (*pSD).lastInstalledColormap = NULL as Colormap;
    (*pSD).shrinkWrapGC = ::core::ptr::null_mut::<_XGC>();
    (*pSD).bitmapCache = ::core::ptr::null_mut::<BitmapCache>();
    (*pSD).bitmapCacheSize = 0 as ::core::ffi::c_uint;
    (*pSD).bitmapCacheCount = 0 as ::core::ffi::c_uint;
    (*pSD).dataType = SCREEN_DATA_TYPE;
    (*pSD).managed = False as Boolean;
    (*pSD).initialWorkspace = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pSD).presence.shellW = ::core::ptr::null_mut::<_WidgetRec>();
    (*pSD).presence.onScreen = False as Boolean;
    (*pSD).presence.userDismissed = True as Boolean;
    (*pSD).workspaceList = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pSD).numWorkspaces = 0 as ::core::ffi::c_int;
    (*pSD).numWsDataAllocated = 0 as ::core::ffi::c_int;
    (*pSD).lastBackdropWin = None_0 as Window;
    (*pSD).pSessionItems = ::core::ptr::null_mut::<_WmSessionItem>();
    (*pSD).totalSessionItems = 0 as ::core::ffi::c_int;
    (*pSD).remainingSessionItems = 0 as ::core::ffi::c_int;
    (*pSD).bMarqueeSelectionInitialized = False as Boolean;
    (*pSD).woN = 0 as ::core::ffi::c_long as Window;
    (*pSD).woS = 0 as ::core::ffi::c_long as Window;
    (*pSD).woE = 0 as ::core::ffi::c_long as Window;
    (*pSD).woW = 0 as ::core::ffi::c_long as Window;
    (*pSD).displayResolutionType = XGA_RES_DISPLAY;
    ProcessWmColors(pSD);
    if strcmp(
        *wmGD.screenNames.offset(sNum as isize) as *mut ::core::ffi::c_char,
        UNSPECIFIED_SCREEN_NAME.as_ptr(),
    ) == 0
    {
        sprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"%d\0" as *const u8 as *const ::core::ffi::c_char,
            sNum,
        );
        buf_size = strlen(&raw mut buffer as *mut ::core::ffi::c_char).wrapping_add(1 as size_t)
            as ::core::ffi::c_uint;
        let ref mut fresh0 = *wmGD.screenNames.offset(sNum as isize);
        *fresh0 = XtRealloc(
            *wmGD.screenNames.offset(sNum as isize) as *mut ::core::ffi::c_char,
            buf_size as Cardinal,
        ) as *mut ::core::ffi::c_uchar;
        if (*fresh0).is_null() {
            Warning(
                b"Cannot create enough memory for the screen names\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            ExitWM(WM_ERROR_EXIT_VALUE);
        } else {
            strcpy(
                *wmGD.screenNames.offset(sNum as isize) as *mut ::core::ffi::c_char,
                &raw mut buffer as *mut ::core::ffi::c_char,
            );
        }
    }
    XSaveContext(
        wmGD.display,
        (*pSD).rootWindow as XID,
        wmGD.screenContextType,
        pSD as XPointer as *const ::core::ffi::c_char,
    );
    argnum = 0 as ::core::ffi::c_int;
    args[argnum as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(469 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = ::core::ptr::null_mut::<::core::ffi::c_void>() as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(885 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 10000 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(887 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 10000 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 10 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 10 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(186 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(163 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset(sNum as isize))
    .root_depth as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(584 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = (*(wmGD.display as _XPrivDisplay))
        .screens
        .offset(sNum as isize) as *mut Screen as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(154 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset(sNum as isize))
    .cmap as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(104 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset(sNum as isize))
    .root_visual as XtArgVal;
    argnum += 1;
    (*pSD).screenTopLevelW = XtCreatePopupShell(
        *wmGD.screenNames.offset(sNum as isize) as String_0 as *const ::core::ffi::c_char,
        vendorShellWidgetClass,
        wmGD.topLevelW,
        &raw mut args as ArgList,
        argnum as Cardinal,
    );
    XtRealizeWidget((*pSD).screenTopLevelW);
    argnum = 0 as ::core::ffi::c_int;
    args[argnum as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(469 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = ::core::ptr::null_mut::<::core::ffi::c_void>() as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(885 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 10000 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(887 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 10000 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 10 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 10 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(186 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 1 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(453 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 0 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(163 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = (*(*(wmGD.display1 as _XPrivDisplay))
        .screens
        .offset(sNum as isize))
    .root_depth as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(584 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = (*(wmGD.display1 as _XPrivDisplay))
        .screens
        .offset(sNum as isize) as *mut Screen as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(154 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = (*(*(wmGD.display1 as _XPrivDisplay))
        .screens
        .offset(sNum as isize))
    .cmap as XtArgVal;
    argnum += 1;
    (*pSD).screenTopLevelW1 = XtCreatePopupShell(
        *wmGD.screenNames.offset(sNum as isize) as String_0 as *const ::core::ffi::c_char,
        vendorShellWidgetClass,
        wmGD.topLevelW1,
        &raw mut args as ArgList,
        argnum as Cardinal,
    );
    XtRealizeWidget((*pSD).screenTopLevelW1);
    ProcessScreenResources(pSD, *wmGD.screenNames.offset(sNum as isize));
    MakeXorGC(pSD);
    InitIconSize(pSD);
    wsnum = 0 as ::core::ffi::c_int;
    pwsI = (*pSD).pWS as *mut WmWorkspaceData;
    while wsnum < (*pSD).numWorkspaces {
        InitWmWorkspace(pwsI, pSD);
        wsnum += 1;
        pwsI = pwsI.offset(1);
    }
    if !(*pSD).initialWorkspace.is_null() {
        iwsx = 0 as ::core::ffi::c_int;
        while iwsx < (*pSD).numWorkspaces {
            if strcmp(
                (*(*pSD).pWS.offset(iwsx as isize)).name as *const ::core::ffi::c_char,
                (*pSD).initialWorkspace as *const ::core::ffi::c_char,
            ) == 0
            {
                break;
            }
            iwsx += 1;
        }
        if iwsx >= (*pSD).numWorkspaces {
            (*pSD).pActiveWS = (*pSD).pWS as *mut _WmWorkspaceData;
        } else {
            (*pSD).pActiveWS =
                (*pSD).pWS.offset(iwsx as isize) as *mut _WmWorkspaceData as *mut _WmWorkspaceData;
        }
    } else {
        (*pSD).pActiveWS = (*pSD).pWS as *mut _WmWorkspaceData;
    }
    pDisplayName = (*(wmGD.display as _XPrivDisplay)).display_name;
    strcpy(
        &raw mut displayName as *mut ::core::ffi::c_char,
        pDisplayName,
    );
    token1 = strtok(
        &raw mut displayName as *mut ::core::ffi::c_char,
        b":\0" as *const u8 as *const ::core::ffi::c_char,
    );
    token2 = strtok(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        b".\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !token2.is_null()
        || {
            token2 = strtok(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                b"\0" as *const u8 as *const ::core::ffi::c_char,
            );
            !token2.is_null()
        }
        || displayName[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == ':' as i32
    {
        if displayName[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == ':' as i32 {
            token2 = strtok(token1, b".\0" as *const u8 as *const ::core::ffi::c_char);
            if !token2.is_null() {
                sprintf(
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    b"DISPLAY=:%s.%d\0" as *const u8 as *const ::core::ffi::c_char,
                    token2,
                    sNum,
                );
            }
        } else {
            sprintf(
                &raw mut buffer as *mut ::core::ffi::c_char,
                b"DISPLAY=%s:%s.%d\0" as *const u8 as *const ::core::ffi::c_char,
                token1,
                token2,
                sNum,
            );
        }
        (*pSD).displayString = XtMalloc(
            strlen(&raw mut buffer as *mut ::core::ffi::c_char).wrapping_add(1 as size_t)
                as Cardinal,
        );
        if (*pSD).displayString.is_null() {
            Warning(
                b"Insufficient memory for displayString\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            strcpy(
                (*pSD).displayString as *mut ::core::ffi::c_char,
                &raw mut buffer as *mut ::core::ffi::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitWmWorkspace(
    mut pWS: *mut WmWorkspaceData,
    mut pSD: *mut WmScreenData,
) {
    let mut args: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 10];
    let mut argnum: ::core::ffi::c_int = 0;
    (*pWS).pSD = pSD;
    (*pWS).pIconBox = ::core::ptr::null_mut::<IconBoxData>();
    (*pWS).dataType = WORKSPACE_DATA_TYPE;
    (*pWS).IPData = ::core::ptr::null_mut::<IconPlacementData>();
    (*pWS).backdrop.window = 0 as Window;
    (*pWS).backdrop.nameAtom = 0 as Atom;
    (*pWS).backdrop.image = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pWS).numClients = 0 as ::core::ffi::c_uint;
    (*pWS).sizeClientList = 0 as ::core::ffi::c_uint;
    (*pWS).ppClients = ::core::ptr::null_mut::<*mut _ClientData>();
    (*pWS).buttonW = ::core::ptr::null_mut::<_WidgetRec>();
    (*pWS).keyFocus = ::core::ptr::null_mut::<_ClientData>();
    (*pWS).nextKeyFocus = ::core::ptr::null_mut::<_ClientData>();
    argnum = 0 as ::core::ffi::c_int;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(163 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*pSD).screen as isize))
    .root_depth as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(584 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = (*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*pSD).screen as isize) as *mut Screen
        as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(154 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*pSD).screen as isize))
    .cmap as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    args[argnum as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[argnum as usize].value = 5 as ::core::ffi::c_int as XtArgVal;
    argnum += 1;
    (*pWS).workspaceTopLevelW = XtCreateWidget(
        (*pWS).name as *const ::core::ffi::c_char,
        xmPrimitiveWidgetClass,
        (*pSD).screenTopLevelW,
        &raw mut args as ArgList,
        argnum as Cardinal,
    );
    XtRealizeWidget((*pWS).workspaceTopLevelW);
    (*pWS).id = XInternAtom(
        wmGD.display,
        (*pWS).name as *const ::core::ffi::c_char,
        False,
    ) as WorkspaceID;
    ProcessWorkspaceResources(pWS);
    if wmGD.iconAutoPlace != 0 {
        InitIconPlacement(pWS);
    }
}
unsafe extern "C" fn InsureDefaultBackdropDir(mut ppchBackdropDirs: *mut *mut ::core::ffi::c_char) {
    let mut len: ::core::ffi::c_int = 0;
    let mut bFound = False as Boolean;
    let mut pch = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pchEnd = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pch2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tmpptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pchD = DEFAULT_BACKDROP_DIR.as_ptr() as *mut ::core::ffi::c_char;
    let mut chlen: ::core::ffi::c_uint = 0;
    let mut homeDir = ::core::ptr::null_mut::<::core::ffi::c_char>();
    pch = *ppchBackdropDirs;
    len = strlen(pchD) as ::core::ffi::c_int;
    pchEnd = pch.offset(strlen(pch) as isize);
    while bFound == 0 && !pch.is_null() && *pch as ::core::ffi::c_int != '\0' as i32 {
        if strncmp(pch, pchD, len as size_t) == 0 as ::core::ffi::c_int {
            pch2 = pch.offset(len as isize);
            if pch2 <= pchEnd
                && (*pch2 as ::core::ffi::c_int == '\0' as i32
                    || mblen(pch2, __ctype_get_mb_cur_max()) == 1 as ::core::ffi::c_int
                        && *pch2 as ::core::ffi::c_int == ':' as i32)
            {
                bFound = True as Boolean;
            }
        } else {
            pch = strchr(pch, ':' as i32);
            if !pch.is_null() && *pch as ::core::ffi::c_int != '\0' as i32 {
                chlen = mblen(pch, __ctype_get_mb_cur_max()) as ::core::ffi::c_uint;
                pch = pch.offset(chlen as isize);
            }
        }
    }
    homeDir = XmeGetHomeDirName();
    if bFound != 0 {
        len = strlen(homeDir)
            .wrapping_add(strlen(
                b"/.backdrops\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(strlen(*ppchBackdropDirs))
            .wrapping_add(strlen(
                b"/usr/share/backdrops\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(3 as size_t) as ::core::ffi::c_int;
        tmpptr = XtMalloc(
            (len as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                as Cardinal,
        );
        strcpy(tmpptr, homeDir);
        strcat(
            tmpptr,
            b"/.backdrops\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcat(tmpptr, b":\0" as *const u8 as *const ::core::ffi::c_char);
        strcat(
            tmpptr,
            b"/usr/share/backdrops\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcat(tmpptr, b":\0" as *const u8 as *const ::core::ffi::c_char);
        strcat(tmpptr, *ppchBackdropDirs);
        *ppchBackdropDirs = tmpptr;
    } else {
        len = strlen(homeDir)
            .wrapping_add(strlen(
                b"/.backdrops\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(strlen(*ppchBackdropDirs))
            .wrapping_add(strlen(pchD))
            .wrapping_add(strlen(
                b"/usr/share/backdrops\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(4 as size_t) as ::core::ffi::c_int;
        tmpptr = XtMalloc(
            (len as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                as Cardinal,
        );
        strcpy(tmpptr, homeDir);
        strcat(
            tmpptr,
            b"/.backdrops\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcat(tmpptr, b":\0" as *const u8 as *const ::core::ffi::c_char);
        strcat(tmpptr, *ppchBackdropDirs);
        strcat(tmpptr, b":\0" as *const u8 as *const ::core::ffi::c_char);
        strcat(
            tmpptr,
            b"/usr/share/backdrops\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcat(tmpptr, b":\0" as *const u8 as *const ::core::ffi::c_char);
        strcat(tmpptr, pchD);
        *ppchBackdropDirs = tmpptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ProcessMotifWmInfo(mut rootWindowOfScreen: Window) {
    let mut pMwmInfo = ::core::ptr::null_mut::<MwmInfo>();
    wmGD.xa_MWM_INFO = XInternAtom(wmGD.display, _XA_MWM_INFO.as_ptr(), False);
    pMwmInfo = GetMwmInfo(rootWindowOfScreen) as *mut MotifWmInfo as *mut MwmInfo;
    if !pMwmInfo.is_null() {
        wmGD.useStandardBehavior = (if (*pMwmInfo).flags & MWM_INFO_STARTUP_STANDARD != 0 {
            True
        } else {
            False
        }) as Boolean;
        wmGD.wmRestarted = True as Boolean;
        XFree(pMwmInfo as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    } else {
        wmGD.useStandardBehavior = False as Boolean;
        wmGD.wmRestarted = False as Boolean;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SetupWmWorkspaceWindows() {
    let mut scr: ::core::ffi::c_int = 0;
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
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
    scr = 0 as ::core::ffi::c_int;
    while scr < wmGD.numScreens {
        pSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
        if (*pSD).managed != 0 {
            sAttributes.override_redirect = True;
            sAttributes.event_mask = FocusChangeMask | PropertyChangeMask;
            (*pSD).wmWorkspaceWin = XCreateWindow(
                wmGD.display,
                (*pSD).rootWindow,
                -(100 as ::core::ffi::c_int),
                -(100 as ::core::ffi::c_int),
                10 as ::core::ffi::c_uint,
                10 as ::core::ffi::c_uint,
                0 as ::core::ffi::c_uint,
                0 as ::core::ffi::c_int,
                InputOnly as ::core::ffi::c_uint,
                ::core::ptr::null_mut::<Visual>(),
                (CWOverrideRedirect | CWEventMask) as ::core::ffi::c_ulong,
                &raw mut sAttributes,
            );
            XMapWindow(wmGD.display, (*pSD).wmWorkspaceWin);
            SetMwmInfo(
                (*pSD).rootWindow,
                if wmGD.useStandardBehavior as ::core::ffi::c_int != 0 {
                    MWM_INFO_STARTUP_STANDARD
                } else {
                    MWM_INFO_STARTUP_CUSTOM
                },
                (*pSD).wmWorkspaceWin,
            );
            XSaveContext(
                wmGD.display,
                (*pSD).wmWorkspaceWin as XID,
                wmGD.mwmWindowContextType,
                pSD as XPointer as *const ::core::ffi::c_char,
            );
        }
        scr += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MakeWorkspaceCursors() {
    wmGD.workspaceCursor = XCreateFontCursor(wmGD.display, XC_left_ptr as ::core::ffi::c_uint);
    wmGD.stretchCursors[STRETCH_NORTH_WEST as usize] =
        XCreateFontCursor(wmGD.display, XC_top_left_corner as ::core::ffi::c_uint);
    wmGD.stretchCursors[STRETCH_NORTH as usize] =
        XCreateFontCursor(wmGD.display, XC_top_side as ::core::ffi::c_uint);
    wmGD.stretchCursors[STRETCH_NORTH_EAST as usize] =
        XCreateFontCursor(wmGD.display, XC_top_right_corner as ::core::ffi::c_uint);
    wmGD.stretchCursors[STRETCH_EAST as usize] =
        XCreateFontCursor(wmGD.display, XC_right_side as ::core::ffi::c_uint);
    wmGD.stretchCursors[STRETCH_SOUTH_EAST as usize] =
        XCreateFontCursor(wmGD.display, XC_bottom_right_corner as ::core::ffi::c_uint);
    wmGD.stretchCursors[STRETCH_SOUTH as usize] =
        XCreateFontCursor(wmGD.display, XC_bottom_side as ::core::ffi::c_uint);
    wmGD.stretchCursors[STRETCH_SOUTH_WEST as usize] =
        XCreateFontCursor(wmGD.display, XC_bottom_left_corner as ::core::ffi::c_uint);
    wmGD.stretchCursors[STRETCH_WEST as usize] =
        XCreateFontCursor(wmGD.display, XC_left_side as ::core::ffi::c_uint);
    wmGD.configCursor = XCreateFontCursor(wmGD.display, XC_fleur as ::core::ffi::c_uint);
    wmGD.movePlacementCursor = XCreateFontCursor(wmGD.display, XC_ul_angle as ::core::ffi::c_uint);
    wmGD.sizePlacementCursor = XCreateFontCursor(wmGD.display, XC_lr_angle as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn MakeWmFunctionResources(mut pSD: *mut WmScreenData) {
    let mut buttonSpec = ::core::ptr::null_mut::<ButtonSpec>();
    let mut keySpec = ::core::ptr::null_mut::<KeySpec>();
    let mut menuSpec = ::core::ptr::null_mut::<MenuSpec>();
    let mut menuContext: Context = 0;
    buttonSpec = (*pSD).buttonSpecs;
    while !buttonSpec.is_null() {
        if (*buttonSpec).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Menu as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) {
            if (*buttonSpec).context & F_CONTEXT_WINDOW as Context != 0 {
                menuContext = F_CONTEXT_WINDOW as Context;
            } else if (*buttonSpec).context & F_CONTEXT_ICON as Context != 0 {
                menuContext = F_CONTEXT_ICON as Context;
            } else {
                menuContext = F_CONTEXT_ROOT as Context;
            }
            menuSpec = MakeMenu(
                pSD,
                (*buttonSpec).wmFuncArgs,
                menuContext,
                (*buttonSpec).context,
                NULL as *mut MenuItem,
                FALSE as Boolean,
            );
            if !menuSpec.is_null() {
                SaveMenuAccelerators(pSD, menuSpec);
            } else {
                (*buttonSpec).wmFunction = Some(
                    F_Nop
                        as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
                ) as WmFunction;
            }
        }
        buttonSpec = (*buttonSpec).nextButtonSpec as *mut ButtonSpec;
    }
    keySpec = (*pSD).keySpecs;
    while !keySpec.is_null() {
        if (*keySpec).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Menu as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) {
            if (*keySpec).context & F_CONTEXT_WINDOW as Context != 0 {
                menuContext = F_CONTEXT_WINDOW as Context;
            } else if (*keySpec).context & F_CONTEXT_ICON as Context != 0 {
                menuContext = F_CONTEXT_ICON as Context;
            } else {
                menuContext = F_CONTEXT_ROOT as Context;
            }
            menuSpec = MakeMenu(
                pSD,
                (*keySpec).wmFuncArgs,
                menuContext,
                (*keySpec).context,
                NULL as *mut MenuItem,
                FALSE as Boolean,
            );
            if !menuSpec.is_null() {
                SaveMenuAccelerators(pSD, menuSpec);
            } else {
                (*keySpec).wmFunction = Some(
                    F_Nop
                        as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
                ) as WmFunction;
            }
        }
        keySpec = (*keySpec).nextKeySpec as *mut KeySpec;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MakeXorGC(mut pSD: *mut WmScreenData) {
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
    let mut mask: XtGCMask = 0;
    mask = (GCFunction | GCLineWidth | GCSubwindowMode | GCCapStyle) as XtGCMask;
    gcv.function = GXinvert;
    gcv.line_width = 0 as ::core::ffi::c_int;
    gcv.cap_style = CapNotLast;
    gcv.subwindow_mode = IncludeInferiors;
    gcv.plane_mask = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
    .black_pixel
        ^ (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(wmGD.display as _XPrivDisplay)).default_screen as isize))
        .white_pixel;
    mask = mask | GCPlaneMask as XtGCMask;
    (*pSD).xorGC = XCreateGC(
        wmGD.display,
        (*pSD).rootWindow as Drawable,
        mask as ::core::ffi::c_ulong,
        &raw mut gcv,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CopyArgv(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0;
    wmGD.argv = XtMalloc(
        ((argc + 1 as ::core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as Cardinal,
    ) as *mut *mut ::core::ffi::c_char;
    if wmGD.argv.is_null() {
        Warning(
            b"Insufficient memory for window manager data\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        wmGD.argv = argv as *mut *mut ::core::ffi::c_char;
        dpy2Argv = argv as *mut *mut ::core::ffi::c_char;
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            let ref mut fresh2 = *wmGD.argv.offset(i as isize);
            *fresh2 = *argv.offset(i as isize);
            i += 1;
        }
        let ref mut fresh3 = *wmGD.argv.offset(i as isize);
        *fresh3 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        dpy2Argv = XtMalloc(
            ((argc + 1 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                as Cardinal,
        ) as *mut *mut ::core::ffi::c_char;
        if dpy2Argv.is_null() {
            Warning(
                b"Insufficient memory for window manager data\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            dpy2Argv = argv as *mut *mut ::core::ffi::c_char;
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < argc {
                let ref mut fresh4 = *dpy2Argv.offset(i as isize);
                *fresh4 = *argv.offset(i as isize);
                i += 1;
            }
            dpy2Argc = argc;
            let ref mut fresh5 = *dpy2Argv.offset(i as isize);
            *fresh5 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn InitScreenNames() {
    let mut num: ::core::ffi::c_int = 0;
    let mut numScreens: ::core::ffi::c_int = 0;
    numScreens = (*(wmGD.display as _XPrivDisplay)).nscreens;
    wmGD.screenNames = XtMalloc(
        (numScreens as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as Cardinal,
    ) as *mut *mut ::core::ffi::c_uchar;
    if wmGD.screenNames.is_null() {
        ShowWaitState(FALSE as Boolean);
        Warning(
            b"Insufficient memory for screen names\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        ExitWM(WM_ERROR_EXIT_VALUE);
    }
    num = 0 as ::core::ffi::c_int;
    while num < numScreens {
        let ref mut fresh1 = *wmGD.screenNames.offset(num as isize);
        *fresh1 = XtMalloc(
            (4 as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as Cardinal,
        ) as *mut ::core::ffi::c_uchar;
        if (*fresh1).is_null() {
            ShowWaitState(FALSE as Boolean);
            Warning(
                b"Insufficient memory for screen names\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            ExitWM(WM_ERROR_EXIT_VALUE);
        }
        sprintf(
            *wmGD.screenNames.offset(num as isize) as *mut ::core::ffi::c_char,
            UNSPECIFIED_SCREEN_NAME.as_ptr(),
        );
        num += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitWmDisplayEnv() {
    let mut displayName = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buf_size: size_t = 0;
    displayName = (*(wmGD.display as _XPrivDisplay)).display_name;
    buf_size = strlen(displayName).wrapping_add(10 as size_t);
    buffer = XtMalloc(buf_size as Cardinal) as *mut ::core::ffi::c_char;
    if buffer.is_null() {
        wmGD.displayString = ::core::ptr::null_mut::<::core::ffi::c_char>();
        Warning(
            b"Insufficient memory for displayString\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    } else {
        snprintf(
            buffer,
            buf_size,
            b"DISPLAY=%s\0" as *const u8 as *const ::core::ffi::c_char,
            displayName,
        );
        wmGD.displayString = buffer as String_0;
        putenv(wmGD.displayString as *mut ::core::ffi::c_char);
    };
}
