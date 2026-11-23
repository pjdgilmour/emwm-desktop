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
use crate::abi::*;
use ::c2rust_bitfields;
extern "C" {
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn XDisplayString(_: *mut Display) -> *mut ::core::ffi::c_char;
    fn XSetErrorHandler(_: XErrorHandler) -> XErrorHandler;
    fn XSetIOErrorHandler(_: XIOErrorHandler) -> XIOErrorHandler;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
    fn XtSetErrorHandler(_: Option<unsafe extern "C" fn(String_0) -> !>);
    fn XtSetWarningHandler(_: XtErrorHandler);
    static mut wmGD: WmGlobalData;
    fn ExitWM(exitCode: ::core::ffi::c_int) -> !;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const BadWindow: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_ERROR_EXIT_VALUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLIENT_DESTROYED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn WmInitErrorHandler(mut display: *mut Display) {
    XSetErrorHandler(Some(
        WmXErrorHandler
            as unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> ::core::ffi::c_int,
    ));
    XSetIOErrorHandler(Some(
        WmXIOErrorHandler as unsafe extern "C" fn(*mut Display) -> ::core::ffi::c_int,
    ));
    XtSetWarningHandler(Some(
        WmXtWarningHandler as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> (),
    ));
    XtSetErrorHandler(Some(
        WmXtErrorHandler as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> !,
    ));
}
unsafe extern "C" fn WmXErrorHandler(
    mut display: *mut Display,
    mut errorEvent: *mut XErrorEvent,
) -> ::core::ffi::c_int {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    if (*errorEvent).error_code as ::core::ffi::c_int == BadWindow
        && XFindContext(
            wmGD.display,
            (*errorEvent).resourceid,
            wmGD.windowContextType,
            &raw mut pCD as *mut XPointer,
        ) == 0
    {
        if (*errorEvent).resourceid == (*pCD).client {
            (*pCD).clientFlags |= CLIENT_DESTROYED;
        }
    }
    wmGD.errorFlag = True as Boolean;
    wmGD.errorResource = (*errorEvent).resourceid;
    wmGD.errorRequestCode = (*errorEvent).request_code;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn WmXIOErrorHandler(mut display: *mut Display) -> ::core::ffi::c_int {
    let mut err: [::core::ffi::c_char; 100] = [0; 100];
    sprintf(
        &raw mut err as *mut ::core::ffi::c_char,
        b"%s: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        b"I/O error on display:\0" as *const u8 as *const ::core::ffi::c_char,
        XDisplayString(display),
    );
    Warning(&raw mut err as *mut ::core::ffi::c_char);
    ExitWM(WM_ERROR_EXIT_VALUE);
}
unsafe extern "C" fn WmXtErrorHandler(mut message: *mut ::core::ffi::c_char) -> ! {
    Warning(message);
    ExitWM(WM_ERROR_EXIT_VALUE);
}
unsafe extern "C" fn WmXtWarningHandler(mut message: *mut ::core::ffi::c_char) {}
#[no_mangle]
pub unsafe extern "C" fn Warning(mut message: *mut ::core::ffi::c_char) {
    fprintf(
        stderr,
        b"%s: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        wmGD.mwmName,
        message,
    );
    fflush(stderr);
}
