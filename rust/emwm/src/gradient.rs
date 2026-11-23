// SPDX-License-Identifier: LGPL-2.0-or-later
//! XRender title gradients for EMWM's single-threaded Xt event loop.
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_int, c_ulong, CStr};
use std::ptr;
use x11::{xlib, xrender};

const ONE: i32 = 1 << 16;
const COLORS: [[(&CStr, &CStr); 3]; 2] = [
    [
        (c"Emwm.titleGradientInactiveStart", c"#b0b0b0"),
        (c"Emwm.titleGradientInactiveEnd", c"#707070"),
        (c"Emwm.titleGradientInactiveText", c"#101010"),
    ],
    [
        (c"Emwm.titleGradientActiveStart", c"#24466b"),
        (c"Emwm.titleGradientActiveEnd", c"#7399ba"),
        (c"Emwm.titleGradientActiveText", c"#ffffff"),
    ],
];

struct Resources(xlib::XrmDatabase);

impl Resources {
    fn read(display: *mut xlib::Display, root: xlib::Window) -> Self {
        let (mut kind, mut format, mut count, mut left) = (0, 0, 0, 0);
        let mut data = ptr::null_mut();
        let mut database = ptr::null_mut();
        // SAFETY: Xt owns this live display/root, Xlib sets all out-parameters,
        // and the returned property has a terminating NUL even for format 8.
        unsafe {
            let status = xlib::XGetWindowProperty(
                display,
                root,
                xlib::XA_RESOURCE_MANAGER,
                0,
                262144,
                0,
                xlib::XA_STRING,
                &mut kind,
                &mut format,
                &mut count,
                &mut left,
                &mut data,
            );
            if !data.is_null() {
                if status == 0 && kind == xlib::XA_STRING && format == 8 && left == 0 {
                    database = xlib::XrmGetStringDatabase(data.cast());
                }
                xlib::XFree(data.cast());
            }
        }
        Self(database)
    }

    fn get<'a>(&'a self, key: &CStr, fallback: &'a CStr) -> &'a CStr {
        if self.0.is_null() {
            return fallback;
        }
        let mut class = key.to_bytes_with_nul().to_vec();
        // All resource names here start with "Emwm.".
        class[5].make_ascii_uppercase();
        let mut kind = ptr::null_mut();
        let mut value = xlib::XrmValue {
            size: 0,
            addr: ptr::null_mut(),
        };
        // SAFETY: both strings end in NUL. Xrm owns the resource value until
        // the database is destroyed; the returned borrow cannot outlive it.
        unsafe {
            if xlib::XrmGetResource(
                self.0,
                key.as_ptr(),
                class.as_ptr().cast(),
                &mut kind,
                &mut value,
            ) != 0
                && !value.addr.is_null()
            {
                CStr::from_ptr(value.addr)
            } else {
                fallback
            }
        }
    }
}

impl Drop for Resources {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: Resources owns the non-null result of XrmGetStringDatabase.
            unsafe {
                xlib::XrmDestroyDatabase(self.0);
            }
        }
    }
}

fn line(mode: &CStr) -> Option<xrender::XLinearGradient> {
    let (x, y) = match mode.to_bytes() {
        b"horizontal" => (ONE, 0),
        b"vertical" => (0, ONE),
        b"diagonal" => (ONE, ONE),
        _ => return None,
    };
    Some(xrender::XLinearGradient {
        p1: xrender::XPointFixed { x: 0, y: 0 },
        p2: xrender::XPointFixed { x, y },
    })
}

#[derive(Clone, Copy)]
struct Palette {
    source: xrender::Picture,
    text: c_ulong,
    allocated: bool,
}

fn load(display: *mut xlib::Display, screen: c_int, root: xlib::Window) -> Option<[Palette; 2]> {
    let (mut event, mut error, mut major, mut minor) = (0, 0, 0, 0);
    // SAFETY: the caller supplies a live X display and its screen number.
    let supported = unsafe {
        xrender::XRenderQueryExtension(display, &mut event, &mut error) != 0
            && xrender::XRenderQueryVersion(display, &mut major, &mut minor) != 0
            && (major > 0 || minor >= 10)
    };
    if !supported {
        return None;
    }
    let db = Resources::read(display, root);
    let line = line(db.get(c"Emwm.titleGradient", c"none"))?;
    // SAFETY: screen belongs to display. All XColor outputs are initialized
    // and the static fallback colors are valid X11 hexadecimal colors.
    unsafe {
        let cmap = xlib::XDefaultColormap(display, screen);
        Some(COLORS.map(|keys| {
            let [start, end, mut text] = keys.map(|(key, fallback)| {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };
                if xlib::XParseColor(display, cmap, db.get(key, fallback).as_ptr(), &mut color) == 0
                {
                    xlib::XParseColor(display, cmap, fallback.as_ptr(), &mut color);
                }
                color
            });
            let colors = [start, end].map(|c| xrender::XRenderColor {
                red: c.red,
                green: c.green,
                blue: c.blue,
                alpha: u16::MAX,
            });
            let allocated = xlib::XAllocColor(display, cmap, &mut text) != 0;
            Palette {
                source: xrender::XRenderCreateLinearGradient(
                    display,
                    &line,
                    [0, ONE].as_ptr(),
                    colors.as_ptr(),
                    2,
                ),
                text: if allocated {
                    text.pixel
                } else {
                    xlib::XWhitePixel(display, screen)
                },
                allocated,
            }
        }))
    }
}

