// Copyright (C) 2018-2026 alx@fastestcode.org
// SPDX-License-Identifier: MIT

use std::cell::RefCell;
use std::ffi::{c_char, c_int, c_short, c_uint, c_ulong, c_void};

/// Matches XineramaScreenInfo from X11/extensions/Xinerama.h.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct Screen {
    number: c_int,
    x: c_short,
    y: c_short,
    width: c_short,
    height: c_short,
}

impl Screen {
    fn contains(&self, x: i32, y: i32) -> bool {
        let (left, top) = (i32::from(self.x), i32::from(self.y));
        (left..left + i32::from(self.width)).contains(&x)
            && (top..top + i32::from(self.height)).contains(&y)
    }
}

extern "C" {
    fn EmwmDisplay() -> *mut c_void;
    fn EmwmActiveRoot() -> c_ulong;
    fn EmwmFocusLocation(x: *mut c_int, y: *mut c_int) -> c_int;
    fn EmwmPrimaryScreen() -> c_int;
    fn Warning(message: *const c_char);
    fn XQueryExtension(
        display: *mut c_void,
        name: *const c_char,
        opcode: *mut c_int,
        event: *mut c_int,
        error: *mut c_int,
    ) -> c_int;
}
#[link(name = "Xinerama")]
extern "C" {
    fn XineramaIsActive(display: *mut c_void) -> c_int;
    fn XineramaQueryScreens(display: *mut c_void, count: *mut c_int) -> *mut Screen;
}
#[link(name = "X11")]
extern "C" {
    fn XFree(pointer: *mut c_void) -> c_int;
    fn XQueryPointer(
        display: *mut c_void,
        window: c_ulong,
        root: *mut c_ulong,
        child: *mut c_ulong,
        root_x: *mut c_int,
        root_y: *mut c_int,
        child_x: *mut c_int,
        child_y: *mut c_int,
        mask: *mut c_uint,
    ) -> c_int;
}

#[derive(Default)]
struct Topology {
    present: bool,
    screens: Option<Vec<Screen>>,
}

// All entry points are called on the single Xt event loop thread.
thread_local! {
    static TOPOLOGY: RefCell<Topology> = const { RefCell::new(Topology { present: false, screens: None }) };
}

fn query_screens() -> Option<Vec<Screen>> {
    let mut count = 0;
    // SAFETY: EMWM has opened its display before setting up Xinerama and keeps
    // it live throughout the event loop. Xlib initializes count on success.
    let data = unsafe {
        let display = EmwmDisplay();
        if XineramaIsActive(display) == 0 {
            return None;
        }
        XineramaQueryScreens(display, &mut count)
    };
    if data.is_null() {
        return None;
    }
    // SAFETY: Xlib owns this array of count Screen records until XFree. Copy
    // it before releasing it so a later RandR event can safely replace the list.
    unsafe {
        let screens = std::slice::from_raw_parts(data, count.max(0) as usize).to_vec();
        XFree(data.cast());
        Some(screens)
    }
}

#[no_mangle]
extern "C" fn SetupXinerama() {
    let (mut opcode, mut event, mut error) = (0, 0, 0);
    // SAFETY: the display and Xlib out parameters are valid for this call.
    let present = unsafe {
        XQueryExtension(
            EmwmDisplay(),
            c"XINERAMA".as_ptr(),
            &mut opcode,
            &mut event,
            &mut error,
        )
    } != 0;
    TOPOLOGY.with_borrow_mut(|state| {
        state.present = present;
        if present {
            state.screens = query_screens();
        }
    });
}

#[no_mangle]
extern "C" fn UpdateXineramaInfo() {
    TOPOLOGY.with_borrow_mut(|state| {
        if state.present {
            state.screens = query_screens();
        }
    });
}

#[no_mangle]
extern "C" fn GetXineramaScreenCount(out: *mut c_int) -> c_int {
    TOPOLOGY.with_borrow(|state| {
        // SAFETY: WmXinerama.h requires callers to supply a writable int.
        unsafe { out.write(state.screens.as_ref().map_or(0, |v| v.len() as c_int)) };
        state.screens.is_some().into()
    })
}

fn report_screen(screen: Option<Screen>, out: *mut Screen) -> c_int {
    if let Some(screen) = screen {
        // SAFETY: the EMWM caller supplies a writable XineramaScreenInfo.
        unsafe { out.write(screen) };
        1
    } else {
        0
    }
}

