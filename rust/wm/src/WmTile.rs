/* SPDX-License-Identifier: LGPL-2.0-or-later
 * Keyboard tiling for floating windows. All changes use the existing Motif
 * configuration/state machinery; no extra windows, grabs, timers or daemon.
 */
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed_0;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_1;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_2;
use crate::abi::C2RustUnnamed_WmArgb as C2RustUnnamed;
use crate::abi::*;
extern "C" {
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn XInternAtom(_: *mut Display, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int) -> Atom;
    fn XFree(_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn XGetWindowProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_int,
        _: Atom,
        _: *mut Atom,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_ulong,
        _: *mut ::core::ffi::c_ulong,
        _: *mut *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn XGetWindowAttributes(
        _: *mut Display,
        _: Window,
        _: *mut XWindowAttributes,
    ) -> ::core::ffi::c_int;
    fn XQueryTree(
        _: *mut Display,
        _: Window,
        _: *mut Window,
        _: *mut Window,
        _: *mut *mut Window,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut wmGD: WmGlobalData;
    fn F_Restore(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn GetTimestamp() -> Time;
    fn ProcessNewConfiguration(
        pCD: *mut ClientData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        clientRequest: Boolean,
    );
    fn FixWindowConfiguration(
        pCD: *mut ClientData,
        pWidth: *mut ::core::ffi::c_uint,
        pHeight: *mut ::core::ffi::c_uint,
        widthInc: ::core::ffi::c_uint,
        heightInc: ::core::ffi::c_uint,
    );
    fn SetClientState(pCD: *mut ClientData, newState: ::core::ffi::c_int, setTime: Time);
    fn GetXineramaScreenInfo(
        index: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
    fn GetXineramaScreenCount(count: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn GetCurrentWorkspaceIndex(pSD: *mut WmScreenData) -> ::core::ffi::c_int;
    fn XRRQueryVersion(
        dpy: *mut Display,
        major_version_return: *mut ::core::ffi::c_int,
        minor_version_return: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XRRGetMonitors(
        dpy: *mut Display,
        window: Window,
        get_active: ::core::ffi::c_int,
        nmonitors: *mut ::core::ffi::c_int,
    ) -> *mut XRRMonitorInfo;
    fn XRRFreeMonitors(monitors: *mut XRRMonitorInfo);
    fn EmwmChooseTileMonitor(
        client: TileRect,
        candidate: TileRect,
        best: *mut TileRect,
        area: *mut ::core::ffi::c_longlong,
        distance: *mut ::core::ffi::c_longlong,
    );
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const TILE_MAX: C2RustUnnamed_2 = 7;
pub const TILE_BR: C2RustUnnamed_2 = 6;
pub const TILE_BL: C2RustUnnamed_2 = 5;
pub const TILE_TR: C2RustUnnamed_2 = 4;
pub const TILE_RIGHT: C2RustUnnamed_2 = 2;
pub const TILE_TL: C2RustUnnamed_2 = 3;
pub const TILE_NONE: C2RustUnnamed_2 = 0;
pub const TILE_LEFT: C2RustUnnamed_2 = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const Success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const IsViewable: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PAspect: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
pub const XA_ATOM: Atom = 4 as ::core::ffi::c_int as Atom;
pub const XA_CARDINAL: Atom = 6 as ::core::ffi::c_int as Atom;
pub const MWM_FUNC_RESIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const MWM_FUNC_MOVE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const MWM_FUNC_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const MWM_INPUT_MODELESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LLONG_MAX: ::core::ffi::c_longlong = __LONG_LONG_MAX__;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAXIMIZED_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ICON_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CONFIRM_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 12 as ::core::ffi::c_int;
pub const CLIENT_WM_CLIENTS: ::core::ffi::c_long = ICON_BOX | CONFIRM_BOX;
static mut partialAtom: Atom = 0;
static mut normalAtom: Atom = 0;
static mut desktopAtom: Atom = 0;
static mut specialAtoms: [Atom; 13] = [0; 13];
static mut typeAtom: Atom = 0;
static mut strutAtom: Atom = 0;
unsafe extern "C" fn InitTileAtoms() {
    static mut names: [*const ::core::ffi::c_char; 13] = [
        b"DESKTOP\0" as *const u8 as *const ::core::ffi::c_char,
        b"DOCK\0" as *const u8 as *const ::core::ffi::c_char,
        b"TOOLBAR\0" as *const u8 as *const ::core::ffi::c_char,
        b"MENU\0" as *const u8 as *const ::core::ffi::c_char,
        b"UTILITY\0" as *const u8 as *const ::core::ffi::c_char,
        b"SPLASH\0" as *const u8 as *const ::core::ffi::c_char,
        b"DIALOG\0" as *const u8 as *const ::core::ffi::c_char,
        b"DROPDOWN_MENU\0" as *const u8 as *const ::core::ffi::c_char,
        b"POPUP_MENU\0" as *const u8 as *const ::core::ffi::c_char,
        b"TOOLTIP\0" as *const u8 as *const ::core::ffi::c_char,
        b"NOTIFICATION\0" as *const u8 as *const ::core::ffi::c_char,
        b"COMBO\0" as *const u8 as *const ::core::ffi::c_char,
        b"DND\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut i: ::core::ffi::c_uint = 0;
    let mut name: [::core::ffi::c_char; 80] = [0; 80];
    if typeAtom != 0 {
        return;
    }
    typeAtom = XInternAtom(
        wmGD.display,
        b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char,
        False,
    );
    normalAtom = XInternAtom(
        wmGD.display,
        b"_NET_WM_WINDOW_TYPE_NORMAL\0" as *const u8 as *const ::core::ffi::c_char,
        False,
    );
    strutAtom = XInternAtom(
        wmGD.display,
        b"_NET_WM_STRUT\0" as *const u8 as *const ::core::ffi::c_char,
        False,
    );
    partialAtom = XInternAtom(
        wmGD.display,
        b"_NET_WM_STRUT_PARTIAL\0" as *const u8 as *const ::core::ffi::c_char,
        False,
    );
    desktopAtom = XInternAtom(
        wmGD.display,
        b"_NET_WM_DESKTOP\0" as *const u8 as *const ::core::ffi::c_char,
        False,
    );
    i = 0 as ::core::ffi::c_uint;
    while i
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 13]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
            as Cardinal
    {
        snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
            b"_NET_WM_WINDOW_TYPE_%s\0" as *const u8 as *const ::core::ffi::c_char,
            names[i as usize],
        );
        specialAtoms[i as usize] = XInternAtom(
            wmGD.display,
            &raw mut name as *mut ::core::ffi::c_char,
            False,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ReadWords(
    mut window: Window,
    mut property: Atom,
    mut type_0: Atom,
    mut words: *mut ::core::ffi::c_ulong,
    mut capacity: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut actual: Atom = 0;
    let mut format: ::core::ffi::c_int = 0;
    let mut count = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_ulong = 0;
    let mut remaining: ::core::ffi::c_ulong = 0;
    let mut data = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if XGetWindowProperty(
        wmGD.display,
        window,
        property,
        0 as ::core::ffi::c_long,
        capacity as ::core::ffi::c_long,
        False,
        type_0,
        &raw mut actual,
        &raw mut format,
        &raw mut n,
        &raw mut remaining,
        &raw mut data,
    ) == Success
        && actual == type_0
        && format == 32 as ::core::ffi::c_int
        && remaining == 0
        && n <= capacity as ::core::ffi::c_uint as ::core::ffi::c_ulong
    {
        count = n as ::core::ffi::c_int;
        if count != 0 {
            memcpy(
                words as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                (n as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_ulong>() as size_t),
            );
        }
    }
    if !data.is_null() {
        XFree(data as *mut ::core::ffi::c_void);
    }
    return count;
}
unsafe extern "C" fn CanTile(mut cd: *mut ClientData) -> Boolean {
    let mut types: [::core::ffi::c_ulong; 32] = [0; 32];
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_uint = 0;
    if cd.is_null()
        || (*cd).clientState != NORMAL_STATE && (*cd).clientState != MAXIMIZED_STATE
        || (*cd).fullScreen as ::core::ffi::c_int != 0
        || (*cd).transientFor != 0
        || !(*cd).transientLeader.is_null()
        || (*cd).inputMode != MWM_INPUT_MODELESS
        || (*cd).clientFlags & CLIENT_WM_CLIENTS != 0
        || (*cd).clientFunctions as ::core::ffi::c_long & (MWM_FUNC_MOVE | MWM_FUNC_RESIZE)
            != MWM_FUNC_MOVE | MWM_FUNC_RESIZE
        || (*cd).minWidth >= (*cd).maxWidthLimit && (*cd).minHeight >= (*cd).maxHeightLimit
    {
        return False as Boolean;
    }
    n = ReadWords(
        (*cd).client,
        typeAtom,
        XA_ATOM,
        &raw mut types as *mut ::core::ffi::c_ulong,
        (::core::mem::size_of::<[::core::ffi::c_ulong; 32]>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_ulong>() as usize)
            as Cardinal as ::core::ffi::c_int,
    );
    i = 0 as ::core::ffi::c_int;
    while i < n {
        if types[i as usize] == normalAtom {
            return True as Boolean;
        }
        j = 0 as ::core::ffi::c_uint;
        while j
            < (::core::mem::size_of::<[Atom; 13]>() as usize)
                .wrapping_div(::core::mem::size_of::<Atom>() as usize) as Cardinal
        {
            if types[i as usize] == specialAtoms[j as usize] {
                return False as Boolean;
            }
            j = j.wrapping_add(1);
        }
        i += 1;
    }
    return True as Boolean;
}
unsafe extern "C" fn ClientMonitor(mut cd: *mut ClientData) -> TileRect {
    let mut best = TileRect {
        x: 0 as ::core::ffi::c_int,
        y: 0 as ::core::ffi::c_int,
        width: (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*cd).pSD).screen as isize))
        .width,
        height: (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*cd).pSD).screen as isize))
        .height,
    };
    let mut client = TileRect {
        x: (*cd).frameInfo.x,
        y: (*cd).frameInfo.y,
        width: (*cd).frameInfo.width as ::core::ffi::c_int,
        height: (*cd).frameInfo.height as ::core::ffi::c_int,
    };
    let mut area = -(1 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
    let mut distance = LLONG_MAX;
    let mut i: ::core::ffi::c_int = 0;
    let mut n = 0 as ::core::ffi::c_int;
    let mut si = XineramaScreenInfo {
        screen_number: 0,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    };
    let mut major: ::core::ffi::c_int = 0;
    let mut minor: ::core::ffi::c_int = 0;
    let mut monitors = ::core::ptr::null_mut::<XRRMonitorInfo>();
    if XRRQueryVersion(wmGD.display, &raw mut major, &raw mut minor) != 0
        && (major > 1 as ::core::ffi::c_int
            || major == 1 as ::core::ffi::c_int && minor >= 5 as ::core::ffi::c_int)
    {
        monitors = XRRGetMonitors(wmGD.display, (*(*cd).pSD).rootWindow, False, &raw mut n);
        i = 0 as ::core::ffi::c_int;
        while !monitors.is_null() && i < n {
            let mut r = TileRect {
                x: (*monitors.offset(i as isize)).x,
                y: (*monitors.offset(i as isize)).y,
                width: (*monitors.offset(i as isize)).width,
                height: (*monitors.offset(i as isize)).height,
            };
            EmwmChooseTileMonitor(client, r, &raw mut best, &raw mut area, &raw mut distance);
            i += 1;
        }
        if !monitors.is_null() {
            XRRFreeMonitors(monitors);
        }
        if area >= 0 as ::core::ffi::c_longlong {
            return best;
        }
    }
    GetXineramaScreenCount(&raw mut n);
    i = 0 as ::core::ffi::c_int;
    while i < n {
        if GetXineramaScreenInfo(i, &raw mut si) != 0 {
            let mut r_0 = TileRect {
                x: si.x_org as ::core::ffi::c_int,
                y: si.y_org as ::core::ffi::c_int,
                width: si.width as ::core::ffi::c_int,
                height: si.height as ::core::ffi::c_int,
            };
            EmwmChooseTileMonitor(client, r_0, &raw mut best, &raw mut area, &raw mut distance);
        }
        i += 1;
    }
    return best;
}
unsafe extern "C" fn Overlaps(
    mut start: ::core::ffi::c_ulong,
    mut end: ::core::ffi::c_ulong,
    mut pos: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
) -> Boolean {
    return (start <= end
        && (start as ::core::ffi::c_longlong)
            < pos as ::core::ffi::c_longlong + size as ::core::ffi::c_longlong
        && end as ::core::ffi::c_longlong >= pos as ::core::ffi::c_longlong)
        as ::core::ffi::c_int as Boolean;
}
unsafe extern "C" fn WorkArea(mut cd: *mut ClientData) -> TileRect {
    let mut r = ClientMonitor(cd);
    let mut left = r.x;
    let mut top = r.y;
    let mut right = r.x + r.width;
    let mut bottom = r.y + r.height;
    let mut sw = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*cd).pSD).screen as isize))
    .width;
    let mut sh = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*(*cd).pSD).screen as isize))
    .height;
    let mut root: Window = 0;
    let mut parent: Window = 0;
    let mut children = ::core::ptr::null_mut::<Window>();
    let mut i: ::core::ffi::c_uint = 0;
    let mut n: ::core::ffi::c_uint = 0;
    if XQueryTree(
        wmGD.display,
        (*(*cd).pSD).rootWindow,
        &raw mut root,
        &raw mut parent,
        &raw mut children,
        &raw mut n,
    ) == 0
    {
        return r;
    }
    let mut current_block_15: u64;
    i = 0 as ::core::ffi::c_uint;
    while i < n {
        let mut panel = ::core::ptr::null_mut::<ClientData>();
        let mut attr = XWindowAttributes {
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
        let mut w = *children.offset(i as isize);
        let mut s: [::core::ffi::c_ulong; 12] = [
            0 as ::core::ffi::c_int as ::core::ffi::c_ulong,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut desktop: ::core::ffi::c_ulong = 0;
        if !(XGetWindowAttributes(wmGD.display, w, &raw mut attr) == 0
            || attr.map_state != IsViewable)
        {
            if XFindContext(
                wmGD.display,
                w as XID,
                wmGD.windowContextType,
                &raw mut panel as *mut XPointer,
            ) == 0
            {
                w = (*panel).client;
            }
            if !(w == (*cd).client) {
                if !(ReadWords(
                    w,
                    desktopAtom,
                    XA_CARDINAL,
                    &raw mut desktop,
                    1 as ::core::ffi::c_int,
                ) == 1 as ::core::ffi::c_int
                    && desktop != 0xffffffff as ::core::ffi::c_ulong
                    && desktop != GetCurrentWorkspaceIndex((*cd).pSD) as ::core::ffi::c_ulong)
                {
                    if ReadWords(
                        w,
                        partialAtom,
                        XA_CARDINAL,
                        &raw mut s as *mut ::core::ffi::c_ulong,
                        12 as ::core::ffi::c_int,
                    ) != 12 as ::core::ffi::c_int
                    {
                        if ReadWords(
                            w,
                            strutAtom,
                            XA_CARDINAL,
                            &raw mut s as *mut ::core::ffi::c_ulong,
                            4 as ::core::ffi::c_int,
                        ) != 4 as ::core::ffi::c_int
                        {
                            current_block_15 = 715039052867723359;
                        } else {
                            s[10 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_ulong;
                            s[8 as ::core::ffi::c_int as usize] =
                                s[10 as ::core::ffi::c_int as usize];
                            s[6 as ::core::ffi::c_int as usize] =
                                s[8 as ::core::ffi::c_int as usize];
                            s[4 as ::core::ffi::c_int as usize] =
                                s[6 as ::core::ffi::c_int as usize];
                            s[7 as ::core::ffi::c_int as usize] =
                                (sh - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong;
                            s[5 as ::core::ffi::c_int as usize] =
                                s[7 as ::core::ffi::c_int as usize];
                            s[11 as ::core::ffi::c_int as usize] =
                                (sw - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong;
                            s[9 as ::core::ffi::c_int as usize] =
                                s[11 as ::core::ffi::c_int as usize];
                            current_block_15 = 13056961889198038528;
                        }
                    } else {
                        current_block_15 = 13056961889198038528;
                    }
                    match current_block_15 {
                        715039052867723359 => {}
                        _ => {
                            if s[0 as ::core::ffi::c_int as usize] != 0
                                && s[0 as ::core::ffi::c_int as usize]
                                    <= sw as ::core::ffi::c_uint as ::core::ffi::c_ulong
                                && Overlaps(
                                    s[4 as ::core::ffi::c_int as usize],
                                    s[5 as ::core::ffi::c_int as usize],
                                    r.y,
                                    r.height,
                                ) as ::core::ffi::c_int
                                    != 0
                            {
                                left = if left
                                    > s[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                {
                                    left
                                } else {
                                    s[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                };
                            }
                            if s[1 as ::core::ffi::c_int as usize] != 0
                                && s[1 as ::core::ffi::c_int as usize]
                                    <= sw as ::core::ffi::c_uint as ::core::ffi::c_ulong
                                && Overlaps(
                                    s[6 as ::core::ffi::c_int as usize],
                                    s[7 as ::core::ffi::c_int as usize],
                                    r.y,
                                    r.height,
                                ) as ::core::ffi::c_int
                                    != 0
                            {
                                right = if right
                                    < sw - s[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                {
                                    right
                                } else {
                                    sw - s[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                };
                            }
                            if s[2 as ::core::ffi::c_int as usize] != 0
                                && s[2 as ::core::ffi::c_int as usize]
                                    <= sh as ::core::ffi::c_uint as ::core::ffi::c_ulong
                                && Overlaps(
                                    s[8 as ::core::ffi::c_int as usize],
                                    s[9 as ::core::ffi::c_int as usize],
                                    r.x,
                                    r.width,
                                ) as ::core::ffi::c_int
                                    != 0
                            {
                                top = if top
                                    > s[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                {
                                    top
                                } else {
                                    s[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                };
                            }
                            if s[3 as ::core::ffi::c_int as usize] != 0
                                && s[3 as ::core::ffi::c_int as usize]
                                    <= sh as ::core::ffi::c_uint as ::core::ffi::c_ulong
                                && Overlaps(
                                    s[10 as ::core::ffi::c_int as usize],
                                    s[11 as ::core::ffi::c_int as usize],
                                    r.x,
                                    r.width,
                                ) as ::core::ffi::c_int
                                    != 0
                            {
                                bottom = if bottom
                                    < sh - s[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                {
                                    bottom
                                } else {
                                    sh - s[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                };
                            }
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if !children.is_null() {
        XFree(children as *mut ::core::ffi::c_void);
    }
    r.x = left;
    r.y = top;
    r.width = right - left;
    r.height = bottom - top;
    return r;
}
unsafe extern "C" fn FitSize(
    mut cd: *mut ClientData,
    mut availableW: ::core::ffi::c_int,
    mut availableH: ::core::ffi::c_int,
    mut width: *mut ::core::ffi::c_uint,
    mut height: *mut ::core::ffi::c_uint,
) -> Boolean {
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut maxW = if availableW < (*cd).maxWidthLimit {
        availableW
    } else {
        (*cd).maxWidthLimit
    };
    let mut minH: ::core::ffi::c_longlong = 0;
    let mut maxH = if availableH < (*cd).maxHeightLimit {
        availableH
    } else {
        (*cd).maxHeightLimit
    };
    let mut wi = if 1 as ::core::ffi::c_int > (*cd).widthInc {
        1 as ::core::ffi::c_int
    } else {
        (*cd).widthInc
    };
    let mut hi = if 1 as ::core::ffi::c_int > (*cd).heightInc {
        1 as ::core::ffi::c_int
    } else {
        (*cd).heightInc
    };
    if maxW
        < (if (*cd).minWidth > (*cd).baseWidth {
            (*cd).minWidth
        } else {
            (*cd).baseWidth
        })
        || maxH
            < (if (*cd).minHeight > (*cd).baseHeight {
                (*cd).minHeight
            } else {
                (*cd).baseHeight
            })
    {
        return False as Boolean;
    }
    maxW -= (maxW - (*cd).baseWidth) % wi;
    w = maxW;
    while w
        >= (if 1 as ::core::ffi::c_int > (*cd).minWidth {
            1 as ::core::ffi::c_int
        } else {
            (*cd).minWidth
        })
    {
        h = maxH;
        minH = (if 1 as ::core::ffi::c_int > (*cd).minHeight {
            1 as ::core::ffi::c_int
        } else {
            (*cd).minHeight
        }) as ::core::ffi::c_longlong;
        if (*cd).sizeFlags & PAspect != 0 {
            if (*cd).minAspect.x > 0 as ::core::ffi::c_int
                && (*cd).minAspect.y > 0 as ::core::ffi::c_int
            {
                h = (if (h as ::core::ffi::c_longlong)
                    < w as ::core::ffi::c_longlong * (*cd).minAspect.y as ::core::ffi::c_longlong
                        / (*cd).minAspect.x as ::core::ffi::c_longlong
                {
                    h as ::core::ffi::c_longlong
                } else {
                    w as ::core::ffi::c_longlong * (*cd).minAspect.y as ::core::ffi::c_longlong
                        / (*cd).minAspect.x as ::core::ffi::c_longlong
                }) as ::core::ffi::c_int;
            }
            if (*cd).maxAspect.x > 0 as ::core::ffi::c_int
                && (*cd).maxAspect.y > 0 as ::core::ffi::c_int
            {
                minH = if minH
                    > (w as ::core::ffi::c_longlong * (*cd).maxAspect.y as ::core::ffi::c_longlong
                        + (*cd).maxAspect.x as ::core::ffi::c_longlong
                        - 1 as ::core::ffi::c_longlong)
                        / (*cd).maxAspect.x as ::core::ffi::c_longlong
                {
                    minH
                } else {
                    (w as ::core::ffi::c_longlong * (*cd).maxAspect.y as ::core::ffi::c_longlong
                        + (*cd).maxAspect.x as ::core::ffi::c_longlong
                        - 1 as ::core::ffi::c_longlong)
                        / (*cd).maxAspect.x as ::core::ffi::c_longlong
                };
            }
        }
        if !(h < (*cd).baseHeight) {
            h -= (h - (*cd).baseHeight) % hi;
            if !((h as ::core::ffi::c_longlong) < minH) {
                *width = w as ::core::ffi::c_uint;
                *height = h as ::core::ffi::c_uint;
                FixWindowConfiguration(
                    cd,
                    width,
                    height,
                    wi as ::core::ffi::c_uint,
                    hi as ::core::ffi::c_uint,
                );
                if *width <= maxW as ::core::ffi::c_uint
                    && *height <= maxH as ::core::ffi::c_uint
                    && *width > 0 as ::core::ffi::c_uint
                    && *height > 0 as ::core::ffi::c_uint
                {
                    return True as Boolean;
                }
            }
        }
        w -= wi;
    }
    return False as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn F_Tile(
    mut args: String_0,
    mut cd: *mut ClientData,
    mut event: *mut XEvent,
) -> Boolean {
    let mut r = TileRect {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut w: ::core::ffi::c_uint = 0;
    let mut h: ::core::ffi::c_uint = 0;
    let mut mode: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut fw: ::core::ffi::c_int = 0;
    let mut fh: ::core::ffi::c_int = 0;
    let mut restore = False as Boolean;
    InitTileAtoms();
    if args.is_null() || CanTile(cd) == 0 {
        return False as Boolean;
    }
    if strcmp(
        args as *const ::core::ffi::c_char,
        b"left\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        mode = TILE_LEFT as ::core::ffi::c_int;
    } else if strcmp(
        args as *const ::core::ffi::c_char,
        b"right\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        mode = TILE_RIGHT as ::core::ffi::c_int;
    } else if strcmp(
        args as *const ::core::ffi::c_char,
        b"top-left\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        mode = TILE_TL as ::core::ffi::c_int;
    } else if strcmp(
        args as *const ::core::ffi::c_char,
        b"top-right\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        mode = TILE_TR as ::core::ffi::c_int;
    } else if strcmp(
        args as *const ::core::ffi::c_char,
        b"bottom-left\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        mode = TILE_BL as ::core::ffi::c_int;
    } else if strcmp(
        args as *const ::core::ffi::c_char,
        b"bottom-right\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        mode = TILE_BR as ::core::ffi::c_int;
    } else if strcmp(
        args as *const ::core::ffi::c_char,
        b"up\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        mode = if (*cd).maxConfig == 0 && (*cd).tileState == TILE_LEFT as ::core::ffi::c_int {
            TILE_TL as ::core::ffi::c_int
        } else if (*cd).maxConfig == 0 && (*cd).tileState == TILE_RIGHT as ::core::ffi::c_int {
            TILE_TR as ::core::ffi::c_int
        } else {
            TILE_MAX as ::core::ffi::c_int
        };
    } else if strcmp(
        args as *const ::core::ffi::c_char,
        b"down\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        mode = if (*cd).maxConfig == 0 && (*cd).tileState == TILE_LEFT as ::core::ffi::c_int {
            TILE_BL as ::core::ffi::c_int
        } else if (*cd).maxConfig == 0 && (*cd).tileState == TILE_RIGHT as ::core::ffi::c_int {
            TILE_BR as ::core::ffi::c_int
        } else {
            TILE_NONE as ::core::ffi::c_int
        };
        restore = (mode == TILE_NONE as ::core::ffi::c_int) as ::core::ffi::c_int as Boolean;
    } else if strcmp(
        args as *const ::core::ffi::c_char,
        b"restore\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        mode = TILE_NONE as ::core::ffi::c_int;
        restore = True as Boolean;
    } else {
        return False as Boolean;
    }
    if restore as ::core::ffi::c_int != 0 && (*cd).tileState == 0 {
        return F_Restore(::core::ptr::null_mut::<::core::ffi::c_char>(), cd, event);
    }
    if mode == TILE_MAX as ::core::ffi::c_int
        && ((*cd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MAXIMIZE == 0
            || (*cd).maxConfig as ::core::ffi::c_int != 0)
    {
        return False as Boolean;
    }
    r = WorkArea(cd);
    if r.width <= 0 as ::core::ffi::c_int || r.height <= 0 as ::core::ffi::c_int {
        return False as Boolean;
    }
    fw = 2 as ::core::ffi::c_int * (*cd).clientOffset.x as ::core::ffi::c_int;
    fh = (*cd).clientOffset.x as ::core::ffi::c_int + (*cd).clientOffset.y as ::core::ffi::c_int;
    if restore != 0 {
        if FitSize(
            cd,
            if (*cd).tileWidth < r.width - fw {
                (*cd).tileWidth
            } else {
                r.width - fw
            },
            if (*cd).tileHeight < r.height - fh {
                (*cd).tileHeight
            } else {
                r.height - fh
            },
            &raw mut w,
            &raw mut h,
        ) == 0
        {
            return False as Boolean;
        }
        x = if r.x
            > (if ((*cd).tileX - (*cd).clientOffset.x as ::core::ffi::c_int)
                < r.x + r.width - w as ::core::ffi::c_int - fw
            {
                (*cd).tileX - (*cd).clientOffset.x as ::core::ffi::c_int
            } else {
                r.x + r.width - w as ::core::ffi::c_int - fw
            }) {
            r.x
        } else if ((*cd).tileX - (*cd).clientOffset.x as ::core::ffi::c_int)
            < r.x + r.width - w as ::core::ffi::c_int - fw
        {
            (*cd).tileX - (*cd).clientOffset.x as ::core::ffi::c_int
        } else {
            r.x + r.width - w as ::core::ffi::c_int - fw
        };
        y = if r.y
            > (if ((*cd).tileY - (*cd).clientOffset.y as ::core::ffi::c_int)
                < r.y + r.height - h as ::core::ffi::c_int - fh
            {
                (*cd).tileY - (*cd).clientOffset.y as ::core::ffi::c_int
            } else {
                r.y + r.height - h as ::core::ffi::c_int - fh
            }) {
            r.y
        } else if ((*cd).tileY - (*cd).clientOffset.y as ::core::ffi::c_int)
            < r.y + r.height - h as ::core::ffi::c_int - fh
        {
            (*cd).tileY - (*cd).clientOffset.y as ::core::ffi::c_int
        } else {
            r.y + r.height - h as ::core::ffi::c_int - fh
        };
    } else {
        if mode != TILE_MAX as ::core::ffi::c_int {
            let mut half = r.width / 2 as ::core::ffi::c_int;
            if mode == TILE_RIGHT as ::core::ffi::c_int
                || mode == TILE_TR as ::core::ffi::c_int
                || mode == TILE_BR as ::core::ffi::c_int
            {
                r.x += half;
                r.width -= half;
            } else {
                r.width = half;
            }
        }
        if mode >= TILE_TL as ::core::ffi::c_int && mode <= TILE_BR as ::core::ffi::c_int {
            let mut half_0 = r.height / 2 as ::core::ffi::c_int;
            if mode == TILE_BL as ::core::ffi::c_int || mode == TILE_BR as ::core::ffi::c_int {
                r.y += half_0;
                r.height -= half_0;
            } else {
                r.height = half_0;
            }
        }
        if FitSize(cd, r.width - fw, r.height - fh, &raw mut w, &raw mut h) == 0 {
            return False as Boolean;
        }
        x = r.x;
        y = r.y;
        if mode == TILE_RIGHT as ::core::ffi::c_int
            || mode == TILE_TR as ::core::ffi::c_int
            || mode == TILE_BR as ::core::ffi::c_int
        {
            x = (x as ::core::ffi::c_uint).wrapping_add(
                (r.width as ::core::ffi::c_uint)
                    .wrapping_sub(w)
                    .wrapping_sub(fw as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        if mode == TILE_BL as ::core::ffi::c_int || mode == TILE_BR as ::core::ffi::c_int {
            y = (y as ::core::ffi::c_uint).wrapping_add(
                (r.height as ::core::ffi::c_uint)
                    .wrapping_sub(h)
                    .wrapping_sub(fh as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        if (*cd).tileState == 0 {
            (*cd).tileX = (*cd).clientX;
            (*cd).tileY = (*cd).clientY;
            (*cd).tileWidth = (*cd).clientWidth;
            (*cd).tileHeight = (*cd).clientHeight;
        }
    }
    (*cd).tileApplying = True as Boolean;
    if mode == TILE_MAX as ::core::ffi::c_int {
        (*cd).maxX = x + (*cd).clientOffset.x as ::core::ffi::c_int;
        (*cd).maxY = y + (*cd).clientOffset.y as ::core::ffi::c_int;
        (*cd).oldMaxWidth = (*cd).maxWidth;
        (*cd).oldMaxHeight = (*cd).maxHeight;
        (*cd).maxWidth = w as ::core::ffi::c_int;
        (*cd).maxHeight = h as ::core::ffi::c_int;
        SetClientState(cd, MAXIMIZED_STATE, GetTimestamp());
    } else {
        if (*cd).maxConfig != 0 {
            SetClientState(cd, NORMAL_STATE, GetTimestamp());
        }
        ProcessNewConfiguration(
            cd,
            x + (*cd).clientOffset.x as ::core::ffi::c_int,
            y + (*cd).clientOffset.y as ::core::ffi::c_int,
            w,
            h,
            True as Boolean,
        );
    }
    (*cd).tileApplying = False as Boolean;
    (*cd).tileState = mode;
    return False as Boolean;
}
pub const __LONG_LONG_MAX__: ::core::ffi::c_longlong =
    9223372036854775807 as ::core::ffi::c_longlong;
