// SPDX-License-Identifier: GPL-3.0-only
//! APIs missing from the x11 crate. Signatures follow Xdamage.h, Xcomposite.h,
//! shape.h and GL/gl.h. All resources belong to the compositor's X connection.
use libc::{c_int, c_uchar, c_uint, c_ulong};
use x11::xlib::{Display, Window, XRectangle};
#[link(name = "Xcomposite")]
extern "C" {
    pub fn XCompositeQueryExtension(d: *mut Display, event: *mut i32, error: *mut i32) -> c_int;
    pub fn XCompositeQueryVersion(d: *mut Display, major: *mut i32, minor: *mut i32) -> c_int;
    pub fn XCompositeRedirectSubwindows(d: *mut Display, window: Window, update: c_int);
    pub fn XCompositeUnredirectSubwindows(d: *mut Display, window: Window, update: c_int);
    pub fn XCompositeGetOverlayWindow(d: *mut Display, root: Window) -> Window;
    pub fn XCompositeReleaseOverlayWindow(d: *mut Display, root: Window);
    pub fn XCompositeNameWindowPixmap(d: *mut Display, window: Window) -> c_ulong;
}
#[link(name = "Xdamage")]
extern "C" {
    pub fn XDamageQueryExtension(d: *mut Display, event: *mut i32, error: *mut i32) -> c_int;
    pub fn XDamageCreate(d: *mut Display, drawable: Window, level: c_int) -> c_ulong;
    pub fn XDamageDestroy(d: *mut Display, damage: c_ulong);
    pub fn XDamageSubtract(d: *mut Display, damage: c_ulong, repair: c_ulong, parts: c_ulong);
}
#[link(name = "Xext")]
extern "C" {
    pub fn XShapeQueryExtension(d: *mut Display, event: *mut i32, error: *mut i32) -> c_int;
    pub fn XShapeSelectInput(d: *mut Display, window: Window, mask: c_ulong);
    pub fn XShapeGetRectangles(
        d: *mut Display,
        window: Window,
        kind: c_int,
        count: *mut c_int,
        ordering: *mut c_int,
    ) -> *mut XRectangle;
}
#[link(name = "GL")]
extern "C" {
    pub fn glEnable(cap: c_uint);
    pub fn glDisable(cap: c_uint);
    pub fn glBlendFunc(src: c_uint, dst: c_uint);
    pub fn glTexEnvi(target: c_uint, name: c_uint, param: c_int);
    pub fn glTexParameteri(target: c_uint, name: c_uint, param: c_int);
    pub fn glGetString(name: c_uint) -> *const c_uchar;
    pub fn glViewport(x: i32, y: i32, w: i32, h: i32);
    pub fn glMatrixMode(mode: c_uint);
    pub fn glLoadIdentity();
    pub fn glOrtho(l: f64, r: f64, b: f64, t: f64, near: f64, far: f64);
    pub fn glGenTextures(n: i32, names: *mut u32);
    pub fn glDeleteTextures(n: i32, names: *const u32);
    pub fn glBindTexture(target: u32, name: u32);
    pub fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    pub fn glClear(mask: u32);
    pub fn glColor4d(r: f64, g: f64, b: f64, a: f64);
    pub fn glBegin(mode: u32);
    pub fn glEnd();
    pub fn glTexCoord2d(s: f64, t: f64);
    pub fn glVertex2d(x: f64, y: f64);
    pub fn glScissor(x: i32, y: i32, w: i32, h: i32);
}
pub const TEXTURE_2D: u32 = 0x0DE1;
pub const SCISSOR_TEST: u32 = 0x0C11;
pub const QUADS: u32 = 7;
pub const FRONT_LEFT: i32 = 0x20de;
#[link(name = "m")]
extern "C" {
    pub fn erf(x: f64) -> f64;
}

// OpenGL and GLX_EXT_texture_from_pixmap tokens used by the fixed-function renderer.
pub const GL_BLEND: u32 = 0x0be2;
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
pub const GL_DEPTH_TEST: u32 = 0x0b71;
pub const GL_TEXTURE_ENV: u32 = 0x2300;
pub const GL_TEXTURE_ENV_MODE: u32 = 0x2200;
pub const GL_MODULATE: i32 = 0x2100;
pub const GL_RENDERER: u32 = 0x1f01;
pub const GL_PROJECTION: u32 = 0x1701;
pub const GL_MODELVIEW: u32 = 0x1700;
pub const GLX_BIND_TO_TEXTURE_RGBA_EXT: i32 = 0x20d1;
pub const GLX_BIND_TO_TEXTURE_RGB_EXT: i32 = 0x20d0;
pub const GLX_BIND_TO_TEXTURE_TARGETS_EXT: i32 = 0x20d3;
pub const GLX_Y_INVERTED_EXT: i32 = 0x20d4;
pub const GLX_TEXTURE_TARGET_EXT: i32 = 0x20d6;
pub const GLX_TEXTURE_2D_EXT: i32 = 0x20dc;
pub const GLX_TEXTURE_FORMAT_EXT: i32 = 0x20d5;
pub const GLX_TEXTURE_FORMAT_RGBA_EXT: i32 = 0x20da;
pub const GLX_TEXTURE_FORMAT_RGB_EXT: i32 = 0x20d9;
pub const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const GL_TEXTURE_WRAP_S: u32 = 0x2802;
pub const GL_TEXTURE_WRAP_T: u32 = 0x2803;
pub const GL_LINEAR: i32 = 0x2601;
pub const GL_CLAMP_TO_EDGE: i32 = 0x812f;
pub const GL_COLOR_BUFFER_BIT: u32 = 0x4000;
pub const GL_LINE_LOOP: u32 = 2;
pub const GLX_TEXTURE_2D_BIT_EXT: i32 = 2;
