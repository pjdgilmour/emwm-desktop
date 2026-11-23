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
    fn XCreateGC(_: *mut Display, _: Drawable, _: ::core::ffi::c_ulong, _: *mut XGCValues) -> GC;
    fn XCreateBitmapFromData(
        _: *mut Display,
        _: Drawable,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> Pixmap;
    fn XScreenOfDisplay(_: *mut Display, _: ::core::ffi::c_int) -> *mut Screen;
    fn XScreenNumberOfScreen(_: *mut Screen) -> ::core::ffi::c_int;
    fn XAllocColor(_: *mut Display, _: Colormap, _: *mut XColor) -> ::core::ffi::c_int;
    fn XParseColor(
        _: *mut Display,
        _: Colormap,
        _: *const ::core::ffi::c_char,
        _: *mut XColor,
    ) -> ::core::ffi::c_int;
    fn XQueryColor(_: *mut Display, _: Colormap, _: *mut XColor) -> ::core::ffi::c_int;
    fn XrmPutResource(
        _: *mut XrmDatabase,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *mut XrmValue,
    );
    fn XrmPutStringResource(
        _: *mut XrmDatabase,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    );
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XtGetMultiClickTime(_: *mut Display) -> ::core::ffi::c_int;
    fn XtDestroyWidget(_: Widget);
    fn XtDisplayOfObject(_: Widget) -> *mut Display;
    fn XtScreenOfObject(_: Widget) -> *mut Screen;
    fn XtCreateWidget(
        _: *const ::core::ffi::c_char,
        _: WidgetClass,
        _: Widget,
        _: ArgList,
        _: Cardinal,
    ) -> Widget;
    fn XtScreenDatabase(_: *mut Screen) -> XrmDatabase;
    fn XtGetApplicationResources(
        _: Widget,
        _: XtPointer,
        _: XtResourceList,
        _: Cardinal,
        _: ArgList,
        _: Cardinal,
    );
    fn XtGetSubresources(
        _: Widget,
        _: XtPointer,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: XtResourceList,
        _: Cardinal,
        _: ArgList,
        _: Cardinal,
    );
    fn XtSetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtRealloc(_: *mut ::core::ffi::c_char, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    fn XtGetGC(_: Widget, _: XtGCMask, _: *mut XGCValues) -> GC;
    static XtStrings: [::core::ffi::c_char; 0];
    static XtShellStrings: [::core::ffi::c_char; 0];
    static mut _XmStrings: [::core::ffi::c_char; 0];
    static mut _XmStrings23: [::core::ffi::c_char; 0];
    fn XmGetPixmap(
        screen: *mut Screen,
        image_name: *mut ::core::ffi::c_char,
        foreground: Pixel,
        background: Pixel,
    ) -> Pixmap;
    fn XmGetColors(
        screen: *mut Screen,
        color_map: Colormap,
        background: Pixel,
        foreground_ret: *mut Pixel,
        top_shadow_ret: *mut Pixel,
        bottom_shadow_ret: *mut Pixel,
        select_ret: *mut Pixel,
    );
    fn XmStringCreateLocalized(text: String_0) -> XmString;
    fn XmStringCopy(string: XmString) -> XmString;
    fn XmRenditionCreate(
        widget: Widget,
        tag: XmStringTag,
        arglist: ArgList,
        argcount: Cardinal,
    ) -> XmRendition;
    fn XmRenderTableAddRenditions(
        oldtable: XmRenderTable,
        renditions: *mut XmRendition,
        rendition_count: Cardinal,
        merge_mode: XmMergeMode,
    ) -> XmRenderTable;
    fn XmRenderTableGetDefaultFontExtents(
        rendertable: XmRenderTable,
        height: *mut ::core::ffi::c_int,
        ascent: *mut ::core::ffi::c_int,
        descent: *mut ::core::ffi::c_int,
    );
    fn XmeGetPixelData(
        screen_number: ::core::ffi::c_int,
        colorUse: *mut ::core::ffi::c_int,
        pixelSet: *mut XmPixelSet,
        a: *mut ::core::ffi::c_short,
        i: *mut ::core::ffi::c_short,
        p: *mut ::core::ffi::c_short,
        s: *mut ::core::ffi::c_short,
    ) -> Boolean;
    static mut wmGD: WmGlobalData;
    fn exit(__status: ::core::ffi::c_int) -> !;
    static mut xmRowColumnWidgetClass: WidgetClass;
    fn XmGetXmScreen(screen: *mut Screen) -> Widget;
    fn Warning(message: *mut ::core::ffi::c_char);
    fn MWarning(format: *mut ::core::ffi::c_char, message: *mut ::core::ffi::c_char);
    fn GetString(linePP: *mut *mut ::core::ffi::c_uchar) -> *mut ::core::ffi::c_uchar;
    fn ParseButtonStr(pSD: *mut WmScreenData, buttonStr: *mut ::core::ffi::c_uchar);
    fn ParseKeyStr(pSD: *mut WmScreenData, keyStr: *mut ::core::ffi::c_uchar);
    fn ProcessBackdropResources(pWS: *mut WmWorkspaceData, callFlags: ::core::ffi::c_ulong);
    fn WmXmStringToString(xmString: XmString) -> String_0;
    fn GenerateWorkspaceName(
        pSD: *mut WmScreenData,
        wsnum: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_uchar;
    fn ExitWM(exitCode: ::core::ffi::c_int) -> !;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const XmFONT_IS_XFT: C2RustUnnamed_1 = 2;
pub const XmFONT_IS_FONTSET: C2RustUnnamed_1 = 1;
pub const XmFONT_IS_FONT: C2RustUnnamed_1 = 0;
pub const XmDUPLICATE: XmMergeMode = 4;
pub const XmMERGE_NEW: XmMergeMode = 3;
pub const XmMERGE_OLD: XmMergeMode = 2;
pub const XmMERGE_REPLACE: XmMergeMode = 1;
pub const XmSKIP: XmMergeMode = 0;
pub const XmCO_HIGH_COLOR: C2RustUnnamed_2 = 3;
pub const XmCO_MEDIUM_COLOR: C2RustUnnamed_2 = 2;
pub const XmCO_LOW_COLOR: C2RustUnnamed_2 = 1;
pub const XmCO_BLACK_WHITE: C2RustUnnamed_2 = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const FillSolid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FillTiled: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GCForeground: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const GCBackground: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const GCLineWidth: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const GCFillStyle: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int;
pub const GCTile: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 10 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XmUNSPECIFIED_PIXMAP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XmBACKGROUND: ::core::ffi::c_uchar =
    ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
pub const XmFOREGROUND: ::core::ffi::c_uchar =
    ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
pub const XmTOP_SHADOW: ::core::ffi::c_uchar =
    ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
pub const XmBOTTOM_SHADOW: ::core::ffi::c_uchar =
    ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
pub const XmSELECT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const XmCO_MAX_NUM_COLORS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const XmCO_NUM_COLORS: ::core::ffi::c_int = XmCO_MAX_NUM_COLORS;
pub const WM_RESOURCE_CLASS: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Emwm\0") };
pub const CSERVE_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOW_RES_DISPLAY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VGA_RES_DISPLAY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_ERROR_EXIT_VALUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FRAME_INTERNAL_SHADOW_WIDTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_TOP_TITLE_PADDING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_BOTTOM_TITLE_PADDING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_TOP_TITLE_SHADOW: ::core::ffi::c_int = FRAME_INTERNAL_SHADOW_WIDTH;
pub const WM_BOTTOM_TITLE_SHADOW: ::core::ffi::c_int = FRAME_INTERNAL_SHADOW_WIDTH;
pub const WM_TITLE_BAR_PADDING: ::core::ffi::c_int =
    WM_TOP_TITLE_PADDING + WM_BOTTOM_TITLE_PADDING + WM_TOP_TITLE_SHADOW + WM_BOTTOM_TITLE_SHADOW;
pub const WS_ALLOC_AMOUNT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const WM_FUNC_DEFAULT: ::core::ffi::c_long = MWM_FUNC_ALL;
pub const WM_FUNC_ALL: ::core::ffi::c_long =
    MWM_FUNC_RESIZE | MWM_FUNC_MOVE | MWM_FUNC_MINIMIZE | MWM_FUNC_MAXIMIZE | MWM_FUNC_CLOSE;
pub const WM_DECOR_DEFAULT: ::core::ffi::c_long = MWM_DECOR_ALL;
pub const WM_DECOR_BORDER: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const WM_DECOR_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const WM_DECOR_SYSTEM: ::core::ffi::c_long = WM_DECOR_TITLE | MWM_DECOR_MENU;
pub const WM_DECOR_RESIZEH: ::core::ffi::c_long = WM_DECOR_BORDER | MWM_DECOR_RESIZEH;
pub const WM_SHOW_FB_BEHAVIOR: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const WM_SHOW_FB_MOVE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const WM_SHOW_FB_PLACEMENT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const WM_SHOW_FB_RESIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const WM_SHOW_FB_RESTART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const WM_SHOW_FB_QUIT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const WM_SHOW_FB_KILL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const WM_SHOW_FB_ALL: ::core::ffi::c_long = WM_SHOW_FB_BEHAVIOR
    | WM_SHOW_FB_MOVE
    | WM_SHOW_FB_PLACEMENT
    | WM_SHOW_FB_RESIZE
    | WM_SHOW_FB_RESTART
    | WM_SHOW_FB_QUIT
    | WM_SHOW_FB_KILL;
pub const WM_SHOW_FB_DEFAULT: ::core::ffi::c_long = WM_SHOW_FB_ALL;
pub const MAX_WORKSPACE_COUNT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const DEFAULT_BACKDROP_DIR: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"/usr/share/backdrops\0")
};
pub const MIN_INTERNAL_BEVEL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAX_INTERNAL_BEVEL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAXWMPATH: ::core::ffi::c_int = 1023 as ::core::ffi::c_int;
pub const MAXBUF: ::core::ffi::c_int = MAXWMPATH + 1 as ::core::ffi::c_int;
pub const AMAP_BEHAVIOR_ADD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CMAP_FOCUS_KEYBOARD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XRS_FOCUS_POINTER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ICON_LABEL_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const ICON_IMAGE_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const ICON_ACTIVE_LABEL_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const USE_ICON_DEFAULT_APPEARANCE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const ICON_APPEARANCE_STANDALONE: ::core::ffi::c_long =
    ICON_LABEL_PART | ICON_IMAGE_PART | ICON_ACTIVE_LABEL_PART;
pub const ICON_APPEARANCE_ICONBOX: ::core::ffi::c_long = ICON_LABEL_PART | ICON_IMAGE_PART;
pub const ICON_PLACE_LEFT_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const ICON_PLACE_BOTTOM_SECONDARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
pub const MAXIMUM_ICON_MARGIN: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const ICON_IMAGE_MAX_WIDTH: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const ICON_IMAGE_MAX_HEIGHT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const ICON_IMAGE_MIN_WIDTH: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ICON_IMAGE_MIN_HEIGHT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MAXIMUM_FRAME_BORDER_WIDTH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const USE_PPOSITION_NONZERO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BIGSIZE: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
pub const WmNautoKeyFocus: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"autoKeyFocus\0") };
pub const WmNautoRaiseDelay: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"autoRaiseDelay\0") };
pub const WmNbackdropDirectories: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"backdropDirectories\0")
};
pub const WmNbitmapDirectory: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"bitmapDirectory\0") };
pub const WmNbuttonBindings: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"buttonBindings\0") };
pub const WmNcleanText: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"cleanText\0") };
pub const WmNclientAutoPlace: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"clientAutoPlace\0") };
pub const WmNcolormapFocusPolicy: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"colormapFocusPolicy\0")
};
pub const WmNconfigFile: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"configFile\0") };
pub const WmNcppCommand: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"cppCommand\0") };
pub const WmNdeiconifyKeyFocus: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"deiconifyKeyFocus\0")
};
pub const WmNdoubleClickTime: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"doubleClickTime\0") };
pub const WmNenableWarp: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"enableWarp\0") };
pub const WmNenforceKeyFocus: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"enforceKeyFocus\0") };
pub const WmNfadeNormalIcon: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"fadeNormalIcon\0") };
pub const WmNfeedbackGeometry: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"feedbackGeometry\0")
};
pub const WmNframeBorderWidth: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"frameBorderWidth\0")
};
pub const WmNframeExternalShadowWidth: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"frameExternalShadowWidth\0")
};
pub const WmNiconAutoPlace: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"iconAutoPlace\0") };
pub const WmNiconBoxGeometry: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"iconBoxGeometry\0") };
pub const WmNiconBoxName: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"iconBoxName\0") };
pub const WmNiconBoxSBDisplayPolicy: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"iconBoxSBDisplayPolicy\0")
};
pub const WmNiconBoxScheme: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"iconBoxScheme\0") };
pub const WmNiconBoxTitle: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"iconBoxTitle\0") };
pub const WmNiconClick: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"iconClick\0") };
pub const WmNiconDecoration: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"iconDecoration\0") };
pub const WmNiconExternalShadowWidth: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"iconExternalShadowWidth\0")
};
pub const WmNiconImageMaximum: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"iconImageMaximum\0")
};
pub const WmNiconImageMinimum: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"iconImageMinimum\0")
};
pub const WmNiconPlacement: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"iconPlacement\0") };
pub const WmNiconPlacementMargin: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"iconPlacementMargin\0")
};
pub const WmNimage: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"image\0") };
pub const WmNimageBackground: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"imageBackground\0") };
pub const WmNimageForeground: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"imageForeground\0") };
pub const WmNinteractivePlacement: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"interactivePlacement\0")
};
pub const WmNkeyBindings: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"keyBindings\0") };
pub const WmNkeyboardFocusPolicy: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"keyboardFocusPolicy\0")
};
pub const WmNlimitResize: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"limitResize\0") };
pub const WmNlowerOnIconify: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"lowerOnIconify\0") };
pub const WmNmarqueeSelectGranularity: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"marqueeSelectGranularity\0")
};
pub const WmNmaximumMaximumSize: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"maximumMaximumSize\0")
};
pub const WmNmoveThreshold: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"moveThreshold\0") };
pub const WmNmultiScreen: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"multiScreen\0") };
pub const WmNpassButtons: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"passButtons\0") };
pub const WmNpassSelectButton: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"passSelectButton\0")
};
pub const WmNpositionOnScreen: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"positionOnScreen\0")
};
pub const WmNquitTimeout: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"quitTimeout\0") };
pub const WmNraiseKeyFocus: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"raiseKeyFocus\0") };
pub const WmNrefreshByClearing: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"refreshByClearing\0")
};
pub const WmNresizeBorderWidth: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"resizeBorderWidth\0")
};
pub const WmNresizeCursors: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"resizeCursors\0") };
pub const WmNrootButtonClick: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"rootButtonClick\0") };
pub const WmNsecondariesOnTop: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"secondariesOnTop\0")
};
pub const WmNsessionClientDB: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"sessionClientDB\0") };
pub const WmNshowFeedback: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"showFeedback\0") };
pub const WmNstartupKeyFocus: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"startupKeyFocus\0") };
pub const WmNsystemButtonClick: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"wMenuButtonClick\0")
};
pub const WmNsystemButtonClick2: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"wMenuButtonClick2\0")
};
pub const WmNtransientDecoration: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"transientDecoration\0")
};
pub const WmNtransientFunctions: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"transientFunctions\0")
};
pub const WmNuseIconBox: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"useIconBox\0") };
pub const WmNuseWindowOutline: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"useWindowOutline\0")
};
pub const WmNoutlineWidth: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"outlineWidth\0") };
pub const WmNmoveOpaque: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"moveOpaque\0") };
pub const WmNworkspaceList: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"workspaceList\0") };
pub const WmNworkspaceCount: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"workspaceCount\0") };
pub const WmNframeStyle: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"frameStyle\0") };
pub const WmNutilityDecoration: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"utilityDecoration\0")
};
pub const WmNutilityFunctions: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"utilityFunctions\0")
};
pub const WmNprimaryXineramaScreen: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"primaryXineramaScreen\0")
};
pub const WmNxineramaScreenFocus: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"xineramaScreenFocus\0")
};
pub const WmNxineramaIconifyToPrimary: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"xineramaIconifyToPrimary\0")
};
pub const WmNactiveBackground: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"activeBackground\0")
};
pub const WmNactiveBackgroundPixmap: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"activeBackgroundPixmap\0")
};
pub const WmNactiveBottomShadowColor: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"activeBottomShadowColor\0")
};
pub const WmNactiveBottomShadowPixmap: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"activeBottomShadowPixmap\0")
};
pub const WmNactiveForeground: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"activeForeground\0")
};
pub const WmNactiveTopShadowColor: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"activeTopShadowColor\0")
};
pub const WmNactiveTopShadowPixmap: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"activeTopShadowPixmap\0")
};
pub const WmNbackground: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"background\0") };
pub const WmNbackgroundPixmap: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"backgroundPixmap\0")
};
pub const WmNbottomShadowColor: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"bottomShadowColor\0")
};
pub const WmNbottomShadowPixmap: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"bottomShadowPixmap\0")
};
pub const WmNforeground: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"foreground\0") };
pub const WmNsaveUnder: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"saveUnder\0") };
pub const WmNtopShadowColor: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"topShadowColor\0") };
pub const WmNtopShadowPixmap: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"topShadowPixmap\0") };
pub const WmNtitleLeft: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"titleLeft\0") };
pub const WmNabsentMapBehavior: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"absentMapBehavior\0")
};
pub const WmNclientDecoration: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"clientDecoration\0")
};
pub const WmNclientFunctions: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"clientFunctions\0") };
pub const WmNfocusAutoRaise: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"focusAutoRaise\0") };
pub const WmNiconImage: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"iconImage\0") };
pub const WmNiconImageBackground: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"iconImageBackground\0")
};
pub const WmNiconImageBottomShadowColor: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*b"iconImageBottomShadowColor\0")
};
pub const WmNiconImageBottomShadowPixmap: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"iconImageBottomShadowPixmap\0")
};
pub const WmNiconImageForeground: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"iconImageForeground\0")
};
pub const WmNiconImageTopShadowColor: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"iconImageTopShadowColor\0")
};
pub const WmNiconImageTopShadowPixmap: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"iconImageTopShadowPixmap\0")
};
pub const WmNignoreWMSaveHints: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"ignoreWMSaveHints\0")
};
pub const WmNinitialWorkspace: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"initialWorkspace\0")
};
pub const WmNmatteBackground: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"matteBackground\0") };
pub const WmNmatteBottomShadowColor: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"matteBottomShadowColor\0")
};
pub const WmNmatteBottomShadowPixmap: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"matteBottomShadowPixmap\0")
};
pub const WmNmatteForeground: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"matteForeground\0") };
pub const WmNmatteTopShadowColor: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"matteTopShadowColor\0")
};
pub const WmNmatteTopShadowPixmap: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"matteTopShadowPixmap\0")
};
pub const WmNmatteWidth: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"matteWidth\0") };
pub const WmNmaximumClientSize: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"maximumClientSize\0")
};
pub const WmNscreens: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"screens\0") };
pub const WmNsystemMenu: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"windowMenu\0") };
pub const WmNuseClientIcon: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"useClientIcon\0") };
pub const WmNusePPosition: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"usePPosition\0") };
pub const WmNoverrideGeometry: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"overrideGeometry\0")
};
pub const WmNoccupyWorkspaces: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"occupyWorkspaces\0")
};
pub const WmNclient: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"client\0") };
pub const WmNfeedback: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"feedback\0") };
pub const WmNicon: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"icon\0") };
pub const WmNtitle: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"title\0") };
pub const WmNdefaults: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"defaults\0") };
pub const WmNbackdrop: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"backdrop\0") };
pub const WmNcolorSetId: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"colorSetId\0") };
pub const WmNworkspacePresence: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"workspacePresence\0")
};
pub const WmNcolor: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"color\0") };
pub const WmCAutoKeyFocus: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"AutoKeyFocus\0") };
pub const WmCAutoRaiseDelay: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"AutoRaiseDelay\0") };
pub const WmCBackdropDirectories: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"BackdropDirectories\0")
};
pub const WmCBitmapDirectory: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"BitmapDirectory\0") };
pub const WmCButtonBindings: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"ButtonBindings\0") };
pub const WmCCleanText: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"CleanText\0") };
pub const WmCClientAutoPlace: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"ClientAutoPlace\0") };
pub const WmCColormapFocusPolicy: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"ColormapFocusPolicy\0")
};
pub const WmCConfigFile: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"ConfigFile\0") };
pub const WmCCppCommand: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"CppCommand\0") };
pub const WmCDeiconifyKeyFocus: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"DeiconifyKeyFocus\0")
};
pub const WmCDoubleClickTime: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"DoubleClickTime\0") };
pub const WmCEnableWarp: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"EnableWarp\0") };
pub const WmCEnforceKeyFocus: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"EnforceKeyFocus\0") };
pub const WmCFadeNormalIcon: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"FadeNormalIcon\0") };
pub const WmCFeedbackGeometry: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"FeedbackGeometry\0")
};
pub const WmCFrameBorderWidth: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"FrameBorderWidth\0")
};
pub const WmCFrameExternalShadowWidth: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"FrameExternalShadowWidth\0")
};
pub const WmCIconAutoPlace: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"IconAutoPlace\0") };
pub const WmCIconBoxGeometry: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"IconBoxGeometry\0") };
pub const WmCIconBoxName: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"IconBoxName\0") };
pub const WmCIconBoxSBDisplayPolicy: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"IconBoxSBDisplayPolicy\0")
};
pub const WmCIconBoxScheme: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"IconBoxScheme\0") };
pub const WmCIconBoxTitle: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"IconBoxTitle\0") };
pub const WmCIconClick: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"IconClick\0") };
pub const WmCIconDecoration: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"IconDecoration\0") };
pub const WmCIconExternalShadowWidth: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"IconExternalShadowWidth\0")
};
pub const WmCIconImageMaximum: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"IconImageMaximum\0")
};
pub const WmCIconImageMinimum: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"IconImageMinimum\0")
};
pub const WmCIconPlacement: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"IconPlacement\0") };
pub const WmCIconPlacementMargin: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"IconPlacementMargin\0")
};
pub const WmCImage: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Image\0") };
pub const WmCImageBackground: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"ImageBackground\0") };
pub const WmCImageForeground: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"ImageForeground\0") };
pub const WmCInteractivePlacement: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"InteractivePlacement\0")
};
pub const WmCKeyBindings: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"KeyBindings\0") };
pub const WmCKeyboardFocusPolicy: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"KeyboardFocusPolicy\0")
};
pub const WmCLimitResize: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"LimitResize\0") };
pub const WmCLowerOnIconify: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"LowerOnIconify\0") };
pub const WmCMarqueeSelectGranularity: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"MarqueeSelectGranularity\0")
};
pub const WmCMaximumMaximumSize: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"MaximumMaximumSize\0")
};
pub const WmCMoveThreshold: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"MoveThreshold\0") };
pub const WmCMultiScreen: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"MultiScreen\0") };
pub const WmCPassButtons: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"PassButtons\0") };
pub const WmCPassSelectButton: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"PassSelectButton\0")
};
pub const WmCPositionOnScreen: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"PositionOnScreen\0")
};
pub const WmCQuitTimeout: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"QuitTimeout\0") };
pub const WmCRaiseKeyFocus: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"RaiseKeyFocus\0") };
pub const WmCRefreshByClearing: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"RefreshByClearing\0")
};
pub const WmCResizeCursors: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"ResizeCursors\0") };
pub const WmCRootButtonClick: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"RootButtonClick\0") };
pub const WmCSecondariesOnTop: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"SecondariesOnTop\0")
};
pub const WmCSessionClientDB: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"SessionClientDB\0") };
pub const WmCScreens: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"Screens\0") };
pub const WmCShowFeedback: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"ShowFeedback\0") };
pub const WmCStartupKeyFocus: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"StartupKeyFocus\0") };
pub const WmCSystemButtonClick: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"WMenuButtonClick\0")
};
pub const WmCSystemButtonClick2: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"WMenuButtonClick2\0")
};
pub const WmCTransientDecoration: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"TransientDecoration\0")
};
pub const WmCTransientFunctions: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"TransientFunctions\0")
};
pub const WmCUseIconBox: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"UseIconBox\0") };
pub const WmCUseWindowOutline: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"UseWindowOutline\0")
};
pub const WmCOutlineWidth: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"OutlineWidth\0") };
pub const WmCMoveOpaque: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"MoveOpaque\0") };
pub const WmCFrameStyle: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"FrameStyle\0") };
pub const WmCWorkspaceList: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"WorkspaceList\0") };
pub const WmCWorkspaceCount: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"WorkspaceCount\0") };
pub const WmCUtilityDecoration: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"UtilityDecoration\0")
};
pub const WmCUtilityFunctions: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"UtilityFunctions\0")
};
pub const WmCPrimaryXineramaScreen: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"PrimaryXineramaScreen\0")
};
pub const WmCXineramaScreenFocus: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"XineramaScreenFocus\0")
};
pub const WmCXineramaIconifyToPrimary: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"XineramaIconifyToPrimary\0")
};
pub const WmCSaveUnder: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SaveUnder\0") };
pub const WmCTitleLeft: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"TitleLeft\0") };
pub const WmCAbsentMapBehavior: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"AbsentMapBehavior\0")
};
pub const WmCClientDecoration: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"ClientDecoration\0")
};
pub const WmCClientFunctions: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"ClientFunctions\0") };
pub const WmCFocusAutoRaise: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"FocusAutoRaise\0") };
pub const WmCIconImage: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"IconImage\0") };
pub const WmCIconImageBackground: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"IconImageBackground\0")
};
pub const WmCIconImageBottomShadowColor: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*b"IconImageBottomShadowColor\0")
};
pub const WmCIconImageBottomShadowPixmap: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"IconImageBottomShadowPixmap\0")
};
pub const WmCIconImageForeground: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"IconImageForeground\0")
};
pub const WmCIconImageTopShadowColor: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"IconImageTopShadowColor\0")
};
pub const WmCIconImageTopShadowPixmap: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"IconImageTopShadowPixmap\0")
};
pub const WmCIgnoreWMSaveHints: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"IgnoreWMSaveHints\0")
};
pub const WmCInitialWorkspace: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"InitialWorkspace\0")
};
pub const WmCMatteBackground: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"MatteBackground\0") };
pub const WmCMatteBottomShadowColor: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"MatteBottomShadowColor\0")
};
pub const WmCMatteBottomShadowPixmap: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"MatteBottomShadowPixmap\0")
};
pub const WmCMatteForeground: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"MatteForeground\0") };
pub const WmCMatteTopShadowColor: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"MatteTopShadowColor\0")
};
pub const WmCMatteTopShadowPixmap: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"MatteTopShadowPixmap\0")
};
pub const WmCMatteWidth: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"MatteWidth\0") };
pub const WmCMaximumClientSize: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"MaximumClientSize\0")
};
pub const WmCSystemMenu: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"WindowMenu\0") };
pub const WmCUseClientIcon: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"UseClientIcon\0") };
pub const WmCUsePPosition: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"UsePPosition\0") };
pub const WmCOverrideGeometry: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"OverrideGeometry\0")
};
pub const WmCOccupyWorkspaces: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"OccupyWorkspaces\0")
};
pub const WmCClient: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"Client\0") };
pub const WmCFeedback: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"Feedback\0") };
pub const WmCIcon: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Icon\0") };
pub const WmCTitle: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Title\0") };
pub const WmCBackdrop: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"Backdrop\0") };
pub const WmCColorSetId: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"ColorSetId\0") };
pub const WmCWorkspacePresence: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"WorkspacePresence\0")
};
pub const WmCColor: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Color\0") };
pub const WmRAbsentMapBehavior: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"WmAMBehavior\0") };
pub const WmRCFocusPolicy: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"WmCFocus\0") };
pub const WmRClientDecor: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"WmCDecor\0") };
pub const WmRClientFunction: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"WmCFunc\0") };
pub const WmRFrameStyle: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"WmFrameStyle\0") };
pub const WmRIconDecor: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"WmIDecor\0") };
pub const WmRIconPlacement: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"WmIPlace\0") };
pub const WmRKFocusPolicy: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"WmKFocus\0") };
pub const WmRSize: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"WmSize\0") };
pub const WmRShowFeedback: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"WmShowFeedback\0") };
pub const WmRUsePPosition: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"WmUsePPosition\0") };
pub const WmRXineramaScreenFocus: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"WmXineramaScreenFocus\0")
};
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut defaultSystemMenuName: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"DefaultWindowMenu\0")
};
#[no_mangle]
pub static mut builtinSystemMenuName: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"_MwmWindowMenu_\0") };
pub const BUILTINSYSTEMMENU: [::core::ffi::c_char; 267] = unsafe {
    ::core::mem::transmute::<
        [u8; 267],
        [::core::ffi::c_char; 267],
    >(
        *b"_MwmWindowMenu_\n{\n\tRestore\t\t_R\tAlt<Key>F5\tf.restore\n\tMove\t\t_M\tAlt<Key>F7\tf.move\n\tSize\t\t_S\tAlt<Key>F8\tf.resize\n\tMinimize\t_n\tAlt<Key>F9\tf.minimize\n\tMaximize\t_x\tAlt<Key>F10\tf.maximize\n\tLower\t\t_L\tAlt<Key>F3\tf.lower\n\tno-label\t\t\t\tf.separator\n\tClose\t\t_C\tAlt<Key>F4\tf.kill\n}\0",
    )
};
#[no_mangle]
pub static mut builtinSystemMenu: [::core::ffi::c_char; 267] = BUILTINSYSTEMMENU;
pub const HARD_CODED_PRIMARY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[no_mangle]
pub static mut defaultRootMenuName: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"DefaultRootMenu\0") };
#[no_mangle]
pub static mut builtinRootMenuName: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"_MwmRootMenu_\0") };
#[no_mangle]
pub static mut builtinRootMenu: [::core::ffi::c_char; 250] = unsafe {
    ::core::mem::transmute::<
        [u8; 250],
        [::core::ffi::c_char; 250],
    >(
        *b"_MwmRootMenu_\n{\n\t\"Root Menu\"\t\tf.title\n\t\"New Window\"\t\tf.exec \"xterm &\"\n\t\"Shuffle Up\"\t\tf.circle_up\n\t\"Shuffle Down\"\tf.circle_down\n\t\"Refresh\"\t\tf.refresh\n\t\"Pack Icons\"\t\tf.pack_icons\n\t no-label\t\tf.separator\n\t\"Restart...\"\t\tf.restart\n\t\"Quit...\"\t\tf.quit_wm\n}\0",
    )
};
#[no_mangle]
pub static mut defaultKeyBindingsName: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"DefaultKeyBindings\0")
};
#[no_mangle]
pub static mut builtinKeyBindingsName: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"_MwmKeyBindings_\0")
};
#[no_mangle]
pub static mut builtinKeyBindings: [::core::ffi::c_char; 820] = unsafe {
    ::core::mem::transmute::<
        [u8; 820],
        [::core::ffi::c_char; 820],
    >(
        *b"_MwmKeyBindings_\n{\n\tSuper<Key>Left window f.tile left\n\tSuper<Key>Right window f.tile right\n\tSuper<Key>Up window f.tile up\n\tSuper<Key>Down window f.tile down\n\tSuper<Key>KP_Left window f.tile left\n\tSuper<Key>KP_Right window f.tile right\n\tSuper<Key>KP_Up window f.tile up\n\tSuper<Key>KP_Down window f.tile down\n\tShift<Key>Escape\twindow|icon\t\tf.post_wmenu\n\tAlt<Key>space\t\twindow|icon\t\tf.post_wmenu\n\tAlt<Key>Tab\t\troot|icon|window\tf.next_key\n\tAlt Shift<Key>Tab\troot|icon|window\tf.prev_key\n\tAlt<Key>Escape\t\troot|icon|window\tf.circle_down\n\tAlt Shift<Key>Escape\troot|icon|window\tf.circle_up\n\tAlt Shift Ctrl<Key>exclam root|icon|window\tf.set_behavior\n\tAlt Ctrl<Key>1\t\t  root|icon|window\tf.set_behavior\n\tAlt<Key>F6\t\twindow\t\t\tf.next_key transient\n\tAlt Shift<Key>F6\twindow\t\t\tf.prev_key transient\n\tShift<Key>F10\t\ticon\t\t\tf.post_wmenu\n}\0",
    )
};
#[no_mangle]
pub static mut defaultButtonBindingsName: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"DefaultButtonBindings\0")
};
#[no_mangle]
pub static mut builtinButtonBindingsName: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"_MwmButtonBindings_\0")
};
#[no_mangle]
pub static mut builtinButtonBindings: [::core::ffi::c_char; 119] = unsafe {
    ::core::mem::transmute::<
        [u8; 119],
        [::core::ffi::c_char; 119],
    >(
        *b"_MwmButtonBindings_\n{\n\t<Btn1Down>\ticon|frame\tf.raise\n\t<Btn3Down>\ticon|frame\tf.post_wmenu\n\t<Btn3Down>\troot\t\tf.menu %s\n}\0",
    )
};
static mut _pCD: *mut ClientData = ::core::ptr::null::<ClientData>() as *mut ClientData;
static mut _defaultBackground: String_0 =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut _defaultActiveBackground: String_0 =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut _pAppearanceData: *mut AppearanceData =
    ::core::ptr::null::<AppearanceData>() as *mut AppearanceData;
