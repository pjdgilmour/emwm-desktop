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
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn XCreateGC(_: *mut Display, _: Drawable, _: ::core::ffi::c_ulong, _: *mut XGCValues) -> GC;
    fn XCreatePixmap(
        _: *mut Display,
        _: Drawable,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> Pixmap;
    fn XChangeGC(
        _: *mut Display,
        _: GC,
        _: ::core::ffi::c_ulong,
        _: *mut XGCValues,
    ) -> ::core::ffi::c_int;
    fn XCopyArea(
        _: *mut Display,
        _: Drawable,
        _: Drawable,
        _: GC,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XCopyPlane(
        _: *mut Display,
        _: Drawable,
        _: Drawable,
        _: GC,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    fn XFillRectangle(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XFillRectangles(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XRectangle,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XFreeGC(_: *mut Display, _: GC) -> ::core::ffi::c_int;
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
    fn XReadBitmapFile(
        _: *mut Display,
        _: Drawable,
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
        _: *mut Pixmap,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XSetForeground(_: *mut Display, _: GC, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
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
    fn XtGetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtRealloc(_: *mut ::core::ffi::c_char, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    fn XtFindFile(
        _: *const ::core::ffi::c_char,
        _: Substitution,
        _: Cardinal,
        _: XtFilePredicate,
    ) -> *mut ::core::ffi::c_char;
    fn XtResolvePathname(
        _: *mut Display,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: Substitution,
        _: Cardinal,
        _: XtFilePredicate,
    ) -> *mut ::core::ffi::c_char;
    static XtStrings: [::core::ffi::c_char; 0];
    fn XmGetPixmapByDepth(
        screen: *mut Screen,
        image_name: *mut ::core::ffi::c_char,
        foreground: Pixel,
        background: Pixel,
        depth: ::core::ffi::c_int,
    ) -> Pixmap;
    fn XmeGetPixmapData(
        screen: *mut Screen,
        pixmap: Pixmap,
        image_name: *mut *mut ::core::ffi::c_char,
        depth: *mut ::core::ffi::c_int,
        foreground: *mut Pixel,
        background: *mut Pixel,
        hot_x: *mut ::core::ffi::c_int,
        hot_y: *mut ::core::ffi::c_int,
        width: *mut ::core::ffi::c_uint,
        height: *mut ::core::ffi::c_uint,
    ) -> Boolean;
    fn XmeGetHomeDirName() -> String_0;
    static mut wmGD: WmGlobalData;
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
    fn GetHighlightGC(pSD: *mut WmScreenData, fg: Pixel, bg: Pixel, pixmap: Pixmap) -> GC;
    fn MWarning(format: *mut ::core::ffi::c_char, message: *mut ::core::ffi::c_char);
    fn Warning(message: *mut ::core::ffi::c_char);
    fn _XmOSInitPath(_: String_0, _: String_0, _: *mut Boolean) -> String_0;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const GCForeground: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const GCBackground: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const GCGraphicsExposures: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int;
pub const GCClipXOrigin: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 17 as ::core::ffi::c_int;
pub const GCClipYOrigin: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 18 as ::core::ffi::c_int;
pub const GCClipMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 19 as ::core::ffi::c_int;
pub const BitmapSuccess: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XmUNSPECIFIED_PIXMAP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICON_INTERNAL_SHADOW_WIDTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LABEL_PIXMAP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAXWMPATH: ::core::ffi::c_int = 1023 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MATCH_CHAR: ::core::ffi::c_int = 'P' as i32;
pub const MATCH_XBM: ::core::ffi::c_int = 'B' as i32;
pub const MATCH_PATH: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"XBMLANGPATH\0") };
#[no_mangle]
pub unsafe extern "C" fn MakeClientIconPixmap(
    mut pCD: *mut ClientData,
    mut iconBitmap: Pixmap,
    mut iconMask: Pixmap,
) -> Pixmap {
    let mut bitmapWidth: ::core::ffi::c_uint = 0;
    let mut bitmapHeight: ::core::ffi::c_uint = 0;
    let mut depth: ::core::ffi::c_uint = 0;
    if GetPixmapInfo(
        iconBitmap,
        &raw mut bitmapWidth,
        &raw mut bitmapHeight,
        &raw mut depth,
    ) == 0
    {
        Warning(
            b"Invalid icon pixmap\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return NULL as Pixmap;
    }
    return MakeIconPixmap(pCD, iconBitmap, iconMask, bitmapWidth, bitmapHeight, depth);
}
#[no_mangle]
pub unsafe extern "C" fn GetNamedPixmap(
    mut scr: *mut Screen,
    mut iconName: String_0,
    mut fg: Pixel,
    mut bg: Pixel,
    mut depth: ::core::ffi::c_int,
) -> Pixmap {
    let mut pixmap = XmUNSPECIFIED_PIXMAP as Pixmap;
    if !iconName.is_null() {
        if *iconName.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '/' as i32
            && !wmGD.bitmapDirectory.is_null()
        {
            let mut fullPathName = ::core::ptr::null_mut::<::core::ffi::c_char>();
            fullPathName = XtMalloc(
                strlen(wmGD.bitmapDirectory as *const ::core::ffi::c_char)
                    .wrapping_add(strlen(iconName as *const ::core::ffi::c_char))
                    .wrapping_add(2 as size_t) as Cardinal,
            );
            sprintf(
                fullPathName,
                b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
                wmGD.bitmapDirectory,
                iconName,
            );
            pixmap = XmGetPixmapByDepth(scr, fullPathName, fg, bg, depth);
            XtFree(fullPathName);
        }
        if !(pixmap != XmUNSPECIFIED_PIXMAP as Pixmap
            && pixmap != None_0 as Pixmap
            && pixmap != NULL as Pixmap)
        {
            pixmap = XmGetPixmapByDepth(scr, iconName as *mut ::core::ffi::c_char, fg, bg, depth);
        }
    }
    return pixmap;
}
#[no_mangle]
pub unsafe extern "C" fn MakeNamedIconPixmap(
    mut pCD: *mut ClientData,
    mut iconName: String_0,
) -> Pixmap {
    let mut pixmap: Pixmap = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    if !iconName.is_null() {
        pixmap = GetNamedPixmap(
            (*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(*pCD).pSD).screen as isize) as *mut Screen,
            iconName,
            (*pCD).iconImageForeground,
            (*pCD).iconImageBackground,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(*pCD).pSD).screen as isize))
            .root_depth,
        );
        if !(pixmap != XmUNSPECIFIED_PIXMAP as Pixmap
            && pixmap != None_0 as Pixmap
            && pixmap != NULL as Pixmap)
        {
            Warning(
                b"Invalid icon pixmap\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else {
            let mut depth = 0 as ::core::ffi::c_int;
            XmeGetPixmapData(
                (*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset((*(*pCD).pSD).screen as isize) as *mut Screen,
                pixmap,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut depth,
                ::core::ptr::null_mut::<Pixel>(),
                ::core::ptr::null_mut::<Pixel>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                &raw mut width,
                &raw mut height,
            );
            pixmap = MakeIconPixmap(
                pCD,
                pixmap,
                None_0 as Pixmap,
                width,
                height,
                depth as ::core::ffi::c_uint,
            );
        }
    }
    if iconName.is_null()
        || !(pixmap != XmUNSPECIFIED_PIXMAP as Pixmap
            && pixmap != None_0 as Pixmap
            && pixmap != NULL as Pixmap)
    {
        let mut depth_0 = 0 as ::core::ffi::c_uint;
        GetPixmapInfo(
            (*(*pCD).pSD).builtinIconPixmap,
            &raw mut width,
            &raw mut height,
            &raw mut depth_0,
        );
        pixmap = MakeIconPixmap(
            pCD,
            (*(*pCD).pSD).builtinIconPixmap,
            None_0 as Pixmap,
            width,
            height,
            depth_0,
        );
    }
    return pixmap;
}
#[no_mangle]
pub unsafe extern "C" fn MakeIconPixmap(
    mut pCD: *mut ClientData,
    mut bitmap: Pixmap,
    mut mask: Pixmap,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
    mut depth: ::core::ffi::c_uint,
) -> Pixmap {
    let mut iconPixmap: Pixmap = 0;
    let mut imageGC = ::core::ptr::null_mut::<_XGC>();
    let mut topGC = ::core::ptr::null_mut::<_XGC>();
    let mut botGC = ::core::ptr::null_mut::<_XGC>();
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
    let mut gc_mask: ::core::ffi::c_ulong = 0;
    let mut pPS = ::core::ptr::null_mut::<XmPixelSet>();
    let mut imageWidth: ::core::ffi::c_uint = 0;
    let mut imageHeight: ::core::ffi::c_uint = 0;
    let mut dest_x: ::core::ffi::c_int = 0;
    let mut dest_y: ::core::ffi::c_int = 0;
    let mut src_x: ::core::ffi::c_int = 0;
    let mut src_y: ::core::ffi::c_int = 0;
    let mut fg: Pixel = 0;
    let mut bg: Pixel = 0;
    static mut top_rects: *mut RList = ::core::ptr::null::<RList>() as *mut RList;
    static mut bot_rects: *mut RList = ::core::ptr::null::<RList>() as *mut RList;
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    if top_rects.is_null() && {
        top_rects = AllocateRList(
            (2 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint),
        );
        top_rects.is_null()
    } {
        Warning(
            b"Insufficient memory to bevel icon image\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return NULL as Pixmap;
    }
    if bot_rects.is_null() && {
        bot_rects = AllocateRList(
            (2 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint),
        );
        bot_rects.is_null()
    } {
        Warning(
            b"Insufficient memory to bevel icon image\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return NULL as Pixmap;
    }
    if !pCD.is_null() {
        pSD = (*pCD).pSD;
    } else {
        pSD = wmGD.pActiveSD;
    }
    if width < (*pSD).iconImageMinimum.width as ::core::ffi::c_uint
        || height < (*pSD).iconImageMinimum.height as ::core::ffi::c_uint
    {
        return NULL as Pixmap;
    }
    if width > (*pSD).iconImageMaximum.width as ::core::ffi::c_uint {
        src_x = width
            .wrapping_sub((*pSD).iconImageMaximum.width as ::core::ffi::c_uint)
            .wrapping_div(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    } else {
        src_x = 0 as ::core::ffi::c_int;
    }
    if height > (*pSD).iconImageMaximum.height as ::core::ffi::c_uint {
        src_y = height
            .wrapping_sub((*pSD).iconImageMaximum.height as ::core::ffi::c_uint)
            .wrapping_div(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    } else {
        src_y = 0 as ::core::ffi::c_int;
    }
    imageWidth = ((*pSD).iconImageMaximum.width
        + 2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH
        + (if wmGD.frameStyle as ::core::ffi::c_uint
            == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })) as ::core::ffi::c_uint;
    imageHeight = ((*pSD).iconImageMaximum.height
        + 2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH
        + (if wmGD.frameStyle as ::core::ffi::c_uint
            == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })) as ::core::ffi::c_uint;
    iconPixmap = XCreatePixmap(
        wmGD.display,
        (*pSD).rootWindow as Drawable,
        imageWidth,
        imageHeight,
        (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*pSD).screen as isize))
        .root_depth as ::core::ffi::c_uint,
    );
    if !pCD.is_null() {
        bg = (*pCD).iconImageBackground;
        fg = (*pCD).iconImageForeground;
    } else {
        bg = (*pSD).iconAppearance.background;
        fg = (*pSD).iconAppearance.foreground;
    }
    gc_mask = (GCForeground | GCBackground | GCGraphicsExposures) as ::core::ffi::c_ulong;
    if mask != 0 {
        if !(*pSD).pPrimaryPixelSet.is_null() {
            pPS = (*pSD).pPrimaryPixelSet;
            gcv.background = (*pPS).bg as ::core::ffi::c_ulong;
            gcv.foreground = (*pPS).bg as ::core::ffi::c_ulong;
        } else {
            gcv.background = (*(*pCD).pSD).iconAppearance.background as ::core::ffi::c_ulong;
            gcv.foreground = (*(*pCD).pSD).iconAppearance.background as ::core::ffi::c_ulong;
        }
    } else {
        gcv.foreground = bg as ::core::ffi::c_ulong;
        gcv.background = bg as ::core::ffi::c_ulong;
    }
    gcv.graphics_exposures = False;
    imageGC = XCreateGC(wmGD.display, iconPixmap as Drawable, gc_mask, &raw mut gcv);
    XFillRectangle(
        wmGD.display,
        iconPixmap as Drawable,
        imageGC,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        imageWidth,
        imageHeight,
    );
    if width > (*pSD).iconImageMaximum.width as ::core::ffi::c_uint {
        width = (*pSD).iconImageMaximum.width as ::core::ffi::c_uint;
    }
    if height > (*pSD).iconImageMaximum.height as ::core::ffi::c_uint {
        height = (*pSD).iconImageMaximum.height as ::core::ffi::c_uint;
    }
    dest_x = imageWidth
        .wrapping_sub(width)
        .wrapping_div(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    dest_y = imageHeight
        .wrapping_sub(height)
        .wrapping_div(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    if mask != 0 {
        gcv.foreground = (if !pPS.is_null() {
            (*pPS).fg
        } else {
            (*(*pCD).pSD).iconAppearance.foreground
        }) as ::core::ffi::c_ulong;
    } else {
        gcv.foreground = fg as ::core::ffi::c_ulong;
    }
    gc_mask = GCForeground as ::core::ffi::c_ulong;
    if mask != 0 {
        gcv.clip_mask = mask;
        gcv.clip_x_origin = dest_x - src_x;
        gcv.clip_y_origin = dest_y - src_y;
        gc_mask |= (GCClipXOrigin | GCClipYOrigin | GCClipMask) as ::core::ffi::c_ulong;
    }
    XChangeGC(wmGD.display, imageGC, gc_mask, &raw mut gcv);
    if depth > 1 as ::core::ffi::c_uint
        && depth
            == (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .root_depth as ::core::ffi::c_uint
    {
        XCopyArea(
            wmGD.display,
            bitmap as Drawable,
            iconPixmap as Drawable,
            imageGC,
            src_x,
            src_y,
            width,
            height,
            dest_x,
            dest_y,
        );
    } else {
        XCopyPlane(
            wmGD.display,
            bitmap as Drawable,
            iconPixmap as Drawable,
            imageGC,
            src_x,
            src_y,
            width,
            height,
            dest_x,
            dest_y,
            1 as ::core::ffi::c_ulong,
        );
    }
    XFreeGC(wmGD.display, imageGC);
    if !pCD.is_null() {
        if mask != 0 && !pPS.is_null() {
            topGC = GetHighlightGC(pSD, (*pPS).ts, (*pPS).bg, (*pCD).iconImageTopShadowPixmap);
            botGC = GetHighlightGC(
                pSD,
                (*pPS).bs,
                (*pPS).bg,
                (*pCD).iconImageBottomShadowPixmap,
            );
        } else {
            topGC = GetHighlightGC(
                pSD,
                (*pCD).iconImageTopShadowColor,
                (*pCD).iconImageBackground,
                (*pCD).iconImageTopShadowPixmap,
            );
            botGC = GetHighlightGC(
                pSD,
                (*pCD).iconImageBottomShadowColor,
                (*pCD).iconImageBackground,
                (*pCD).iconImageBottomShadowPixmap,
            );
        }
        (*top_rects).used = 0 as ::core::ffi::c_int;
        (*bot_rects).used = 0 as ::core::ffi::c_int;
        BevelRectangle(
            top_rects,
            bot_rects,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            imageWidth,
            imageHeight,
            ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
            ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
            ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
            ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
        );
        XFillRectangles(
            wmGD.display,
            iconPixmap as Drawable,
            topGC,
            (*top_rects).prect,
            (*top_rects).used,
        );
        XFillRectangles(
            wmGD.display,
            iconPixmap as Drawable,
            botGC,
            (*bot_rects).prect,
            (*bot_rects).used,
        );
    }
    return iconPixmap;
}
#[no_mangle]
pub unsafe extern "C" fn MakeCachedLabelPixmap(
    mut pSD: *mut WmScreenData,
    mut menuW: Widget,
    mut bitmapIndex: ::core::ffi::c_int,
) -> Pixmap {
    let mut bitmapc = ::core::ptr::null_mut::<BitmapCache>();
    let mut pixmapc = ::core::ptr::null_mut::<PixmapCache>();
    let mut i: ::core::ffi::c_int = 0;
    let mut args: [Arg; 5] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 5];
    let mut fg: Pixel = 0;
    let mut bg: Pixel = 0;
    let mut pixmap = NULL as Pixmap;
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
    if bitmapIndex < 0 as ::core::ffi::c_int {
        return NULL as Pixmap;
    }
    bitmapc = (*pSD).bitmapCache.offset(bitmapIndex as isize) as *mut BitmapCache;
    i = 0 as ::core::ffi::c_int;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(214 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = &raw mut fg as XtArgVal;
    i += 1;
    args[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(52 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[i as usize].value = &raw mut bg as XtArgVal;
    i += 1;
    XtGetValues(menuW, &raw mut args as *mut Arg as ArgList, i as Cardinal);
    pixmapc = (*bitmapc).pixmapCache;
    while !pixmapc.is_null() {
        if (*pixmapc).pixmapType == LABEL_PIXMAP as ::core::ffi::c_uint
            && (*pixmapc).foreground == fg
            && (*pixmapc).background == bg
        {
            pixmap = (*pixmapc).pixmap;
            break;
        } else {
            pixmapc = (*pixmapc).next as *mut PixmapCache;
        }
    }
    if pixmap == 0 {
        pixmap = XCreatePixmap(
            wmGD.display,
            (*pSD).rootWindow as Drawable,
            (*bitmapc).width,
            (*bitmapc).height,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*pSD).screen as isize))
            .root_depth as ::core::ffi::c_uint,
        );
        gcv.foreground = bg as ::core::ffi::c_ulong;
        gcv.background = bg as ::core::ffi::c_ulong;
        gcv.graphics_exposures = False;
        gc = XCreateGC(
            wmGD.display,
            pixmap as Drawable,
            (GCForeground | GCBackground) as ::core::ffi::c_ulong,
            &raw mut gcv,
        );
        XFillRectangle(
            wmGD.display,
            pixmap as Drawable,
            gc,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*bitmapc).width,
            (*bitmapc).height,
        );
        XSetForeground(wmGD.display, gc, fg as ::core::ffi::c_ulong);
        XCopyPlane(
            wmGD.display,
            (*bitmapc).bitmap as Drawable,
            pixmap as Drawable,
            gc,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*bitmapc).width,
            (*bitmapc).height,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_ulong,
        );
        XFreeGC(wmGD.display, gc);
        pixmapc = XtMalloc(::core::mem::size_of::<PixmapCache>() as Cardinal) as *mut PixmapCache;
        if !pixmapc.is_null() {
            (*pixmapc).pixmapType = LABEL_PIXMAP as ::core::ffi::c_uint;
            (*pixmapc).foreground = fg;
            (*pixmapc).background = bg;
            (*pixmapc).pixmap = pixmap;
            (*pixmapc).next = (*bitmapc).pixmapCache as *mut _PixmapCache;
            (*bitmapc).pixmapCache = pixmapc;
        }
    }
    return pixmap;
}
pub const BITMAP_CACHE_INC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn GetBitmapIndex(
    mut pSD: *mut WmScreenData,
    mut name: *mut ::core::ffi::c_char,
    mut bReportError: Boolean,
) -> ::core::ffi::c_int {
    let mut path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bitmapc = ::core::ptr::null_mut::<BitmapCache>();
    let mut n: ::core::ffi::c_uint = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    path = BitmapPathName(name);
    n = 0 as ::core::ffi::c_uint;
    bitmapc = (*pSD).bitmapCache;
    while n < (*pSD).bitmapCacheCount {
        if path.is_null() && (*bitmapc).path.is_null()
            || !path.is_null() && !(*bitmapc).path.is_null() && strcmp(path, (*bitmapc).path) == 0
        {
            return n as ::core::ffi::c_int;
        }
        n = n.wrapping_add(1);
        bitmapc = bitmapc.offset(1);
    }
    if (*pSD).bitmapCacheSize == 0 as ::core::ffi::c_uint {
        (*pSD).bitmapCacheSize = BITMAP_CACHE_INC as ::core::ffi::c_uint;
        (*pSD).bitmapCache = XtMalloc(
            (BITMAP_CACHE_INC as usize).wrapping_mul(::core::mem::size_of::<BitmapCache>() as usize)
                as Cardinal,
        ) as *mut BitmapCache;
    } else if (*pSD).bitmapCacheCount == (*pSD).bitmapCacheSize {
        (*pSD).bitmapCacheSize = (*pSD)
            .bitmapCacheSize
            .wrapping_add(BITMAP_CACHE_INC as ::core::ffi::c_uint);
        (*pSD).bitmapCache = XtRealloc(
            (*pSD).bitmapCache as *mut ::core::ffi::c_char,
            ((*pSD).bitmapCacheSize as usize)
                .wrapping_mul(::core::mem::size_of::<BitmapCache>() as usize)
                as Cardinal,
        ) as *mut BitmapCache;
    }
    if (*pSD).bitmapCache.is_null() {
        MWarning(
            b"Insufficient memory for bitmap %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            name,
        );
        (*pSD).bitmapCacheSize = 0 as ::core::ffi::c_uint;
        (*pSD).bitmapCacheCount = 0 as ::core::ffi::c_uint;
        return -(1 as ::core::ffi::c_int);
    }
    bitmapc = (*pSD).bitmapCache.offset((*pSD).bitmapCacheCount as isize) as *mut BitmapCache;
    if !path.is_null() {
        (*bitmapc).path = XtMalloc(strlen(path).wrapping_add(1 as size_t) as Cardinal)
            as *mut ::core::ffi::c_char;
        if (*bitmapc).path.is_null() {
            MWarning(
                b"Insufficient memory for bitmap %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                name,
            );
            return -(1 as ::core::ffi::c_int);
        }
        strcpy((*bitmapc).path, path);
        if XReadBitmapFile(
            wmGD.display,
            (*pSD).rootWindow as Drawable,
            path,
            &raw mut (*bitmapc).width,
            &raw mut (*bitmapc).height,
            &raw mut (*bitmapc).bitmap,
            &raw mut x,
            &raw mut y,
        ) != BitmapSuccess
        {
            if bReportError != 0 {
                MWarning(
                    b"Unable to read bitmap file %s\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    path,
                );
            }
            XtFree((*bitmapc).path);
            return -(1 as ::core::ffi::c_int);
        }
        if (*bitmapc).width == 0 as ::core::ffi::c_uint
            || (*bitmapc).height == 0 as ::core::ffi::c_uint
        {
            if bReportError != 0 {
                MWarning(
                    b"Invalid bitmap file %s\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    path,
                );
            }
            XtFree((*bitmapc).path);
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        (*bitmapc).path = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*bitmapc).bitmap = (*pSD).builtinIconPixmap;
        GetPixmapInfo(
            (*pSD).builtinIconPixmap,
            &raw mut (*bitmapc).width,
            &raw mut (*bitmapc).height,
            ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        );
    }
    (*bitmapc).pixmapCache = ::core::ptr::null_mut::<PixmapCache>();
    let fresh0 = (*pSD).bitmapCacheCount;
    (*pSD).bitmapCacheCount = (*pSD).bitmapCacheCount.wrapping_add(1);
    return fresh0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BitmapPathName(
    mut string: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut fileName: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut retname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut subs: [SubstitutionRec; 1] = [SubstitutionRec {
        match_0: 0,
        substitution: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }; 1];
    let mut homeDir = XmeGetHomeDirName() as *mut ::core::ffi::c_char;
    if string.is_null() || *string == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if *string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '~' as i32
        && *string.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32
    {
        strcpy(&raw mut fileName as *mut ::core::ffi::c_char, homeDir);
        strncat(
            &raw mut fileName as *mut ::core::ffi::c_char,
            string.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
            (MAXWMPATH as size_t)
                .wrapping_sub(strlen(&raw mut fileName as *mut ::core::ffi::c_char)),
        );
        return &raw mut fileName as *mut ::core::ffi::c_char;
    }
    if *string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
        return string;
    }
    if !wmGD.bitmapDirectory.is_null() && *wmGD.bitmapDirectory as ::core::ffi::c_int != 0 {
        if *wmGD
            .bitmapDirectory
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '~' as i32
            && *wmGD
                .bitmapDirectory
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '/' as i32
        {
            strcpy(&raw mut fileName as *mut ::core::ffi::c_char, homeDir);
            strncat(
                &raw mut fileName as *mut ::core::ffi::c_char,
                wmGD.bitmapDirectory
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                (MAXWMPATH as size_t)
                    .wrapping_sub(strlen(&raw mut fileName as *mut ::core::ffi::c_char)),
            );
        } else {
            strcpy(
                &raw mut fileName as *mut ::core::ffi::c_char,
                wmGD.bitmapDirectory as *const ::core::ffi::c_char,
            );
        }
        strncat(
            &raw mut fileName as *mut ::core::ffi::c_char,
            b"/\0" as *const u8 as *const ::core::ffi::c_char,
            (MAXWMPATH as size_t)
                .wrapping_sub(strlen(&raw mut fileName as *mut ::core::ffi::c_char)),
        );
        strncat(
            &raw mut fileName as *mut ::core::ffi::c_char,
            string,
            (MAXWMPATH as size_t)
                .wrapping_sub(strlen(&raw mut fileName as *mut ::core::ffi::c_char)),
        );
        subs[0 as ::core::ffi::c_int as usize].substitution =
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        retname = XtFindFile(
            &raw mut fileName as *mut ::core::ffi::c_char,
            &raw mut subs as Substitution,
            0 as Cardinal,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, XtFilePredicate>(NULL),
        );
        if !retname.is_null() {
            XtFree(retname);
            return &raw mut fileName as *mut ::core::ffi::c_char;
        }
    }
    let mut search_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut user_path: Boolean = 0;
    search_path = _XmOSInitPath(
        string as String_0,
        MATCH_PATH.as_ptr() as String_0,
        &raw mut user_path,
    ) as *mut ::core::ffi::c_char;
    subs[0 as ::core::ffi::c_int as usize].match_0 = (if user_path as ::core::ffi::c_int != 0 {
        MATCH_XBM
    } else {
        MATCH_CHAR
    }) as ::core::ffi::c_char;
    subs[0 as ::core::ffi::c_int as usize].substitution = string;
    retname = XtResolvePathname(
        wmGD.display,
        b"bitmaps\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        search_path,
        &raw mut subs as Substitution,
        (::core::mem::size_of::<[SubstitutionRec; 1]>() as usize)
            .wrapping_div(::core::mem::size_of::<SubstitutionRec>() as usize) as Cardinal,
        ::core::mem::transmute::<*mut ::core::ffi::c_void, XtFilePredicate>(NULL),
    );
    XtFree(search_path);
    if retname.is_null() {
        return string;
    }
    strncpy(
        &raw mut fileName as *mut ::core::ffi::c_char,
        retname,
        MAXWMPATH as size_t,
    );
    XtFree(retname);
    return &raw mut fileName as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn GetPixmapInfo(
    mut pixmap: Pixmap,
    mut rwidth: *mut ::core::ffi::c_uint,
    mut rheight: *mut ::core::ffi::c_uint,
    mut rdepth: *mut ::core::ffi::c_uint,
) -> Boolean {
    let mut root: Window = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut border: ::core::ffi::c_uint = 0;
    let mut depth: ::core::ffi::c_uint = 0;
    if XGetGeometry(
        wmGD.display,
        pixmap as Drawable,
        &raw mut root,
        &raw mut x,
        &raw mut y,
        &raw mut width,
        &raw mut height,
        &raw mut border,
        &raw mut depth,
    ) == 0
    {
        return False as Boolean;
    }
    if !rwidth.is_null() {
        *rwidth = width;
    }
    if !rheight.is_null() {
        *rheight = height;
    }
    if !rdepth.is_null() {
        *rdepth = depth;
    }
    return True as Boolean;
}
