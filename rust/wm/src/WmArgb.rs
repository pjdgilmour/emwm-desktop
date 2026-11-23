/*
 * ARGB client frame support for EMWM.
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Keep the client's alpha through the reparenting hierarchy. Motif GCs,
 * tiled backgrounds and font rendering still use the screen's default depth:
 * draw decorations there, then copy only their opaque regions with XRender.
 */
use crate::abi::_ArgbFrame_WmArgb as _ArgbFrame;
use crate::abi::_ButtonSpec_WmArgb as _ButtonSpec;
use crate::abi::_ClickData_WmArgb as _ClickData;
use crate::abi::_ClientData_WmArgb as _ClientData;
use crate::abi::_ClientListEntry_WmArgb as _ClientListEntry;
use crate::abi::_IconBoxData_WmArgb as _IconBoxData;
use crate::abi::_IconInfo_WmArgb as _IconInfo;
use crate::abi::_IconPlacementData_WmArgb as _IconPlacementData;
use crate::abi::_KeySpec_WmArgb as _KeySpec;
use crate::abi::_MenuButton_WmArgb as _MenuButton;
use crate::abi::_MenuItem_WmArgb as _MenuItem;
use crate::abi::_MenuSpec_WmArgb as _MenuSpec;
use crate::abi::_WmGlobalData_WmArgb as _WmGlobalData;
use crate::abi::_WmScreenData_WmArgb as _WmScreenData;
use crate::abi::_WmTimer_WmArgb as _WmTimer;
use crate::abi::_WmWorkspaceData_WmArgb as _WmWorkspaceData;
use crate::abi::_WsClientData_WmArgb as _WsClientData;
use crate::abi::_WsPresenceData_WmArgb as _WsPresenceData;
use crate::abi::ButtonSpec_WmArgb as ButtonSpec;
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed_0;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_1;
use crate::abi::C2RustUnnamed_WmArgb as C2RustUnnamed;
use crate::abi::ClickData_WmArgb as ClickData;
use crate::abi::ClientData_WmArgb as ClientData;
use crate::abi::ClientListEntry_WmArgb as ClientListEntry;
use crate::abi::IconBoxData_WmArgb as IconBoxData;
use crate::abi::IconInfo_WmArgb as IconInfo;
use crate::abi::IconPlacementData_WmArgb as IconPlacementData;
use crate::abi::KeySpec_WmArgb as KeySpec;
use crate::abi::MenuButton_WmArgb as MenuButton;
use crate::abi::MenuItem_WmArgb as MenuItem;
use crate::abi::MenuSpec_WmArgb as MenuSpec;
use crate::abi::WmFunction_WmArgb as WmFunction;
use crate::abi::WmGlobalData_WmArgb as WmGlobalData;
use crate::abi::WmScreenData_WmArgb as WmScreenData;
use crate::abi::WmTimer_WmArgb as WmTimer;
use crate::abi::WsPresenceData_WmArgb as WsPresenceData;
use crate::abi::*;
extern "C" {
    fn XCreateColormap(
        _: *mut Display,
        _: Window,
        _: *mut Visual,
        _: ::core::ffi::c_int,
    ) -> Colormap;
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
    fn XFillRectangle(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XFreeColormap(_: *mut Display, _: Colormap) -> ::core::ffi::c_int;
    fn XFreeGC(_: *mut Display, _: GC) -> ::core::ffi::c_int;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> ::core::ffi::c_int;
    fn XGetWindowAttributes(
        _: *mut Display,
        _: Window,
        _: *mut XWindowAttributes,
    ) -> ::core::ffi::c_int;
    fn XtCalloc(_: Cardinal, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    static mut wmGD: WmGlobalData;
    fn XRenderQueryExtension(
        dpy: *mut Display,
        event_basep: *mut ::core::ffi::c_int,
        error_basep: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XRenderFindVisualFormat(dpy: *mut Display, visual: *const Visual) -> *mut XRenderPictFormat;
    fn XRenderCreatePicture(
        dpy: *mut Display,
        drawable: Drawable,
        format: *const XRenderPictFormat,
        valuemask: ::core::ffi::c_ulong,
        attributes: *const XRenderPictureAttributes,
    ) -> Picture;
    fn XRenderFreePicture(dpy: *mut Display, picture: Picture);
    fn XRenderComposite(
        dpy: *mut Display,
        op: ::core::ffi::c_int,
        src: Picture,
        mask: Picture,
        dst: Picture,
        src_x: ::core::ffi::c_int,
        src_y: ::core::ffi::c_int,
        mask_x: ::core::ffi::c_int,
        mask_y: ::core::ffi::c_int,
        dst_x: ::core::ffi::c_int,
        dst_y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
    );
    fn BaseWindowX(pcd: *mut ClientData) -> ::core::ffi::c_int;
    fn BaseWindowY(pcd: *mut ClientData) -> ::core::ffi::c_int;
    fn BaseWindowWidth(pcd: *mut ClientData) -> ::core::ffi::c_uint;
    fn BaseWindowHeight(pcd: *mut ClientData) -> ::core::ffi::c_uint;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const FillSolid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FillTiled: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GCForeground: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const GCFillStyle: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int;
pub const GCTile: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 10 as ::core::ffi::c_int;
pub const AllocNone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PictTypeDirect: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PictOpSrc: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn InitArgbFrame(mut pcd: *mut ClientData) {
    let mut attrs = XWindowAttributes {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        depth: 0,
        visual: ::core::ptr::null::<Visual>() as *mut Visual,
        root: 0,
        class: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        colormap: 0,
        map_installed: 0,
        map_state: 0,
        all_event_masks: 0,
        your_event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        screen: ::core::ptr::null::<Screen>() as *mut Screen,
    };
    let mut format = ::core::ptr::null_mut::<XRenderPictFormat>();
    let mut event: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0;
    let mut argb = ::core::ptr::null_mut::<ArgbFrame>();
    if XRenderQueryExtension(wmGD.display, &raw mut event, &raw mut error) == 0
        || XGetWindowAttributes(wmGD.display, (*pcd).client, &raw mut attrs) == 0
    {
        return;
    }
    format = XRenderFindVisualFormat(wmGD.display, attrs.visual);
    if format.is_null() || (*format).type_0 != PictTypeDirect || (*format).direct.alphaMask == 0 {
        return;
    }
    argb = XtCalloc(
        1 as Cardinal,
        ::core::mem::size_of::<ArgbFrame>() as Cardinal,
    ) as *mut ArgbFrame;
    if argb.is_null() {
        return;
    }
    (*argb).visual = attrs.visual;
    (*argb).depth = attrs.depth;
    (*argb).colormap = XCreateColormap(
        wmGD.display,
        (*(*pcd).pSD).rootWindow,
        attrs.visual,
        AllocNone,
    );
    (*pcd).argbFrame = argb as *mut _ArgbFrame;
}
#[no_mangle]
pub unsafe extern "C" fn ClientDecorDrawable(
    mut pcd: *mut ClientData,
    mut base: Boolean,
) -> Drawable {
    let mut argb = (*pcd).argbFrame as *mut ArgbFrame;
    let mut window = if base as ::core::ffi::c_int != 0 {
        (*pcd).clientBaseWin
    } else {
        (*pcd).clientFrameWin
    };
    let mut buffer = ::core::ptr::null_mut::<ArgbBuffer>();
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut screen = (*(*pcd).pSD).screen;
    if argb.is_null() {
        return window as Drawable;
    }
    buffer = if base as ::core::ffi::c_int != 0 {
        &raw mut (*argb).base
    } else {
        &raw mut (*argb).frame
    };
    width = if base as ::core::ffi::c_int != 0 {
        BaseWindowWidth(pcd)
    } else {
        (*pcd).frameInfo.width
    };
    height = if base as ::core::ffi::c_int != 0 {
        BaseWindowHeight(pcd)
    } else {
        (*pcd).frameInfo.height
    };
    if (*buffer).pixmap != 0 && (width != (*buffer).width || height != (*buffer).height) {
        XRenderFreePicture(wmGD.display, (*buffer).source);
        XFreePixmap(wmGD.display, (*buffer).pixmap);
        (*buffer).pixmap = None_0 as Pixmap;
    }
    if (*buffer).pixmap == 0 {
        (*buffer).width = width;
        (*buffer).height = height;
        (*buffer).pixmap = XCreatePixmap(
            wmGD.display,
            (*(*pcd).pSD).rootWindow as Drawable,
            width,
            height,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset(screen as isize))
            .root_depth as ::core::ffi::c_uint,
        );
        (*buffer).source = XRenderCreatePicture(
            wmGD.display,
            (*buffer).pixmap as Drawable,
            XRenderFindVisualFormat(
                wmGD.display,
                (*(*(wmGD.display as _XPrivDisplay))
                    .screens
                    .offset(screen as isize))
                .root_visual,
            ),
            0 as ::core::ffi::c_ulong,
            ::core::ptr::null::<XRenderPictureAttributes>(),
        );
        if (*argb).backgroundGC.is_null() {
            (*argb).backgroundGC = XCreateGC(
                wmGD.display,
                (*buffer).pixmap as Drawable,
                0 as ::core::ffi::c_ulong,
                ::core::ptr::null_mut::<XGCValues>(),
            );
        }
    }
    if (*buffer).target == 0 {
        (*buffer).target = XRenderCreatePicture(
            wmGD.display,
            window as Drawable,
            XRenderFindVisualFormat(wmGD.display, (*argb).visual),
            0 as ::core::ffi::c_ulong,
            ::core::ptr::null::<XRenderPictureAttributes>(),
        );
    }
    return (*buffer).pixmap as Drawable;
}
#[no_mangle]
pub unsafe extern "C" fn PaintArgbBackground(
    mut pcd: *mut ClientData,
    mut base: Boolean,
    mut area: *mut XRectangle,
) {
    let mut argb = (*pcd).argbFrame as *mut ArgbFrame;
    let mut buffer = ::core::ptr::null_mut::<ArgbBuffer>();
    let mut tile: Pixmap = 0;
    let mut pixel: Pixel = 0;
    let mut drawable: Drawable = 0;
    let mut values = XGCValues {
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
    let mut mask = (GCForeground | GCFillStyle) as ::core::ffi::c_ulong;
    if argb.is_null() {
        return;
    }
    drawable = ClientDecorDrawable(pcd, base);
    buffer = if base as ::core::ffi::c_int != 0 {
        &raw mut (*argb).base
    } else {
        &raw mut (*argb).frame
    };
    if base != 0 {
        tile = None_0 as Pixmap;
        pixel = (*pcd).matteBackground;
    } else if pcd == wmGD.keyboardFocus {
        tile = (*(*pcd).pSD).clientAppearance.activeBackgroundPixmap;
        pixel = (*(*pcd).pSD).clientAppearance.activeBackground;
    } else {
        tile = (*(*pcd).pSD).clientAppearance.backgroundPixmap;
        pixel = (*(*pcd).pSD).clientAppearance.background;
    }
    values.foreground = pixel as ::core::ffi::c_ulong;
    values.fill_style = if tile != 0 { FillTiled } else { FillSolid };
    if tile != 0 {
        values.tile = tile;
        mask |= GCTile as ::core::ffi::c_ulong;
    }
    XChangeGC(wmGD.display, (*argb).backgroundGC, mask, &raw mut values);
    XFillRectangle(
        wmGD.display,
        drawable,
        (*argb).backgroundGC,
        if !area.is_null() {
            (*area).x as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
        if !area.is_null() {
            (*area).y as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
        if !area.is_null() {
            (*area).width as ::core::ffi::c_uint
        } else {
            (*buffer).width
        },
        if !area.is_null() {
            (*area).height as ::core::ffi::c_uint
        } else {
            (*buffer).height
        },
    );
}
unsafe extern "C" fn CopyDecoration(
    mut buffer: *mut ArgbBuffer,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    if width <= 0 as ::core::ffi::c_int || height <= 0 as ::core::ffi::c_int {
        return;
    }
    XRenderComposite(
        wmGD.display,
        PictOpSrc,
        (*buffer).source,
        None_0 as Picture,
        (*buffer).target,
        x,
        y,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        x,
        y,
        width as ::core::ffi::c_uint,
        height as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PresentArgbDecoration(mut pcd: *mut ClientData, mut base: Boolean) {
    let mut argb = (*pcd).argbFrame as *mut ArgbFrame;
    let mut buffer = ::core::ptr::null_mut::<ArgbBuffer>();
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    if argb.is_null() {
        return;
    }
    buffer = if base as ::core::ffi::c_int != 0 {
        &raw mut (*argb).base
    } else {
        &raw mut (*argb).frame
    };
    if (*buffer).pixmap == 0 {
        return;
    }
    if base != 0 {
        y = (*pcd).matteWidth;
        x = y;
        width = (*buffer)
            .width
            .wrapping_sub((2 as ::core::ffi::c_int * x) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        height = (*buffer)
            .height
            .wrapping_sub((2 as ::core::ffi::c_int * y) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    } else {
        x = BaseWindowX(pcd);
        y = BaseWindowY(pcd);
        width = BaseWindowWidth(pcd) as ::core::ffi::c_int;
        height = BaseWindowHeight(pcd) as ::core::ffi::c_int;
    }
    CopyDecoration(
        buffer,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        (*buffer).width as ::core::ffi::c_int,
        y,
    );
    CopyDecoration(buffer, 0 as ::core::ffi::c_int, y, x, height);
    CopyDecoration(
        buffer,
        x + width,
        y,
        (*buffer)
            .width
            .wrapping_sub(x as ::core::ffi::c_uint)
            .wrapping_sub(width as ::core::ffi::c_uint) as ::core::ffi::c_int,
        height,
    );
    CopyDecoration(
        buffer,
        0 as ::core::ffi::c_int,
        y + height,
        (*buffer).width as ::core::ffi::c_int,
        (*buffer)
            .height
            .wrapping_sub(y as ::core::ffi::c_uint)
            .wrapping_sub(height as ::core::ffi::c_uint) as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn FreeArgbFrame(mut pcd: *mut ClientData) {
    let mut argb = (*pcd).argbFrame as *mut ArgbFrame;
    let mut buffers: [*mut ArgbBuffer; 2] = [::core::ptr::null_mut::<ArgbBuffer>(); 2];
    let mut i: ::core::ffi::c_int = 0;
    if argb.is_null() {
        return;
    }
    buffers[0 as ::core::ffi::c_int as usize] = &raw mut (*argb).frame;
    buffers[1 as ::core::ffi::c_int as usize] = &raw mut (*argb).base;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        if (*buffers[i as usize]).target != 0 {
            XRenderFreePicture(wmGD.display, (*buffers[i as usize]).target);
        }
        if (*buffers[i as usize]).source != 0 {
            XRenderFreePicture(wmGD.display, (*buffers[i as usize]).source);
        }
        if (*buffers[i as usize]).pixmap != 0 {
            XFreePixmap(wmGD.display, (*buffers[i as usize]).pixmap);
        }
        i += 1;
    }
    if !(*argb).backgroundGC.is_null() {
        XFreeGC(wmGD.display, (*argb).backgroundGC);
    }
    XFreeColormap(wmGD.display, (*argb).colormap);
    XtFree(argb as *mut ::core::ffi::c_char);
    (*pcd).argbFrame = ::core::ptr::null_mut::<_ArgbFrame>();
}
