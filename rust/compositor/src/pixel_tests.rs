// SPDX-License-Identifier: GPL-3.0-only
//! Same sampling fixtures as the C++ renderer oracle in tests/render-effects.cpp.
use crate::{
    effects::{Effect, Rect},
    renderer::Renderer,
};
use std::{env, ptr};
use x11::{glx::glXWaitGL, xlib::*};

#[test]
#[ignore = "select a backend and run on a private X server with test-render-effects"]
fn renderer_pixels() {
    let backend = env::var("EMWM_PIXEL_TEST_BACKEND").expect("backend is required");
    assert!(matches!(backend.as_str(), "xrender" | "glx"));
    // SAFETY: test runs on a private display and releases the image and renderer before closing it.
    unsafe {
        let d = XOpenDisplay(ptr::null());
        assert!(!d.is_null());
        let screen = XDefaultScreen(d);
        let root = XRootWindow(d, screen);
        let window = XCreateSimpleWindow(d, root, 0, 0, 1024, 768, 0, 0, 0);
        XMapWindow(d, window);
        XSync(d, 0);
        let mut renderer =
            Renderer::open(d, screen, window, &backend, false).expect("renderer init");
        assert_eq!(renderer.accelerated(), backend == "glx");
        renderer.begin();
        renderer.present();
        XSync(d, 0);
        let depth = XDefaultDepth(d, screen);
        let pix = XCreatePixmap(d, window, 240, 160, depth as u32);
        let gc = XCreateGC(d, pix, 0, ptr::null_mut());
        for (i, color) in [0xff0000, 0x00ff00, 0x0000ff, 0xffff00]
            .into_iter()
            .enumerate()
        {
            XSetForeground(d, gc, color);
            XFillRectangle(
                d,
                pix,
                gc,
                (i % 2) as i32 * 120,
                (i / 2) as i32 * 80,
                120,
                80,
            );
        }
        XFreeGC(d, gc);
        let im = renderer
            .image(pix, XDefaultVisual(d, screen), depth, 240, 160)
            .expect("pixmap import");
        let pixel = |x, y| {
            let shot = XGetImage(d, root, x, y, 1, 1, !0, ZPixmap);
            assert!(!shot.is_null());
            let value = XGetPixel(shot, 0, 0) & 0xffffff;
            XDestroyImage(shot);
            value
        };
        let draw = |effect, t, shape: Option<&[XRectangle]>| {
            renderer.begin();
            renderer.transformed(
                &im,
                Rect::new(200., 180., 240., 160.).outline(effect, t),
                1.,
                shape,
            );
            renderer.present();
            if renderer.accelerated() {
                glXWaitGL();
            }
            XSync(d, 0);
        };
        draw(Effect::Zoom, 0.25, None);
        let bg = pixel(100, 100);
        assert_eq!(
            [
                pixel(260, 220),
                pixel(380, 220),
                pixel(260, 300),
                pixel(380, 300)
            ],
            [0xff0000, 0x00ff00, 0x0000ff, 0xffff00]
        );
        draw(Effect::Twist, 0.25, None);
        assert_eq!(
            [
                pixel(360, 200),
                pixel(280, 200),
                pixel(360, 320),
                pixel(280, 320),
                pixel(210, 190)
            ],
            [0xff0000, 0x0000ff, 0x00ff00, 0xffff00, bg]
        );
        draw(Effect::Flip, 0.5, None);
        assert_eq!(
            [
                pixel(260, 220),
                pixel(380, 220),
                pixel(260, 300),
                pixel(380, 300)
            ],
            [0x0000ff, 0xffff00, 0xff0000, 0x00ff00]
        );
        for t in [0.25, 0.75] {
            draw(Effect::Flip, t, None);
            assert_eq!(pixel(320, 260), bg);
        }
        draw(
            Effect::Twist,
            0.25,
            Some(&[XRectangle {
                x: 0,
                y: 0,
                width: 120,
                height: 160,
            }]),
        );
        assert_eq!(
            [
                pixel(360, 200),
                pixel(280, 200),
                pixel(360, 320),
                pixel(280, 320)
            ],
            [0xff0000, 0x0000ff, bg, bg]
        );
        for effect in [Effect::Twist, Effect::Flip] {
            for t in [0., 1.] {
                draw(effect, t, None);
                assert_eq!((pixel(260, 220), pixel(380, 300)), (0xff0000, 0xffff00));
            }
        }
        drop(im);
        drop(renderer);
        XDestroyWindow(d, window);
        XSync(d, 0);
        XCloseDisplay(d);
    }
}