// Keep normalized Render sources between repaints. Reload explicitly frees
// their server resources; closing the X connection frees the final generation.
thread_local! {
    static GRADIENTS: RefCell<HashMap<c_int, Option<[Palette; 2]>>> = RefCell::default();
}

fn palette(
    display: *mut xlib::Display,
    screen: c_int,
    root: xlib::Window,
    active: c_int,
) -> Option<Palette> {
    GRADIENTS.with_borrow_mut(|cache| {
        (*cache
            .entry(screen)
            .or_insert_with(|| load(display, screen, root)))
        .map(|states| states[usize::from(active != 0)])
    })
}

#[no_mangle]
extern "C" fn EmwmReloadTitleGradient(
    display: *mut xlib::Display,
    screen: c_int,
    root: xlib::Window,
) {
    GRADIENTS.with_borrow_mut(|cache| {
        if let Some(Some(old)) = cache.remove(&screen) {
            // SAFETY: these pictures and color references were allocated on
            // the same screen and are released exactly once on replacement.
            unsafe {
                let cmap = xlib::XDefaultColormap(display, screen);
                for mut p in old {
                    if p.source != 0 {
                        xrender::XRenderFreePicture(display, p.source);
                    }
                    if p.allocated {
                        xlib::XFreeColors(display, cmap, &mut p.text, 1, 0);
                    }
                }
            }
        }
        cache.insert(screen, load(display, screen, root));
    });
}

#[no_mangle]
extern "C" fn EmwmTitleTextPixel(
    display: *mut xlib::Display,
    screen: c_int,
    root: xlib::Window,
    active: c_int,
    pixel: &mut c_ulong,
) -> c_int {
    match palette(display, screen, root, active) {
        Some(p) => {
            *pixel = p.text;
            1
        }
        None => 0,
    }
}

fn scale(size: i32) -> i32 {
    ONE / (size - 1).max(1)
}

#[no_mangle]
extern "C" fn EmwmPaintTitleGradient(
    display: *mut xlib::Display,
    screen: c_int,
    root: xlib::Window,
    active: c_int,
    drawable: xlib::Drawable,
    offset: c_int,
    width: c_int,
    height: c_int,
    clip: *const xlib::XRectangle,
) {
    if width <= 0 || height <= 0 {
        return;
    }
    let Some(p) = palette(display, screen, root, active) else {
        return;
    };
    let mut transform = xrender::XTransform {
        matrix: [[scale(width), 0, 0], [0, scale(height), 0], [0, 0, ONE]],
    };
    // SAFETY: the Motif caller provides its current title drawable and an
    // optional clip rectangle that stays live for this call. The target
    // Picture is released after compositing, while the cached source survives.
    unsafe {
        let format =
            xrender::XRenderFindVisualFormat(display, xlib::XDefaultVisual(display, screen));
        if format.is_null() {
            return;
        }
        let target = xrender::XRenderCreatePicture(display, drawable, format, 0, ptr::null());
        if !clip.is_null() {
            xrender::XRenderSetPictureClipRectangles(display, target, 0, 0, clip, 1);
        }
        xrender::XRenderSetPictureTransform(display, p.source, &mut transform);
        xrender::XRenderComposite(
            display,
            xrender::PictOpSrc,
            p.source,
            0,
            target,
            0,
            0,
            0,
            0,
            offset,
            offset,
            width as u32,
            height as u32,
        );
        xrender::XRenderFreePicture(display, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalized_gradient_endpoints_and_one_pixel_titles() {
        assert!(line(c"none").is_none());
        assert!(line(c"Horizontal").is_none());
        assert_eq!(
            line(c"horizontal").map(|l| (l.p2.x, l.p2.y)),
            Some((ONE, 0))
        );
        assert_eq!(line(c"vertical").map(|l| (l.p2.x, l.p2.y)), Some((0, ONE)));
        assert_eq!(
            line(c"diagonal").map(|l| (l.p2.x, l.p2.y)),
            Some((ONE, ONE))
        );
        assert_eq!(scale(1), ONE);
        assert_eq!(scale(200), 329);
    }
}
