// SPDX-License-Identifier: GPL-3.0-only
//! Single-threaded renderer. Surface images are released before changing GLX contexts.
use crate::{
    effects::{Point, Quad, Rect},
    gl::{Gl, Release},
    native::*,
};
use std::{cell::Cell, mem, ptr, rc::Rc};
use x11::{glx::*, xlib::*, xrender::*};

pub struct Image {
    d: *mut Display,
    pixmap: Pixmap,
    picture: Picture,
    glpixmap: GLXPixmap,
    texture: u32,
    release: Option<Release>,
    bound: Cell<bool>,
    inverted: bool,
    w: i32,
    h: i32,
}
impl Drop for Image {
    fn drop(&mut self) {
        // SAFETY: images are owned by a live renderer context and X connection.
        unsafe {
            if let Some(release) = self.release.filter(|_| self.bound.get()) {
                release(self.d, self.glpixmap, FRONT_LEFT);
            }
            if self.texture != 0 {
                glDeleteTextures(1, &self.texture);
            }
            if self.glpixmap != 0 {
                glXDestroyPixmap(self.d, self.glpixmap);
            }
            if self.picture != 0 {
                XRenderFreePicture(self.d, self.picture);
            }
            if self.pixmap != 0 {
                XFreePixmap(self.d, self.pixmap);
            }
        }
    }
}
pub struct Renderer {
    d: *mut Display,
    screen: i32,
    overlay: Window,
    w: i32,
    h: i32,
    gl: Option<Gl>,
    buffer: Pixmap,
    target: Picture,
    destination: Picture,
    shadow: Option<Rc<Image>>,
}
fn transform(matrix: [[f64; 3]; 3]) -> XTransform {
    XTransform {
        matrix: matrix.map(|row| row.map(|v| (v * 65536.) as i32)),
    }
}
fn clip_rect(x: i32, y: i32, w: i32, h: i32) -> XRectangle {
    XRectangle {
        x: x as i16,
        y: y as i16,
        width: w as u16,
        height: h as u16,
    }
}
unsafe fn textured_quad(im: &Image, corners: Quad, uv: Rect) {
    glBegin(QUADS);
    for (p, tex) in corners.iter().zip(uv.quad()) {
        glTexCoord2d(tex.x, if im.inverted { tex.y } else { 1. - tex.y });
        glVertex2d(p.x, p.y);
    }
    glEnd();
}
impl Renderer {
    // SAFETY: d, overlay, and image visuals remain valid until all renderer resources drop.
    pub unsafe fn open(
        d: *mut Display,
        screen: i32,
        overlay: Window,
        backend: &str,
        vsync: bool,
    ) -> Result<Self, String> {
        let gl = if backend == "xrender" {
            None
        } else {
            Gl::open(d, screen, overlay, vsync)
        };
        if gl.is_none() {
            match backend {
                "glx" => return Err("requested backend failed".into()),
                "auto" => eprintln!("emwm-compositor: GLX unavailable; using XRender"),
                _ => {}
            }
        }
        let mut r = Self {
            d,
            screen,
            overlay,
            w: 0,
            h: 0,
            gl,
            buffer: 0,
            target: 0,
            destination: 0,
            shadow: None,
        };
        if r.gl.is_none() {
            r.destination = r.picture(overlay, XDefaultVisual(d, screen));
            eprintln!("emwm-compositor: backend=xrender");
        }
        r.resize(XDisplayWidth(d, screen), XDisplayHeight(d, screen));
        r.make_shadow()?;
        Ok(r)
    }
    unsafe fn picture(&self, drawable: Drawable, visual: *mut Visual) -> Picture {
        let mut pa: XRenderPictureAttributes = mem::zeroed();
        pa.subwindow_mode = IncludeInferiors;
        XRenderCreatePicture(
            self.d,
            drawable,
            XRenderFindVisualFormat(self.d, visual),
            CPSubwindowMode as _,
            &pa,
        )
    }
    unsafe fn make_shadow(&mut self) -> Result<(), String> {
        let d = self.d;
        let mut vi: XVisualInfo = mem::zeroed();
        if XMatchVisualInfo(d, self.screen, 32, TrueColor, &mut vi) == 0 {
            return Ok(());
        }
        let size = 65;
        let xi = XCreateImage(
            d,
            vi.visual,
            32,
            ZPixmap,
            0,
            ptr::null_mut(),
            size,
            size,
            32,
            0,
        );
        if xi.is_null() {
            return Ok(());
        }
        (*xi).data = libc::calloc(size as usize, (*xi).bytes_per_line as usize).cast();
        if (*xi).data.is_null() {
            XDestroyImage(xi);
            return Ok(());
        }
        let axis = |p: i32| {
            let p = p as f64;
            0.5 * (erf((p - 16.) / 7.) - erf((p - 49.) / 7.))
        };
        for y in 0..size as i32 {
            for x in 0..size as i32 {
                XPutPixel(
                    xi,
                    x,
                    y,
                    ((255. * axis(x) * axis(y)).round() as u64 as libc::c_ulong) << 24,
                );
            }
        }
        let pix = XCreatePixmap(d, self.overlay, size, size, 32);
        let gc = XCreateGC(d, pix, 0, ptr::null_mut());
        XPutImage(d, pix, gc, xi, 0, 0, 0, 0, size, size);
        XFreeGC(d, gc);
        XDestroyImage(xi);
        self.shadow = Some(self.image(pix, vi.visual, 32, size as i32, size as i32)?);
        Ok(())
    }
    pub fn accelerated(&self) -> bool {
        self.gl.is_some()
    }
    pub fn output(&self) -> Window {
        self.gl.as_ref().map_or(0, |g| g.output)
    }
    pub unsafe fn resize(&mut self, w: i32, h: i32) {
        self.w = w.max(1);
        self.h = h.max(1);
        if let Some(gl) = &self.gl {
            gl.resize(self.w, self.h);
        } else {
            if self.target != 0 {
                XRenderFreePicture(self.d, self.target);
            }
            if self.buffer != 0 {
                XFreePixmap(self.d, self.buffer);
            }
            self.buffer = XCreatePixmap(
                self.d,
                self.overlay,
                self.w as u32,
                self.h as u32,
                XDefaultDepth(self.d, self.screen) as u32,
            );
            self.target = XRenderCreatePicture(
                self.d,
                self.buffer,
                XRenderFindVisualFormat(self.d, XDefaultVisual(self.d, self.screen)),
                0,
                ptr::null(),
            );
        }
    }
    pub unsafe fn image(
        &mut self,
        pixmap: Pixmap,
        visual: *mut Visual,
        depth: i32,
        w: i32,
        h: i32,
    ) -> Result<Rc<Image>, String> {
        let mut im = Image {
            d: self.d,
            pixmap,
            picture: 0,
            glpixmap: 0,
            texture: 0,
            release: None,
            bound: Cell::new(false),
            inverted: true,
            w,
            h,
        };
        if let Some(gl) = &mut self.gl {
            let (config, inverted) = gl
                .config(visual, depth)
                .ok_or("No GLX texture configuration for window visual")?;
            im.inverted = inverted;
            let attrs = [
                GLX_TEXTURE_TARGET_EXT,
                GLX_TEXTURE_2D_EXT,
                GLX_TEXTURE_FORMAT_EXT,
                if depth == 32 {
                    GLX_TEXTURE_FORMAT_RGBA_EXT
                } else {
                    GLX_TEXTURE_FORMAT_RGB_EXT
                },
                0,
            ];
            im.glpixmap = glXCreatePixmap(self.d, config, pixmap, attrs.as_ptr());
            im.release = Some(gl.release);
            glGenTextures(1, &mut im.texture);
            glBindTexture(TEXTURE_2D, im.texture);
            for (name, value) in [
                (GL_TEXTURE_MIN_FILTER, GL_LINEAR),
                (GL_TEXTURE_MAG_FILTER, GL_LINEAR),
                (GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE),
                (GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE),
            ] {
                glTexParameteri(TEXTURE_2D, name, value);
            }
            self.refresh(&im);
        } else {
            im.picture = self.picture(pixmap, visual);
            XRenderSetPictureFilter(self.d, im.picture, c"bilinear".as_ptr(), ptr::null_mut(), 0);
        }
        Ok(Rc::new(im))
    }
    pub unsafe fn capture(
        &mut self,
        window: Window,
        a: &XWindowAttributes,
    ) -> Result<Rc<Image>, String> {
        self.image(
            XCompositeNameWindowPixmap(self.d, window),
            a.visual,
            a.depth,
            a.width + 2 * a.border_width,
            a.height + 2 * a.border_width,
        )
    }
    pub unsafe fn refresh(&self, im: &Image) {
        if let Some(gl) = self.gl.as_ref() {
            gl.refresh(im.texture, im.glpixmap, im.bound.replace(true));
        }
    }
    pub unsafe fn begin(&self) {
        if self.accelerated() {
            glDisable(SCISSOR_TEST);
            glClearColor(0.22, 0.25, 0.29, 1.);
            glClear(GL_COLOR_BUFFER_BIT);
        } else {
            let color = XRenderColor {
                red: 14418,
                green: 16384,
                blue: 19005,
                alpha: 65535,
            };
            XRenderFillRectangle(
                self.d,
                PictOpSrc,
                self.target,
                &color,
                0,
                0,
                self.w as _,
                self.h as _,
            );
        }
    }
    unsafe fn composite(&self, picture: Picture, opacity: f64, x: i32, y: i32, w: u32, h: u32) {
        let color = XRenderColor {
            red: 0,
            green: 0,
            blue: 0,
            alpha: (65535. * opacity) as u16,
        };
        let mask = if opacity < 0.9999 {
            XRenderCreateSolidFill(self.d, &color)
        } else {
            0
        };
        XRenderComposite(
            self.d,
            PictOpOver,
            picture,
            mask,
            self.target,
            0,
            0,
            0,
            0,
            x,
            y,
            w,
            h,
        );
        if mask != 0 {
            XRenderFreePicture(self.d, mask);
        }
    }
    unsafe fn reset_clip(&self) {
        XRenderChangePicture(self.d, self.target, CPClipMask as _, &mem::zeroed());
    }
    pub unsafe fn draw(&self, im: &Image, r: Rect, opacity: f64, shape: Option<&[XRectangle]>) {
        if r.w < 0.1 || r.h < 0.1 || opacity <= 0. {
            return;
        }
        let opacity = opacity.min(1.);
        let sx = r.w / im.w as f64;
        let sy = r.h / im.h as f64;
        if self.accelerated() {
            glEnable(TEXTURE_2D);
            glBindTexture(TEXTURE_2D, im.texture);
            glColor4d(opacity, opacity, opacity, opacity);
            let quad = || textured_quad(im, r.quad(), Rect::new(0., 0., 1., 1.));
            if let Some(shape) = shape {
                glEnable(SCISSOR_TEST);
                for s in shape {
                    let x = (r.x + s.x as f64 * sx).floor() as i32;
                    let y = (r.y + s.y as f64 * sy).floor() as i32;
                    let w = (s.width as f64 * sx).ceil() as i32;
                    let h = (s.height as f64 * sy).ceil() as i32;
                    glScissor(x, self.h - y - h, w, h);
                    quad();
                }
                glDisable(SCISSOR_TEST);
            } else {
                quad()
            }
            glDisable(TEXTURE_2D);
        } else {
            let mut t = transform([[1. / sx, 0., 0.], [0., 1. / sy, 0.], [0., 0., 1.]]);
            XRenderSetPictureTransform(self.d, im.picture, &mut t);
            if let Some(shape) = shape {
                let rects: Vec<_> = shape
                    .iter()
                    .map(|s| {
                        clip_rect(
                            (r.x + s.x as f64 * sx) as _,
                            (r.y + s.y as f64 * sy) as _,
                            (s.width as f64 * sx).ceil() as _,
                            (s.height as f64 * sy).ceil() as _,
                        )
                    })
                    .collect();
                XRenderSetPictureClipRectangles(
                    self.d,
                    self.target,
                    0,
                    0,
                    rects.as_ptr(),
                    rects.len() as i32,
                );
            }
            self.composite(
                im.picture,
                opacity,
                r.x.round() as _,
                r.y.round() as _,
                r.w.ceil() as _,
                r.h.ceil() as _,
            );
            if shape.is_some() {
                self.reset_clip();
            }
        }
    }
    pub unsafe fn transformed(
        &self,
        im: &Image,
        q: Quad,
        opacity: f64,
        shape: Option<&[XRectangle]>,
    ) {
        let origin = q[0];
        let a = Point {
            x: q[1].x - origin.x,
            y: q[1].y - origin.y,
        };
        let b = Point {
            x: q[3].x - origin.x,
            y: q[3].y - origin.y,
        };
        let det = a.x * b.y - a.y * b.x;
        if opacity <= 0. || det.abs() < 1. || det.abs() / a.x.hypot(a.y).max(1.) < 0.5 {
            return;
        }
        let opacity = opacity.min(1.);
        let point = |uv: Point| Point {
            x: origin.x + uv.x * a.x + uv.y * b.x,
            y: origin.y + uv.x * a.y + uv.y * b.y,
        };
        let uv = |s: &XRectangle| {
            Rect::new(
                s.x as f64 / im.w as f64,
                s.y as f64 / im.h as f64,
                s.width as f64 / im.w as f64,
                s.height as f64 / im.h as f64,
            )
        };
        if self.accelerated() {
            glEnable(TEXTURE_2D);
            glBindTexture(TEXTURE_2D, im.texture);
            glColor4d(opacity, opacity, opacity, opacity);
            if let Some(shape) = shape {
                for s in shape {
                    let r = uv(s);
                    textured_quad(im, r.quad().map(point), r);
                }
            } else {
                textured_quad(im, q, Rect::new(0., 0., 1., 1.));
            }
            glDisable(TEXTURE_2D);
        } else {
            let (mut left, mut top, mut right, mut bottom) =
                (origin.x, origin.y, origin.x, origin.y);
            for p in q {
                left = left.min(p.x);
                top = top.min(p.y);
                right = right.max(p.x);
                bottom = bottom.max(p.y);
            }
            let x = (left.floor() as i32).clamp(0, self.w);
            let y = (top.floor() as i32).clamp(0, self.h);
            let w = (right.ceil() as i32).clamp(0, self.w) - x;
            let h = (bottom.ceil() as i32).clamp(0, self.h) - y;
            if w <= 0 || h <= 0 {
                return;
            }
            let (xx, xy, yx, yy) = (
                im.w as f64 * b.y / det,
                -(im.w as f64) * b.x / det,
                -(im.h as f64) * a.y / det,
                im.h as f64 * a.x / det,
            );
            let (dx, dy) = (x as f64 - origin.x, y as f64 - origin.y);
            let mut t = transform([
                [xx, xy, xx * dx + xy * dy],
                [yx, yy, yx * dx + yy * dy],
                [0., 0., 1.],
            ]);
            XRenderSetPictureTransform(self.d, im.picture, &mut t);
            if let Some(shape) = shape {
                let region = XCreateRegion();
                for s in shape {
                    let mut points = uv(s).quad().map(point).map(|p| XPoint {
                        x: p.x.round().clamp(-32000., 32000.) as _,
                        y: p.y.round().clamp(-32000., 32000.) as _,
                    });
                    let part = XPolygonRegion(points.as_mut_ptr(), 4, WindingRule);
                    XUnionRegion(region, part, region);
                    XDestroyRegion(part);
                }
                XRenderSetPictureClipRegion(self.d, self.target, region);
                XDestroyRegion(region);
            }
            self.composite(im.picture, opacity, x, y, w as _, h as _);
            if shape.is_some() {
                self.reset_clip();
            }
        }
    }
    pub unsafe fn shadow_at(&self, r: Rect, opacity: f64, radius: i32, offset: i32) {
        let Some(im) = &self.shadow else { return };
        if opacity <= 0. || radius <= 0 {
            return;
        }
        let (left, right) = (
            (r.x.floor() as i32).clamp(0, self.w),
            ((r.x + r.w).ceil() as i32).clamp(0, self.w),
        );
        let (top, bottom) = (
            (r.y.floor() as i32).clamp(0, self.h),
            ((r.y + r.h).ceil() as i32).clamp(0, self.h),
        );
        let clips = [
            clip_rect(0, 0, self.w, top),
            clip_rect(0, bottom, self.w, self.h - bottom),
            clip_rect(0, top, left, bottom - top),
            clip_rect(right, top, self.w - right, bottom - top),
        ];
        let gl = self.accelerated();
        if gl {
            glEnable(SCISSOR_TEST);
            glEnable(TEXTURE_2D);
            glBindTexture(TEXTURE_2D, im.texture);
            glColor4d(opacity, opacity, opacity, opacity);
        } else {
            XRenderSetPictureClipRectangles(self.d, self.target, 0, 0, clips.as_ptr(), 4);
        }
        let radius = radius as f64;
        let offset = offset as f64;
        let edge = (2. * radius).min(r.w.min(r.h) * 0.5 + radius);
        let edges = |start, len| {
            [
                start - radius + offset,
                start - radius + offset + edge,
                start + len + radius + offset - edge,
                start + len + radius + offset,
            ]
        };
        let xs = edges(r.x, r.w);
        let ys = edges(r.y, r.h);
        let uv = [0., 32. / 65., 33. / 65., 1.];
        for j in 0..3 {
            for i in 0..3 {
                let dst = Rect::new(xs[i], ys[j], xs[i + 1] - xs[i], ys[j + 1] - ys[j]);
                if dst.w <= 0. || dst.h <= 0. {
                    continue;
                }
                let tex = Rect::new(uv[i], uv[j], uv[i + 1] - uv[i], uv[j + 1] - uv[j]);
                if gl {
                    for s in clips {
                        if s.width != 0 && s.height != 0 {
                            glScissor(
                                i32::from(s.x),
                                self.h - i32::from(s.y) - i32::from(s.height),
                                i32::from(s.width),
                                i32::from(s.height),
                            );
                            textured_quad(im, dst.quad(), tex);
                        }
                    }
                } else {
                    let mut t = transform([
                        [tex.w * 65. / dst.w, 0., tex.x * 65.],
                        [0., tex.h * 65. / dst.h, tex.y * 65.],
                        [0., 0., 1.],
                    ]);
                    XRenderSetPictureTransform(self.d, im.picture, &mut t);
                    self.composite(
                        im.picture,
                        opacity,
                        dst.x.round() as _,
                        dst.y.round() as _,
                        dst.w.ceil() as _,
                        dst.h.ceil() as _,
                    );
                }
            }
        }
        if gl {
            glDisable(TEXTURE_2D);
            glDisable(SCISSOR_TEST);
        } else {
            self.reset_clip();
        }
    }
    pub unsafe fn line(&self, q: Quad, opacity: f64) {
        if self.accelerated() {
            glDisable(TEXTURE_2D);
            glColor4d(0.8 * opacity, 0.9 * opacity, opacity, opacity);
            glBegin(GL_LINE_LOOP);
            for p in q {
                glVertex2d(p.x, p.y);
            }
            glEnd();
        } else {
            let mut v: XGCValues = mem::zeroed();
            v.foreground = 0xb8d8f0;
            let gc = XCreateGC(self.d, self.buffer, GCForeground as _, &mut v);
            let mut points: [XPoint; 5] = std::array::from_fn(|i| XPoint {
                x: q[i % 4].x.clamp(-32000., 32000.) as _,
                y: q[i % 4].y.clamp(-32000., 32000.) as _,
            });
            XDrawLines(
                self.d,
                self.buffer,
                gc,
                points.as_mut_ptr(),
                5,
                CoordModeOrigin,
            );
            XFreeGC(self.d, gc);
        }
    }
    pub unsafe fn present(&self) {
        if let Some(gl) = &self.gl {
            gl.present();
        } else {
            XRenderComposite(
                self.d,
                PictOpSrc,
                self.target,
                0,
                self.destination,
                0,
                0,
                0,
                0,
                0,
                0,
                self.w as _,
                self.h as _,
            );
        }
        XFlush(self.d);
    }
}
impl Drop for Renderer {
    fn drop(&mut self) {
        self.shadow.take();
        // SAFETY: destination, target, and buffer belong to this open display.
        unsafe {
            if self.target != 0 {
                XRenderFreePicture(self.d, self.target);
            }
            if self.buffer != 0 {
                XFreePixmap(self.d, self.buffer);
            }
            if self.destination != 0 {
                XRenderFreePicture(self.d, self.destination);
            }
        }
    }
}
