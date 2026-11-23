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
use crate::abi::WmWorkspaceData_WmBackdrop as WmWorkspaceData;
use crate::abi::*;
extern "C" {
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    fn SetNewBackdrop(pWS: *mut WmWorkspaceData, bitmapFile: String_0);
    fn ChangeToWorkspace(pNewWS: *mut WmWorkspaceData);
    fn ChangeWorkspaceTitle(pWS: *mut WmWorkspaceData, pchTitle: *mut ::core::ffi::c_char);
    fn CreateWorkspace(
        pSD: *mut WmScreenData,
        name: *mut ::core::ffi::c_uchar,
    ) -> *mut WmWorkspaceData;
    fn DeleteWorkspace(pWS: *mut WmWorkspaceData);
    fn GetWorkspaceData(pSD: *mut WmScreenData, wsID: WorkspaceID) -> *mut WmWorkspaceData;
    fn GetString(linePP: *mut *mut ::core::ffi::c_uchar) -> *mut ::core::ffi::c_uchar;
    fn Warning(message: *mut ::core::ffi::c_char);
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const REQ_NAME_LEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const REQ_BACKDROP: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"BACKDROP\0") };
pub const REQ_WRKSPACE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"WRKSPACE\0") };
pub const REQ_ADDWKSPC: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"ADDWKSPC\0") };
pub const REQ_DELWKSPC: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"DELWKSPC\0") };
pub const REQ_NAMWKSPC: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"NAMWKSPC\0") };
#[no_mangle]
pub unsafe extern "C" fn ProcessWmRequest(
    mut pSD: *mut WmScreenData,
    mut pchReq: *mut ::core::ffi::c_char,
) {
    let sz_msgfmt: [::core::ffi::c_char; 38] = ::core::mem::transmute::<
        [u8; 38],
        [::core::ffi::c_char; 38],
    >(*b"Invalid window manager request: \"%s\".\0");
    let mut len: size_t = 0;
    let mut msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if strncmp(pchReq, REQ_BACKDROP.as_ptr(), REQ_NAME_LEN as size_t) == 0 {
        ProcessChangeBackdrop(pSD, pchReq.offset(REQ_NAME_LEN as isize));
        return;
    }
    if strncmp(pchReq, REQ_WRKSPACE.as_ptr(), REQ_NAME_LEN as size_t) == 0 {
        ProcessChangeWorkspace(pSD, pchReq.offset(REQ_NAME_LEN as isize));
        return;
    }
    if strncmp(pchReq, REQ_ADDWKSPC.as_ptr(), REQ_NAME_LEN as size_t) == 0 {
        ProcessAddWorkspace(pSD, pchReq.offset(REQ_NAME_LEN as isize));
        return;
    }
    if strncmp(pchReq, REQ_DELWKSPC.as_ptr(), REQ_NAME_LEN as size_t) == 0 {
        ProcessDeleteWorkspace(pSD, pchReq.offset(REQ_NAME_LEN as isize));
        return;
    }
    if strncmp(pchReq, REQ_NAMWKSPC.as_ptr(), REQ_NAME_LEN as size_t) == 0 {
        ProcessNameWorkspace(pSD, pchReq.offset(REQ_NAME_LEN as isize));
        return;
    }
    len = snprintf(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
        &raw const sz_msgfmt as *const ::core::ffi::c_char,
        pchReq,
    ) as size_t;
    msg = XtMalloc(len.wrapping_add(1 as size_t) as Cardinal);
    sprintf(
        msg,
        &raw const sz_msgfmt as *const ::core::ffi::c_char,
        pchReq,
    );
    Warning(msg);
    XtFree(msg);
}
unsafe extern "C" fn ProcessChangeWorkspace(
    mut pSD: *mut WmScreenData,
    mut param: *const ::core::ffi::c_char,
) {
    let mut ws_id: Atom = 0;
    let mut i: ::core::ffi::c_int = 0;
    ws_id = strtol(
        param,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
    ) as Atom;
    i = 0 as ::core::ffi::c_int;
    while i < (*pSD).numWorkspaces {
        if (*(*pSD).pWS.offset(i as isize)).id == ws_id {
            break;
        }
        i += 1;
    }
    if i == (*pSD).numWorkspaces {
        Warning(
            b"Invalid workspace ID in Change Workspace request.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        ChangeToWorkspace((*pSD).pWS.offset(i as isize) as *mut WmWorkspaceData);
    };
}
unsafe extern "C" fn ProcessChangeBackdrop(
    mut psd: *mut WmScreenData,
    mut param: *const ::core::ffi::c_char,
) {
    let mut pwd = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut psz_param = if !param.is_null() {
        strcpy(
            XtMalloc((strlen(param) as Cardinal).wrapping_add(1 as Cardinal)),
            param,
        )
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    let mut pctx = psz_param as *mut ::core::ffi::c_uchar;
    let mut file_name =
        b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let mut n = 0 as ::core::ffi::c_int;
    let mut ws_id: Atom = 0;
    let mut fg: Pixel = 0;
    let mut bg: Pixel = 0;
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if psz_param.is_null() {
        return;
    }
    loop {
        p = GetString(&raw mut pctx) as *mut ::core::ffi::c_char;
        if p.is_null() {
            break;
        }
        match n {
            0 => {
                ws_id = strtol(
                    p,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    0 as ::core::ffi::c_int,
                ) as Atom;
            }
            1 => {
                fg = strtol(
                    p,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    0 as ::core::ffi::c_int,
                ) as Pixel;
            }
            2 => {
                bg = strtol(
                    p,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    0 as ::core::ffi::c_int,
                ) as Pixel;
            }
            3 => {
                file_name = p;
            }
            _ => {}
        }
        n += 1;
    }
    if n < 3 as ::core::ffi::c_int {
        Warning(
            b"Missing parameters in Change Backdrop request.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        XtFree(psz_param);
        return;
    }
    pwd = GetWorkspaceData(psd, ws_id as WorkspaceID);
    if pwd.is_null() {
        Warning(
            b"Invalid workspace ID in Change Backdrop request.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        XtFree(psz_param);
        return;
    }
    (*pwd).backdrop.background = bg;
    (*pwd).backdrop.foreground = fg;
    SetNewBackdrop(pwd, file_name as String_0);
    XtFree(psz_param);
}
unsafe extern "C" fn ProcessAddWorkspace(
    mut pSD: *mut WmScreenData,
    mut param: *const ::core::ffi::c_char,
) {
    let mut psz_param = if !param.is_null() {
        strcpy(
            XtMalloc((strlen(param) as Cardinal).wrapping_add(1 as Cardinal)),
            param,
        )
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    let mut pctx = psz_param as *mut ::core::ffi::c_uchar;
    let mut title = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if strlen(psz_param) != 0 {
        title = GetString(&raw mut pctx) as *mut ::core::ffi::c_char;
    }
    CreateWorkspace(pSD, title as *mut ::core::ffi::c_uchar);
    XtFree(psz_param);
}
unsafe extern "C" fn ProcessDeleteWorkspace(
    mut pSD: *mut WmScreenData,
    mut param: *const ::core::ffi::c_char,
) {
    let mut ws_id: Atom = 0;
    let mut pwd = ::core::ptr::null_mut::<WmWorkspaceData>();
    ws_id = strtol(
        param,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
    ) as Atom;
    pwd = GetWorkspaceData(pSD, ws_id as WorkspaceID);
    if pwd.is_null() {
        Warning(
            b"Invalid workspace ID in Delete Workspace request.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    DeleteWorkspace(pwd);
}
unsafe extern "C" fn ProcessNameWorkspace(
    mut pSD: *mut WmScreenData,
    mut param: *const ::core::ffi::c_char,
) {
    let mut psz_param = if !param.is_null() {
        strcpy(
            XtMalloc((strlen(param) as Cardinal).wrapping_add(1 as Cardinal)),
            param,
        )
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    let mut pctx = psz_param as *mut ::core::ffi::c_uchar;
    let mut pwd = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut ws_id: Atom = 0;
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut title = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n = 0 as ::core::ffi::c_int;
    if psz_param.is_null() {
        return;
    }
    loop {
        p = GetString(&raw mut pctx) as *mut ::core::ffi::c_char;
        if p.is_null() {
            break;
        }
        match n {
            0 => {
                ws_id = strtol(
                    p,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    0 as ::core::ffi::c_int,
                ) as Atom;
            }
            1 => {
                title = p;
            }
            _ => {}
        }
        n += 1;
    }
    if n < 2 as ::core::ffi::c_int {
        Warning(
            b"Missing parameters in Name Workspace request.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        XtFree(psz_param);
        return;
    }
    pwd = GetWorkspaceData(pSD, ws_id as WorkspaceID);
    if pwd.is_null() {
        Warning(
            b"Invalid workspace ID in Name Workspace request.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        XtFree(psz_param);
        return;
    }
    ChangeWorkspaceTitle(pwd, title);
    XtFree(psz_param);
}
