// SPDX-License-Identifier: GPL-3.0-only
//! GLX context and texture-from-pixmap imports, owned by the X11 render thread.
use crate::native::*;
use std::{collections::HashMap, ffi::CStr, mem, ptr};
use x11::{glx::*, xfixes::*, xlib::*};
pub type Release = unsafe extern "C" fn(*mut Display, GLXPixmap, i32);
type Bind = unsafe extern "C" fn(*mut Display, GLXPixmap, i32, *const i32);
pub struct Gl {
    d: *mut Display,
    screen: i32,
    pub output: Window,
    drawable: GLXWindow,
    context: GLXContext,
    colormap: Colormap,
    bind: Bind,
    pub release: Release,
    configs: HashMap<(VisualID, i32), (GLXFBConfig, bool)>,
}
impl Gl {
    // SAFETY: caller supplies a live connection and overlay on its default screen.
    pub unsafe fn open(d: *mut Display, screen: i32, overlay: Window, vsync: bool) -> Option<Self> {
        let (mut major, mut minor) = (0, 0);
        if glXQueryVersion(d, &mut major, &mut minor) == 0 || (major, minor) < (1, 3) {
            return None;
        }
        let extensions = glXQueryExtensionsString(d, screen);
        if extensions.is_null() {
            return None;
        }
        let extensions = CStr::from_ptr(extensions).to_bytes();
        let has = |name: &[u8]| extensions.split(|b| *b == b' ').any(|word| word == name);
        if !has(b"GLX_EXT_texture_from_pixmap") {
            return None;
        }
        let bind = glXGetProcAddressARB(c"glXBindTexImageEXT".as_ptr().cast())?;
        let release = glXGetProcAddressARB(c"glXReleaseTexImageEXT".as_ptr().cast())?;
        let mut gl = Self {
            d,
            screen,
            output: 0,
            drawable: 0,
            context: ptr::null_mut(),
            colormap: 0,
            bind: mem::transmute::<unsafe extern "C" fn(), Bind>(bind),
            release: mem::transmute::<unsafe extern "C" fn(), Release>(release),
            configs: HashMap::new(),
        };
        let attrs = [
            GLX_X_RENDERABLE,
            1,
            GLX_DRAWABLE_TYPE,
            GLX_WINDOW_BIT,
            GLX_RENDER_TYPE,
            GLX_RGBA_BIT,
            GLX_DOUBLEBUFFER,
            1,
            GLX_RED_SIZE,
            8,
            GLX_GREEN_SIZE,
            8,
            GLX_BLUE_SIZE,
            8,
            0,
        ];
        let mut n = 0;
        let configs = glXChooseFBConfig(d, screen, attrs.as_ptr(), &mut n);
        if configs.is_null() {
            return None;
        }
        let mut found = None;
        for &config in std::slice::from_raw_parts(configs, n as usize) {
            let vi = glXGetVisualFromFBConfig(d, config);
            if !vi.is_null() {
                let info = *vi;
                XFree(vi.cast());
                if info.depth == XDefaultDepth(d, screen) {
                    found = Some((config, info));
                    break;
                }
            }
        }
        XFree(configs.cast());
        let (config, vi) = found?;
        gl.context = glXCreateNewContext(d, config, GLX_RGBA_TYPE, ptr::null_mut(), 1);
        if gl.context.is_null() {
            return None;
        }
        gl.colormap = XCreateColormap(d, overlay, vi.visual, AllocNone);
        let mut wa: XSetWindowAttributes = mem::zeroed();
        wa.colormap = gl.colormap;
        wa.override_redirect = 1;
        gl.output = XCreateWindow(
            d,
            overlay,
            0,
            0,
            1,
            1,
            0,
            vi.depth,
            InputOutput as u32,
            vi.visual,
            CWColormap | CWBorderPixel | CWOverrideRedirect,
            &mut wa,
        );
        gl.drawable = glXCreateWindow(d, config, gl.output, ptr::null());
        if glXMakeContextCurrent(d, gl.drawable, gl.drawable, gl.context) == 0 {
            return None;
        }
        let empty = XFixesCreateRegion(d, ptr::null_mut(), 0);
        XFixesSetWindowShapeRegion(d, gl.output, 2, 0, 0, empty);
        XFixesDestroyRegion(d, empty);
        XMapWindow(d, gl.output);
        glEnable(GL_BLEND);
        glBlendFunc(1, GL_ONE_MINUS_SRC_ALPHA);
        glDisable(GL_DEPTH_TEST);
        glTexEnvi(GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_MODULATE);
        let mut sync = false;
        if has(b"GLX_EXT_swap_control") {
            if let Some(f) = glXGetProcAddressARB(c"glXSwapIntervalEXT".as_ptr().cast()) {
                let swap = mem::transmute::<
                    unsafe extern "C" fn(),
                    unsafe extern "C" fn(*mut Display, GLXDrawable, i32),
                >(f);
                swap(d, gl.drawable, vsync.into());
                sync = vsync;
            }
        } else if has(b"GLX_MESA_swap_control") {
            if let Some(f) = glXGetProcAddressARB(c"glXSwapIntervalMESA".as_ptr().cast()) {
                let swap =
                    mem::transmute::<unsafe extern "C" fn(), unsafe extern "C" fn(u32) -> i32>(f);
                sync = swap(vsync.into()) == 0 && vsync;
            }
        }
        let renderer = glGetString(GL_RENDERER);
        let renderer = if renderer.is_null() {
            "unknown".into()
        } else {
            CStr::from_ptr(renderer.cast()).to_string_lossy()
        };
        eprintln!(
            "emwm-compositor: backend=glx renderer={renderer} direct={} vsync={}",
            glXIsDirect(d, gl.context),
            i32::from(sync)
        );
        Some(gl)
    }
    pub unsafe fn config(
        &mut self,
        visual: *mut Visual,
        depth: i32,
    ) -> Option<(GLXFBConfig, bool)> {
        let key = (XVisualIDFromVisual(visual), depth);
        if let Some(&config) = self.configs.get(&key) {
            return Some(config);
        }
        let mut n = 0;
        let configs = glXGetFBConfigs(self.d, self.screen, &mut n);
        if configs.is_null() {
            return None;
        }
        let mut found = None;
        for &config in std::slice::from_raw_parts(configs, n as usize) {
            let vi = glXGetVisualFromFBConfig(self.d, config);
            if vi.is_null() {
                continue;
            }
            let info = *vi;
            XFree(vi.cast());
            if info.depth != depth
                || info.class != TrueColor
                || info.red_mask != (*visual).red_mask
                || info.green_mask != (*visual).green_mask
                || info.blue_mask != (*visual).blue_mask
            {
                continue;
            }
            let attr = |name| {
                let mut val = 0;
                glXGetFBConfigAttrib(self.d, config, name, &mut val);
                val
            };
            if attr(GLX_DRAWABLE_TYPE) & GLX_PIXMAP_BIT != 0
                && attr(if depth == 32 {
                    GLX_BIND_TO_TEXTURE_RGBA_EXT
                } else {
                    GLX_BIND_TO_TEXTURE_RGB_EXT
                }) != 0
                && attr(GLX_BIND_TO_TEXTURE_TARGETS_EXT) & GLX_TEXTURE_2D_BIT_EXT != 0
            {
                found = Some((config, attr(GLX_Y_INVERTED_EXT) != 0));
                break;
            }
        }
        XFree(configs.cast());
        if let Some(pair) = found {
            self.configs.insert(key, pair);
        }
        found
    }
    pub unsafe fn refresh(&self, texture: u32, pixmap: GLXPixmap, bound: bool) {
        glBindTexture(TEXTURE_2D, texture);
        if bound {
            (self.release)(self.d, pixmap, FRONT_LEFT)
        }
        (self.bind)(self.d, pixmap, FRONT_LEFT, ptr::null());
    }
    pub unsafe fn resize(&self, w: i32, h: i32) {
        XResizeWindow(self.d, self.output, w as u32, h as u32);
        glViewport(0, 0, w, h);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glOrtho(0., w as f64, h as f64, 0., -1., 1.);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();
    }
    pub unsafe fn present(&self) {
        glXSwapBuffers(self.d, self.drawable);
    }
}
impl Drop for Gl {
    fn drop(&mut self) {
        // SAFETY: owning context is still live during renderer teardown, including partial initialization.
        unsafe {
            if !self.context.is_null() {
                glXMakeContextCurrent(self.d, 0, 0, ptr::null_mut());
                if self.drawable != 0 {
                    glXDestroyWindow(self.d, self.drawable);
                }
                glXDestroyContext(self.d, self.context);
            }
            if self.output != 0 {
                XDestroyWindow(self.d, self.output);
            }
            if self.colormap != 0 {
                XFreeColormap(self.d, self.colormap);
            }
        }
    }
}
