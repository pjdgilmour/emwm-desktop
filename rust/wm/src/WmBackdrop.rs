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
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed_0;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_1;
use crate::abi::C2RustUnnamed_WmArgb as C2RustUnnamed;
use crate::abi::WmWorkspaceData_WmBackdrop as WmWorkspaceData;
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
    fn XCreateGC(_: *mut Display, _: Drawable, _: ::core::ffi::c_ulong, _: *mut XGCValues) -> GC;
    fn XCreatePixmap(
        _: *mut Display,
        _: Drawable,
        _: ::core::ffi::c_uint,
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
    fn XClearWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
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
    fn XDefaultDepth(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn XLowerWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XSetWindowBackground(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    fn XSetWindowBackgroundPixmap(_: *mut Display, _: Window, _: Pixmap) -> ::core::ffi::c_int;
    fn XUnmapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XtDisplayOfObject(_: Widget) -> *mut Display;
    fn XtScreenOfObject(_: Widget) -> *mut Screen;
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XmGetPixmap(
        screen: *mut Screen,
        image_name: *mut ::core::ffi::c_char,
        foreground: Pixel,
        background: Pixel,
    ) -> Pixmap;
    fn XmDestroyPixmap(screen: *mut Screen, pixmap: Pixmap) -> Boolean;
    static mut wmGD: WmGlobalData;
    fn RestackDesktopSurfaces(screen: *mut WmScreenData);
    fn DesktopOwnsBackground(screen: *mut WmScreenData) -> Boolean;
    fn Warning(message: *mut ::core::ffi::c_char);
    fn SetWorkspaceInfoProperty(pWS: *mut WmWorkspaceData);
    fn XmInternAtom(display: *mut Display, name: String_0, only_if_exists: Boolean) -> Atom;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn BitmapPathName(string: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn GetString(linePP: *mut *mut ::core::ffi::c_uchar) -> *mut ::core::ffi::c_uchar;
    fn ToLower(string: *mut ::core::ffi::c_uchar);
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const CHANGE_BACKDROP: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CopyFromParent: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const EnterWindowMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const LeaveWindowMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const BadDrawable: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const CWBackPixel: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const CWBackingStore: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const CWOverrideRedirect: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const CWSaveUnder: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 10 as ::core::ffi::c_int;
pub const CWEventMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const NotUseful: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XmUNSPECIFIED_PIXMAP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BACKDROP_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BACKDROP_BITMAP: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const BACKDROP_IMAGE_ALLOCED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const CMAP_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut xa_NO_BACKDROP: ::core::ffi::c_int = None_0 as ::core::ffi::c_int;
static mut none_string: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"none\0") };
#[no_mangle]
pub unsafe extern "C" fn ChangeBackdrop(mut pWS: *mut WmWorkspaceData) {
    if DesktopOwnsBackground((*pWS).pSD) != 0 {
        (*(*pWS).pSD).lastBackdropWin = (*pWS).backdrop.window;
        RestackDesktopSurfaces((*pWS).pSD);
        return;
    }
    if (*pWS).backdrop.window != 0 {
        if (*pWS).backdrop.window == (*(*pWS).pSD).lastBackdropWin {
            XClearWindow(wmGD.display, (*pWS).backdrop.window);
        } else {
            XLowerWindow(wmGD.display, (*pWS).backdrop.window);
            XMapWindow(wmGD.display, (*pWS).backdrop.window);
        }
    }
    if (*(*pWS).pSD).lastBackdropWin != 0 && (*pWS).backdrop.window != (*(*pWS).pSD).lastBackdropWin
    {
        XUnmapWindow(wmGD.display, (*(*pWS).pSD).lastBackdropWin);
    }
    (*(*pWS).pSD).lastBackdropWin = (*pWS).backdrop.window;
    RestackDesktopSurfaces((*pWS).pSD);
}
#[no_mangle]
pub unsafe extern "C" fn ProcessBackdropResources(
    mut pWS: *mut WmWorkspaceData,
    mut callFlags: ::core::ffi::c_ulong,
) {
    let mut xswa = XSetWindowAttributes {
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
    let mut xswamask: ::core::ffi::c_uint = 0;
    let mut pchImageName = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut pchL = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut pch = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut pLine = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut tmpPix: Pixmap = 0;
    let mut bNone = False as Boolean;
    if xa_NO_BACKDROP as ::core::ffi::c_long == None_0 {
        xa_NO_BACKDROP = XmInternAtom(
            wmGD.display,
            &raw mut none_string as String_0,
            False as Boolean,
        ) as ::core::ffi::c_int;
    }
    (*pWS).backdrop.flags = BACKDROP_NONE as ::core::ffi::c_uint;
    if (*pWS).backdrop.window == 0 {
        xswa.override_redirect = True;
        xswa.background_pixel = (*pWS).backdrop.background as ::core::ffi::c_ulong;
        xswamask = (CWOverrideRedirect | CWBackPixel) as ::core::ffi::c_uint;
        if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER
            || wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER
        {
            xswamask = (xswamask as ::core::ffi::c_long | CWEventMask) as ::core::ffi::c_uint;
            xswa.event_mask = EnterWindowMask | LeaveWindowMask;
        }
        xswa.backing_store = NotUseful;
        xswa.save_under = False;
        xswamask = (xswamask as ::core::ffi::c_long | (CWBackingStore | CWSaveUnder))
            as ::core::ffi::c_uint;
        (*pWS).backdrop.window = XCreateWindow(
            wmGD.display,
            (*(*pWS).pSD).rootWindow,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(*pWS).pSD).screen as isize))
            .width as ::core::ffi::c_uint,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset((*(*pWS).pSD).screen as isize))
            .height as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            XDefaultDepth(wmGD.display, (*(*pWS).pSD).screen),
            CopyFromParent as ::core::ffi::c_uint,
            ::core::ptr::null_mut::<Visual>(),
            xswamask as ::core::ffi::c_ulong,
            &raw mut xswa,
        );
    }
    if !(*pWS).backdrop.image.is_null() {
        pchImageName = strdup((*pWS).backdrop.image as *const ::core::ffi::c_char)
            as *mut ::core::ffi::c_uchar;
        pLine = pchImageName;
        if !pLine.is_null() && {
            pch = GetString(&raw mut pLine);
            !pch.is_null()
        } {
            pchL = strdup(pch as *mut ::core::ffi::c_char) as *mut ::core::ffi::c_uchar;
            if *pchL != 0 {
                ToLower(pchL);
            }
            if strcmp(
                pchL as *mut ::core::ffi::c_char,
                &raw mut none_string as *mut ::core::ffi::c_char,
            ) == 0
            {
                XSetWindowBackground(
                    wmGD.display,
                    (*pWS).backdrop.window,
                    (*pWS).backdrop.background as ::core::ffi::c_ulong,
                );
                (*pWS).backdrop.nameAtom = xa_NO_BACKDROP as Atom;
                bNone = True as Boolean;
            }
            if !pch.is_null() && bNone == 0 {
                if callFlags & CHANGE_BACKDROP as ::core::ffi::c_ulong != 0 {
                    let mut gc = ::core::ptr::null_mut::<_XGC>();
                    let mut display = ::core::ptr::null_mut::<Display>();
                    let mut win: Window = 0;
                    let mut status: ::core::ffi::c_int = 0;
                    let mut x: ::core::ffi::c_int = 0;
                    let mut y: ::core::ffi::c_int = 0;
                    let mut bw: ::core::ffi::c_uint = 0;
                    let mut depth: ::core::ffi::c_uint = 0;
                    let mut h: ::core::ffi::c_uint = 0;
                    let mut w: ::core::ffi::c_uint = 0;
                    tmpPix = (*pWS).backdrop.imagePixmap;
                    if XmUNSPECIFIED_PIXMAP as Pixmap != tmpPix {
                        display = XtDisplayOfObject((*pWS).workspaceTopLevelW);
                        XGetGeometry(
                            display,
                            tmpPix as Drawable,
                            &raw mut win,
                            &raw mut x,
                            &raw mut y,
                            &raw mut w,
                            &raw mut h,
                            &raw mut bw,
                            &raw mut depth,
                        );
                        (*pWS).backdrop.imagePixmap =
                            XCreatePixmap(display, tmpPix as Drawable, w, h, depth);
                        gc = XCreateGC(
                            display,
                            tmpPix as Drawable,
                            0 as ::core::ffi::c_ulong,
                            ::core::ptr::null_mut::<XGCValues>(),
                        );
                        status = XCopyArea(
                            XtDisplayOfObject((*pWS).workspaceTopLevelW),
                            tmpPix as Drawable,
                            (*pWS).backdrop.imagePixmap as Drawable,
                            gc,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            w,
                            h,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        );
                        XFreeGC(display, gc);
                    }
                    if XmUNSPECIFIED_PIXMAP as Pixmap == tmpPix || BadDrawable == status {
                        (*pWS).backdrop.imagePixmap = WmXmGetPixmap2(
                            XtScreenOfObject((*pWS).workspaceTopLevelW),
                            pch as *mut ::core::ffi::c_char,
                            (*pWS).backdrop.foreground,
                            (*pWS).backdrop.background,
                        );
                    }
                } else {
                    (*pWS).backdrop.imagePixmap = WmXmGetPixmap2(
                        XtScreenOfObject((*pWS).workspaceTopLevelW),
                        pch as *mut ::core::ffi::c_char,
                        (*pWS).backdrop.foreground,
                        (*pWS).backdrop.background,
                    );
                    XSetWindowBackgroundPixmap(
                        wmGD.display,
                        (*pWS).backdrop.window,
                        (*pWS).backdrop.imagePixmap,
                    );
                }
                if callFlags & CHANGE_BACKDROP as ::core::ffi::c_ulong != 0 {
                    if (*pWS).backdrop.imagePixmap != XmUNSPECIFIED_PIXMAP as Pixmap {
                        XSetWindowBackgroundPixmap(
                            wmGD.display,
                            (*pWS).backdrop.window,
                            (*pWS).backdrop.imagePixmap,
                        );
                    } else {
                        XSetWindowBackground(
                            wmGD.display,
                            (*pWS).backdrop.window,
                            (*pWS).backdrop.background as ::core::ffi::c_ulong,
                        );
                    }
                }
                if !pch.is_null()
                    && (*pWS).backdrop.imagePixmap != XmUNSPECIFIED_PIXMAP as Pixmap
                    && (*pWS).backdrop.window != 0
                {
                    (*pWS).backdrop.flags = ((*pWS).backdrop.flags as ::core::ffi::c_long
                        | BACKDROP_BITMAP)
                        as ::core::ffi::c_uint;
                    (*pWS).backdrop.nameAtom =
                        XmInternAtom(wmGD.display, (*pWS).backdrop.image, False as Boolean);
                } else {
                    let mut msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    let mut len: size_t = 0;
                    let fmt: [::core::ffi::c_char; 41] =
                        ::core::mem::transmute::<[u8; 41], [::core::ffi::c_char; 41]>(
                            *b"Unable to get image %s for workspace %s.\0",
                        );
                    len = snprintf(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        0 as size_t,
                        &raw const fmt as *const ::core::ffi::c_char,
                        (*pWS).backdrop.image,
                        (*pWS).name,
                    ) as size_t;
                    msg = malloc(len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
                    sprintf(
                        msg,
                        &raw const fmt as *const ::core::ffi::c_char,
                        (*pWS).backdrop.image,
                        (*pWS).name,
                    );
                    Warning(msg);
                    free(msg as *mut ::core::ffi::c_void);
                }
                pch = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            if !pchImageName.is_null() {
                free(pchImageName as *mut ::core::ffi::c_void);
            }
            if !pchL.is_null() {
                free(pchL as *mut ::core::ffi::c_void);
            }
        }
    } else {
        XSetWindowBackground(
            wmGD.display,
            (*pWS).backdrop.window,
            (*pWS).backdrop.background as ::core::ffi::c_ulong,
        );
    };
}
unsafe extern "C" fn WmXmGetPixmap2(
    mut screen: *mut Screen,
    mut pchName: *mut ::core::ffi::c_char,
    mut fg: Pixel,
    mut bg: Pixel,
) -> Pixmap {
    let mut pixReturn: Pixmap = 0;
    let mut pch = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !pchName.is_null() && *pchName as ::core::ffi::c_int != 0 {
        pixReturn = XmGetPixmap(screen, pchName, fg, bg);
        if pixReturn == XmUNSPECIFIED_PIXMAP as Pixmap {
            pch = strrchr(pchName, '/' as i32);
            if !pch.is_null()
                && pch
                    < pchName
                        .offset(strlen(pchName) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
            {
                pch = pch.offset(1);
                pixReturn = XmGetPixmap(screen, pch, fg, bg);
            }
        }
    } else {
        pixReturn = XmUNSPECIFIED_PIXMAP as Pixmap;
    }
    return pixReturn;
}
#[no_mangle]
pub unsafe extern "C" fn FullBitmapFilePath(mut pch: String_0) -> String_0 {
    let mut pchR = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if *pch as ::core::ffi::c_int != '/' as i32 {
        pchR = BitmapPathName(pch as *mut ::core::ffi::c_char);
        if stat(pchR as *const ::core::ffi::c_char, &raw mut buf) == -(1 as ::core::ffi::c_int)
            && *pch as ::core::ffi::c_int != '~' as i32
        {
            pchR = pch;
        }
    } else {
        pchR = pch;
    }
    return pchR;
}
#[no_mangle]
pub unsafe extern "C" fn SetNewBackdrop(mut pWS: *mut WmWorkspaceData, mut bitmapFile: String_0) {
    let mut pchNewBitmap = if !bitmapFile.is_null() {
        strcpy(
            XtMalloc(
                (strlen(bitmapFile as *const ::core::ffi::c_char) as Cardinal)
                    .wrapping_add(1 as Cardinal),
            ),
            bitmapFile as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    if (*pWS).backdrop.imagePixmap != 0 {
        XmDestroyPixmap(
            XtScreenOfObject((*pWS).workspaceTopLevelW),
            (*pWS).backdrop.imagePixmap,
        );
        (*pWS).backdrop.imagePixmap = None_0 as Pixmap;
    }
    if (*pWS).backdrop.flags as ::core::ffi::c_long & BACKDROP_IMAGE_ALLOCED != 0
        && !(*pWS).backdrop.image.is_null()
    {
        free((*pWS).backdrop.image as *mut ::core::ffi::c_void);
    }
    (*pWS).backdrop.imagePixmap = XmUNSPECIFIED_PIXMAP as Pixmap;
    (*pWS).backdrop.image = pchNewBitmap;
    ProcessBackdropResources(pWS, CHANGE_BACKDROP as ::core::ffi::c_ulong);
    (*pWS).backdrop.flags = ((*pWS).backdrop.flags as ::core::ffi::c_long | BACKDROP_IMAGE_ALLOCED)
        as ::core::ffi::c_uint;
    if pWS == (*wmGD.pActiveSD).pActiveWS {
        ChangeBackdrop(pWS);
    }
    SetWorkspaceInfoProperty(pWS);
}
#[no_mangle]
pub unsafe extern "C" fn IsBackdropWindow(mut pSD: *mut WmScreenData, mut win: Window) -> Boolean {
    let mut rval = False as Boolean;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pSD).numWorkspaces && rval == 0 {
        if (*(*pSD).pWS.offset(i as isize)).backdrop.window == win {
            rval = True as Boolean;
        }
        i += 1;
    }
    return rval;
}
