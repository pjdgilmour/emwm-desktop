/*
 * Copyright (C) 2018-2026 alx@fastestcode.org
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_0;
use crate::abi::*;
extern "C" {
    static mut wmGD: WmGlobalData;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn EmwmDisplay() -> *mut Display {
    return wmGD.display;
}
#[no_mangle]
pub unsafe extern "C" fn EmwmActiveRoot() -> Window {
    return (*wmGD.pActiveSD).rootWindow;
}
#[no_mangle]
pub unsafe extern "C" fn EmwmFocusLocation(
    mut x: *mut ::core::ffi::c_int,
    mut y: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if wmGD.keyboardFocus.is_null() {
        return False;
    }
    *x = (*wmGD.keyboardFocus).clientX;
    *y = (*wmGD.keyboardFocus).clientY;
    return True;
}
#[no_mangle]
pub unsafe extern "C" fn EmwmPrimaryScreen() -> ::core::ffi::c_int {
    return wmGD.primaryXineramaScreen;
}
