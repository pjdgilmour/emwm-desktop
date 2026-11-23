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
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_0;
use crate::abi::C2RustUnnamed_1_WmResCvt as C2RustUnnamed_1;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_2;
use crate::abi::*;
extern "C" {
    fn XtAppAddConverter(
        _: XtAppContext,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: XtConverter,
        _: XtConvertArgList,
        _: Cardinal,
    );
    static XtStrings: [::core::ffi::c_char; 0];
    static mut wmGD: WmGlobalData;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn mblen(__s: *const ::core::ffi::c_char, __n: size_t) -> ::core::ffi::c_int;
    fn mbtowc(
        __pwc: *mut wchar_t,
        __s: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn ScanWhitespace(linePP: *mut *mut ::core::ffi::c_uchar);
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const XtProcedureArg: XtAddressMode = 6;
pub const XtWidgetBaseOffset: XtAddressMode = 5;
pub const XtResourceQuark: XtAddressMode = 4;
pub const XtResourceString: XtAddressMode = 3;
pub const XtImmediate: XtAddressMode = 2;
pub const XtBaseOffset: XtAddressMode = 1;
pub const XtAddress: XtAddressMode = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
pub const WM_FUNC_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WM_FUNC_ALL: ::core::ffi::c_long =
    MWM_FUNC_RESIZE | MWM_FUNC_MOVE | MWM_FUNC_MINIMIZE | MWM_FUNC_MAXIMIZE | MWM_FUNC_CLOSE;
pub const WM_DECOR_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const WM_SHOW_FB_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WM_SHOW_FB_DEFAULT: ::core::ffi::c_long = WM_SHOW_FB_ALL;
pub const AMAP_BEHAVIOR_ADD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMAP_BEHAVIOR_MOVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AMAP_BEHAVIOR_IGNORE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CMAP_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CMAP_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CMAP_FOCUS_KEYBOARD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XRS_FOCUS_POINTER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XRS_FOCUS_KEYBOARD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XRS_FOCUS_PRIMARY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICON_LABEL_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const ICON_IMAGE_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const ICON_ACTIVE_LABEL_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ICON_PLACE_LEFT_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const ICON_PLACE_RIGHT_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const ICON_PLACE_TOP_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ICON_PLACE_BOTTOM_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const ICON_PLACE_LEFT_SECONDARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const ICON_PLACE_RIGHT_SECONDARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const ICON_PLACE_TOP_SECONDARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const ICON_PLACE_BOTTOM_SECONDARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
pub const ICON_PLACE_TIGHT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const USE_PPOSITION_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const USE_PPOSITION_ON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const USE_PPOSITION_NONZERO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BIGSIZE: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn AddWmResourceConverters() {
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRAbsentMapBehavior.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToAMBehavior
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRCFocusPolicy.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToCFocus
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRClientDecor.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToCDecor
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRClientFunction.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToCFunc
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRFrameStyle.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToFrameStyle
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRIconDecor.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToIDecor
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRIconPlacement.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToIPlace
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRKFocusPolicy.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToKFocus
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRSize.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToSize
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRShowFeedback.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToShowFeedback
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRUsePPosition.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToUsePPosition
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
    XtAppAddConverter(
        wmGD.mwmAppContext,
        (&raw const XtStrings as *const ::core::ffi::c_char)
            .offset(1797 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        WmRXineramaScreenFocus.as_ptr(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut XrmValue, Cardinal, *mut XrmValue, *mut XrmValue) -> (),
            >,
            XtConverter,
        >(Some(
            WmCvtStringToXRSFocus
                as unsafe extern "C" fn(
                    *mut XrmValue,
                    Cardinal,
                    *mut XrmValue,
                    *mut XrmValue,
                ) -> (),
        )),
        ::core::ptr::null_mut::<C2RustUnnamed_1>(),
        0 as Cardinal,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToAMBehavior(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    static mut cval: ::core::ffi::c_int = 0;
    let mut fHit = False as Boolean;
    if *pch as ::core::ffi::c_int != 0 && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
    {
        if *pch as ::core::ffi::c_int == 'A' as i32 || *pch as ::core::ffi::c_int == 'a' as i32 {
            if StringsAreEqual(pch, AMAP_BEHAVIOR_ADD_STR, len) != 0 {
                cval = AMAP_BEHAVIOR_ADD;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'M' as i32
            || *pch as ::core::ffi::c_int == 'm' as i32
        {
            if StringsAreEqual(pch, AMAP_BEHAVIOR_MOVE_STR, len) != 0 {
                cval = AMAP_BEHAVIOR_MOVE;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'I' as i32
            || *pch as ::core::ffi::c_int == 'i' as i32
        {
            if StringsAreEqual(pch, AMAP_BEHAVIOR_IGNORE_STR, len) != 0 {
                cval = AMAP_BEHAVIOR_IGNORE;
                fHit = True as Boolean;
            }
        }
    }
    if fHit == 0 {
        cval = AMAP_BEHAVIOR_ADD;
    }
    (*toVal).size = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut cval as XtPointer as XPointer;
}
pub const AMAP_BEHAVIOR_ADD_STR: *mut ::core::ffi::c_uchar =
    b"add\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const AMAP_BEHAVIOR_MOVE_STR: *mut ::core::ffi::c_uchar =
    b"move\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const AMAP_BEHAVIOR_IGNORE_STR: *mut ::core::ffi::c_uchar =
    b"ignore\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToCFocus(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    static mut cval: ::core::ffi::c_int = 0;
    let mut fHit = False as Boolean;
    if *pch as ::core::ffi::c_int != 0 && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
    {
        if *pch as ::core::ffi::c_int == 'E' as i32 || *pch as ::core::ffi::c_int == 'e' as i32 {
            if StringsAreEqual(pch, CMAP_FOCUS_EXPLICIT_STR, len) != 0 {
                cval = CMAP_FOCUS_EXPLICIT;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'K' as i32
            || *pch as ::core::ffi::c_int == 'k' as i32
        {
            if StringsAreEqual(pch, CMAP_FOCUS_KEYBOARD_STR, len) != 0 {
                cval = CMAP_FOCUS_KEYBOARD;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'P' as i32
            || *pch as ::core::ffi::c_int == 'p' as i32
        {
            if StringsAreEqual(pch, CMAP_FOCUS_POINTER_STR, len) != 0 {
                cval = CMAP_FOCUS_POINTER;
                fHit = True as Boolean;
            }
        }
    }
    if fHit == 0 {
        cval = CMAP_FOCUS_KEYBOARD;
    }
    (*toVal).size = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut cval as XtPointer as XPointer;
}
pub const CMAP_FOCUS_EXPLICIT_STR: *mut ::core::ffi::c_uchar =
    b"explicit\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const CMAP_FOCUS_KEYBOARD_STR: *mut ::core::ffi::c_uchar =
    b"keyboard\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const CMAP_FOCUS_POINTER_STR: *mut ::core::ffi::c_uchar =
    b"pointer\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToCDecor(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    static mut cval: ::core::ffi::c_int = 0;
    let mut fHit = False as Boolean;
    let mut fAddNext = True as Boolean;
    if *pch as ::core::ffi::c_int != 0
        && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
        && *pch as ::core::ffi::c_int == '-' as i32
    {
        cval = WM_DECOR_ALL as ::core::ffi::c_int;
        fHit = True as Boolean;
    } else {
        cval = WM_DECOR_NONE;
    }
    while *pch as ::core::ffi::c_int != 0
        && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
    {
        if *pch as ::core::ffi::c_int == '+' as i32 {
            if len != 1 as ::core::ffi::c_int {
                pchNext = pch.offset(1 as ::core::ffi::c_int as isize);
            }
            fAddNext = TRUE as Boolean;
        } else if *pch as ::core::ffi::c_int == '-' as i32 {
            if len != 1 as ::core::ffi::c_int {
                pchNext = pch.offset(1 as ::core::ffi::c_int as isize);
            }
            fAddNext = FALSE as Boolean;
        } else if *pch as ::core::ffi::c_int == 'A' as i32
            || *pch as ::core::ffi::c_int == 'a' as i32
        {
            if StringsAreEqual(pch, WM_DECOR_ALL_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_DECOR_ALL
                } else {
                    cval as ::core::ffi::c_long & !WM_DECOR_ALL
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'N' as i32
            || *pch as ::core::ffi::c_int == 'n' as i32
        {
            if StringsAreEqual(pch, WM_DECOR_NONE_STR, len) != 0 {
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'T' as i32
            || *pch as ::core::ffi::c_int == 't' as i32
        {
            if StringsAreEqual(pch, WM_DECOR_TITLE_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_DECOR_TITLE
                } else {
                    cval as ::core::ffi::c_long & !WM_DECOR_TITLEBAR
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            } else if StringsAreEqual(pch, WM_DECOR_TITLEBAR_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_DECOR_TITLEBAR
                } else {
                    cval as ::core::ffi::c_long & !WM_DECOR_TITLEBAR
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'M' as i32
            || *pch as ::core::ffi::c_int == 'm' as i32
        {
            if StringsAreEqual(pch, WM_DECOR_MINIMIZE_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_DECOR_MINIMIZE
                } else {
                    cval as ::core::ffi::c_long & !MWM_DECOR_MINIMIZE
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            } else if StringsAreEqual(pch, WM_DECOR_MAXIMIZE_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_DECOR_MAXIMIZE
                } else {
                    cval as ::core::ffi::c_long & !MWM_DECOR_MAXIMIZE
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            } else if StringsAreEqual(pch, WM_DECOR_MENU_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_DECOR_SYSTEM
                } else {
                    cval as ::core::ffi::c_long & !MWM_DECOR_MENU
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'R' as i32
            || *pch as ::core::ffi::c_int == 'r' as i32
        {
            if StringsAreEqual(pch, WM_DECOR_RESIZE_STR, len) as ::core::ffi::c_int != 0
                || StringsAreEqual(pch, WM_DECOR_RESIZEH_STR, len) as ::core::ffi::c_int != 0
            {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_DECOR_RESIZEH
                } else {
                    cval as ::core::ffi::c_long & !MWM_DECOR_RESIZEH
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'B' as i32
            || *pch as ::core::ffi::c_int == 'b' as i32
        {
            if StringsAreEqual(pch, WM_DECOR_BORDER_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_DECOR_BORDER
                } else {
                    cval as ::core::ffi::c_long & !WM_DECOR_BORDER
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        }
        pch = pchNext;
    }
    if fHit == 0 {
        cval = WM_DECOR_ALL as ::core::ffi::c_int;
    }
    (*toVal).size = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut cval as XtPointer as XPointer;
}
pub const WM_DECOR_ALL_STR: *mut ::core::ffi::c_uchar =
    b"all\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_DECOR_NONE_STR: *mut ::core::ffi::c_uchar =
    b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_DECOR_BORDER_STR: *mut ::core::ffi::c_uchar =
    b"border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_DECOR_RESIZEH_STR: *mut ::core::ffi::c_uchar =
    b"resizeh\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_DECOR_TITLE_STR: *mut ::core::ffi::c_uchar =
    b"title\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_DECOR_TITLEBAR_STR: *mut ::core::ffi::c_uchar =
    b"titlebar\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_DECOR_MINIMIZE_STR: *mut ::core::ffi::c_uchar =
    b"minimize\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_DECOR_MAXIMIZE_STR: *mut ::core::ffi::c_uchar =
    b"maximize\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_DECOR_MENU_STR: *mut ::core::ffi::c_uchar =
    b"menu\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_DECOR_RESIZE_STR: *mut ::core::ffi::c_uchar =
    b"resize\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToCFunc(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    static mut cval: ::core::ffi::c_int = 0;
    let mut fHit = False as Boolean;
    let mut fAddNext = True as Boolean;
    if *pch as ::core::ffi::c_int != 0
        && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
        && *pch as ::core::ffi::c_int == '-' as i32
    {
        cval = WM_FUNC_ALL as ::core::ffi::c_int;
        fHit = True as Boolean;
    } else {
        cval = WM_FUNC_NONE;
    }
    while *pch as ::core::ffi::c_int != 0
        && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
    {
        if *pch as ::core::ffi::c_int == '+' as i32 {
            if len != 1 as ::core::ffi::c_int {
                pchNext = pch.offset(1 as ::core::ffi::c_int as isize);
            }
            fAddNext = TRUE as Boolean;
        } else if *pch as ::core::ffi::c_int == '-' as i32 {
            if len != 1 as ::core::ffi::c_int {
                pchNext = pch.offset(1 as ::core::ffi::c_int as isize);
            }
            fAddNext = FALSE as Boolean;
        } else if *pch as ::core::ffi::c_int == 'A' as i32
            || *pch as ::core::ffi::c_int == 'a' as i32
        {
            if StringsAreEqual(pch, WM_FUNC_ALL_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_FUNC_ALL
                } else {
                    cval as ::core::ffi::c_long & !WM_FUNC_ALL
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'N' as i32
            || *pch as ::core::ffi::c_int == 'n' as i32
        {
            if StringsAreEqual(pch, WM_FUNC_NONE_STR, len) != 0 {
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'R' as i32
            || *pch as ::core::ffi::c_int == 'r' as i32
        {
            if StringsAreEqual(pch, WM_FUNC_RESIZE_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | MWM_FUNC_RESIZE
                } else {
                    cval as ::core::ffi::c_long & !MWM_FUNC_RESIZE
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'M' as i32
            || *pch as ::core::ffi::c_int == 'm' as i32
        {
            if StringsAreEqual(pch, WM_FUNC_MINIMIZE_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | MWM_FUNC_MINIMIZE
                } else {
                    cval as ::core::ffi::c_long & !MWM_FUNC_MINIMIZE
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            } else if StringsAreEqual(pch, WM_FUNC_MAXIMIZE_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | MWM_FUNC_MAXIMIZE
                } else {
                    cval as ::core::ffi::c_long & !MWM_FUNC_MAXIMIZE
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            } else if StringsAreEqual(pch, WM_FUNC_MOVE_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | MWM_FUNC_MOVE
                } else {
                    cval as ::core::ffi::c_long & !MWM_FUNC_MOVE
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'C' as i32
            || *pch as ::core::ffi::c_int == 'c' as i32
        {
            if StringsAreEqual(pch, WM_FUNC_CLOSE_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | MWM_FUNC_CLOSE
                } else {
                    cval as ::core::ffi::c_long & !MWM_FUNC_CLOSE
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        }
        pch = pchNext;
    }
    if fHit == 0 {
        cval = WM_FUNC_ALL as ::core::ffi::c_int;
    }
    (*toVal).size = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut cval as XtPointer as XPointer;
}
pub const WM_FUNC_ALL_STR: *mut ::core::ffi::c_uchar =
    b"all\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_FUNC_NONE_STR: *mut ::core::ffi::c_uchar =
    b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_FUNC_RESIZE_STR: *mut ::core::ffi::c_uchar =
    b"resize\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_FUNC_MOVE_STR: *mut ::core::ffi::c_uchar =
    b"move\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_FUNC_MINIMIZE_STR: *mut ::core::ffi::c_uchar =
    b"minimize\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_FUNC_MAXIMIZE_STR: *mut ::core::ffi::c_uchar =
    b"maximize\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const WM_FUNC_CLOSE_STR: *mut ::core::ffi::c_uchar =
    b"close\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToFrameStyle(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    static mut frameStyle: FrameStyle = WmRECESSED;
    let mut fHit = False as Boolean;
    if *pch as ::core::ffi::c_int != 0 && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
    {
        if *pch as ::core::ffi::c_int == 'R' as i32 || *pch as ::core::ffi::c_int == 'r' as i32 {
            if StringsAreEqual(pch, FRAME_STYLE_RECESSED_STR, len) != 0 {
                frameStyle = WmRECESSED;
                fHit = True as Boolean;
            }
        } else if StringsAreEqual(pch, FRAME_STYLE_SLAB_STR, len) != 0 {
            frameStyle = WmSLAB;
            fHit = True as Boolean;
        }
    }
    if fHit == 0 {
        frameStyle = WmRECESSED;
    }
    (*toVal).size = ::core::mem::size_of::<FrameStyle>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut frameStyle as XtPointer as XPointer;
}
pub const FRAME_STYLE_RECESSED_STR: *mut ::core::ffi::c_uchar =
    b"recessed\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const FRAME_STYLE_SLAB_STR: *mut ::core::ffi::c_uchar =
    b"slab\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToIDecor(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    static mut cval: ::core::ffi::c_int = 0;
    let mut fHit = False as Boolean;
    cval = 0 as ::core::ffi::c_int;
    while *pch as ::core::ffi::c_int != 0
        && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
    {
        if *pch as ::core::ffi::c_int == 'A' as i32 || *pch as ::core::ffi::c_int == 'a' as i32 {
            if StringsAreEqual(pch, ICON_DECOR_ACTIVE_LABEL_STR, len) != 0 {
                cval = (cval as ::core::ffi::c_long | ICON_ACTIVE_LABEL_PART) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'I' as i32
            || *pch as ::core::ffi::c_int == 'i' as i32
        {
            if StringsAreEqual(pch, ICON_DECOR_IMAGE_STR, len) != 0 {
                cval = (cval as ::core::ffi::c_long | ICON_IMAGE_PART) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'L' as i32
            || *pch as ::core::ffi::c_int == 'l' as i32
        {
            if StringsAreEqual(pch, ICON_DECOR_LABEL_STR, len) != 0 {
                cval = (cval as ::core::ffi::c_long | ICON_LABEL_PART) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        }
        pch = pchNext;
    }
    if fHit == 0 || cval as ::core::ffi::c_long == ICON_ACTIVE_LABEL_PART {
        cval = (ICON_LABEL_PART | ICON_IMAGE_PART | ICON_ACTIVE_LABEL_PART) as ::core::ffi::c_int;
    }
    (*toVal).size = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut cval as XtPointer as XPointer;
}
pub const ICON_DECOR_IMAGE_STR: *mut ::core::ffi::c_uchar =
    b"image\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const ICON_DECOR_LABEL_STR: *mut ::core::ffi::c_uchar =
    b"label\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const ICON_DECOR_ACTIVE_LABEL_STR: *mut ::core::ffi::c_uchar =
    b"activelabel\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToIPlace(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    static mut cval: ::core::ffi::c_int = 0;
    let mut fPrimarySet = False as Boolean;
    let mut fSecondarySet = False as Boolean;
    cval = 0 as ::core::ffi::c_int;
    while *pch as ::core::ffi::c_int != 0
        && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
    {
        if *pch as ::core::ffi::c_int == 'B' as i32 || *pch as ::core::ffi::c_int == 'b' as i32 {
            if StringsAreEqual(pch, ICON_PLACE_BOTTOM_STR, len) != 0 {
                if fPrimarySet == 0 {
                    cval = (cval as ::core::ffi::c_long | ICON_PLACE_BOTTOM_PRIMARY)
                        as ::core::ffi::c_int;
                    fPrimarySet = True as Boolean;
                } else if fSecondarySet == 0 {
                    if cval as ::core::ffi::c_long
                        & (ICON_PLACE_BOTTOM_PRIMARY | ICON_PLACE_TOP_PRIMARY)
                        == 0
                    {
                        cval = (cval as ::core::ffi::c_long | ICON_PLACE_BOTTOM_SECONDARY)
                            as ::core::ffi::c_int;
                        fSecondarySet = True as Boolean;
                    }
                }
            }
        } else if *pch as ::core::ffi::c_int == 'L' as i32
            || *pch as ::core::ffi::c_int == 'l' as i32
        {
            if StringsAreEqual(pch, ICON_PLACE_LEFT_STR, len) != 0 {
                if fPrimarySet == 0 {
                    cval = (cval as ::core::ffi::c_long | ICON_PLACE_LEFT_PRIMARY)
                        as ::core::ffi::c_int;
                    fPrimarySet = True as Boolean;
                } else if fSecondarySet == 0 {
                    if cval as ::core::ffi::c_long
                        & (ICON_PLACE_LEFT_PRIMARY | ICON_PLACE_RIGHT_PRIMARY)
                        == 0
                    {
                        cval = (cval as ::core::ffi::c_long | ICON_PLACE_LEFT_SECONDARY)
                            as ::core::ffi::c_int;
                        fSecondarySet = True as Boolean;
                    }
                }
            }
        } else if *pch as ::core::ffi::c_int == 'R' as i32
            || *pch as ::core::ffi::c_int == 'r' as i32
        {
            if StringsAreEqual(pch, ICON_PLACE_RIGHT_STR, len) != 0 {
                if fPrimarySet == 0 {
                    cval = (cval as ::core::ffi::c_long | ICON_PLACE_RIGHT_PRIMARY)
                        as ::core::ffi::c_int;
                    fPrimarySet = True as Boolean;
                } else if fSecondarySet == 0 {
                    if cval as ::core::ffi::c_long
                        & (ICON_PLACE_RIGHT_PRIMARY | ICON_PLACE_LEFT_PRIMARY)
                        == 0
                    {
                        cval = (cval as ::core::ffi::c_long | ICON_PLACE_RIGHT_SECONDARY)
                            as ::core::ffi::c_int;
                        fSecondarySet = True as Boolean;
                    }
                }
            }
        } else if *pch as ::core::ffi::c_int == 'T' as i32
            || *pch as ::core::ffi::c_int == 't' as i32
        {
            if StringsAreEqual(pch, ICON_PLACE_TOP_STR, len) != 0 {
                if fPrimarySet == 0 {
                    cval = (cval as ::core::ffi::c_long | ICON_PLACE_TOP_PRIMARY)
                        as ::core::ffi::c_int;
                    fPrimarySet = True as Boolean;
                } else if fSecondarySet == 0 {
                    if cval as ::core::ffi::c_long
                        & (ICON_PLACE_TOP_PRIMARY | ICON_PLACE_BOTTOM_PRIMARY)
                        == 0
                    {
                        cval = (cval as ::core::ffi::c_long | ICON_PLACE_TOP_SECONDARY)
                            as ::core::ffi::c_int;
                        fSecondarySet = True as Boolean;
                    }
                }
            } else if StringsAreEqual(pch, ICON_PLACE_TIGHT_STR, len) != 0 {
                cval = (cval as ::core::ffi::c_long | ICON_PLACE_TIGHT) as ::core::ffi::c_int;
            }
        }
        pch = pchNext;
    }
    if fPrimarySet == 0 {
        cval = ICON_PLACE_LEFT_PRIMARY as ::core::ffi::c_int;
    }
    if fSecondarySet == 0 {
        if cval as ::core::ffi::c_long & (ICON_PLACE_LEFT_PRIMARY | ICON_PLACE_RIGHT_PRIMARY) != 0 {
            cval =
                (cval as ::core::ffi::c_long | ICON_PLACE_BOTTOM_SECONDARY) as ::core::ffi::c_int;
        } else {
            cval = (cval as ::core::ffi::c_long | ICON_PLACE_LEFT_SECONDARY) as ::core::ffi::c_int;
        }
    }
    (*toVal).size = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut cval as XtPointer as XPointer;
}
pub const ICON_PLACE_BOTTOM_STR: *mut ::core::ffi::c_uchar =
    b"bottom\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const ICON_PLACE_LEFT_STR: *mut ::core::ffi::c_uchar =
    b"left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const ICON_PLACE_RIGHT_STR: *mut ::core::ffi::c_uchar =
    b"right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const ICON_PLACE_TOP_STR: *mut ::core::ffi::c_uchar =
    b"top\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const ICON_PLACE_TIGHT_STR: *mut ::core::ffi::c_uchar =
    b"tight\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToKFocus(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    static mut cval: ::core::ffi::c_int = 0;
    let mut fHit = False as Boolean;
    if *pch as ::core::ffi::c_int != 0 && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
    {
        if *pch as ::core::ffi::c_int == 'E' as i32 || *pch as ::core::ffi::c_int == 'e' as i32 {
            if StringsAreEqual(pch, KEYBOARD_FOCUS_EXPLICIT_STR, len) != 0 {
                cval = KEYBOARD_FOCUS_EXPLICIT;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'P' as i32
            || *pch as ::core::ffi::c_int == 'p' as i32
        {
            if StringsAreEqual(pch, KEYBOARD_FOCUS_POINTER_STR, len) != 0 {
                cval = KEYBOARD_FOCUS_POINTER;
                fHit = True as Boolean;
            }
        }
    }
    if fHit == 0 {
        cval = KEYBOARD_FOCUS_EXPLICIT;
    }
    (*toVal).size = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut cval as XtPointer as XPointer;
}
pub const KEYBOARD_FOCUS_EXPLICIT_STR: *mut ::core::ffi::c_uchar =
    b"explicit\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const KEYBOARD_FOCUS_POINTER_STR: *mut ::core::ffi::c_uchar =
    b"pointer\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToSize(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    static mut cval: WHSize = _WHSize {
        width: 0,
        height: 0,
    };
    let mut len: ::core::ffi::c_int = 0;
    cval.width = 0 as ::core::ffi::c_int;
    cval.height = 0 as ::core::ffi::c_int;
    if *pch != 0 {
        cval.width = DecStrToL(pch, &raw mut pchNext) as ::core::ffi::c_int;
        if !(cval.width == 0 as ::core::ffi::c_int && pchNext == pch) {
            pch = pchNext;
            if *pch as ::core::ffi::c_int != 0
                && (*pch as ::core::ffi::c_int == 'x' as i32
                    || *pch as ::core::ffi::c_int == 'X' as i32)
            {
                pch = pch.offset(1);
                cval.height = DecStrToL(pch, &raw mut pchNext) as ::core::ffi::c_int;
            }
        } else if *pch as ::core::ffi::c_int != 0
            && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
        {
            if *pch as ::core::ffi::c_int == 'V' as i32 || *pch as ::core::ffi::c_int == 'v' as i32
            {
                if StringsAreEqual(pch, VERTICAL_STR, len) != 0 {
                    cval.height = BIGSIZE;
                }
            } else if *pch as ::core::ffi::c_int == 'H' as i32
                || *pch as ::core::ffi::c_int == 'h' as i32
            {
                if StringsAreEqual(pch, HORIZONTAL_STR, len) != 0 {
                    cval.width = BIGSIZE;
                }
            }
        }
    }
    (*toVal).size = ::core::mem::size_of::<WHSize>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut cval as XtPointer as XPointer;
}
pub const VERTICAL_STR: *mut ::core::ffi::c_uchar =
    b"vertical\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const HORIZONTAL_STR: *mut ::core::ffi::c_uchar =
    b"horizontal\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToShowFeedback(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    static mut cval: ::core::ffi::c_int = 0;
    let mut fHit = False as Boolean;
    let mut fAddNext = True as Boolean;
    if *pch as ::core::ffi::c_int != 0
        && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
        && *pch as ::core::ffi::c_int == '-' as i32
    {
        cval = WM_SHOW_FB_DEFAULT as ::core::ffi::c_int;
        fHit = True as Boolean;
    } else {
        cval = WM_SHOW_FB_NONE;
    }
    while *pch as ::core::ffi::c_int != 0
        && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
    {
        if *pch as ::core::ffi::c_int == '+' as i32 {
            if len != 1 as ::core::ffi::c_int {
                pchNext = pch.offset(1 as ::core::ffi::c_int as isize);
            }
            fAddNext = TRUE as Boolean;
        } else if *pch as ::core::ffi::c_int == '-' as i32 {
            if len != 1 as ::core::ffi::c_int {
                pchNext = pch.offset(1 as ::core::ffi::c_int as isize);
            }
            fAddNext = FALSE as Boolean;
        }
        if *pch as ::core::ffi::c_int == 'A' as i32 || *pch as ::core::ffi::c_int == 'a' as i32 {
            if StringsAreEqual(pch, SHOW_FB_ALL_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_SHOW_FB_ALL
                } else {
                    cval as ::core::ffi::c_long & !WM_SHOW_FB_ALL
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'B' as i32
            || *pch as ::core::ffi::c_int == 'b' as i32
        {
            if StringsAreEqual(pch, SHOW_FB_BEHAVIOR_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_SHOW_FB_BEHAVIOR
                } else {
                    cval as ::core::ffi::c_long & !WM_SHOW_FB_BEHAVIOR
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'K' as i32
            || *pch as ::core::ffi::c_int == 'k' as i32
        {
            if StringsAreEqual(pch, SHOW_FB_KILL_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_SHOW_FB_KILL
                } else {
                    cval as ::core::ffi::c_long & !WM_SHOW_FB_KILL
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'M' as i32
            || *pch as ::core::ffi::c_int == 'm' as i32
        {
            if StringsAreEqual(pch, SHOW_FB_MOVE_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_SHOW_FB_MOVE
                } else {
                    cval as ::core::ffi::c_long & !WM_SHOW_FB_MOVE
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'N' as i32
            || *pch as ::core::ffi::c_int == 'n' as i32
        {
            if StringsAreEqual(pch, SHOW_FB_NONE_STR, len) != 0 {
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'P' as i32
            || *pch as ::core::ffi::c_int == 'p' as i32
        {
            if StringsAreEqual(pch, SHOW_FB_PLACEMENT_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_SHOW_FB_PLACEMENT
                } else {
                    cval as ::core::ffi::c_long & !WM_SHOW_FB_PLACEMENT
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'Q' as i32
            || *pch as ::core::ffi::c_int == 'q' as i32
        {
            if StringsAreEqual(pch, SHOW_FB_QUIT_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_SHOW_FB_QUIT
                } else {
                    cval as ::core::ffi::c_long & !WM_SHOW_FB_QUIT
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'R' as i32
            || *pch as ::core::ffi::c_int == 'r' as i32
        {
            if StringsAreEqual(pch, SHOW_FB_RESIZE_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_SHOW_FB_RESIZE
                } else {
                    cval as ::core::ffi::c_long & !WM_SHOW_FB_RESIZE
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            } else if StringsAreEqual(pch, SHOW_FB_RESTART_STR, len) != 0 {
                cval = (if fAddNext as ::core::ffi::c_int != 0 {
                    cval as ::core::ffi::c_long | WM_SHOW_FB_RESTART
                } else {
                    cval as ::core::ffi::c_long & !WM_SHOW_FB_RESTART
                }) as ::core::ffi::c_int;
                fHit = True as Boolean;
            }
        }
        pch = pchNext;
    }
    if fHit == 0 {
        cval = WM_SHOW_FB_DEFAULT as ::core::ffi::c_int;
    }
    (*toVal).size = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut cval as XtPointer as XPointer;
}
pub const SHOW_FB_ALL_STR: *mut ::core::ffi::c_uchar =
    b"all\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const SHOW_FB_BEHAVIOR_STR: *mut ::core::ffi::c_uchar =
    b"behavior\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const SHOW_FB_KILL_STR: *mut ::core::ffi::c_uchar =
    b"kill\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const SHOW_FB_MOVE_STR: *mut ::core::ffi::c_uchar =
    b"move\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const SHOW_FB_NONE_STR: *mut ::core::ffi::c_uchar =
    b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const SHOW_FB_PLACEMENT_STR: *mut ::core::ffi::c_uchar =
    b"placement\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const SHOW_FB_QUIT_STR: *mut ::core::ffi::c_uchar =
    b"quit\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const SHOW_FB_RESIZE_STR: *mut ::core::ffi::c_uchar =
    b"resize\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const SHOW_FB_RESTART_STR: *mut ::core::ffi::c_uchar =
    b"restart\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToUsePPosition(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    static mut cval: ::core::ffi::c_int = 0;
    let mut fHit = False as Boolean;
    if *pch as ::core::ffi::c_int != 0 && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
    {
        if *pch as ::core::ffi::c_int == 'N' as i32 || *pch as ::core::ffi::c_int == 'n' as i32 {
            if StringsAreEqual(pch, USE_PPOSITION_NONZERO_STR, len) != 0 {
                cval = USE_PPOSITION_NONZERO;
                fHit = True as Boolean;
            }
        } else if StringsAreEqual(pch, USE_PPOSITION_OFF_STR, len) != 0 {
            cval = USE_PPOSITION_OFF;
            fHit = True as Boolean;
        } else if StringsAreEqual(pch, USE_PPOSITION_ON_STR, len) != 0 {
            cval = USE_PPOSITION_ON;
            fHit = True as Boolean;
        }
    }
    if fHit == 0 {
        cval = USE_PPOSITION_NONZERO;
    }
    (*toVal).size = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut cval as XtPointer as XPointer;
}
pub const USE_PPOSITION_NONZERO_STR: *mut ::core::ffi::c_uchar =
    b"nonzero\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const USE_PPOSITION_ON_STR: *mut ::core::ffi::c_uchar =
    b"on\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
pub const USE_PPOSITION_OFF_STR: *mut ::core::ffi::c_uchar =
    b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn WmCvtStringToXRSFocus(
    mut args: *mut XrmValue,
    mut numArgs: Cardinal,
    mut fromVal: *mut XrmValue,
    mut toVal: *mut XrmValue,
) {
    let mut pch = (*fromVal).addr as *mut ::core::ffi::c_uchar;
    let mut pchNext = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    static mut cval: ::core::ffi::c_int = 0;
    let mut fHit = False as Boolean;
    let mut sz_keyboard: [::core::ffi::c_uchar; 9] =
        ::core::mem::transmute::<[u8; 9], [::core::ffi::c_uchar; 9]>(*b"keyboard\0");
    let mut sz_pointer: [::core::ffi::c_uchar; 8] =
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_uchar; 8]>(*b"pointer\0");
    let mut sz_primary: [::core::ffi::c_uchar; 8] =
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_uchar; 8]>(*b"primary\0");
    if *pch as ::core::ffi::c_int != 0 && !NextToken(pch, &raw mut len, &raw mut pchNext).is_null()
    {
        if *pch as ::core::ffi::c_int == 'P' as i32 || *pch as ::core::ffi::c_int == 'p' as i32 {
            if StringsAreEqual(pch, &raw mut sz_pointer as *mut ::core::ffi::c_uchar, len) != 0 {
                cval = XRS_FOCUS_POINTER;
                fHit = True as Boolean;
            } else if StringsAreEqual(pch, &raw mut sz_primary as *mut ::core::ffi::c_uchar, len)
                != 0
            {
                cval = XRS_FOCUS_PRIMARY;
                fHit = True as Boolean;
            }
        } else if *pch as ::core::ffi::c_int == 'K' as i32
            || *pch as ::core::ffi::c_int == 'k' as i32
        {
            if StringsAreEqual(pch, &raw mut sz_keyboard as *mut ::core::ffi::c_uchar, len) != 0 {
                cval = XRS_FOCUS_KEYBOARD;
                fHit = True as Boolean;
            }
        }
    }
    if fHit == 0 {
        cval = XRS_FOCUS_KEYBOARD;
    }
    (*toVal).size = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
    (*toVal).addr = &raw mut cval as XtPointer as XPointer;
}
#[no_mangle]
pub unsafe extern "C" fn NextToken(
    mut pchIn: *mut ::core::ffi::c_uchar,
    mut pLen: *mut ::core::ffi::c_int,
    mut ppchNext: *mut *mut ::core::ffi::c_uchar,
) -> *mut ::core::ffi::c_uchar {
    let mut pchR = pchIn;
    let mut i: ::core::ffi::c_int = 0;
    let mut chlen: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    loop {
        chlen = mblen(pchIn as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
        if !(chlen > 0 as ::core::ffi::c_int
            && *pchIn.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32)
        {
            break;
        }
        if chlen == 1 as ::core::ffi::c_int
            && *(*__ctype_b_loc()).offset(*pchIn as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
        {
            break;
        }
        pchIn = pchIn.offset(chlen as isize);
        i += 1;
    }
    ScanWhitespace(&raw mut pchIn);
    *ppchNext = pchIn;
    *pLen = i;
    if i != 0 {
        return pchR;
    } else {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn StringsAreEqual(
    mut pch1: *mut ::core::ffi::c_uchar,
    mut pch2: *mut ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_int,
) -> Boolean {
    let mut chlen1: ::core::ffi::c_int = 0;
    let mut chlen2: ::core::ffi::c_int = 0;
    let mut wch1: wchar_t = 0;
    let mut wch2: wchar_t = 0;
    while len != 0
        && {
            chlen1 = mbtowc(
                &raw mut wch1,
                pch1 as *mut ::core::ffi::c_char,
                __ctype_get_mb_cur_max(),
            );
            chlen1 > 0 as ::core::ffi::c_int
        }
        && {
            chlen2 = mbtowc(
                &raw mut wch2,
                pch2 as *mut ::core::ffi::c_char,
                __ctype_get_mb_cur_max(),
            );
            chlen2 == chlen1
        }
    {
        if chlen1 == 1 as ::core::ffi::c_int {
            if (if *(*__ctype_b_loc()).offset(*pch1 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
            {
                ({
                    let mut __res: ::core::ffi::c_int = 0;
                    if ::core::mem::size_of::<::core::ffi::c_uchar>() as usize > 1 as usize {
                        if 0 != 0 {
                            let mut __c = *pch1 as ::core::ffi::c_int;
                            __res = (if __c < -(128 as ::core::ffi::c_int)
                                || __c > 255 as ::core::ffi::c_int
                            {
                                __c as __int32_t
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            }) as ::core::ffi::c_int;
                        } else {
                            __res = tolower(*pch1 as ::core::ffi::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*pch1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int;
                    }
                    __res
                })
            } else {
                *pch1 as ::core::ffi::c_int
            }) != (if *(*__ctype_b_loc()).offset(*pch2 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
            {
                ({
                    let mut __res: ::core::ffi::c_int = 0;
                    if ::core::mem::size_of::<::core::ffi::c_uchar>() as usize > 1 as usize {
                        if 0 != 0 {
                            let mut __c = *pch2 as ::core::ffi::c_int;
                            __res = (if __c < -(128 as ::core::ffi::c_int)
                                || __c > 255 as ::core::ffi::c_int
                            {
                                __c as __int32_t
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            }) as ::core::ffi::c_int;
                        } else {
                            __res = tolower(*pch2 as ::core::ffi::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*pch2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int;
                    }
                    __res
                })
            } else {
                *pch2 as ::core::ffi::c_int
            }) {
                break;
            }
        } else if wch1 != wch2 {
            break;
        }
        pch1 = pch1.offset(chlen1 as isize);
        pch2 = pch2.offset(chlen2 as isize);
        len -= 1;
    }
    return (len == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn DecStrToL(
    mut str: *mut ::core::ffi::c_uchar,
    mut ptr: *mut *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_long {
    let mut val = 0 as ::core::ffi::c_long;
    *ptr = str;
    while mblen(str as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max())
        == 1 as ::core::ffi::c_int
        && *(*__ctype_b_loc()).offset(*str as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
    {
        str = str.offset(1);
    }
    if mblen(str as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max()) == 1 as ::core::ffi::c_int
        && *(*__ctype_b_loc()).offset(*str as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
    {
        while mblen(str as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max())
            == 1 as ::core::ffi::c_int
            && *(*__ctype_b_loc()).offset(*str as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
        {
            val = val * 10 as ::core::ffi::c_long
                + (*str as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_long;
            str = str.offset(1);
        }
        *ptr = str;
    }
    return val;
}
