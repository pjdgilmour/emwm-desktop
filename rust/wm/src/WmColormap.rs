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
use crate::abi::*;
extern "C" {
    fn XCheckIfEvent(
        _: *mut Display,
        _: *mut XEvent,
        _: Option<unsafe extern "C" fn(*mut Display, *mut XEvent, XPointer) -> ::core::ffi::c_int>,
        _: XPointer,
    ) -> ::core::ffi::c_int;
    fn XGrabServer(_: *mut Display) -> ::core::ffi::c_int;
    fn XInstallColormap(_: *mut Display, _: Colormap) -> ::core::ffi::c_int;
    fn XSync(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XUngrabServer(_: *mut Display) -> ::core::ffi::c_int;
    fn XDeleteContext(_: *mut Display, _: XID, _: XContext) -> ::core::ffi::c_int;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
    fn XSaveContext(
        _: *mut Display,
        _: XID,
        _: XContext,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    static mut wmGD: WmGlobalData;
    fn GetClientUnderPointer(pSameScreen: *mut Boolean) -> *mut ClientData;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ColormapNotify: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const ColormapUninstalled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ColormapInstalled: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAXIMIZED_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CMAP_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut firstRequest: ::core::ffi::c_ulong = 0;
static mut lastRequest: ::core::ffi::c_ulong = 0;
#[no_mangle]
pub unsafe extern "C" fn InitWorkspaceColormap(mut pSD: *mut WmScreenData) {
    (*pSD).workspaceColormap = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*pSD).screen as isize))
    .cmap;
}
#[no_mangle]
pub unsafe extern "C" fn InitColormapFocus(mut pSD: *mut WmScreenData) {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut sameScreen: Boolean = 0;
    (*pSD).colormapFocus = ::core::ptr::null_mut::<_ClientData>();
    if wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER {
        if wmGD.keyboardFocusPolicy != KEYBOARD_FOCUS_POINTER {
            pCD = GetClientUnderPointer(&raw mut sameScreen);
            if !pCD.is_null() {
                SetColormapFocus(pSD, pCD);
            } else {
                WmInstallColormap(pSD, (*pSD).workspaceColormap);
            }
        }
    } else {
        WmInstallColormap(pSD, (*pSD).workspaceColormap);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ForceColormapFocus(mut pSD: *mut WmScreenData, mut pCD: *mut ClientData) {
    if !pCD.is_null()
        && ((*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE)
    {
        (*pSD).colormapFocus = pCD as *mut _ClientData;
        ProcessColormapList(pSD, pCD);
    } else {
        (*pSD).colormapFocus = ::core::ptr::null_mut::<_ClientData>();
        WmInstallColormap(pSD, (*pSD).workspaceColormap);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SetColormapFocus(mut pSD: *mut WmScreenData, mut pCD: *mut ClientData) {
    if pCD == (*pSD).colormapFocus {
        return;
    }
    ForceColormapFocus(pSD, pCD);
}
#[no_mangle]
pub unsafe extern "C" fn WmInstallColormap(mut pSD: *mut WmScreenData, mut colormap: Colormap) {
    if colormap != (*pSD).lastInstalledColormap {
        XInstallColormap(wmGD.display, colormap);
        (*pSD).lastInstalledColormap = colormap;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ResetColormapData(
    mut pCD: *mut ClientData,
    mut pWindows: *mut Window,
    mut count: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    if (*pCD).clientCmapCount != 0 {
        if count == 0 as ::core::ffi::c_int {
            i = 0 as ::core::ffi::c_int;
            while i < (*pCD).clientCmapCount {
                if *(*pCD).cmapWindows.offset(i as isize) == (*pCD).client {
                    (*pCD).clientColormap = *(*pCD).clientCmapList.offset(i as isize);
                    break;
                } else {
                    i += 1;
                }
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pCD).clientCmapCount {
            if *(*pCD).cmapWindows.offset(i as isize) != (*pCD).client {
                RemoveColormapWindowReference(pCD, *(*pCD).cmapWindows.offset(i as isize));
            }
            i += 1;
        }
        XtFree((*pCD).cmapWindows as *mut ::core::ffi::c_char);
        XtFree((*pCD).clientCmapList as *mut ::core::ffi::c_char);
        (*pCD).clientCmapCount = 0 as ::core::ffi::c_int;
        XtFree((*pCD).clientCmapFlags as *mut ::core::ffi::c_char);
        (*pCD).clientCmapFlags = ::core::ptr::null_mut::<::core::ffi::c_int>();
        (*pCD).clientCmapFlagsInitialized = 0 as ::core::ffi::c_int;
    }
    if count != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < count {
            if *pWindows.offset(i as isize) != (*pCD).client {
                AddColormapWindowReference(pCD, *pWindows.offset(i as isize));
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn AddColormapWindowReference(mut pCD: *mut ClientData, mut window: Window) {
    let mut cmap_window_data = ::core::ptr::null_mut::<*mut ClientData>();
    let mut context_exists: Boolean = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut new_cmap_window_data = ::core::ptr::null_mut::<*mut ClientData>();
    context_exists = (XFindContext(
        wmGD.display,
        window as XID,
        wmGD.cmapWindowContextType,
        &raw mut cmap_window_data as *mut XPointer,
    ) == 0) as ::core::ffi::c_int as Boolean;
    if context_exists != 0 {
        i = 0 as ::core::ffi::c_int;
        while !(*cmap_window_data.offset(i as isize)).is_null() {
            if *cmap_window_data.offset(i as isize) == pCD {
                return;
            }
            i += 1;
        }
        new_cmap_window_data = XtMalloc(
            ((i + 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<*mut ClientData>() as usize)
                as Cardinal,
        ) as *mut *mut ClientData;
        memcpy(
            new_cmap_window_data as *mut ::core::ffi::c_void,
            cmap_window_data as *mut ::core::ffi::c_void,
            ((i + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut ClientData>() as size_t),
        );
        XtFree(cmap_window_data as *mut ::core::ffi::c_char);
        XDeleteContext(wmGD.display, window as XID, wmGD.cmapWindowContextType);
    } else {
        i = 0 as ::core::ffi::c_int;
        new_cmap_window_data = XtMalloc(
            (2 as usize).wrapping_mul(::core::mem::size_of::<*mut ClientData>() as usize)
                as Cardinal,
        ) as *mut *mut ClientData;
    }
    let ref mut fresh0 = *new_cmap_window_data.offset(i as isize);
    *fresh0 = pCD;
    let ref mut fresh1 = *new_cmap_window_data.offset((i + 1 as ::core::ffi::c_int) as isize);
    *fresh1 = ::core::ptr::null_mut::<ClientData>();
    XSaveContext(
        wmGD.display,
        window as XID,
        wmGD.cmapWindowContextType,
        new_cmap_window_data as XPointer as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RemoveColormapWindowReference(
    mut pCD: *mut ClientData,
    mut window: Window,
) {
    let mut cmap_window_data = ::core::ptr::null_mut::<*mut ClientData>();
    let mut context_exists: Boolean = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut reference_idx = -(1 as ::core::ffi::c_int);
    let mut new_cmap_window_data = ::core::ptr::null_mut::<*mut ClientData>();
    context_exists = (XFindContext(
        wmGD.display,
        window as XID,
        wmGD.cmapWindowContextType,
        &raw mut cmap_window_data as *mut XPointer,
    ) == 0) as ::core::ffi::c_int as Boolean;
    if context_exists != 0 {
        i = 0 as ::core::ffi::c_int;
        while !(*cmap_window_data.offset(i as isize)).is_null() {
            if *cmap_window_data.offset(i as isize) == pCD {
                reference_idx = i;
            }
            i += 1;
        }
        if reference_idx < 0 as ::core::ffi::c_int {
            return;
        }
        if i > 1 as ::core::ffi::c_int {
            let mut j: ::core::ffi::c_int = 0;
            let mut idx: ::core::ffi::c_int = 0;
            new_cmap_window_data = XtMalloc(
                (i as usize).wrapping_mul(::core::mem::size_of::<*mut ClientData>() as usize)
                    as Cardinal,
            ) as *mut *mut ClientData;
            idx = 0 as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while !(*cmap_window_data.offset(j as isize)).is_null() {
                if j != reference_idx {
                    let ref mut fresh2 = *new_cmap_window_data.offset(idx as isize);
                    *fresh2 = *cmap_window_data.offset(j as isize);
                    idx += 1;
                }
                j += 1;
            }
            let ref mut fresh3 = *new_cmap_window_data.offset(idx as isize);
            *fresh3 = ::core::ptr::null_mut::<ClientData>();
        }
        XtFree(cmap_window_data as *mut ::core::ffi::c_char);
        XDeleteContext(wmGD.display, window as XID, wmGD.cmapWindowContextType);
        if i > 1 as ::core::ffi::c_int {
            XSaveContext(
                wmGD.display,
                window as XID,
                wmGD.cmapWindowContextType,
                new_cmap_window_data as XPointer as *const ::core::ffi::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessColormapList(mut pSD: *mut WmScreenData, mut pCD: *mut ClientData) {
    let mut i: ::core::ffi::c_int = 0;
    let mut event = _XEvent { type_0: 0 };
    if pCD.is_null() {
        return;
    }
    if pCD != (*pSD).colormapFocus {
        return;
    }
    if (*pCD).clientState != NORMAL_STATE && (*pCD).clientState != MAXIMIZED_STATE {
        return;
    }
    if (*pCD).clientCmapCount == 0 as ::core::ffi::c_int {
        WmInstallColormap(pSD, (*pCD).clientColormap);
        return;
    }
    if (*pCD).clientCmapCount == 1 as ::core::ffi::c_int {
        WmInstallColormap(
            pSD,
            *(*pCD)
                .clientCmapList
                .offset(0 as ::core::ffi::c_int as isize),
        );
        return;
    }
    if (*pCD).clientCmapFlagsInitialized != 0 {
        i = (*pCD).clientCmapIndex;
        loop {
            i -= 1;
            if !(i >= 0 as ::core::ffi::c_int) {
                break;
            }
            if *(*pCD).clientCmapFlags.offset(i as isize) == ColormapInstalled {
                WmInstallColormap(pSD, *(*pCD).clientCmapList.offset(i as isize));
            }
        }
        i = (*pCD).clientCmapCount;
        loop {
            i -= 1;
            if !(i >= (*pCD).clientCmapIndex) {
                break;
            }
            if *(*pCD).clientCmapFlags.offset(i as isize) == ColormapInstalled {
                WmInstallColormap(pSD, *(*pCD).clientCmapList.offset(i as isize));
            }
        }
        return;
    }
    XGrabServer(wmGD.display);
    XSync(wmGD.display, FALSE);
    firstRequest = (*(wmGD.display as _XPrivDisplay))
        .request
        .wrapping_add(1 as ::core::ffi::c_ulong);
    i = (*pCD).clientCmapIndex;
    loop {
        i -= 1;
        if !(i >= 0 as ::core::ffi::c_int) {
            break;
        }
        WmInstallColormap(pSD, *(*pCD).clientCmapList.offset(i as isize));
        *(*pCD).clientCmapFlags.offset(i as isize) = ColormapInstalled;
    }
    i = (*pCD).clientCmapCount;
    loop {
        i -= 1;
        if !(i >= (*pCD).clientCmapIndex) {
            break;
        }
        WmInstallColormap(pSD, *(*pCD).clientCmapList.offset(i as isize));
        *(*pCD).clientCmapFlags.offset(i as isize) = ColormapInstalled;
    }
    XSync(wmGD.display, FALSE);
    lastRequest = (*(wmGD.display as _XPrivDisplay))
        .request
        .wrapping_add(1 as ::core::ffi::c_ulong);
    XUngrabServer(wmGD.display);
    XCheckIfEvent(
        wmGD.display,
        &raw mut event,
        Some(
            ProcessEvents
                as unsafe extern "C" fn(
                    *mut Display,
                    *mut XEvent,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        pCD as XPointer,
    );
    (*pCD).clientCmapFlagsInitialized = True;
}
unsafe extern "C" fn ProcessEvents(
    mut dpy: *mut Display,
    mut Event: *mut XEvent,
    mut c_pCD: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut pEvent = Event as *mut XColormapEvent;
    let mut pCD = c_pCD as *mut ClientData;
    if (*pEvent).type_0 == ColormapNotify
        && (*pEvent).serial >= firstRequest
        && (*pEvent).serial < lastRequest
        && (*pEvent).colormap != None_0 as Colormap
        && (*pEvent).new == 0
    {
        match (*pEvent).state {
            ColormapInstalled => {
                i = 0 as ::core::ffi::c_int;
                while i < (*pCD).clientCmapCount {
                    if *(*pCD).clientCmapList.offset(i as isize) == (*pEvent).colormap
                        && *(*pCD).cmapWindows.offset(i as isize) == (*pEvent).window
                    {
                        *(*pCD).clientCmapFlags.offset(i as isize) = ColormapInstalled;
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
            ColormapUninstalled => {
                i = 0 as ::core::ffi::c_int;
                while i < (*pCD).clientCmapCount {
                    if *(*pCD).clientCmapList.offset(i as isize) == (*pEvent).colormap
                        && *(*pCD).cmapWindows.offset(i as isize) == (*pEvent).window
                    {
                        *(*pCD).clientCmapFlags.offset(i as isize) = ColormapUninstalled;
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
            _ => {}
        }
    }
    return False;
}