static mut pResWS: *mut WmWorkspaceData =
    ::core::ptr::null::<WmWorkspaceData>() as *mut WmWorkspaceData;
static mut pResSD: *mut WmScreenData = ::core::ptr::null::<WmScreenData>() as *mut WmScreenData;
static mut _defaultColor1HEX: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"#A8A8A8A8A8A8\0") };
static mut _defaultColor2HEX: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"#5F5F92929E9E\0") };
static mut _defaultColor1: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"LightGrey\0") };
static mut _defaultColor2: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"CadetBlue\0") };
pub const DEFAULT_COLOR_NONE: *mut ::core::ffi::c_void = NULL;
#[no_mangle]
pub static mut _foreground: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"foreground\0") };
#[no_mangle]
pub static mut _75_foreground: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"75_foreground\0") };
#[no_mangle]
pub static mut _50_foreground: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"50_foreground\0") };
#[no_mangle]
pub static mut _25_foreground: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"25_foreground\0") };
#[no_mangle]
pub static mut _Dither: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
pub static mut _NoDither: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
pub static mut CLIENT_FRAME_PART: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"client\0") };
#[no_mangle]
pub static mut ICON_FRAME_PART: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"icon\0") };
#[no_mangle]
pub static mut FEEDBACK_FRAME_PART: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"feedback\0") };
#[no_mangle]
pub static mut MENU_ITEM_PART: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"menu\0") };
pub const WmBGC: ::core::ffi::c_uchar = XmBACKGROUND;
pub const WmFGC: ::core::ffi::c_uchar = XmFOREGROUND;
pub const WmTSC: ::core::ffi::c_uchar = XmTOP_SHADOW;
pub const WmBSC: ::core::ffi::c_uchar = XmBOTTOM_SHADOW;
pub const BITMAPDIR: [::core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(*b"/usr/include/X11/bitmaps/\0")
};
#[no_mangle]
pub static mut wmGlobalResources: [XtResource; 38] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 38];
#[no_mangle]
pub static mut wmGlobalScreenResources: [XtResource; 3] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 3];
#[no_mangle]
pub static mut wmStdGlobalResources: [XtResource; 9] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 9];
#[no_mangle]
pub static mut wmScreenResources: [XtResource; 29] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 29];
#[no_mangle]
pub static mut wmStdScreenResources: [XtResource; 6] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 6];
#[no_mangle]
pub static mut wmWorkspaceResources: [XtResource; 2] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 2];
#[no_mangle]
pub static mut wmStdWorkspaceResources: [XtResource; 1] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 1];
#[no_mangle]
pub static mut wmBackdropResources: [XtResource; 5] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 5];
#[no_mangle]
pub static mut wmWsPresenceResources: [XtResource; 1] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 1];
#[no_mangle]
pub static mut wmClientResources: [XtResource; 20] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 20];
#[no_mangle]
pub static mut wmStdClientResources: [XtResource; 11] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 11];
#[no_mangle]
pub static mut wmClientResourcesM: [XtResource; 6] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 6];
#[no_mangle]
pub static mut wmAppearanceResources: [XtResource; 16] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_class: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 16];
#[no_mangle]
pub unsafe extern "C" fn _WmIconImageFDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmFGC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pCD).iconImageBackground,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmIconImageBDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    (*value).addr =
        &raw mut (*(*_pCD).pSD).iconAppearance.background as *mut ::core::ffi::c_char as XPointer;
    (*value).size = ::core::mem::size_of::<Pixel>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmIconImageBSCDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmBSC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pCD).iconImageBackground,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmIconImageBSPDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    (*value).addr = (*(*_pCD).pSD).iconAppearance.bottomShadowPStr as XPointer;
    (*value).size = ::core::mem::size_of::<String_0>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmIconImageTSCDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmTSC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pCD).iconImageBackground,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmIconImageTSPDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    (*value).addr = (*(*_pCD).pSD).iconAppearance.topShadowPStr as XPointer;
    (*value).size = ::core::mem::size_of::<String_0>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmMatteFDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmFGC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pCD).matteBackground,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmMatteBDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    (*value).addr =
        &raw mut (*(*_pCD).pSD).clientAppearance.background as *mut ::core::ffi::c_char as XPointer;
    (*value).size = ::core::mem::size_of::<Pixel>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmMatteBSCDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmBSC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pCD).matteBackground,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmMatteBSPDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    (*value).addr = (*(*_pCD).pSD).clientAppearance.bottomShadowPStr as XPointer;
    (*value).size = ::core::mem::size_of::<String_0>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmMatteTSCDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmTSC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pCD).matteBackground,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmMatteTSPDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    (*value).addr = (*(*_pCD).pSD).clientAppearance.topShadowPStr as XPointer;
    (*value).size = ::core::mem::size_of::<String_0>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmBackgroundDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(widget, WmBGC, _defaultBackground, 0 as Pixel, value);
}
#[no_mangle]
pub unsafe extern "C" fn _WmForegroundDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmFGC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pAppearanceData).background,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmBackgroundPixmapDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    static mut string: String_0 =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    if Monochrome(XtScreenOfObject(widget)) as ::core::ffi::c_int != 0
        || (*_pAppearanceData).topShadowColor == (*_pAppearanceData).background
    {
        string = &raw const _25_foreground as *const ::core::ffi::c_char as String_0;
    } else {
        string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*value).addr = string as XPointer;
    (*value).size = ::core::mem::size_of::<String_0>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmBottomShadowColorDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmBSC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pAppearanceData).background,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmTopShadowColorDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmTSC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pAppearanceData).background,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmTopShadowPixmapDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    static mut string: String_0 =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    if Monochrome(XtScreenOfObject(widget)) as ::core::ffi::c_int != 0
        || (*_pAppearanceData).topShadowColor == (*_pAppearanceData).background
    {
        string = &raw const _50_foreground as *const ::core::ffi::c_char as String_0;
        if !(*_pAppearanceData).backgroundPStr.is_null() {
            if strcmp(
                (*_pAppearanceData).backgroundPStr as *const ::core::ffi::c_char,
                &raw const _25_foreground as *const ::core::ffi::c_char,
            ) == 0
                || strcmp(
                    (*_pAppearanceData).backgroundPStr as *const ::core::ffi::c_char,
                    &raw const _50_foreground as *const ::core::ffi::c_char,
                ) == 0
            {
                string = &raw const _foreground as *const ::core::ffi::c_char as String_0;
            }
        }
    } else {
        string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*value).addr = string as XPointer;
    (*value).size = ::core::mem::size_of::<String_0>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmABackgroundDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(widget, WmBGC, _defaultActiveBackground, 0 as Pixel, value);
}
#[no_mangle]
pub unsafe extern "C" fn _WmAForegroundDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmFGC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pAppearanceData).activeBackground,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmABackgroundPixmapDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    static mut string: String_0 =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    if Monochrome(XtScreenOfObject(widget)) as ::core::ffi::c_int != 0
        || (*_pAppearanceData).activeTopShadowColor == (*_pAppearanceData).activeBackground
    {
        string = &raw const _50_foreground as *const ::core::ffi::c_char as String_0;
    } else {
        string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*value).addr = string as XPointer;
    (*value).size = ::core::mem::size_of::<String_0>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmABottomShadowColorDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmBSC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pAppearanceData).activeBackground,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmATopShadowColorDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    _WmGetDynamicDefault(
        widget,
        WmTSC,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*_pAppearanceData).activeBackground,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _WmATopShadowPixmapDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    static mut string: String_0 =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    if Monochrome(XtScreenOfObject(widget)) as ::core::ffi::c_int != 0
        || (*_pAppearanceData).activeTopShadowColor == (*_pAppearanceData).activeBackground
    {
        string = &raw const _50_foreground as *const ::core::ffi::c_char as String_0;
        if !(*_pAppearanceData).activeBackgroundPStr.is_null() {
            if strcmp(
                (*_pAppearanceData).activeBackgroundPStr as *const ::core::ffi::c_char,
                &raw const _25_foreground as *const ::core::ffi::c_char,
            ) == 0
                || strcmp(
                    (*_pAppearanceData).activeBackgroundPStr as *const ::core::ffi::c_char,
                    &raw const _50_foreground as *const ::core::ffi::c_char,
                ) == 0
            {
                string = &raw const _foreground as *const ::core::ffi::c_char as String_0;
            }
        }
    } else {
        string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*value).addr = string as XPointer;
    (*value).size = ::core::mem::size_of::<String_0>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmBackdropBgDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    static mut pixValue: Pixel = 0;
    let mut colorSetId = (*pResWS).backdrop.colorSet as ::core::ffi::c_uint;
    if wmGD.statusColorServer == CSERVE_NORMAL {
        if colorSetId == 0 as ::core::ffi::c_uint
            || colorSetId > XmCO_MAX_NUM_COLORS as ::core::ffi::c_uint
        {
            colorSetId = DefaultWsColorSetId(pResWS) as ::core::ffi::c_uint;
        }
        match (*(*pResWS).pSD).colorUse {
            0 => {
                pixValue = (*(*(*pResWS).pSD)
                    .pPixelData
                    .offset(colorSetId.wrapping_sub(1 as ::core::ffi::c_uint) as isize))
                .bg;
            }
            1 | 2 | 3 | _ => {
                pixValue = (*(*(*pResWS).pSD)
                    .pPixelData
                    .offset(colorSetId.wrapping_sub(1 as ::core::ffi::c_uint) as isize))
                .bs;
            }
        }
    } else {
        pixValue = (*pResWS).backdrop.defBackground;
    }
    (*value).addr = &raw mut pixValue as *mut ::core::ffi::c_char as XPointer;
    (*value).size = ::core::mem::size_of::<Pixel>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmBackdropFgDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    static mut pixValue: Pixel = 0;
    let mut colorSetId = (*pResWS).backdrop.colorSet as ::core::ffi::c_uint;
    if wmGD.statusColorServer == CSERVE_NORMAL {
        if colorSetId == 0 as ::core::ffi::c_uint
            || colorSetId > XmCO_MAX_NUM_COLORS as ::core::ffi::c_uint
        {
            colorSetId = DefaultWsColorSetId(pResWS) as ::core::ffi::c_uint;
        }
        match (*(*pResWS).pSD).colorUse {
            0 => {
                pixValue = (*(*(*pResWS).pSD)
                    .pPixelData
                    .offset(colorSetId.wrapping_sub(1 as ::core::ffi::c_uint) as isize))
                .fg;
            }
            1 | 2 | 3 | _ => {
                pixValue = (*(*(*pResWS).pSD)
                    .pPixelData
                    .offset(colorSetId.wrapping_sub(1 as ::core::ffi::c_uint) as isize))
                .bg;
            }
        }
    } else {
        pixValue = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*pResWS).pSD).screen as isize))
        .black_pixel as Pixel;
    }
    (*value).addr = &raw mut pixValue as *mut ::core::ffi::c_char as XPointer;
    (*value).size = ::core::mem::size_of::<Pixel>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmBackdropColorSetDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    static mut colorSetId: ::core::ffi::c_uint = 0;
    if wmGD.statusColorServer == CSERVE_NORMAL {
        colorSetId = DefaultWsColorSetId(pResWS) as ::core::ffi::c_uint;
    } else {
        colorSetId = 0 as ::core::ffi::c_uint;
    }
    (*value).addr = &raw mut colorSetId as *mut ::core::ffi::c_char as XPointer;
    (*value).size = ::core::mem::size_of::<Pixel>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmIconImageMaximumDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    static mut cval: WHSize = _WHSize {
        width: 0,
        height: 0,
    };
    if (*pResSD).displayResolutionType == LOW_RES_DISPLAY
        || (*pResSD).displayResolutionType == VGA_RES_DISPLAY
    {
        cval.width = 32 as ::core::ffi::c_int;
        cval.height = 32 as ::core::ffi::c_int;
    } else {
        cval.width = 48 as ::core::ffi::c_int;
        cval.height = 48 as ::core::ffi::c_int;
    }
    (*value).addr = &raw mut cval as *mut ::core::ffi::c_char as XPointer;
    (*value).size = ::core::mem::size_of::<WHSize>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn DefaultWsColorSetId(mut pWS: *mut WmWorkspaceData) -> ::core::ffi::c_int {
    static mut _ws_high_color_map: [::core::ffi::c_int; 4] = [
        3 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        6 as ::core::ffi::c_int,
        7 as ::core::ffi::c_int,
    ];
    let mut i: ::core::ffi::c_int = 0;
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    let mut iIndex: ::core::ffi::c_int = 0;
    let mut rval = 8 as ::core::ffi::c_int;
    if !pWS.is_null() {
        pSD = (*pWS).pSD;
        iIndex = ((pWS as ::core::ffi::c_ulong).wrapping_sub((*pSD).pWS as ::core::ffi::c_ulong)
            as ::core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<WmWorkspaceData>() as usize)
            as ::core::ffi::c_int;
        if iIndex < 0 as ::core::ffi::c_int {
            iIndex = 0 as ::core::ffi::c_int;
        }
        match (*pSD).colorUse {
            0 | 1 => {
                rval = (1 as usize).wrapping_add(
                    ((*pSD).pInactivePixelSet.offset_from((*pSD).pPixelData) as ::core::ffi::c_long
                        as usize)
                        .wrapping_div(::core::mem::size_of::<XmPixelSet>() as usize),
                ) as ::core::ffi::c_int;
            }
            2 => {
                rval = HARD_CODED_PRIMARY;
            }
            3 => {
                i = iIndex % _WS_HIGH_COLOR_COUNT;
                rval = _ws_high_color_map[i as usize];
            }
            _ => {}
        }
    }
    return rval;
}
pub const _WS_HIGH_COLOR_COUNT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn _WmFocusAutoRaiseDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    static mut focusAutoRaise: Boolean = 0;
    if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
        focusAutoRaise = True as Boolean;
    } else {
        focusAutoRaise = False as Boolean;
    }
    (*value).addr = &raw mut focusAutoRaise as *mut ::core::ffi::c_char as XPointer;
    (*value).size = ::core::mem::size_of::<Boolean>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmMultiClickTimeDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    static mut multiClickTime: ::core::ffi::c_int = 0;
    multiClickTime = XtGetMultiClickTime(XtDisplayOfObject(widget));
    (*value).addr = &raw mut multiClickTime as *mut ::core::ffi::c_char as XPointer;
    (*value).size = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _WmSecondariesOnTopDefault(
    mut widget: Widget,
    mut offset: ::core::ffi::c_int,
    mut value: *mut XrmValue,
) {
    static mut secondariesOnTop: Boolean = 0;
    if !(*_pCD).transientLeader.is_null() {
        secondariesOnTop = (*(*_pCD).transientLeader).secondariesOnTop;
    } else {
        secondariesOnTop = True as Boolean;
    }
    (*value).addr = &raw mut secondariesOnTop as *mut ::core::ffi::c_char as XPointer;
    (*value).size = ::core::mem::size_of::<Boolean>() as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWmResources() {
    if wmGD.useStandardBehavior != 0 {
        XtGetApplicationResources(
            wmGD.topLevelW,
            &raw mut wmGD as XtPointer,
            &raw mut wmStdGlobalResources as XtResourceList,
            (::core::mem::size_of::<[XtResource; 9]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtResource>() as usize)
                as Cardinal,
            ::core::ptr::null_mut::<Arg>(),
            0 as Cardinal,
        );
        SetStdGlobalResourceValues();
    } else {
        XtGetApplicationResources(
            wmGD.topLevelW,
            &raw mut wmGD as XtPointer,
            &raw mut wmGlobalResources as XtResourceList,
            (::core::mem::size_of::<[XtResource; 38]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtResource>() as usize)
                as Cardinal,
            ::core::ptr::null_mut::<Arg>(),
            0 as Cardinal,
        );
    }
    if wmGD.autoRaiseDelay < 0 as ::core::ffi::c_int {
        wmGD.autoRaiseDelay = 500 as ::core::ffi::c_int;
        Warning(
            b"Out of range autoRaiseDelay resource value. Must be non-negative\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessGlobalScreenResources() {
    XtGetApplicationResources(
        wmGD.topLevelW,
        &raw mut wmGD as XtPointer,
        &raw mut wmGlobalScreenResources as XtResourceList,
        (::core::mem::size_of::<[XtResource; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<XtResource>() as usize) as Cardinal,
        ::core::ptr::null_mut::<Arg>(),
        0 as Cardinal,
    );
    if wmGD.multiScreen != 0 {
        wmGD.numScreens = (*(wmGD.display as _XPrivDisplay)).nscreens;
    } else {
        wmGD.numScreens = 1 as ::core::ffi::c_int;
    }
    if !wmGD.screenList.is_null() {
        ProcessScreenListResource();
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetStdGlobalResourceValues() {
    wmGD.autoKeyFocus = True as Boolean;
    wmGD.clientAutoPlace = True as Boolean;
    wmGD.colormapFocusPolicy = CMAP_FOCUS_KEYBOARD;
    wmGD.deiconifyKeyFocus = True as Boolean;
    wmGD.doubleClickTime = 500 as ::core::ffi::c_int;
    wmGD.iconAutoPlace = True as Boolean;
    wmGD.iconClick = True as Boolean;
    wmGD.interactivePlacement = False as Boolean;
    wmGD.keyboardFocusPolicy = KEYBOARD_FOCUS_EXPLICIT;
    wmGD.lowerOnIconify = True as Boolean;
    wmGD.passSelectButton = True as Boolean;
    wmGD.startupKeyFocus = True as Boolean;
    wmGD.systemButtonClick = True as Boolean;
    wmGD.systemButtonClick2 = True as Boolean;
    wmGD.useWindowOutline = True as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ProcessScreenListResource() {
    let mut lineP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut sNum = 0 as ::core::ffi::c_int;
    let mut nameCount = 0 as ::core::ffi::c_int;
    lineP = wmGD.screenList as *mut ::core::ffi::c_uchar;
    loop {
        string = GetString(&raw mut lineP);
        if !(!string.is_null() && sNum < (*(wmGD.display as _XPrivDisplay)).nscreens) {
            break;
        }
        let ref mut fresh0 = *wmGD.screenNames.offset(sNum as isize);
        *fresh0 = WmRealloc(
            *wmGD.screenNames.offset(sNum as isize) as *mut ::core::ffi::c_char,
            strlen(string as *mut ::core::ffi::c_char).wrapping_add(1 as size_t)
                as ::core::ffi::c_uint,
        ) as *mut ::core::ffi::c_uchar;
        if (*fresh0).is_null() {
            ExitWM(WM_ERROR_EXIT_VALUE);
        } else {
            strcpy(
                *wmGD.screenNames.offset(sNum as isize) as *mut ::core::ffi::c_char,
                string as *mut ::core::ffi::c_char,
            );
            nameCount += 1;
            sNum += 1;
        }
    }
    if nameCount > 0 as ::core::ffi::c_int {
        string = *wmGD.screenNames.offset(0 as ::core::ffi::c_int as isize);
        while sNum < (*(wmGD.display as _XPrivDisplay)).nscreens {
            let ref mut fresh1 = *wmGD.screenNames.offset(sNum as isize);
            *fresh1 = WmRealloc(
                *wmGD.screenNames.offset(sNum as isize) as *mut ::core::ffi::c_char,
                strlen(string as *mut ::core::ffi::c_char).wrapping_add(1 as size_t)
                    as ::core::ffi::c_uint,
            ) as *mut ::core::ffi::c_uchar;
            if (*fresh1).is_null() {
                ExitWM(WM_ERROR_EXIT_VALUE);
            } else {
                strcpy(
                    *wmGD.screenNames.offset(sNum as isize) as *mut ::core::ffi::c_char,
                    string as *mut ::core::ffi::c_char,
                );
                sNum += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWmColors(mut pSD: *mut WmScreenData) {
    let mut active: ::core::ffi::c_short = 0;
    let mut inactive: ::core::ffi::c_short = 0;
    let mut primary: ::core::ffi::c_short = 0;
    let mut secondary: ::core::ffi::c_short = 0;
    (*pSD).pPixelData = XtMalloc(
        (XmCO_NUM_COLORS as usize).wrapping_mul(::core::mem::size_of::<XmPixelSet>() as usize)
            as Cardinal,
    ) as *mut XmPixelSet;
    if !(*pSD).pPixelData.is_null() {
        if XmeGetPixelData(
            (*pSD).screen,
            &raw mut (*pSD).colorUse,
            (*pSD).pPixelData,
            &raw mut active,
            &raw mut inactive,
            &raw mut primary,
            &raw mut secondary,
        ) != 0
        {
            (*pSD).pActivePixelSet = (*pSD).pPixelData.offset(active as isize) as *mut XmPixelSet;
            (*pSD).pInactivePixelSet =
                (*pSD).pPixelData.offset(inactive as isize) as *mut XmPixelSet;
            (*pSD).pPrimaryPixelSet = (*pSD).pPixelData.offset(primary as isize) as *mut XmPixelSet;
            (*pSD).pSecondaryPixelSet =
                (*pSD).pPixelData.offset(secondary as isize) as *mut XmPixelSet;
            (*pSD).pTextPixelSet =
                (*pSD).pPixelData.offset(3 as ::core::ffi::c_int as isize) as *mut XmPixelSet;
            wmGD.statusColorServer = CSERVE_NORMAL;
        } else {
            XtFree((*pSD).pPixelData as *mut ::core::ffi::c_char);
            (*pSD).pPixelData = ::core::ptr::null_mut::<XmPixelSet>();
            (*pSD).pActivePixelSet = ::core::ptr::null_mut::<XmPixelSet>();
            (*pSD).pInactivePixelSet = ::core::ptr::null_mut::<XmPixelSet>();
            (*pSD).pPrimaryPixelSet = ::core::ptr::null_mut::<XmPixelSet>();
            (*pSD).pSecondaryPixelSet = ::core::ptr::null_mut::<XmPixelSet>();
            (*pSD).pTextPixelSet = ::core::ptr::null_mut::<XmPixelSet>();
        }
    } else {
        Warning(
            b"Insufficient memory for color data\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        ExitWM(WM_ERROR_EXIT_VALUE);
    };
}
unsafe extern "C" fn WriteOutXrmColors(mut pSD: *mut WmScreenData) {
    let mut db = ::core::ptr::null_mut::<_XrmHashBucketRec>();
    let mut value = XrmValue {
        size: 0,
        addr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut thisScreen = (*pSD).screen;
    let mut tpixset = ::core::ptr::null_mut::<XmPixelSet>();
    let mut spixset = ::core::ptr::null_mut::<XmPixelSet>();
    let mut res_class = WM_RESOURCE_CLASS.as_ptr() as *mut ::core::ffi::c_char;
    let mut screen_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    screen_name = *wmGD.screenNames.offset((*pSD).screen as isize) as String_0;
    db = XtScreenDatabase(XScreenOfDisplay(wmGD.display, thisScreen));
    value.size = ::core::mem::size_of::<Pixel>() as ::core::ffi::c_uint;
    if !(*pSD).pActivePixelSet.is_null() {
        tpixset = (*pSD).pActivePixelSet;
        spixset = (*pSD).pSecondaryPixelSet;
        if (*pSD).colorUse == XmCO_BLACK_WHITE as ::core::ffi::c_int {
            value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .black_pixel as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNactiveForeground.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .white_pixel as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNactiveBackground.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNactiveBackgroundPixmap.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                &raw const _foreground as *const ::core::ffi::c_char,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNactiveTopShadowPixmap.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                _Dither,
            );
            value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .black_pixel as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNactiveTopShadowColor.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .black_pixel as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNactiveBottomShadowColor.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNactiveBottomShadowPixmap.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                &raw const _foreground as *const ::core::ffi::c_char,
            );
        } else {
            value.addr = &raw mut (*tpixset).bg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNactiveBackground.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*tpixset).fg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNactiveForeground.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*tpixset).ts as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNactiveTopShadowColor.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*tpixset).bs as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNactiveBottomShadowColor.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            if (*tpixset).bs
                == (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset(thisScreen as isize))
                .black_pixel
            {
                XrmPutStringResource(
                    &raw mut db,
                    ResCat(
                        res_class as String_0,
                        screen_name,
                        WmNactiveTopShadowPixmap.as_ptr() as String_0,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ) as *const ::core::ffi::c_char,
                    _Dither,
                );
            }
            if (*tpixset).ts
                == (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset(thisScreen as isize))
                .white_pixel
            {
                XrmPutStringResource(
                    &raw mut db,
                    ResCat(
                        res_class as String_0,
                        screen_name,
                        WmNactiveBottomShadowPixmap.as_ptr() as String_0,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ) as *const ::core::ffi::c_char,
                    _Dither,
                );
            }
        }
    }
    if !(*pSD).pInactivePixelSet.is_null() {
        tpixset = (*pSD).pInactivePixelSet;
        spixset = (*pSD).pSecondaryPixelSet;
        if (*pSD).colorUse == XmCO_BLACK_WHITE as ::core::ffi::c_int {
            value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .black_pixel as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNforeground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const ICON_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNforeground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbackgroundPixmap.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                _Dither,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const ICON_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbackgroundPixmap.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                _Dither,
            );
            value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .white_pixel as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbackground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const ICON_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbackground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowPixmap.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                &raw const _foreground as *const ::core::ffi::c_char,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const ICON_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowPixmap.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                &raw const _foreground as *const ::core::ffi::c_char,
            );
            value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .white_pixel as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const ICON_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowPixmap.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                &raw const _foreground as *const ::core::ffi::c_char,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const ICON_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowPixmap.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                &raw const _foreground as *const ::core::ffi::c_char,
            );
            value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .black_pixel as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const ICON_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .black_pixel as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNiconImageForeground.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .white_pixel as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    WmNiconImageBackground.as_ptr() as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*tpixset).fg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    WmNforeground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*tpixset).bg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    WmNbackground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowPixmap.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                &raw const _50_foreground as *const ::core::ffi::c_char,
            );
            value.addr = &raw mut (*tpixset).fg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowPixmap.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                &raw const _75_foreground as *const ::core::ffi::c_char,
            );
            value.addr = &raw mut (*tpixset).fg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*spixset).fg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNforeground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*spixset).bg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbackground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowPixmap.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                &raw const _50_foreground as *const ::core::ffi::c_char,
            );
            value.addr = &raw mut (*spixset).fg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutStringResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowPixmap.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                &raw const _75_foreground as *const ::core::ffi::c_char,
            );
            value.addr = &raw mut (*spixset).fg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*tpixset).sc as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(10461 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*spixset).sc as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(4902 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(9463 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(10461 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
        } else {
            let mut fpixset = ::core::ptr::null_mut::<XmPixelSet>();
            value.addr = &raw mut (*tpixset).bg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbackground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const ICON_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbackground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    WmNbackground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*spixset).bg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbackground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*tpixset).ts as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const ICON_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*spixset).ts as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNtopShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*tpixset).bs as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const ICON_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*spixset).bs as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNbottomShadowColor.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*tpixset).fg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNforeground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const ICON_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNforeground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    WmNforeground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*spixset).fg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    WmNforeground.as_ptr() as String_0,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*tpixset).sc as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(9463 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(4902 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const MENU_ITEM_PART as *const ::core::ffi::c_char as String_0,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(10461 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const CLIENT_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(10461 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(9463 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*spixset).sc as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(4902 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    &raw const FEEDBACK_FRAME_PART as *const ::core::ffi::c_char as String_0,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(10461 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            fpixset = (*pSD).pSecondaryPixelSet;
            value.addr = &raw mut (*fpixset).bg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    (&raw const XtStrings as *const ::core::ffi::c_char)
                        .offset(52 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*fpixset).fg as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    (&raw const XtStrings as *const ::core::ffi::c_char)
                        .offset(214 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char as String_0,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*fpixset).ts as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(10308 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*fpixset).bs as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(5096 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            value.addr = &raw mut (*fpixset).sc as XtPointer as XPointer;
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(9463 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(4902 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            XrmPutResource(
                &raw mut db,
                ResCat(
                    res_class as String_0,
                    screen_name,
                    (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                        .offset(10461 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ) as *const ::core::ffi::c_char,
                (&raw const XtStrings as *const ::core::ffi::c_char)
                    .offset(1754 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char as String_0
                    as *const ::core::ffi::c_char,
                &raw mut value,
            );
            if (*fpixset).bs
                == (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset(thisScreen as isize))
                .black_pixel
            {
                XrmPutStringResource(
                    &raw mut db,
                    ResCat(
                        res_class as String_0,
                        screen_name,
                        WmNtopShadowPixmap.as_ptr() as String_0,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ) as *const ::core::ffi::c_char,
                    _Dither,
                );
                if (*pSD).colorUse == XmCO_BLACK_WHITE as ::core::ffi::c_int {
                    XrmPutStringResource(
                        &raw mut db,
                        ResCat(
                            res_class as String_0,
                            screen_name,
                            WmNbottomShadowPixmap.as_ptr() as String_0,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        ) as *const ::core::ffi::c_char,
                        _NoDither,
                    );
                }
            }
            if (*fpixset).ts
                == (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset(thisScreen as isize))
                .white_pixel
            {
                XrmPutStringResource(
                    &raw mut db,
                    ResCat(
                        res_class as String_0,
                        screen_name,
                        WmNbottomShadowPixmap.as_ptr() as String_0,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ) as *const ::core::ffi::c_char,
                    _Dither,
                );
                if (*pSD).colorUse == XmCO_BLACK_WHITE as ::core::ffi::c_int {
                    XrmPutStringResource(
                        &raw mut db,
                        ResCat(
                            res_class as String_0,
                            screen_name,
                            WmNtopShadowPixmap.as_ptr() as String_0,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        ) as *const ::core::ffi::c_char,
                        _NoDither,
                    );
                }
            }
            if (*tpixset).bs != (*tpixset).ts {
                value.addr = &raw mut (*tpixset).bs as XtPointer as XPointer;
                XrmPutResource(
                    &raw mut db,
                    ResCat(
                        res_class as String_0,
                        screen_name,
                        WmNiconImageForeground.as_ptr() as String_0,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ) as *const ::core::ffi::c_char,
                    (&raw const XtStrings as *const ::core::ffi::c_char)
                        .offset(1754 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char as String_0
                        as *const ::core::ffi::c_char,
                    &raw mut value,
                );
                value.addr = &raw mut (*tpixset).ts as XtPointer as XPointer;
                XrmPutResource(
                    &raw mut db,
                    ResCat(
                        res_class as String_0,
                        screen_name,
                        WmNiconImageBackground.as_ptr() as String_0,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ) as *const ::core::ffi::c_char,
                    (&raw const XtStrings as *const ::core::ffi::c_char)
                        .offset(1754 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char as String_0
                        as *const ::core::ffi::c_char,
                    &raw mut value,
                );
                value.addr = &raw mut (*tpixset).bs as XtPointer as XPointer;
                XrmPutResource(
                    &raw mut db,
                    ResCat(
                        res_class as String_0,
                        screen_name,
                        WmNiconImageBottomShadowColor.as_ptr() as String_0,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ) as *const ::core::ffi::c_char,
                    (&raw const XtStrings as *const ::core::ffi::c_char)
                        .offset(1754 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char as String_0
                        as *const ::core::ffi::c_char,
                    &raw mut value,
                );
                value.addr = &raw mut (*tpixset).ts as XtPointer as XPointer;
                XrmPutResource(
                    &raw mut db,
                    ResCat(
                        res_class as String_0,
                        screen_name,
                        WmNiconImageTopShadowColor.as_ptr() as String_0,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ) as *const ::core::ffi::c_char,
                    (&raw const XtStrings as *const ::core::ffi::c_char)
                        .offset(1754 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char as String_0
                        as *const ::core::ffi::c_char,
                    &raw mut value,
                );
            } else {
                value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*pSD).screen as isize))
                .black_pixel as XtPointer as XPointer;
                XrmPutResource(
                    &raw mut db,
                    ResCat(
                        res_class as String_0,
                        screen_name,
                        WmNiconImageForeground.as_ptr() as String_0,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ) as *const ::core::ffi::c_char,
                    (&raw const XtStrings as *const ::core::ffi::c_char)
                        .offset(1754 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char as String_0
                        as *const ::core::ffi::c_char,
                    &raw mut value,
                );
                value.addr = &raw mut (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*pSD).screen as isize))
                .white_pixel as XtPointer as XPointer;
                XrmPutResource(
                    &raw mut db,
                    ResCat(
                        res_class as String_0,
                        screen_name,
                        WmNiconImageBackground.as_ptr() as String_0,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ) as *const ::core::ffi::c_char,
                    (&raw const XtStrings as *const ::core::ffi::c_char)
                        .offset(1754 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char as String_0
                        as *const ::core::ffi::c_char,
                    &raw mut value,
                );
            }
        }
    }
    if !(*pSD).pTextPixelSet.is_null() {
        value.addr = &raw mut (*(*pSD).pTextPixelSet).bg as XtPointer as XPointer;
        XrmPutResource(
            &raw mut db,
            ResCat(
                res_class as String_0,
                screen_name,
                b"XmTextField\0" as *const u8 as *const ::core::ffi::c_char as String_0,
                WmNbackground.as_ptr() as String_0,
            ) as *const ::core::ffi::c_char,
            (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0 as *const ::core::ffi::c_char,
            &raw mut value,
        );
        XrmPutResource(
            &raw mut db,
            ResCat(
                res_class as String_0,
                screen_name,
                b"XmText\0" as *const u8 as *const ::core::ffi::c_char as String_0,
                WmNbackground.as_ptr() as String_0,
            ) as *const ::core::ffi::c_char,
            (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0 as *const ::core::ffi::c_char,
            &raw mut value,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ResCat(
    mut s1: String_0,
    mut s2: String_0,
    mut s3: String_0,
    mut s4: String_0,
) -> String_0 {
    let mut count: ::core::ffi::c_int = 0;
    wmGD.tmpBuffer[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_uchar;
    count = MAXBUF - 1 as ::core::ffi::c_int;
    if !s1.is_null() && strcmp(s1 as *const ::core::ffi::c_char, WM_RESOURCE_CLASS.as_ptr()) != 0 {
        strncat(
            &raw mut wmGD.tmpBuffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            s1 as *const ::core::ffi::c_char,
            count as size_t,
        );
        count = (count as size_t).wrapping_sub(strlen(s1 as *const ::core::ffi::c_char))
            as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    if !s2.is_null() && count > 0 as ::core::ffi::c_int {
        strncat(
            &raw mut wmGD.tmpBuffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            b"*\0" as *const u8 as *const ::core::ffi::c_char,
            count as size_t,
        );
        count -= 1 as ::core::ffi::c_int;
        strncat(
            &raw mut wmGD.tmpBuffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            s2 as *const ::core::ffi::c_char,
            count as size_t,
        );
        count = (count as size_t).wrapping_sub(strlen(s2 as *const ::core::ffi::c_char))
            as ::core::ffi::c_int as ::core::ffi::c_int;
        if !s3.is_null() && count > 0 as ::core::ffi::c_int {
            strncat(
                &raw mut wmGD.tmpBuffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
                b"*\0" as *const u8 as *const ::core::ffi::c_char,
                count as size_t,
            );
            count -= 1 as ::core::ffi::c_int;
            strncat(
                &raw mut wmGD.tmpBuffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
                s3 as *const ::core::ffi::c_char,
                count as size_t,
            );
            count = (count as size_t).wrapping_sub(strlen(s3 as *const ::core::ffi::c_char))
                as ::core::ffi::c_int as ::core::ffi::c_int;
            if !s4.is_null() {
                strncat(
                    &raw mut wmGD.tmpBuffer as *mut ::core::ffi::c_uchar
                        as *mut ::core::ffi::c_char,
                    b"*\0" as *const u8 as *const ::core::ffi::c_char,
                    count as size_t,
                );
                count -= 1 as ::core::ffi::c_int;
                strncat(
                    &raw mut wmGD.tmpBuffer as *mut ::core::ffi::c_uchar
                        as *mut ::core::ffi::c_char,
                    s4 as *const ::core::ffi::c_char,
                    count as size_t,
                );
            }
        }
    }
    return &raw mut wmGD.tmpBuffer as *mut ::core::ffi::c_uchar as String_0;
}
#[no_mangle]
pub unsafe extern "C" fn CheckForNoDither(mut pAD: *mut AppearanceData) {
    if !(*pAD).backgroundPStr.is_null()
        && strcmp(
            (*pAD).backgroundPStr as *const ::core::ffi::c_char,
            _NoDither,
        ) == 0
    {
        (*pAD).backgroundPStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*pAD).bottomShadowPStr.is_null()
        && strcmp(
            (*pAD).bottomShadowPStr as *const ::core::ffi::c_char,
            _NoDither,
        ) == 0
    {
        (*pAD).bottomShadowPStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*pAD).topShadowPStr.is_null()
        && strcmp(
            (*pAD).topShadowPStr as *const ::core::ffi::c_char,
            _NoDither,
        ) == 0
    {
        (*pAD).topShadowPStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*pAD).activeBackgroundPStr.is_null()
        && strcmp(
            (*pAD).activeBackgroundPStr as *const ::core::ffi::c_char,
            _NoDither,
        ) == 0
    {
        (*pAD).activeBackgroundPStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*pAD).activeBottomShadowPStr.is_null()
        && strcmp(
            (*pAD).activeBottomShadowPStr as *const ::core::ffi::c_char,
            _NoDither,
        ) == 0
    {
        (*pAD).activeBottomShadowPStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*pAD).activeTopShadowPStr.is_null()
        && strcmp(
            (*pAD).activeTopShadowPStr as *const ::core::ffi::c_char,
            _NoDither,
        ) == 0
    {
        (*pAD).activeTopShadowPStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessAppearanceResources(mut pSD: *mut WmScreenData) {
    let mut clientW = ::core::ptr::null_mut::<_WidgetRec>();
    let mut i: ::core::ffi::c_int = 0;
    let mut args: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 10];
    _defaultBackground = &raw mut _defaultColor1 as *mut ::core::ffi::c_char as String_0;
    _defaultActiveBackground = &raw mut _defaultColor2 as *mut ::core::ffi::c_char as String_0;
    _pAppearanceData = &raw mut (*pSD).clientAppearance;
    XtGetSubresources(
        (*pSD).screenTopLevelW,
        &raw mut (*pSD).clientAppearance as XtPointer,
        WmNclient.as_ptr(),
        WmCClient.as_ptr(),
        &raw mut wmAppearanceResources as XtResourceList,
        (::core::mem::size_of::<[XtResource; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<XtResource>() as usize) as Cardinal,
        ::core::ptr::null_mut::<Arg>(),
        0 as Cardinal,
    );
    CheckForNoDither(&raw mut (*pSD).clientAppearance);
    MakeAppearanceResources(pSD, &raw mut (*pSD).clientAppearance, True as Boolean);
    i = 0 as ::core::ffi::c_int;
    clientW = XtCreateWidget(
        WmNclient.as_ptr(),
        xmRowColumnWidgetClass,
        (*pSD).screenTopLevelW,
        &raw mut args as *mut Arg as ArgList,
        i as Cardinal,
    );
    _pAppearanceData = &raw mut (*pSD).clientTitleAppearance;
    XtGetSubresources(
        clientW,
        &raw mut (*pSD).clientTitleAppearance as XtPointer,
        WmNtitle.as_ptr(),
        WmCTitle.as_ptr(),
        &raw mut wmAppearanceResources as XtResourceList,
        (::core::mem::size_of::<[XtResource; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<XtResource>() as usize) as Cardinal,
        ::core::ptr::null_mut::<Arg>(),
        0 as Cardinal,
    );
    CheckForNoDither(&raw mut (*pSD).clientTitleAppearance);
    if SimilarAppearanceData(
        &raw mut (*pSD).clientAppearance,
        &raw mut (*pSD).clientTitleAppearance,
    ) != 0
    {
        (*pSD).decoupleTitleAppearance = False as Boolean;
    } else {
        MakeAppearanceResources(pSD, &raw mut (*pSD).clientTitleAppearance, True as Boolean);
        (*pSD).decoupleTitleAppearance = True as Boolean;
    }
    XtDestroyWidget(clientW);
    _pAppearanceData = &raw mut (*pSD).iconAppearance;
    XtGetSubresources(
        (*pSD).screenTopLevelW,
        &raw mut (*pSD).iconAppearance as XtPointer,
        WmNicon.as_ptr(),
        WmCIcon.as_ptr(),
        &raw mut wmAppearanceResources as XtResourceList,
        (::core::mem::size_of::<[XtResource; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<XtResource>() as usize) as Cardinal,
        ::core::ptr::null_mut::<Arg>(),
        0 as Cardinal,
    );
    CheckForNoDither(&raw mut (*pSD).iconAppearance);
    MakeAppearanceResources(pSD, &raw mut (*pSD).iconAppearance, True as Boolean);
    _defaultBackground = &raw mut _defaultColor2 as *mut ::core::ffi::c_char as String_0;
    _defaultActiveBackground = &raw mut _defaultColor2 as *mut ::core::ffi::c_char as String_0;
    _pAppearanceData = &raw mut (*pSD).feedbackAppearance;
    XtGetSubresources(
        (*pSD).screenTopLevelW,
        &raw mut (*pSD).feedbackAppearance as XtPointer,
        WmNfeedback.as_ptr(),
        WmCFeedback.as_ptr(),
        &raw mut wmAppearanceResources as XtResourceList,
        (::core::mem::size_of::<[XtResource; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<XtResource>() as usize) as Cardinal,
        ::core::ptr::null_mut::<Arg>(),
        0 as Cardinal,
    );
    CheckForNoDither(&raw mut (*pSD).feedbackAppearance);
    MakeAppearanceResources(pSD, &raw mut (*pSD).feedbackAppearance, False as Boolean);
}
#[no_mangle]
pub unsafe extern "C" fn MakeAppearanceResources(
    mut pSD: *mut WmScreenData,
    mut pAData: *mut AppearanceData,
    mut makeActiveResources: Boolean,
) {
    let mut foreground: Pixel = 0;
    let mut fontHeight = 0 as ::core::ffi::c_int;
    if !(*pAData).renderTable.is_null() {
        XmRenderTableGetDefaultFontExtents(
            (*pAData).renderTable,
            &raw mut fontHeight,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
    }
    if fontHeight == 0 {
        let mut rendition = ::core::ptr::null_mut::<*mut __XmRenditionRec>();
        let mut args: [Arg; 5] = [Arg {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            value: 0,
        }; 5];
        let mut n = 0 as Cardinal;
        Warning(
            b"Title-bar render table contains no valid font information; using fallback font.\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        args[n as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(199 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = 255 as ::core::ffi::c_int as XtArgVal;
        n = n.wrapping_add(1);
        args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(15594 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = XmFONT_IS_XFT as ::core::ffi::c_int as XtArgVal;
        n = n.wrapping_add(1);
        args[n as usize].name = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(15576 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = b"Sans\0" as *const u8 as *const ::core::ffi::c_char as XtArgVal;
        n = n.wrapping_add(1);
        args[n as usize].name = (&raw mut _XmStrings23 as *mut ::core::ffi::c_char)
            .offset(137 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as String_0;
        args[n as usize].value = 12 as ::core::ffi::c_int as XtArgVal;
        n = n.wrapping_add(1);
        rendition = XmRenditionCreate(
            wmGD.topLevelW,
            (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(12578 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            &raw mut args as ArgList,
            n,
        );
        (*pAData).renderTable = XmRenderTableAddRenditions(
            ::core::ptr::null_mut::<*mut __XmRenderTableRec>(),
            &raw mut rendition,
            1 as Cardinal,
            XmMERGE_NEW,
        );
        if !(*pAData).renderTable.is_null() {
            XmRenderTableGetDefaultFontExtents(
                (*pAData).renderTable,
                &raw mut fontHeight,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        }
        if fontHeight == 0 {
            Warning(
                b"Failed to create font.\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            exit(WM_ERROR_EXIT_VALUE);
        }
    }
    (*pAData).fontHeight = fontHeight as ::core::ffi::c_uint;
    (*pAData).titleHeight = (fontHeight + WM_TITLE_BAR_PADDING) as ::core::ffi::c_uint;
    if !(*pAData).backgroundPStr.is_null() {
        (*pAData).backgroundPixmap = XmGetPixmap(
            (*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize) as *mut Screen,
            (*pAData).backgroundPStr as *mut ::core::ffi::c_char,
            (*pAData).foreground,
            (*pAData).background,
        );
        if (*pAData).backgroundPixmap == XmUNSPECIFIED_PIXMAP as Pixmap {
            (*pAData).backgroundPixmap = NULL as Pixmap;
        }
    } else {
        (*pAData).backgroundPixmap = NULL as Pixmap;
    }
    if !(*pAData).topShadowPStr.is_null() {
        if (*pAData).topShadowColor != (*pAData).background {
            foreground = (*pAData).topShadowColor;
        } else {
            foreground = (*pAData).foreground;
        }
        (*pAData).topShadowPixmap = XmGetPixmap(
            (*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize) as *mut Screen,
            (*pAData).topShadowPStr as *mut ::core::ffi::c_char,
            foreground,
            (*pAData).background,
        );
        if (*pAData).topShadowPixmap == XmUNSPECIFIED_PIXMAP as Pixmap {
            (*pAData).topShadowPixmap = NULL as Pixmap;
        }
    } else {
        (*pAData).topShadowPixmap = NULL as Pixmap;
    }
    if !(*pAData).bottomShadowPStr.is_null() {
        if (*pAData).bottomShadowColor != (*pAData).background {
            foreground = (*pAData).bottomShadowColor;
        } else {
            foreground = (*pAData).foreground;
        }
        (*pAData).bottomShadowPixmap = XmGetPixmap(
            (*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize) as *mut Screen,
            (*pAData).bottomShadowPStr as *mut ::core::ffi::c_char,
            foreground,
            (*pAData).background,
        );
        if (*pAData).bottomShadowPixmap == XmUNSPECIFIED_PIXMAP as Pixmap {
            (*pAData).bottomShadowPixmap = NULL as Pixmap;
        }
    } else {
        (*pAData).bottomShadowPixmap = NULL as Pixmap;
    }
    GetAppearanceGCs(
        pSD,
        (*pAData).foreground,
        (*pAData).background,
        (*pAData).backgroundPixmap,
        (*pAData).topShadowColor,
        (*pAData).topShadowPixmap,
        (*pAData).bottomShadowColor,
        (*pAData).bottomShadowPixmap,
        &raw mut (*pAData).inactiveGC,
        &raw mut (*pAData).inactiveTopShadowGC,
        &raw mut (*pAData).inactiveBottomShadowGC,
    );
    if makeActiveResources == 0 {
        return;
    }
    if !(*pAData).activeBackgroundPStr.is_null() {
        (*pAData).activeBackgroundPixmap = XmGetPixmap(
            (*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize) as *mut Screen,
            (*pAData).activeBackgroundPStr as *mut ::core::ffi::c_char,
            (*pAData).activeForeground,
            (*pAData).activeBackground,
        );
        if (*pAData).activeBackgroundPixmap == XmUNSPECIFIED_PIXMAP as Pixmap {
            (*pAData).activeBackgroundPixmap = NULL as Pixmap;
        }
    } else {
        (*pAData).activeBackgroundPixmap = NULL as Pixmap;
    }
    if !(*pAData).activeTopShadowPStr.is_null() {
        (*pAData).activeTopShadowPixmap = XmGetPixmap(
            (*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize) as *mut Screen,
            (*pAData).activeTopShadowPStr as *mut ::core::ffi::c_char,
            (*pAData).activeTopShadowColor,
            (*pAData).activeBackground,
        );
        if (*pAData).activeTopShadowPixmap == XmUNSPECIFIED_PIXMAP as Pixmap {
            (*pAData).activeTopShadowPixmap = NULL as Pixmap;
        }
    } else {
        (*pAData).activeTopShadowPixmap = NULL as Pixmap;
    }
    if !(*pAData).activeBottomShadowPStr.is_null() {
        (*pAData).activeBottomShadowPixmap = XmGetPixmap(
            (*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize) as *mut Screen,
            (*pAData).activeBottomShadowPStr as *mut ::core::ffi::c_char,
            (*pAData).activeBottomShadowColor,
            (*pAData).activeBackground,
        );
        if (*pAData).activeBottomShadowPixmap == XmUNSPECIFIED_PIXMAP as Pixmap {
            (*pAData).activeBottomShadowPixmap = NULL as Pixmap;
        }
    } else {
        (*pAData).activeBottomShadowPixmap = NULL as Pixmap;
    }
    GetAppearanceGCs(
        pSD,
        (*pAData).activeForeground,
        (*pAData).activeBackground,
        (*pAData).activeBackgroundPixmap,
        (*pAData).activeTopShadowColor,
        (*pAData).activeTopShadowPixmap,
        (*pAData).activeBottomShadowColor,
        (*pAData).activeBottomShadowPixmap,
        &raw mut (*pAData).activeGC,
        &raw mut (*pAData).activeTopShadowGC,
        &raw mut (*pAData).activeBottomShadowGC,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GetAppearanceGCs(
    mut pSD: *mut WmScreenData,
    mut fg: Pixel,
    mut bg: Pixel,
    mut bg_pixmap: Pixmap,
    mut ts_color: Pixel,
    mut ts_pixmap: Pixmap,
    mut bs_color: Pixel,
    mut bs_pixmap: Pixmap,
    mut pGC: *mut GC,
    mut ptsGC: *mut GC,
    mut pbsGC: *mut GC,
) {
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
    let mut mask: ::core::ffi::c_ulong = 0;
    mask = (GCForeground | GCBackground) as ::core::ffi::c_ulong;
    gcv.foreground = fg as ::core::ffi::c_ulong;
    gcv.background = bg as ::core::ffi::c_ulong;
    if bg_pixmap != 0 {
        mask |= GCTile as ::core::ffi::c_ulong;
        gcv.tile = bg_pixmap;
    }
    *pGC = XCreateGC(
        wmGD.display,
        (*pSD).rootWindow as Drawable,
        mask,
        &raw mut gcv,
    );
    *ptsGC = GetHighlightGC(pSD, ts_color, bg, ts_pixmap);
    *pbsGC = GetHighlightGC(pSD, bs_color, bg, bs_pixmap);
}
#[no_mangle]
pub unsafe extern "C" fn ProcessScreenResources(
    mut pSD: *mut WmScreenData,
    mut screenName: *mut ::core::ffi::c_uchar,
) {
    pResSD = pSD;
    if wmGD.useStandardBehavior != 0 {
        XtGetSubresources(
            wmGD.topLevelW,
            pSD as XtPointer,
            screenName as String_0 as *const ::core::ffi::c_char,
            screenName as String_0 as *const ::core::ffi::c_char,
            &raw mut wmStdScreenResources as XtResourceList,
            (::core::mem::size_of::<[XtResource; 6]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtResource>() as usize)
                as Cardinal,
            ::core::ptr::null_mut::<Arg>(),
            0 as Cardinal,
        );
        SetStdScreenResourceValues(pSD);
    } else {
        XtGetSubresources(
            wmGD.topLevelW,
            pSD as XtPointer,
            screenName as String_0 as *const ::core::ffi::c_char,
            screenName as String_0 as *const ::core::ffi::c_char,
            &raw mut wmScreenResources as XtResourceList,
            (::core::mem::size_of::<[XtResource; 29]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtResource>() as usize)
                as Cardinal,
            ::core::ptr::null_mut::<Arg>(),
            0 as Cardinal,
        );
        (*pSD).moveOpaque = (*(XmGetXmScreen(XtScreenOfObject((*pSD).screenTopLevelW)) as XmScreen))
            .screen
            .moveOpaque as Boolean;
    }
    if (*pSD).iconImageMinimum.width < ICON_IMAGE_MIN_WIDTH {
        (*pSD).iconImageMinimum.width = ICON_IMAGE_MIN_WIDTH;
    } else if (*pSD).iconImageMinimum.width > ICON_IMAGE_MAX_WIDTH {
        (*pSD).iconImageMinimum.width = ICON_IMAGE_MAX_WIDTH;
    }
    if (*pSD).iconImageMinimum.height < ICON_IMAGE_MIN_HEIGHT {
        (*pSD).iconImageMinimum.height = ICON_IMAGE_MIN_HEIGHT;
    } else if (*pSD).iconImageMinimum.height > ICON_IMAGE_MAX_HEIGHT {
        (*pSD).iconImageMinimum.height = ICON_IMAGE_MAX_HEIGHT;
    }
    if (*pSD).iconImageMaximum.width < (*pSD).iconImageMinimum.width {
        (*pSD).iconImageMaximum.width = (*pSD).iconImageMinimum.width;
    } else if (*pSD).iconImageMaximum.width > ICON_IMAGE_MAX_WIDTH {
        (*pSD).iconImageMaximum.width = ICON_IMAGE_MAX_WIDTH;
    }
    if (*pSD).iconImageMaximum.height < (*pSD).iconImageMinimum.height {
        (*pSD).iconImageMaximum.height = (*pSD).iconImageMinimum.height;
    } else if (*pSD).iconImageMaximum.height > ICON_IMAGE_MAX_HEIGHT {
        (*pSD).iconImageMaximum.height = ICON_IMAGE_MAX_HEIGHT;
    }
    if (*pSD).iconPlacementMargin > MAXIMUM_ICON_MARGIN {
        (*pSD).iconPlacementMargin = MAXIMUM_ICON_MARGIN;
    }
    if (*pSD).maximumMaximumSize.width <= 0 as ::core::ffi::c_int {
        (*pSD).maximumMaximumSize.width = 2 as ::core::ffi::c_int
            * (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .width;
    }
    if (*pSD).maximumMaximumSize.height <= 0 as ::core::ffi::c_int {
        (*pSD).maximumMaximumSize.height = 2 as ::core::ffi::c_int
            * (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .height;
    }
    if (*pSD).iconDecoration as ::core::ffi::c_long & USE_ICON_DEFAULT_APPEARANCE != 0 {
        if (*pSD).useIconBox != 0 {
            (*pSD).iconDecoration = ICON_APPEARANCE_ICONBOX as ::core::ffi::c_int;
        } else {
            (*pSD).iconDecoration = ICON_APPEARANCE_STANDALONE as ::core::ffi::c_int;
        }
    }
    if (*pSD).resizeBorderWidth == BIGSIZE as Dimension as ::core::ffi::c_int
        || (*pSD).frameBorderWidth == BIGSIZE as Dimension as ::core::ffi::c_int
    {
        let mut xres: ::core::ffi::c_double = 0.;
        let mut yres: ::core::ffi::c_double = 0.;
        let mut avg_res: ::core::ffi::c_double = 0.;
        xres = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*pSD).screen as isize))
        .width as ::core::ffi::c_double
            / (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .mwidth as ::core::ffi::c_double;
        yres = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*pSD).screen as isize))
        .height as ::core::ffi::c_double
            / (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .mheight as ::core::ffi::c_double;
        avg_res = (xres + yres) / 2.0f64;
        if (*pSD).resizeBorderWidth == BIGSIZE as Dimension as ::core::ffi::c_int {
            (*pSD).resizeBorderWidth = (avg_res * 2.5f64) as ::core::ffi::c_int;
            if (*pSD).resizeBorderWidth > 7 as ::core::ffi::c_int {
                (*pSD).resizeBorderWidth = 7 as ::core::ffi::c_int;
            }
        }
        if (*pSD).frameBorderWidth == BIGSIZE as Dimension as ::core::ffi::c_int {
            (*pSD).frameBorderWidth = (avg_res * 1.7f64) as ::core::ffi::c_int;
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*pSD).frameBorderWidth > 5 as ::core::ffi::c_int {
                    (*pSD).frameBorderWidth = 5 as ::core::ffi::c_int;
                }
            } else if (*pSD).frameBorderWidth > 6 as ::core::ffi::c_int {
                (*pSD).frameBorderWidth = 6 as ::core::ffi::c_int;
            }
        }
    }
    (*pSD).externalBevel = wmGD.frameExternalShadowWidth as ::core::ffi::c_int;
    (*pSD).joinBevel = FRAME_INTERNAL_SHADOW_WIDTH;
    if (*pSD).frameBorderWidth < (*pSD).externalBevel + MIN_INTERNAL_BEVEL {
        (*pSD).frameBorderWidth = (*pSD).externalBevel + MIN_INTERNAL_BEVEL;
    } else if (*pSD).frameBorderWidth > MAXIMUM_FRAME_BORDER_WIDTH {
        (*pSD).frameBorderWidth = MAXIMUM_FRAME_BORDER_WIDTH;
    }
    if (*pSD).resizeBorderWidth < (*pSD).externalBevel + MIN_INTERNAL_BEVEL {
        (*pSD).resizeBorderWidth = (*pSD).externalBevel + MIN_INTERNAL_BEVEL;
    } else if (*pSD).resizeBorderWidth > MAXIMUM_FRAME_BORDER_WIDTH {
        (*pSD).resizeBorderWidth = MAXIMUM_FRAME_BORDER_WIDTH;
    }
    WriteOutXrmColors(pSD);
    ProcessAppearanceResources(pSD);
    ProcessWorkspaceList(pSD);
    ProcessDefaultBackdropImages(pSD);
    (*pSD).builtinIconPixmap = XCreateBitmapFromData(
        wmGD.display,
        (*pSD).rootWindow as Drawable,
        &raw mut iImage_bits as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        iImage_width as ::core::ffi::c_uint,
        iImage_height as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ProcessDefaultBackdropImages(mut pSD: *mut WmScreenData) {}
#[no_mangle]
pub unsafe extern "C" fn ProcessWorkspaceList(mut pSD: *mut WmScreenData) {
    let mut i: ::core::ffi::c_int = 0;
    let mut wsNameCount = 0 as ::core::ffi::c_int;
    let mut wsNamesAlloced = 0 as ::core::ffi::c_int;
    let mut pwsI = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut lineP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut bHaveWorkspaceList: Boolean = 0;
    let mut bHaveWorkspaceCount: Boolean = 0;
    let mut ppchWsNames = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    bHaveWorkspaceCount =
        ((*pSD).numWorkspaces >= 1 as ::core::ffi::c_int) as ::core::ffi::c_int as Boolean;
    bHaveWorkspaceList =
        ((*pSD).workspaceList != NULL as String_0) as ::core::ffi::c_int as Boolean;
    if bHaveWorkspaceList != 0 {
        wsNamesAlloced = WS_ALLOC_AMOUNT;
        ppchWsNames = XtMalloc(
            (wsNamesAlloced as usize)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                as Cardinal,
        ) as *mut *mut ::core::ffi::c_char;
        if !(*pSD).workspaceList.is_null() {
            lineP = (*pSD).workspaceList as *mut ::core::ffi::c_uchar;
        } else {
            lineP = NULL as *mut ::core::ffi::c_uchar;
        }
        wsNameCount = 0 as ::core::ffi::c_int;
        loop {
            string = GetString(&raw mut lineP);
            if string.is_null() {
                break;
            }
            let ref mut fresh3 = *ppchWsNames.offset(wsNameCount as isize);
            *fresh3 = string as *mut ::core::ffi::c_char;
            wsNameCount += 1;
            if wsNameCount >= wsNamesAlloced {
                wsNamesAlloced += WS_ALLOC_AMOUNT;
                ppchWsNames = XtRealloc(
                    ppchWsNames as *mut ::core::ffi::c_char,
                    (wsNamesAlloced as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as Cardinal,
                ) as *mut *mut ::core::ffi::c_char;
                if ppchWsNames.is_null() {
                    ExitWM(WM_ERROR_EXIT_VALUE);
                }
            }
        }
        if bHaveWorkspaceCount == 0 {
            (*pSD).numWorkspaces = wsNameCount;
        }
    } else if bHaveWorkspaceCount == 0 {
        (*pSD).numWorkspaces = 1 as ::core::ffi::c_int;
    }
    if (*pSD).numWorkspaces > MAX_WORKSPACE_COUNT {
        (*pSD).numWorkspaces = MAX_WORKSPACE_COUNT;
    }
    (*pSD).numWsDataAllocated = (*pSD).numWorkspaces + WS_ALLOC_AMOUNT;
    (*pSD).numWsDataAllocated -= (*pSD).numWsDataAllocated % WS_ALLOC_AMOUNT;
    (*pSD).pWS = XtMalloc(
        ((*pSD).numWsDataAllocated as usize)
            .wrapping_mul(::core::mem::size_of::<WmWorkspaceData>() as usize) as Cardinal,
    ) as *mut WmWorkspaceData as *mut _WmWorkspaceData;
    if (*pSD).pWS.is_null() {
        ExitWM(WM_ERROR_EXIT_VALUE);
    }
    pwsI = (*pSD).pWS as *mut WmWorkspaceData;
    i = 0 as ::core::ffi::c_int;
    while i < (*pSD).numWorkspaces {
        if bHaveWorkspaceList as ::core::ffi::c_int != 0 && i < wsNameCount {
            string = *ppchWsNames.offset(i as isize) as *mut ::core::ffi::c_uchar;
        } else {
            string = GenerateWorkspaceName(pSD, i);
        }
        (*pwsI).name = XtMalloc(
            (1 as size_t).wrapping_add(strlen(string as *mut ::core::ffi::c_char)) as Cardinal,
        );
        if (*pwsI).name.is_null() {
            Warning(
                b"Insufficient memory for workspace data\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            ExitWM(WM_ERROR_EXIT_VALUE);
        } else {
            strcpy(
                (*pwsI).name as *mut ::core::ffi::c_char,
                string as *mut ::core::ffi::c_char,
            );
        }
        i += 1;
        pwsI = pwsI.offset(1);
    }
    if !ppchWsNames.is_null() {
        XtFree(ppchWsNames as *mut ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWorkspaceResources(mut pWS: *mut WmWorkspaceData) {
    pResWS = pWS;
    if wmGD.useStandardBehavior != 0 {
        XtGetSubresources(
            (*(*pWS).pSD).screenTopLevelW,
            pWS as XtPointer,
            (*pWS).name as *const ::core::ffi::c_char,
            (*pWS).name as *const ::core::ffi::c_char,
            &raw mut wmStdWorkspaceResources as XtResourceList,
            (::core::mem::size_of::<[XtResource; 1]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtResource>() as usize)
                as Cardinal,
            ::core::ptr::null_mut::<Arg>(),
            0 as Cardinal,
        );
        (*pWS).iconBoxGeometry = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        XtGetSubresources(
            (*(*pWS).pSD).screenTopLevelW,
            pWS as XtPointer,
            (*pWS).name as *const ::core::ffi::c_char,
            (*pWS).name as *const ::core::ffi::c_char,
            &raw mut wmWorkspaceResources as XtResourceList,
            (::core::mem::size_of::<[XtResource; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtResource>() as usize)
                as Cardinal,
            ::core::ptr::null_mut::<Arg>(),
            0 as Cardinal,
        );
        if !(*pWS).iconBoxGeometry.is_null() {
            (*pWS).iconBoxGeometry = (if !(*pWS).iconBoxGeometry.is_null() {
                strcpy(
                    XtMalloc(
                        (strlen((*pWS).iconBoxGeometry as *const ::core::ffi::c_char) as Cardinal)
                            .wrapping_add(1 as Cardinal),
                    ),
                    (*pWS).iconBoxGeometry as *const ::core::ffi::c_char,
                )
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            }) as String_0;
        }
    }
    if (*pWS).title.is_null() {
        (*pWS).title = XmStringCreateLocalized((*pWS).name);
    } else {
        (*pWS).title = XmStringCopy((*pWS).title);
    }
    if !(*pWS).iconBoxGeometry.is_null() {
        (*pWS).iconBoxGeometry = (if !(*pWS).iconBoxGeometry.is_null() {
            strcpy(
                XtMalloc(
                    (strlen((*pWS).iconBoxGeometry as *const ::core::ffi::c_char) as Cardinal)
                        .wrapping_add(1 as Cardinal),
                ),
                (*pWS).iconBoxGeometry as *const ::core::ffi::c_char,
            )
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        }) as String_0;
    }
    XtGetSubresources(
        (*pWS).workspaceTopLevelW,
        &raw mut (*pWS).backdrop as XtPointer,
        WmNbackdrop.as_ptr(),
        WmCBackdrop.as_ptr(),
        &raw mut wmBackdropResources as XtResourceList,
        (::core::mem::size_of::<[XtResource; 5]>() as usize)
            .wrapping_div(::core::mem::size_of::<XtResource>() as usize) as Cardinal,
        ::core::ptr::null_mut::<Arg>(),
        0 as Cardinal,
    );
    ProcessBackdropResources(pWS, 0 as ::core::ffi::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn ProcessPresenceResources(mut pSD: *mut WmScreenData) {
    static mut default_ws_pres_title: *mut ::core::ffi::c_char =
        b"Workspace Presence\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    let mut args: [Arg; 5] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 5];
    let mut n: ::core::ffi::c_int = 0;
    let mut pch1 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut pch2 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !(*pSD).presence.shellW.is_null() {
        XtGetSubresources(
            (*pSD).presence.shellW,
            &raw mut (*pSD).presence as XtPointer,
            WmNworkspacePresence.as_ptr(),
            WmCWorkspacePresence.as_ptr(),
            &raw mut wmWsPresenceResources as XtResourceList,
            (::core::mem::size_of::<[XtResource; 1]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtResource>() as usize)
                as Cardinal,
            ::core::ptr::null_mut::<Arg>(),
            0 as Cardinal,
        );
        pch2 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        if !(*pSD).presence.title.is_null() {
            pch1 = WmXmStringToString((*pSD).presence.title) as *mut ::core::ffi::c_uchar;
            if !pch1.is_null() && {
                pch2 = XtMalloc(
                    (1 as size_t).wrapping_add(strlen(pch1 as *mut ::core::ffi::c_char))
                        as Cardinal,
                ) as *mut ::core::ffi::c_uchar;
                !pch2.is_null()
            } {
                strcpy(
                    pch2 as *mut ::core::ffi::c_char,
                    pch1 as *mut ::core::ffi::c_char,
                );
            }
        }
        if pch2.is_null() {
            pch2 = default_ws_pres_title as *mut ::core::ffi::c_uchar;
        }
        n = 0 as ::core::ffi::c_int;
        args[n as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
            .offset(356 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_char as String_0;
        args[n as usize].value = pch2 as XtArgVal;
        n += 1;
        XtSetValues(
            (*pSD).presence.shellW,
            &raw mut args as ArgList,
            n as Cardinal,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessClientResources(mut pCD: *mut ClientData) {
    let mut clientName = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut clientClass = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pSD = (*pCD).pSD;
    _pCD = pCD;
    clientName = (if !(*pCD).clientName.is_null() {
        (*pCD).clientName as *const ::core::ffi::c_char
    } else {
        WmNdefaults.as_ptr()
    }) as String_0;
    clientClass = (if !(*pCD).clientClass.is_null() {
        (*pCD).clientClass as *const ::core::ffi::c_char
    } else {
        WmNdefaults.as_ptr()
    }) as String_0;
    if wmGD.useStandardBehavior != 0 {
        XtGetSubresources(
            (*pSD).screenTopLevelW,
            pCD as XtPointer,
            clientName as *const ::core::ffi::c_char,
            clientClass as *const ::core::ffi::c_char,
            &raw mut wmStdClientResources as XtResourceList,
            (::core::mem::size_of::<[XtResource; 11]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtResource>() as usize)
                as Cardinal,
            ::core::ptr::null_mut::<Arg>(),
            0 as Cardinal,
        );
        SetStdClientResourceValues(pCD);
    } else {
        XtGetSubresources(
            (*pSD).screenTopLevelW,
            pCD as XtPointer,
            clientName as *const ::core::ffi::c_char,
            clientClass as *const ::core::ffi::c_char,
            &raw mut wmClientResources as XtResourceList,
            (::core::mem::size_of::<[XtResource; 20]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtResource>() as usize)
                as Cardinal,
            ::core::ptr::null_mut::<Arg>(),
            0 as Cardinal,
        );
    }
    if (*pCD).systemMenu == &raw mut defaultSystemMenuName as *mut ::core::ffi::c_char
        && (*pSD).defaultSystemMenuUseBuiltin as ::core::ffi::c_int == TRUE
    {
        (*pCD).systemMenu = &raw mut builtinSystemMenuName as *mut ::core::ffi::c_char as String_0;
    }
    if !(*pCD).iconImageBottomShadowPStr.is_null() {
        if (*pCD).iconImageBottomShadowPStr == (*pSD).iconAppearance.bottomShadowPStr
            && (*pCD).iconImageBottomShadowColor == (*pSD).iconAppearance.bottomShadowColor
            && (*pCD).iconImageBackground == (*pSD).iconAppearance.background
        {
            (*pCD).iconImageBottomShadowPixmap = (*pSD).iconAppearance.bottomShadowPixmap;
        } else {
            (*pCD).iconImageBottomShadowPixmap = XmGetPixmap(
                (*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*pSD).screen as isize) as *mut Screen,
                (*pCD).iconImageBottomShadowPStr as *mut ::core::ffi::c_char,
                (*pCD).iconImageBottomShadowColor,
                (*pCD).iconImageBackground,
            );
            if (*pCD).iconImageBottomShadowPixmap == XmUNSPECIFIED_PIXMAP as Pixmap {
                (*pCD).iconImageBottomShadowPixmap = NULL as Pixmap;
            }
        }
    } else {
        (*pCD).iconImageBottomShadowPixmap = NULL as Pixmap;
    }
    if !(*pCD).iconImageTopShadowPStr.is_null() {
        if (*pCD).iconImageTopShadowPStr == (*pSD).iconAppearance.topShadowPStr
            && (*pCD).iconImageTopShadowColor == (*pSD).iconAppearance.topShadowColor
            && (*pCD).iconImageBackground == (*pSD).iconAppearance.background
        {
            (*pCD).iconImageTopShadowPixmap = (*pSD).iconAppearance.topShadowPixmap;
        } else {
            (*pCD).iconImageTopShadowPixmap = XmGetPixmap(
                (*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*pSD).screen as isize) as *mut Screen,
                (*pCD).iconImageTopShadowPStr as *mut ::core::ffi::c_char,
                (*pCD).iconImageTopShadowColor,
                (*pCD).iconImageBackground,
            );
            if (*pCD).iconImageTopShadowPixmap == XmUNSPECIFIED_PIXMAP as Pixmap {
                (*pCD).iconImageTopShadowPixmap = NULL as Pixmap;
            }
        }
    } else {
        (*pCD).iconImageTopShadowPixmap = NULL as Pixmap;
    }
    if (*pCD).internalBevel < MIN_INTERNAL_BEVEL || (*pCD).internalBevel > MAX_INTERNAL_BEVEL {
        (*pCD).internalBevel = MAX_INTERNAL_BEVEL;
    }
    if (*pCD).matteWidth > 0 as ::core::ffi::c_int {
        XtGetSubresources(
            (*pSD).screenTopLevelW,
            pCD as XtPointer,
            clientName as *const ::core::ffi::c_char,
            clientClass as *const ::core::ffi::c_char,
            &raw mut wmClientResourcesM as XtResourceList,
            (::core::mem::size_of::<[XtResource; 6]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtResource>() as usize)
                as Cardinal,
            ::core::ptr::null_mut::<Arg>(),
            0 as Cardinal,
        );
        if !(*pCD).matteBottomShadowPStr.is_null()
            && strcmp(
                (*pCD).matteBottomShadowPStr as *const ::core::ffi::c_char,
                _NoDither,
            ) == 0
        {
            (*pCD).matteBottomShadowPStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !(*pCD).matteBottomShadowPStr.is_null() {
            if (*pCD).matteBottomShadowPStr == (*pSD).clientAppearance.bottomShadowPStr
                && (*pCD).matteBottomShadowColor == (*pSD).clientAppearance.bottomShadowColor
                && (*pCD).matteBackground == (*pSD).clientAppearance.background
            {
                (*pCD).matteBottomShadowPixmap = (*pSD).clientAppearance.bottomShadowPixmap;
            } else {
                (*pCD).matteBottomShadowPixmap = XmGetPixmap(
                    (*(wmGD.display as _XPrivDisplay))
                        .screens
                        .offset((*pSD).screen as isize) as *mut Screen,
                    (*pCD).matteBottomShadowPStr as *mut ::core::ffi::c_char,
                    (*pCD).matteBottomShadowColor,
                    (*pCD).matteBackground,
                );
                if (*pCD).matteBottomShadowPixmap == XmUNSPECIFIED_PIXMAP as Pixmap {
                    (*pCD).matteBottomShadowPixmap = NULL as Pixmap;
                }
            }
        } else {
            (*pCD).matteBottomShadowPixmap = NULL as Pixmap;
        }
        if !(*pCD).matteTopShadowPStr.is_null()
            && strcmp(
                (*pCD).matteTopShadowPStr as *const ::core::ffi::c_char,
                _NoDither,
            ) == 0
        {
            (*pCD).matteTopShadowPStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !(*pCD).matteTopShadowPStr.is_null() {
            if (*pCD).matteTopShadowPStr == (*pSD).clientAppearance.topShadowPStr
                && (*pCD).matteTopShadowColor == (*pSD).clientAppearance.topShadowColor
                && (*pCD).matteBackground == (*pSD).clientAppearance.background
            {
                (*pCD).matteTopShadowPixmap = (*pSD).clientAppearance.topShadowPixmap;
            } else {
                (*pCD).matteTopShadowPixmap = XmGetPixmap(
                    (*(wmGD.display as _XPrivDisplay))
                        .screens
                        .offset((*pSD).screen as isize) as *mut Screen,
                    (*pCD).matteTopShadowPStr as *mut ::core::ffi::c_char,
                    (*pCD).matteTopShadowColor,
                    (*pCD).matteBackground,
                );
                if (*pCD).matteTopShadowPixmap == XmUNSPECIFIED_PIXMAP as Pixmap {
                    (*pCD).matteTopShadowPixmap = NULL as Pixmap;
                }
            }
        } else {
            (*pCD).matteTopShadowPixmap = NULL as Pixmap;
        }
        (*pCD).clientMatteTopShadowGC = GetHighlightGC(
            (*pCD).pSD,
            (*pCD).matteTopShadowColor,
            (*pCD).matteBackground,
            (*pCD).matteTopShadowPixmap,
        );
        (*pCD).clientMatteBottomShadowGC = GetHighlightGC(
            (*pCD).pSD,
            (*pCD).matteBottomShadowColor,
            (*pCD).matteBackground,
            (*pCD).matteBottomShadowPixmap,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetStdClientResourceValues(mut pCD: *mut ClientData) {
    (*pCD).clientDecoration = WM_DECOR_DEFAULT as ::core::ffi::c_int;
    (*pCD).clientFunctions = WM_FUNC_DEFAULT as ::core::ffi::c_int;
    (*pCD).focusAutoRaise = True as Boolean;
    (*pCD).systemMenu = &raw mut builtinSystemMenuName as *mut ::core::ffi::c_char as String_0;
    (*pCD).usePPosition = USE_PPOSITION_NONZERO;
    (*pCD).ignoreWMSaveHints = True as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn SetStdScreenResourceValues(mut pSD: *mut WmScreenData) {
    (*pSD).buttonBindings =
        &raw mut builtinButtonBindingsName as *mut ::core::ffi::c_char as String_0;
    (*pSD).cleanText = True as Boolean;
    (*pSD).iconDecoration =
        (ICON_LABEL_PART | ICON_IMAGE_PART | ICON_ACTIVE_LABEL_PART) as ::core::ffi::c_int;
    (*pSD).iconPlacement =
        (ICON_PLACE_LEFT_PRIMARY | ICON_PLACE_BOTTOM_SECONDARY) as ::core::ffi::c_int;
    (*pSD).keyBindings = &raw mut builtinKeyBindingsName as *mut ::core::ffi::c_char as String_0;
    (*pSD).limitResize = True as Boolean;
    (*pSD).resizeCursors = True as Boolean;
    (*pSD).transientDecoration = (WM_DECOR_SYSTEM | WM_DECOR_RESIZEH) as ::core::ffi::c_int;
    (*pSD).transientFunctions = (WM_FUNC_ALL
        & !(MWM_FUNC_MAXIMIZE | MWM_FUNC_MINIMIZE | MWM_FUNC_RESIZE))
        as ::core::ffi::c_int;
    (*pSD).useIconBox = False as Boolean;
    (*pSD).feedbackGeometry = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pSD).moveOpaque = False as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn GetHighlightGC(
    mut pSD: *mut WmScreenData,
    mut fg: Pixel,
    mut bg: Pixel,
    mut pixmap: Pixmap,
) -> GC {
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
    mask = (GCForeground | GCBackground | GCLineWidth | GCFillStyle) as XtGCMask;
    gcv.background = bg as ::core::ffi::c_ulong;
    gcv.foreground = fg as ::core::ffi::c_ulong;
    gcv.line_width = 1 as ::core::ffi::c_int;
    if pixmap != 0 {
        mask |= (GCFillStyle | GCTile) as XtGCMask;
        gcv.fill_style = FillTiled;
        gcv.tile = pixmap;
    } else {
        gcv.fill_style = FillSolid;
    }
    return XtGetGC((*pSD).screenTopLevelW, mask, &raw mut gcv);
}
#[no_mangle]
pub unsafe extern "C" fn _WmGetDynamicDefault(
    mut widget: Widget,
    mut type_0: ::core::ffi::c_uchar,
    mut defaultColor: String_0,
    mut newBackground: Pixel,
    mut value: *mut XrmValue,
) {
    static mut oldScreen: *mut Screen = ::core::ptr::null::<Screen>() as *mut Screen;
    static mut newScreen: *mut Screen = ::core::ptr::null::<Screen>() as *mut Screen;
    static mut oldColormap: Colormap = 0;
    static mut newColormap: Colormap = 0;
    static mut newValue: Pixel = 0;
    static mut background: Pixel = 0;
    static mut oldDefaultColor: String_0 =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    static mut colorData: XmColorData = _XmColorData {
        screen: ::core::ptr::null::<Screen>() as *mut Screen,
        color_map: 0,
        allocated: 0,
        background: XColor {
            pixel: 0,
            red: 0,
            green: 0,
            blue: 0,
            flags: 0,
            pad: 0,
        },
        foreground: XColor {
            pixel: 0,
            red: 0,
            green: 0,
            blue: 0,
            flags: 0,
            pad: 0,
        },
        top_shadow: XColor {
            pixel: 0,
            red: 0,
            green: 0,
            blue: 0,
            flags: 0,
            pad: 0,
        },
        bottom_shadow: XColor {
            pixel: 0,
            red: 0,
            green: 0,
            blue: 0,
            flags: 0,
            pad: 0,
        },
        select: XColor {
            pixel: 0,
            red: 0,
            green: 0,
            blue: 0,
            flags: 0,
            pad: 0,
        },
    };
    (*value).size = ::core::mem::size_of::<Pixel>() as ::core::ffi::c_uint;
    (*value).addr = &raw mut newValue as *mut ::core::ffi::c_char as XPointer;
    newScreen = XtScreenOfObject(widget);
    if Monochrome(newScreen) != 0 {
        let mut ok = False as Boolean;
        if wmGD.statusColorServer == CSERVE_NORMAL {
            let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < wmGD.numScreens {
                if XScreenNumberOfScreen(newScreen) == (*wmGD.Screens.offset(i as isize)).screen {
                    pSD = wmGD.Screens.offset(i as isize) as *mut WmScreenData;
                    ok = True as Boolean;
                    break;
                } else {
                    i += 1;
                }
            }
            if ok != 0 {
                ok = False as Boolean;
                i = 0 as ::core::ffi::c_int;
                while i < XmCO_MAX_NUM_COLORS {
                    if (*(*pSD).pPixelData.offset(i as isize)).bg == newBackground {
                        match type_0 as ::core::ffi::c_int {
                            2 => {
                                newValue = (*(*pSD).pPixelData.offset(i as isize)).fg;
                            }
                            1 => {
                                newValue = (*(*pSD).pPixelData.offset(i as isize)).bg;
                            }
                            4 => {
                                newValue = (*(*pSD).pPixelData.offset(i as isize)).ts;
                            }
                            8 => {
                                newValue = (*(*pSD).pPixelData.offset(i as isize)).bs;
                            }
                            _ => {}
                        }
                        ok = True as Boolean;
                    }
                    i += 1;
                }
            }
        }
        if ok == 0 {
            match type_0 as ::core::ffi::c_int {
                2 => {
                    newValue = (*newScreen).black_pixel as Pixel;
                }
                1 => {
                    newValue = (*newScreen).white_pixel as Pixel;
                }
                4 => {
                    newValue = (*newScreen).white_pixel as Pixel;
                }
                8 => {
                    newValue = (*newScreen).black_pixel as Pixel;
                }
                _ => {}
            }
        }
        return;
    }
    newColormap = (*widget).core.colormap;
    if !(!oldScreen.is_null()
        && oldScreen == newScreen
        && oldColormap == newColormap
        && type_0 as ::core::ffi::c_int != WmBGC as ::core::ffi::c_int
        && background == newBackground)
    {
        if !(oldScreen == newScreen
            && oldColormap == newColormap
            && type_0 as ::core::ffi::c_int == WmBGC as ::core::ffi::c_int
            && oldDefaultColor == defaultColor)
        {
            if type_0 as ::core::ffi::c_int == WmBGC as ::core::ffi::c_int {
                oldDefaultColor = defaultColor;
                colorData = *_WmGetDefaultColors(newScreen, newColormap, defaultColor);
            } else {
                oldDefaultColor = ::core::ptr::null_mut::<::core::ffi::c_char>();
                background = newBackground;
                XmGetColors(
                    newScreen,
                    newColormap,
                    background,
                    &raw mut colorData.foreground.pixel,
                    &raw mut colorData.top_shadow.pixel,
                    &raw mut colorData.bottom_shadow.pixel,
                    &raw mut colorData.select.pixel,
                );
            }
        }
    }
    oldScreen = newScreen;
    oldColormap = newColormap;
    colorData.allocated = (colorData.allocated as ::core::ffi::c_int | type_0 as ::core::ffi::c_int)
        as ::core::ffi::c_uchar;
    match type_0 as ::core::ffi::c_int {
        1 => {
            newValue = colorData.background.pixel as Pixel;
        }
        2 => {
            newValue = colorData.foreground.pixel as Pixel;
        }
        4 => {
            newValue = colorData.top_shadow.pixel as Pixel;
        }
        8 => {
            newValue = colorData.bottom_shadow.pixel as Pixel;
        }
        16 => {
            newValue = colorData.select.pixel as Pixel;
        }
        _ => {
            newValue = colorData.background.pixel as Pixel;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _WmGetDefaultColors(
    mut screen: *mut Screen,
    mut colormap: Colormap,
    mut defaultColor: String_0,
) -> *mut XmColorData {
    static mut defaultSet: [*mut XmColorData; 2] = [
        ::core::ptr::null::<XmColorData>() as *mut XmColorData,
        ::core::ptr::null::<XmColorData>() as *mut XmColorData,
    ];
    static mut defaultCount: [::core::ffi::c_int; 2] =
        [0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int];
    static mut defaultSize: [::core::ffi::c_int; 2] =
        [0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int];
    let mut setId: ::core::ffi::c_int = 0;
    let mut set = ::core::ptr::null_mut::<XmColorData>();
    let mut count: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut display = (*screen).display as *mut Display;
    let mut colorDef = XColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    if defaultColor == &raw mut _defaultColor2 as *mut ::core::ffi::c_char {
        setId = 1 as ::core::ffi::c_int;
    } else {
        setId = 0 as ::core::ffi::c_int;
    }
    set = defaultSet[setId as usize];
    count = defaultCount[setId as usize];
    size = defaultSize[setId as usize];
    i = 0 as ::core::ffi::c_int;
    while i < count {
        if (*set.offset(i as isize)).screen == screen
            && (*set.offset(i as isize)).color_map == colormap
        {
            return set.offset(i as isize);
        }
        i += 1;
    }
    if count == size {
        defaultSize[setId as usize] += 10 as ::core::ffi::c_int;
        size = defaultSize[setId as usize];
        defaultSet[setId as usize] = WmRealloc(
            defaultSet[setId as usize] as *mut ::core::ffi::c_char,
            (::core::mem::size_of::<XmColorData>() as usize).wrapping_mul(size as usize)
                as ::core::ffi::c_uint,
        ) as *mut XmColorData;
        set = defaultSet[setId as usize];
    }
    if XParseColor(
        display,
        colormap,
        defaultColor as *const ::core::ffi::c_char,
        &raw mut colorDef,
    ) == 0
    {
        if strcmp(
            defaultColor as *const ::core::ffi::c_char,
            &raw mut _defaultColor1 as *mut ::core::ffi::c_char,
        ) == 0
        {
            XParseColor(
                display,
                colormap,
                &raw mut _defaultColor1HEX as *mut ::core::ffi::c_char,
                &raw mut colorDef,
            );
        } else {
            XParseColor(
                display,
                colormap,
                &raw mut _defaultColor2HEX as *mut ::core::ffi::c_char,
                &raw mut colorDef,
            );
        }
    }
    XAllocColor(display, colormap, &raw mut colorDef);
    XmGetColors(
        screen,
        colormap,
        colorDef.pixel as Pixel,
        &raw mut (*set.offset(count as isize)).foreground.pixel,
        &raw mut (*set.offset(count as isize)).top_shadow.pixel,
        &raw mut (*set.offset(count as isize)).bottom_shadow.pixel,
        &raw mut (*set.offset(count as isize)).select.pixel,
    );
    (*set.offset(count as isize)).background.pixel = colorDef.pixel;
    let ref mut fresh2 = (*set.offset(count as isize)).screen;
    *fresh2 = screen;
    (*set.offset(count as isize)).color_map = colormap;
    (*set.offset(count as isize)).allocated = True as ::core::ffi::c_uchar;
    XQueryColor(
        wmGD.display,
        colormap,
        &raw mut (*set.offset(count as isize)).background,
    );
    XQueryColor(
        wmGD.display,
        colormap,
        &raw mut (*set.offset(count as isize)).foreground,
    );
    XQueryColor(
        wmGD.display,
        colormap,
        &raw mut (*set.offset(count as isize)).top_shadow,
    );
    XQueryColor(
        wmGD.display,
        colormap,
        &raw mut (*set.offset(count as isize)).bottom_shadow,
    );
    XQueryColor(
        wmGD.display,
        colormap,
        &raw mut (*set.offset(count as isize)).select,
    );
    defaultCount[setId as usize] += 1;
    return set.offset(count as isize);
}
#[no_mangle]
pub unsafe extern "C" fn WmRealloc(
    mut ptr: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_char {
    if !ptr.is_null() {
        ptr = XtRealloc(ptr, size as Cardinal);
    } else {
        ptr = XtMalloc(size as Cardinal);
    }
    if ptr.is_null() {
        Warning(
            b"Insufficient memory for window manager data\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn WmMalloc(
    mut ptr: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_char {
    if !ptr.is_null() {
        XtFree(ptr);
    }
    ptr = XtMalloc(size as Cardinal);
    if ptr.is_null() {
        Warning(
            b"Insufficient memory for window manager data\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn SetupDefaultResources(mut pSD: *mut WmScreenData) {
    let mut menuSpec = ::core::ptr::null_mut::<MenuSpec>();
    if (*pSD).keySpecs.is_null() && wmGD.useStandardBehavior == 0 {
        if strcmp(
            (*pSD).keyBindings as *const ::core::ffi::c_char,
            &raw mut defaultKeyBindingsName as *mut ::core::ffi::c_char,
        ) != 0
        {
            MWarning(
                b"Key bindings %s not found, using builtin key bindings\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                (*pSD).keyBindings as *mut ::core::ffi::c_char,
            );
        }
        (*pSD).keyBindings =
            &raw mut builtinKeyBindingsName as *mut ::core::ffi::c_char as String_0;
    }
    if (*pSD).buttonSpecs.is_null() && wmGD.useStandardBehavior == 0 {
        if strcmp(
            (*pSD).buttonBindings as *const ::core::ffi::c_char,
            &raw mut defaultButtonBindingsName as *mut ::core::ffi::c_char,
        ) != 0
        {
            MWarning(
                b"Button bindings %s not found, using builtin button bindings\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                (*pSD).buttonBindings as *mut ::core::ffi::c_char,
            );
        }
        (*pSD).buttonBindings =
            &raw mut builtinButtonBindingsName as *mut ::core::ffi::c_char as String_0;
    }
    if (*pSD).keyBindings == &raw mut builtinKeyBindingsName as *mut ::core::ffi::c_char {
        ParseKeyStr(
            pSD,
            &raw mut builtinKeyBindings as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        );
    }
    if (*pSD).buttonBindings == &raw mut builtinButtonBindingsName as *mut ::core::ffi::c_char {
        ParseButtonStr(
            pSD,
            &raw mut builtinButtonBindings as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        );
    }
    menuSpec = (*pSD).menuSpecs;
    while !menuSpec.is_null() {
        if strcmp(
            (*menuSpec).name as *const ::core::ffi::c_char,
            &raw mut defaultSystemMenuName as *mut ::core::ffi::c_char,
        ) == 0
        {
            (*pSD).defaultSystemMenuUseBuiltin = FALSE as Boolean;
            break;
        } else {
            menuSpec = (*menuSpec).nextMenuSpec as *mut MenuSpec;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn SimilarAppearanceData(
    mut pAD1: *mut AppearanceData,
    mut pAD2: *mut AppearanceData,
) -> Boolean {
    let mut rval: Boolean = 0;
    if (*pAD1).renderTable == (*pAD2).renderTable
        && (*pAD1).background == (*pAD2).background
        && (*pAD1).foreground == (*pAD2).foreground
        && (*pAD1).backgroundPStr == (*pAD2).backgroundPStr
        && (*pAD1).bottomShadowColor == (*pAD2).bottomShadowColor
        && (*pAD1).bottomShadowPStr == (*pAD2).bottomShadowPStr
        && (*pAD1).topShadowColor == (*pAD2).topShadowColor
        && (*pAD1).topShadowPStr == (*pAD2).topShadowPStr
        && (*pAD1).activeBackground == (*pAD2).activeBackground
        && (*pAD1).activeForeground == (*pAD2).activeForeground
        && (*pAD1).activeBackgroundPStr == (*pAD2).activeBackgroundPStr
        && (*pAD1).activeBottomShadowColor == (*pAD2).activeBottomShadowColor
        && (*pAD1).activeBottomShadowPStr == (*pAD2).activeBottomShadowPStr
        && (*pAD1).activeTopShadowColor == (*pAD2).activeTopShadowColor
        && (*pAD1).activeTopShadowPStr == (*pAD2).activeTopShadowPStr
    {
        rval = True as Boolean;
    } else {
        rval = False as Boolean;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn Monochrome(mut screen: *mut Screen) -> Boolean {
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    let mut scr: ::core::ffi::c_int = 0;
    if wmGD.statusColorServer == CSERVE_NORMAL {
        scr = 0 as ::core::ffi::c_int;
        while scr < wmGD.numScreens {
            pSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
            if (*pSD).managed != 0 {
                if XScreenOfDisplay(wmGD.display, (*pSD).screen) == screen {
                    if (*pSD).colorUse == XmCO_BLACK_WHITE as ::core::ffi::c_int {
                        return 1 as Boolean;
                    } else {
                        return 0 as Boolean;
                    }
                }
            }
            scr += 1;
        }
    }
    return ((*screen).root_depth == 1 as ::core::ffi::c_int) as ::core::ffi::c_int as Boolean;
}
unsafe extern "C" fn run_static_initializers() {
    wmGlobalResources = [
        _XtResource {
            resource_name: WmNautoKeyFocus.as_ptr() as String_0,
            resource_class: WmCAutoKeyFocus.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, autoKeyFocus) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNautoRaiseDelay.as_ptr() as String_0,
            resource_class: WmCAutoRaiseDelay.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, autoRaiseDelay) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: 500 as ::core::ffi::c_int as XtPointer,
        },
        _XtResource {
            resource_name: WmNbitmapDirectory.as_ptr() as String_0,
            resource_class: WmCBitmapDirectory.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, bitmapDirectory) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: BITMAPDIR.as_ptr() as XtPointer,
        },
        _XtResource {
            resource_name: WmNframeStyle.as_ptr() as String_0,
            resource_class: WmCFrameStyle.as_ptr() as String_0,
            resource_type: WmRFrameStyle.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<FrameStyle>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, frameStyle) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNclientAutoPlace.as_ptr() as String_0,
            resource_class: WmCClientAutoPlace.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, clientAutoPlace) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNcolormapFocusPolicy.as_ptr() as String_0,
            resource_class: WmCColormapFocusPolicy.as_ptr() as String_0,
            resource_type: WmRCFocusPolicy.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, colormapFocusPolicy) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: CMAP_FOCUS_KEYBOARD as XtPointer,
        },
        _XtResource {
            resource_name: WmNconfigFile.as_ptr() as String_0,
            resource_class: WmCConfigFile.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, configFile) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNcppCommand.as_ptr() as String_0,
            resource_class: WmCCppCommand.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, cppCommand) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNdeiconifyKeyFocus.as_ptr() as String_0,
            resource_class: WmCDeiconifyKeyFocus.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, deiconifyKeyFocus) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNdoubleClickTime.as_ptr() as String_0,
            resource_class: WmCDoubleClickTime.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, doubleClickTime) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmMultiClickTimeDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNenableWarp.as_ptr() as String_0,
            resource_class: WmCEnableWarp.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, enableWarp) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNenforceKeyFocus.as_ptr() as String_0,
            resource_class: WmCEnforceKeyFocus.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, enforceKeyFocus) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNframeExternalShadowWidth.as_ptr() as String_0,
            resource_class: WmCFrameExternalShadowWidth.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1618 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Dimension>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, frameExternalShadowWidth)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: 2 as ::core::ffi::c_int as XtPointer,
        },
        _XtResource {
            resource_name: WmNuseWindowOutline.as_ptr() as String_0,
            resource_class: WmCUseWindowOutline.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, useWindowOutline) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNoutlineWidth.as_ptr() as String_0,
            resource_class: WmCOutlineWidth.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1618 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Dimension>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, outlineWidth) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: 2 as ::core::ffi::c_int as XtPointer,
        },
        _XtResource {
            resource_name: WmNiconAutoPlace.as_ptr() as String_0,
            resource_class: WmCIconAutoPlace.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, iconAutoPlace) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNiconExternalShadowWidth.as_ptr() as String_0,
            resource_class: WmCIconExternalShadowWidth.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1618 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Dimension>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, iconExternalShadowWidth)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: 2 as ::core::ffi::c_int as XtPointer,
        },
        _XtResource {
            resource_name: WmNiconClick.as_ptr() as String_0,
            resource_class: WmCIconClick.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, iconClick) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNinteractivePlacement.as_ptr() as String_0,
            resource_class: WmCInteractivePlacement.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, interactivePlacement)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNkeyboardFocusPolicy.as_ptr() as String_0,
            resource_class: WmCKeyboardFocusPolicy.as_ptr() as String_0,
            resource_type: WmRKFocusPolicy.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, keyboardFocusPolicy) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNlowerOnIconify.as_ptr() as String_0,
            resource_class: WmCLowerOnIconify.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, lowerOnIconify) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNmarqueeSelectGranularity.as_ptr() as String_0,
            resource_class: WmCMarqueeSelectGranularity.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, marqueeSelectGranularity)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNmoveThreshold.as_ptr() as String_0,
            resource_class: WmCMoveThreshold.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, moveThreshold) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: 4 as ::core::ffi::c_int as XtPointer,
        },
        _XtResource {
            resource_name: WmNpassButtons.as_ptr() as String_0,
            resource_class: WmCPassButtons.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, passButtons) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNpassSelectButton.as_ptr() as String_0,
            resource_class: WmCPassSelectButton.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, passSelectButton) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNpositionOnScreen.as_ptr() as String_0,
            resource_class: WmCPositionOnScreen.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, positionOnScreen) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNquitTimeout.as_ptr() as String_0,
            resource_class: WmCQuitTimeout.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, quitTimeout) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: 1000 as ::core::ffi::c_int as XtPointer,
        },
        _XtResource {
            resource_name: WmNraiseKeyFocus.as_ptr() as String_0,
            resource_class: WmCRaiseKeyFocus.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, raiseKeyFocus) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNrootButtonClick.as_ptr() as String_0,
            resource_class: WmCRootButtonClick.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, rootButtonClick) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNrefreshByClearing.as_ptr() as String_0,
            resource_class: WmCRefreshByClearing.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, refreshByClearing) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNsessionClientDB.as_ptr() as String_0,
            resource_class: WmCSessionClientDB.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, sessionClientDB) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNshowFeedback.as_ptr() as String_0,
            resource_class: WmCShowFeedback.as_ptr() as String_0,
            resource_type: WmRShowFeedback.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, showFeedback) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: (WM_SHOW_FB_BEHAVIOR
                | WM_SHOW_FB_MOVE
                | WM_SHOW_FB_PLACEMENT
                | WM_SHOW_FB_RESIZE
                | WM_SHOW_FB_RESTART
                | WM_SHOW_FB_QUIT
                | WM_SHOW_FB_KILL) as XtPointer,
        },
        _XtResource {
            resource_name: WmNstartupKeyFocus.as_ptr() as String_0,
            resource_class: WmCStartupKeyFocus.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, startupKeyFocus) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNsystemButtonClick.as_ptr() as String_0,
            resource_class: WmCSystemButtonClick.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, systemButtonClick) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNsystemButtonClick2.as_ptr() as String_0,
            resource_class: WmCSystemButtonClick2.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, systemButtonClick2) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNprimaryXineramaScreen.as_ptr() as String_0,
            resource_class: WmCPrimaryXineramaScreen.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, primaryXineramaScreen)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNxineramaScreenFocus.as_ptr() as String_0,
            resource_class: WmCXineramaScreenFocus.as_ptr() as String_0,
            resource_type: WmRXineramaScreenFocus.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, xineramaScreenFocus) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNxineramaIconifyToPrimary.as_ptr() as String_0,
            resource_class: WmCXineramaIconifyToPrimary.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, xineramaIconifyToPrimary)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
    ];
    wmStdGlobalResources = [
        _XtResource {
            resource_name: WmNbitmapDirectory.as_ptr() as String_0,
            resource_class: WmCBitmapDirectory.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, bitmapDirectory) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: BITMAPDIR.as_ptr() as XtPointer,
        },
        _XtResource {
            resource_name: WmNconfigFile.as_ptr() as String_0,
            resource_class: WmCConfigFile.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, configFile) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNframeStyle.as_ptr() as String_0,
            resource_class: WmCFrameStyle.as_ptr() as String_0,
            resource_type: WmRFrameStyle.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<FrameStyle>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, frameStyle) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNiconAutoPlace.as_ptr() as String_0,
            resource_class: WmCIconAutoPlace.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, iconAutoPlace) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNmoveThreshold.as_ptr() as String_0,
            resource_class: WmCMoveThreshold.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, moveThreshold) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: 4 as ::core::ffi::c_int as XtPointer,
        },
        _XtResource {
            resource_name: WmNpositionOnScreen.as_ptr() as String_0,
            resource_class: WmCPositionOnScreen.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, positionOnScreen) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNquitTimeout.as_ptr() as String_0,
            resource_class: WmCQuitTimeout.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, quitTimeout) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: 1000 as ::core::ffi::c_int as XtPointer,
        },
        _XtResource {
            resource_name: WmNshowFeedback.as_ptr() as String_0,
            resource_class: WmCShowFeedback.as_ptr() as String_0,
            resource_type: WmRShowFeedback.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, showFeedback) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: (WM_SHOW_FB_BEHAVIOR
                | WM_SHOW_FB_MOVE
                | WM_SHOW_FB_PLACEMENT
                | WM_SHOW_FB_RESIZE
                | WM_SHOW_FB_RESTART
                | WM_SHOW_FB_QUIT
                | WM_SHOW_FB_KILL) as XtPointer,
        },
        _XtResource {
            resource_name: WmNprimaryXineramaScreen.as_ptr() as String_0,
            resource_class: WmCPrimaryXineramaScreen.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, primaryXineramaScreen)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
    ];
    _NoDither = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(20008 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
    wmAppearanceResources = [
        _XtResource {
            resource_name: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(15749 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_class: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(15761 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_type: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(15773 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_size: ::core::mem::size_of::<XmRenderTable>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, renderTable) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: b"Sans\0" as *const u8 as *const ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNsaveUnder.as_ptr() as String_0,
            resource_class: WmCSaveUnder.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, saveUnder) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(52 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_class: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(902 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, background) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmBackgroundDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(214 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_class: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1022 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, foreground) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmForegroundDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(5096 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_class: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1022 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, bottomShadowColor) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmBottomShadowColorDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(5114 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_class: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(372 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, bottomShadowPStr) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(10308 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_class: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1022 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, topShadowColor) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmTopShadowColorDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(63 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_class: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(316 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, backgroundPStr) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmBackgroundPixmapDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(10323 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_class: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(4291 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, topShadowPStr) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmTopShadowPixmapDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNactiveBackground.as_ptr() as String_0,
            resource_class: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(902 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, activeBackground) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmABackgroundDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNactiveForeground.as_ptr() as String_0,
            resource_class: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1022 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, activeForeground) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmAForegroundDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNactiveBottomShadowColor.as_ptr() as String_0,
            resource_class: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1022 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, activeBottomShadowColor)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmABottomShadowColorDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNactiveBottomShadowPixmap.as_ptr() as String_0,
            resource_class: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(372 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, activeBottomShadowPStr)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNactiveTopShadowColor.as_ptr() as String_0,
            resource_class: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1022 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, activeTopShadowColor)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmATopShadowColorDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNactiveBackgroundPixmap.as_ptr() as String_0,
            resource_class: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(316 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, activeBackgroundPStr)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmABackgroundPixmapDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNactiveTopShadowPixmap.as_ptr() as String_0,
            resource_class: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(4291 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(AppearanceData, activeTopShadowPStr)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmATopShadowPixmapDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
    ];
    wmGlobalScreenResources = [
        _XtResource {
            resource_name: WmNmultiScreen.as_ptr() as String_0,
            resource_class: WmCMultiScreen.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, multiScreen) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNscreens.as_ptr() as String_0,
            resource_class: WmCScreens.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, screenList) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNbackdropDirectories.as_ptr() as String_0,
            resource_class: WmCBackdropDirectories.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<*mut ::core::ffi::c_char>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmGlobalData, backdropDirs) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: DEFAULT_BACKDROP_DIR.as_ptr() as XtPointer,
        },
    ];
    _Dither = (&raw mut _XmStrings as *mut ::core::ffi::c_char)
        .offset(19994 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
    wmScreenResources = [
        _XtResource {
            resource_name: WmNbuttonBindings.as_ptr() as String_0,
            resource_class: WmCButtonBindings.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, buttonBindings) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: &raw mut defaultButtonBindingsName as *mut ::core::ffi::c_char
                as XtPointer,
        },
        _XtResource {
            resource_name: WmNcleanText.as_ptr() as String_0,
            resource_class: WmCCleanText.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, cleanText) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNfeedbackGeometry.as_ptr() as String_0,
            resource_class: WmCFeedbackGeometry.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, feedbackGeometry) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNfadeNormalIcon.as_ptr() as String_0,
            resource_class: WmCFadeNormalIcon.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, fadeNormalIcon) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNiconDecoration.as_ptr() as String_0,
            resource_class: WmCIconDecoration.as_ptr() as String_0,
            resource_type: WmRIconDecor.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconDecoration) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: USE_ICON_DEFAULT_APPEARANCE as XtPointer,
        },
        _XtResource {
            resource_name: WmNiconImageMaximum.as_ptr() as String_0,
            resource_class: WmCIconImageMaximum.as_ptr() as String_0,
            resource_type: WmRSize.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<WHSize>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconImageMaximum) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageMaximumDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageMinimum.as_ptr() as String_0,
            resource_class: WmCIconImageMinimum.as_ptr() as String_0,
            resource_type: WmRSize.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<WHSize>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconImageMinimum) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: b"16x16\0" as *const u8 as *const ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNiconPlacement.as_ptr() as String_0,
            resource_class: WmCIconPlacement.as_ptr() as String_0,
            resource_type: WmRIconPlacement.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconPlacement) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: (ICON_PLACE_LEFT_PRIMARY | ICON_PLACE_BOTTOM_SECONDARY) as XtPointer,
        },
        _XtResource {
            resource_name: WmNiconPlacementMargin.as_ptr() as String_0,
            resource_class: WmCIconPlacementMargin.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconPlacementMargin) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: -(1 as ::core::ffi::c_int) as XtPointer,
        },
        _XtResource {
            resource_name: WmNkeyBindings.as_ptr() as String_0,
            resource_class: WmCKeyBindings.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, keyBindings) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: &raw mut defaultKeyBindingsName as *mut ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNframeBorderWidth.as_ptr() as String_0,
            resource_class: WmCFrameBorderWidth.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, frameBorderWidth) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: BIGSIZE as XtPointer,
        },
        _XtResource {
            resource_name: WmNiconBoxName.as_ptr() as String_0,
            resource_class: WmCIconBoxName.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconBoxName) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: b"iconbox\0" as *const u8 as *const ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNiconBoxSBDisplayPolicy.as_ptr() as String_0,
            resource_class: WmCIconBoxSBDisplayPolicy.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconBoxSBDisplayPolicy)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: b"all\0" as *const u8 as *const ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNiconBoxScheme.as_ptr() as String_0,
            resource_class: WmCIconBoxScheme.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconBoxScheme) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNiconBoxTitle.as_ptr() as String_0,
            resource_class: WmCIconBoxTitle.as_ptr() as String_0,
            resource_type: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(12263 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_size: ::core::mem::size_of::<XmString>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconBoxTitle) as Cardinal,
            default_type: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(12263 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNlimitResize.as_ptr() as String_0,
            resource_class: WmCLimitResize.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, limitResize) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNmaximumMaximumSize.as_ptr() as String_0,
            resource_class: WmCMaximumMaximumSize.as_ptr() as String_0,
            resource_type: WmRSize.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<WHSize>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, maximumMaximumSize) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: b"0x0\0" as *const u8 as *const ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNresizeBorderWidth.as_ptr() as String_0,
            resource_class: WmCFrameBorderWidth.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, resizeBorderWidth) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: BIGSIZE as XtPointer,
        },
        _XtResource {
            resource_name: WmNresizeCursors.as_ptr() as String_0,
            resource_class: WmCResizeCursors.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, resizeCursors) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNtransientDecoration.as_ptr() as String_0,
            resource_class: WmCTransientDecoration.as_ptr() as String_0,
            resource_type: WmRClientDecor.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, transientDecoration) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: (WM_DECOR_SYSTEM | WM_DECOR_RESIZEH) as XtPointer,
        },
        _XtResource {
            resource_name: WmNtransientFunctions.as_ptr() as String_0,
            resource_class: WmCTransientFunctions.as_ptr() as String_0,
            resource_type: WmRClientFunction.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, transientFunctions) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: (WM_FUNC_ALL & !(MWM_FUNC_MAXIMIZE | MWM_FUNC_MINIMIZE)) as XtPointer,
        },
        _XtResource {
            resource_name: WmNutilityDecoration.as_ptr() as String_0,
            resource_class: WmCUtilityDecoration.as_ptr() as String_0,
            resource_type: WmRClientDecor.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, utilityDecoration) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: (WM_DECOR_SYSTEM | WM_DECOR_RESIZEH) as XtPointer,
        },
        _XtResource {
            resource_name: WmNutilityFunctions.as_ptr() as String_0,
            resource_class: WmCUtilityFunctions.as_ptr() as String_0,
            resource_type: WmRClientFunction.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, utilityFunctions) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: (WM_FUNC_ALL & !((1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int))
                as XtPointer,
        },
        _XtResource {
            resource_name: WmNuseIconBox.as_ptr() as String_0,
            resource_class: WmCUseIconBox.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, useIconBox) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNmoveOpaque.as_ptr() as String_0,
            resource_class: WmCMoveOpaque.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, moveOpaque) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNtitleLeft.as_ptr() as String_0,
            resource_class: WmCTitleLeft.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, titleLeft) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNinitialWorkspace.as_ptr() as String_0,
            resource_class: WmCInitialWorkspace.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, initialWorkspace) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNworkspaceList.as_ptr() as String_0,
            resource_class: WmCWorkspaceList.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, workspaceList) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNworkspaceCount.as_ptr() as String_0,
            resource_class: WmCWorkspaceCount.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, numWorkspaces) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: 4 as ::core::ffi::c_int as XtPointer,
        },
    ];
    wmStdScreenResources = [
        _XtResource {
            resource_name: WmNframeBorderWidth.as_ptr() as String_0,
            resource_class: WmCFrameBorderWidth.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, frameBorderWidth) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: BIGSIZE as XtPointer,
        },
        _XtResource {
            resource_name: WmNiconImageMaximum.as_ptr() as String_0,
            resource_class: WmCIconImageMaximum.as_ptr() as String_0,
            resource_type: WmRSize.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<WHSize>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconImageMaximum) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageMaximumDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageMinimum.as_ptr() as String_0,
            resource_class: WmCIconImageMinimum.as_ptr() as String_0,
            resource_type: WmRSize.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<WHSize>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconImageMinimum) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: b"32x32\0" as *const u8 as *const ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNiconPlacementMargin.as_ptr() as String_0,
            resource_class: WmCIconPlacementMargin.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, iconPlacementMargin) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: -(1 as ::core::ffi::c_int) as XtPointer,
        },
        _XtResource {
            resource_name: WmNmaximumMaximumSize.as_ptr() as String_0,
            resource_class: WmCMaximumMaximumSize.as_ptr() as String_0,
            resource_type: WmRSize.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<WHSize>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, maximumMaximumSize) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: b"0x0\0" as *const u8 as *const ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNresizeBorderWidth.as_ptr() as String_0,
            resource_class: WmCFrameBorderWidth.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmScreenData, resizeBorderWidth) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: BIGSIZE as XtPointer,
        },
    ];
    wmBackdropResources = [
        _XtResource {
            resource_name: WmNcolor.as_ptr() as String_0,
            resource_class: WmCColor.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(BackdropData, defBackground) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: b"#4C719E\0" as *const u8 as *const ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNcolorSetId.as_ptr() as String_0,
            resource_class: WmCColorSetId.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(BackdropData, colorSet) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmBackdropColorSetDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNimageBackground.as_ptr() as String_0,
            resource_class: WmCImageBackground.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(BackdropData, background) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmBackdropBgDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNimageForeground.as_ptr() as String_0,
            resource_class: WmCImageForeground.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(BackdropData, foreground) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmBackdropFgDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNimage.as_ptr() as String_0,
            resource_class: WmCImage.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(BackdropData, image) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
    ];
    wmWorkspaceResources = [
        _XtResource {
            resource_name: WmNiconBoxGeometry.as_ptr() as String_0,
            resource_class: WmCIconBoxGeometry.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmWorkspaceData, iconBoxGeometry) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNtitle.as_ptr() as String_0,
            resource_class: WmCTitle.as_ptr() as String_0,
            resource_type: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(12263 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            resource_size: ::core::mem::size_of::<XmString>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(WmWorkspaceData, title) as Cardinal,
            default_type: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
                .offset(12263 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            default_addr: NULL,
        },
    ];
    wmStdWorkspaceResources = [_XtResource {
        resource_name: WmNtitle.as_ptr() as String_0,
        resource_class: WmCTitle.as_ptr() as String_0,
        resource_type: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(12263 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        resource_size: ::core::mem::size_of::<XmString>() as Cardinal,
        resource_offset: ::core::mem::offset_of!(WmWorkspaceData, title) as Cardinal,
        default_type: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(12263 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        default_addr: NULL,
    }];
    wmClientResourcesM = [
        _XtResource {
            resource_name: WmNmatteBackground.as_ptr() as String_0,
            resource_class: WmCMatteBackground.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, matteBackground) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmMatteBDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNmatteForeground.as_ptr() as String_0,
            resource_class: WmCMatteForeground.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, matteForeground) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmMatteFDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNmatteBottomShadowColor.as_ptr() as String_0,
            resource_class: WmCMatteBottomShadowColor.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, matteBottomShadowColor)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmMatteBSCDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNmatteBottomShadowPixmap.as_ptr() as String_0,
            resource_class: WmCMatteBottomShadowPixmap.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, matteBottomShadowPStr) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmMatteBSPDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNmatteTopShadowColor.as_ptr() as String_0,
            resource_class: WmCMatteTopShadowColor.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, matteTopShadowColor) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmMatteTSCDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNmatteTopShadowPixmap.as_ptr() as String_0,
            resource_class: WmCMatteTopShadowPixmap.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, matteTopShadowPStr) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmMatteTSPDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
    ];
    wmClientResources = [
        _XtResource {
            resource_name: WmNabsentMapBehavior.as_ptr() as String_0,
            resource_class: WmCAbsentMapBehavior.as_ptr() as String_0,
            resource_type: WmRAbsentMapBehavior.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, absentMapBehavior) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNclientDecoration.as_ptr() as String_0,
            resource_class: WmCClientDecoration.as_ptr() as String_0,
            resource_type: WmRClientDecor.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, clientDecoration) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ((1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int) as XtPointer,
        },
        _XtResource {
            resource_name: WmNclientFunctions.as_ptr() as String_0,
            resource_class: WmCClientFunctions.as_ptr() as String_0,
            resource_type: WmRClientFunction.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, clientFunctions) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ((1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int) as XtPointer,
        },
        _XtResource {
            resource_name: WmNfocusAutoRaise.as_ptr() as String_0,
            resource_class: WmCFocusAutoRaise.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, focusAutoRaise) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmFocusAutoRaiseDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImage.as_ptr() as String_0,
            resource_class: WmCIconImage.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImage) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNiconImageBackground.as_ptr() as String_0,
            resource_class: WmCIconImageBackground.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageBackground) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageBDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageForeground.as_ptr() as String_0,
            resource_class: WmCIconImageForeground.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageForeground) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageFDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageBottomShadowColor.as_ptr() as String_0,
            resource_class: WmCIconImageBottomShadowColor.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageBottomShadowColor)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageBSCDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageBottomShadowPixmap.as_ptr() as String_0,
            resource_class: WmCIconImageBottomShadowPixmap.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageBottomShadowPStr)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageBSPDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageTopShadowColor.as_ptr() as String_0,
            resource_class: WmCIconImageTopShadowColor.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageTopShadowColor)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageTSCDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageTopShadowPixmap.as_ptr() as String_0,
            resource_class: WmCIconImageTopShadowPixmap.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageTopShadowPStr)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageTSPDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNignoreWMSaveHints.as_ptr() as String_0,
            resource_class: WmCIgnoreWMSaveHints.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, ignoreWMSaveHints) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: True as XtPointer,
        },
        _XtResource {
            resource_name: WmNmatteWidth.as_ptr() as String_0,
            resource_class: WmCMatteWidth.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, matteWidth) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNmaximumClientSize.as_ptr() as String_0,
            resource_class: WmCMaximumClientSize.as_ptr() as String_0,
            resource_type: WmRSize.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<WHSize>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, maximumClientSize) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: b"0x0\0" as *const u8 as *const ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNsecondariesOnTop.as_ptr() as String_0,
            resource_class: WmCSecondariesOnTop.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, secondariesOnTop) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmSecondariesOnTopDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNsystemMenu.as_ptr() as String_0,
            resource_class: WmCSystemMenu.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, systemMenu) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: &raw mut defaultSystemMenuName as *mut ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNuseClientIcon.as_ptr() as String_0,
            resource_class: WmCUseClientIcon.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, useClientIcon) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNusePPosition.as_ptr() as String_0,
            resource_class: WmCUsePPosition.as_ptr() as String_0,
            resource_type: WmRUsePPosition.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, usePPosition) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: 2 as ::core::ffi::c_int as XtPointer,
        },
        _XtResource {
            resource_name: WmNoverrideGeometry.as_ptr() as String_0,
            resource_class: WmCOverrideGeometry.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, overrideGeometry) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNoccupyWorkspaces.as_ptr() as String_0,
            resource_class: WmCOccupyWorkspaces.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, occupyWorkspaces) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
    ];
    wmStdClientResources = [
        _XtResource {
            resource_name: WmNiconImage.as_ptr() as String_0,
            resource_class: WmCIconImage.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImage) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: NULL,
        },
        _XtResource {
            resource_name: WmNiconImageBackground.as_ptr() as String_0,
            resource_class: WmCIconImageBackground.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageBackground) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageBDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageForeground.as_ptr() as String_0,
            resource_class: WmCIconImageForeground.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageForeground) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageFDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageBottomShadowColor.as_ptr() as String_0,
            resource_class: WmCIconImageBottomShadowColor.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageBottomShadowColor)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageBSCDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageBottomShadowPixmap.as_ptr() as String_0,
            resource_class: WmCIconImageBottomShadowPixmap.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageBottomShadowPStr)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageBSPDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageTopShadowColor.as_ptr() as String_0,
            resource_class: WmCIconImageTopShadowColor.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1754 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Pixel>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageTopShadowColor)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageTSCDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNiconImageTopShadowPixmap.as_ptr() as String_0,
            resource_class: WmCIconImageTopShadowPixmap.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, iconImageTopShadowPStr)
                as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmIconImageTSPDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNmatteWidth.as_ptr() as String_0,
            resource_class: WmCMatteWidth.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1718 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, matteWidth) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        _XtResource {
            resource_name: WmNmaximumClientSize.as_ptr() as String_0,
            resource_class: WmCMaximumClientSize.as_ptr() as String_0,
            resource_type: WmRSize.as_ptr() as String_0,
            resource_size: ::core::mem::size_of::<WHSize>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, maximumClientSize) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1797 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: b"0x0\0" as *const u8 as *const ::core::ffi::c_char as XtPointer,
        },
        _XtResource {
            resource_name: WmNsecondariesOnTop.as_ptr() as String_0,
            resource_class: WmCSecondariesOnTop.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, secondariesOnTop) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1578 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                _WmSecondariesOnTopDefault
                    as unsafe extern "C" fn(Widget, ::core::ffi::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: WmNuseClientIcon.as_ptr() as String_0,
            resource_class: WmCUseClientIcon.as_ptr() as String_0,
            resource_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1561 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: ::core::mem::offset_of!(ClientData, useClientIcon) as Cardinal,
            default_type: (&raw const XtStrings as *const ::core::ffi::c_char)
                .offset(1695 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            default_addr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
    ];
    wmWsPresenceResources = [_XtResource {
        resource_name: WmNtitle.as_ptr() as String_0,
        resource_class: WmCTitle.as_ptr() as String_0,
        resource_type: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(12263 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        resource_size: ::core::mem::size_of::<XmString>() as Cardinal,
        resource_offset: ::core::mem::offset_of!(WsPresenceData, title) as Cardinal,
        default_type: (&raw mut _XmStrings as *mut ::core::ffi::c_char)
            .offset(12263 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        default_addr: NULL,
    }];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