fn find_screen(screens: &[Screen], x: i32, y: i32) -> Option<Screen> {
    screens
        .iter()
        .copied()
        .find(|screen| screen.contains(x.max(0), y.max(0)))
}

#[no_mangle]
extern "C" fn GetXineramaScreenFromLocation(x: c_int, y: c_int, out: *mut Screen) -> c_int {
    TOPOLOGY.with_borrow(|state| {
        report_screen(
            state
                .screens
                .as_deref()
                .and_then(|screens| find_screen(screens, x, y)),
            out,
        )
    })
}

#[no_mangle]
extern "C" fn GetXineramaScreenFromPointer(out: *mut Screen) -> c_int {
    let (mut root, mut child, mut mask) = (0, 0, 0);
    let (mut x, mut y, mut child_x, mut child_y) = (0, 0, 0, 0);
    // SAFETY: the active screen is initialized at this point and the Xlib
    // query writes only to the provided scalar outputs.
    let same_screen = unsafe {
        XQueryPointer(
            EmwmDisplay(),
            EmwmActiveRoot(),
            &mut root,
            &mut child,
            &mut x,
            &mut y,
            &mut child_x,
            &mut child_y,
            &mut mask,
        )
    };
    if same_screen == 0 {
        return 0;
    }
    GetXineramaScreenFromLocation(x, y, out)
}

#[no_mangle]
extern "C" fn GetActiveXineramaScreen(out: *mut Screen) -> c_int {
    let (mut x, mut y) = (0, 0);
    // SAFETY: the C accessor copies the focused client's coordinates when one exists.
    if unsafe { EmwmFocusLocation(&mut x, &mut y) } != 0 {
        GetXineramaScreenFromLocation(x, y, out)
    } else {
        GetXineramaScreenFromPointer(out)
    }
}

#[no_mangle]
extern "C" fn GetPrimaryXineramaScreen(out: *mut Screen) -> c_int {
    TOPOLOGY.with_borrow(|state| {
        let Some(screens) = &state.screens else {
            return 0;
        };
        // SAFETY: the accessor reads the current integer preference from wmGD.
        let index = unsafe { EmwmPrimaryScreen() } as usize;
        let screen = screens.get(index).or_else(|| {
            // SAFETY: Warning reads this static message without modifying or retaining it.
            unsafe { Warning(c"PrimaryXineramaScreen out of range".as_ptr()) };
            screens.first()
        });
        report_screen(screen.copied(), out)
    })
}

#[no_mangle]
extern "C" fn GetXineramaScreenInfo(index: c_int, out: *mut Screen) -> c_int {
    TOPOLOGY.with_borrow(|state| {
        report_screen(
            state
                .screens
                .as_ref()
                .and_then(|screens| screens.get(index as usize))
                .copied(),
            out,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monitor_bounds_overlap_and_wide_layout() {
        let a = Screen {
            number: 1,
            x: 0,
            y: 0,
            width: 100,
            height: 60,
        };
        let b = Screen {
            number: 2,
            x: 90,
            y: 10,
            width: 32767,
            height: 20,
        };
        let layout = [a, b];
        assert_eq!(find_screen(&layout, -5, -5), Some(a));
        assert_eq!(find_screen(&layout, 95, 10), Some(a));
        assert_eq!(find_screen(&layout, 100, 9), None);
        assert_eq!(find_screen(&layout, 100, 10), Some(b));
        assert_eq!(find_screen(&layout, 32856, 29), Some(b));
        assert_eq!(find_screen(&layout, 32857, 29), None);
        assert_eq!(find_screen(&layout, 100, 30), None);
    }
}

mod gradient;
mod tiling;

#[repr(C)]
#[derive(Clone, Copy)]
struct TileRect {
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
}

#[no_mangle]
extern "C" fn EmwmChooseTileMonitor(
    client: TileRect,
    candidate: TileRect,
    best: &mut TileRect,
    area: &mut i64,
    distance: &mut i64,
) {
    if let Some((new_area, new_distance)) =
        tiling::select_tile_monitor(client, candidate, *area, *distance)
    {
        *area = new_area;
        *distance = new_distance;
        *best = candidate;
    }
}
