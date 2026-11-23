use crate::backdrops::Result;
use std::ptr;
use x11::{xlib, xrandr};

#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub rotation: i32,
}
pub struct Layout {
    pub width: i32,
    pub height: i32,
    pub monitors: Vec<Monitor>,
}

pub fn detect() -> Result<Layout> {
    // SAFETY: all structs and signatures come from x11-rs. Check nullable
    // results, read only the advertised array lengths, copy names before
    // freeing RandR objects, and close the display after its last request.
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            return Err("Não foi possível consultar os monitores X11.".into());
        }
        let screen = xlib::XDefaultScreen(display);
        let width = xlib::XDisplayWidth(display, screen);
        let height = xlib::XDisplayHeight(display, screen);
        let mut monitors = Vec::<Monitor>::new();
        let (mut event, mut error) = (0, 0);
        if xrandr::XRRQueryExtension(display, &mut event, &mut error) != 0 {
            let resources =
                xrandr::XRRGetScreenResourcesCurrent(display, xlib::XRootWindow(display, screen));
            if !resources.is_null() {
                for i in 0..(*resources).noutput {
                    let id = *(*resources).outputs.add(i as usize);
                    let output = xrandr::XRRGetOutputInfo(display, resources, id);
                    if output.is_null() {
                        continue;
                    }
                    if i32::from((*output).connection) == xrandr::RR_Connected
                        && (*output).crtc != 0
                    {
                        let crtc = xrandr::XRRGetCrtcInfo(display, resources, (*output).crtc);
                        if let Some(info) = crtc.as_ref() {
                            let (x, y, w, h) =
                                (info.x, info.y, info.width as i32, info.height as i32);
                            let duplicate = monitors
                                .iter()
                                .any(|m| (m.x, m.y, m.width, m.height) == (x, y, w, h));
                            if w > 0
                                && h > 0
                                && !duplicate
                                && !(*output).name.is_null()
                                && (*output).nameLen > 0
                            {
                                let name = std::slice::from_raw_parts(
                                    (*output).name.cast::<u8>(),
                                    (*output).nameLen as usize,
                                );
                                let mask = i32::from(info.rotation);
                                let rotation = [
                                    (xrandr::RR_Rotate_90, 90),
                                    (xrandr::RR_Rotate_180, 180),
                                    (xrandr::RR_Rotate_270, 270),
                                ]
                                .into_iter()
                                .find(|(flag, _)| mask & flag != 0)
                                .map_or(0, |(_, angle)| angle);
                                monitors.push(Monitor {
                                    name: String::from_utf8_lossy(name).into_owned(),
                                    x,
                                    y,
                                    width: w,
                                    height: h,
                                    rotation,
                                });
                            }
                            xrandr::XRRFreeCrtcInfo(crtc);
                        }
                    }
                    xrandr::XRRFreeOutputInfo(output);
                }
                xrandr::XRRFreeScreenResources(resources);
            }
        }
        xlib::XCloseDisplay(display);
        if monitors.is_empty() {
            monitors.push(Monitor {
                name: "Tela X11".into(),
                x: 0,
                y: 0,
                width,
                height,
                rotation: 0,
            });
        }
        Ok(Layout {
            width,
            height,
            monitors,
        })
    }
}
