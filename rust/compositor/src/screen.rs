// SPDX-License-Identifier: GPL-3.0-only
use crate::native::*;
use libc::{c_int, c_long, c_ulong};
use std::{
    ffi::{CStr, CString},
    mem, ptr,
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
};
use x11::{xfixes::*, xlib::*, xrandr::*, xrender::*};

pub static STOP: AtomicBool = AtomicBool::new(false);
pub static RELOAD: AtomicBool = AtomicBool::new(false);
static TRAPPING: AtomicBool = AtomicBool::new(false);
static LAST_ERROR: AtomicI32 = AtomicI32::new(0);
static DAMAGE_ERROR: AtomicI32 = AtomicI32::new(0);

extern "C" fn signal(sig: c_int) {
    match sig {
        libc::SIGHUP | libc::SIGUSR1 => &RELOAD,
        _ => &STOP,
    }
    .store(true, Ordering::Relaxed);
}
unsafe extern "C" fn error(d: *mut Display, error: *mut XErrorEvent) -> c_int {
    let error = &*error;
    let code = error.error_code;
    LAST_ERROR.store(code.into(), Ordering::Relaxed);
    if !TRAPPING.load(Ordering::Relaxed)
        && i32::from(code) != DAMAGE_ERROR.load(Ordering::Relaxed)
        && ![BadWindow, BadPixmap, BadDrawable].contains(&code)
    {
        let mut text = [0; 160];
        XGetErrorText(d, code.into(), text.as_mut_ptr(), text.len() as _);
        eprintln!(
            "emwm-compositor: X11 {} request={}.{} resource={}",
            CStr::from_ptr(text.as_ptr()).to_string_lossy(),
            error.request_code,
            error.minor_code,
            error.resourceid
        );
    }
    0
}
pub struct Atoms {
    pub client: Atom,
    pub window_type: Atom,
    pub desktop: Atom,
    pub dock: Atom,
    pub opacity: Atom,
    pub state: Atom,
    pub fullscreen: Atom,
    pub current_desktop: Atom,
    pub wm_info: Atom,
    pub workspace_list: Atom,
    pub presence: Atom,
    pub all_workspaces: Atom,
    pub desktop_names: Atom,
    pub active_window: Atom,
    pub wm_name: Atom,
    pub wm_state: Atom,
    pub skip_taskbar: Atom,
    pub animation: Atom,
    pub last_animation: Atom,
    pub backend: Atom,
}
impl Atoms {
    unsafe fn intern(d: *mut Display) -> Self {
        let atom = |name: &CStr| XInternAtom(d, name.as_ptr(), 0);
        Self {
            client: atom(c"_EMWM_FRAME_CLIENT"),
            window_type: atom(c"_NET_WM_WINDOW_TYPE"),
            desktop: atom(c"_NET_WM_WINDOW_TYPE_DESKTOP"),
            dock: atom(c"_NET_WM_WINDOW_TYPE_DOCK"),
            opacity: atom(c"_NET_WM_WINDOW_OPACITY"),
            state: atom(c"_NET_WM_STATE"),
            fullscreen: atom(c"_NET_WM_STATE_FULLSCREEN"),
            current_desktop: atom(c"_NET_CURRENT_DESKTOP"),
            wm_info: atom(c"_MOTIF_WM_INFO"),
            workspace_list: atom(c"_MWM_WORKSPACE_LIST"),
            presence: atom(c"_MWM_WORKSPACE_PRESENCE"),
            all_workspaces: atom(c"all"),
            desktop_names: atom(c"_NET_DESKTOP_NAMES"),
            active_window: atom(c"_NET_ACTIVE_WINDOW"),
            wm_name: atom(c"_NET_WM_NAME"),
            wm_state: atom(c"WM_STATE"),
            skip_taskbar: atom(c"_NET_WM_STATE_SKIP_TASKBAR"),
            animation: atom(c"_EMWM_ANIMATION"),
            last_animation: atom(c"_EMWM_LAST_ANIMATION"),
            backend: atom(c"_EMWM_COMPOSITOR_BACKEND"),
        }
    }
}
pub struct Screen {
    pub d: *mut Display,
    pub screen: i32,
    pub root: Window,
    pub owner: Window,
    pub input: Window,
    pub overlay: Window,
    pub selection: Atom,
    pub damage_event: i32,
    pub shape_event: i32,
    pub randr_event: Option<i32>,
    pub atoms: Atoms,
    redirected: bool,
}
impl Screen {
    pub fn open() -> Result<Self, String> {
        // SAFETY: all Xlib and GLX interaction occurs on this process's sole render thread.
        unsafe {
            let d = XOpenDisplay(ptr::null());
            if d.is_null() {
                return Err("cannot open X display".into());
            }
            XSetErrorHandler(Some(error));
            for sig in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP, libc::SIGUSR1] {
                libc::signal(sig, signal as *const () as libc::sighandler_t);
            }
            let screen = XDefaultScreen(d);
            let mut s = Self {
                d,
                screen,
                root: XRootWindow(d, screen),
                owner: 0,
                input: 0,
                overlay: 0,
                selection: 0,
                damage_event: 0,
                shape_event: 0,
                randr_event: None,
                atoms: Atoms::intern(d),
                redirected: false,
            };
            let (mut err, mut ev, mut major, mut minor, mut damage_error) = (0, 0, 0, 0, 0);
            if XCompositeQueryExtension(d, &mut ev, &mut err) == 0
                || XDamageQueryExtension(d, &mut s.damage_event, &mut damage_error) == 0
                || XFixesQueryExtension(d, &mut ev, &mut err) == 0
                || XRenderQueryExtension(d, &mut ev, &mut err) == 0
                || XShapeQueryExtension(d, &mut s.shape_event, &mut err) == 0
            {
                return Err("Required X11 extensions unavailable".into());
            }
            DAMAGE_ERROR.store(damage_error, Ordering::Relaxed);
            XCompositeQueryVersion(d, &mut major, &mut minor);
            if major == 0 && minor < 3 {
                return Err("XComposite 0.3 or newer is required".into());
            }
            if XRRQueryExtension(d, &mut ev, &mut err) != 0 {
                s.randr_event = Some(ev);
                XRRSelectInput(d, s.root, RRScreenChangeNotifyMask);
            }
            let selection =
                CString::new(format!("_NET_WM_CM_S{screen}")).expect("numeric screen has no NUL");
            s.selection = XInternAtom(d, selection.as_ptr(), 0);
            XGrabServer(d);
            if XGetSelectionOwner(d, s.selection) != 0 {
                XUngrabServer(d);
                return Err("Another compositor already owns this X screen".into());
            }
            s.owner = XCreateSimpleWindow(d, s.root, -1, -1, 1, 1, 0, 0, 0);
            XStoreName(d, s.owner, c"EMWM Compositor".as_ptr());
            XSetSelectionOwner(d, s.selection, s.owner, CurrentTime);
            XUngrabServer(d);
            XSync(d, 0);
            if XGetSelectionOwner(d, s.selection) != s.owner {
                return Err("Cannot acquire compositor selection".into());
            }
            let mut event: XEvent = mem::zeroed();
            let msg = &mut event.client_message;
            msg.type_ = ClientMessage;
            msg.window = s.root;
            msg.message_type = XInternAtom(d, c"MANAGER".as_ptr(), 0);
            msg.format = 32;
            msg.data.set_long(1, s.selection as c_long);
            msg.data.set_long(2, s.owner as c_long);
            XSendEvent(d, s.root, 0, StructureNotifyMask, &mut event);
            if !s.trap(|| XCompositeRedirectSubwindows(d, s.root, 1)).1 {
                return Err(
                    "Cannot redirect X11 windows (another compositor may be active)".into(),
                );
            }
            // The overview receives input through an ordinary window. Screen lockers
            // and other modal X11 clients can still acquire the keyboard and pointer.
            let mut attrs: XSetWindowAttributes = mem::zeroed();
            attrs.override_redirect = True;
            attrs.event_mask = KeyPressMask
                | KeyReleaseMask
                | ButtonPressMask
                | ButtonReleaseMask
                | PointerMotionMask
                | FocusChangeMask;
            s.input = XCreateWindow(
                d,
                s.root,
                0,
                0,
                1,
                1,
                0,
                0,
                InputOnly as _,
                ptr::null_mut(),
                CWOverrideRedirect | CWEventMask,
                &mut attrs,
            );
            s.redirected = true;
            s.overlay = XCompositeGetOverlayWindow(d, s.root);
            let empty = XFixesCreateRegion(d, ptr::null_mut(), 0);
            XFixesSetWindowShapeRegion(d, s.overlay, 2, 0, 0, empty);
            XFixesDestroyRegion(d, empty);
            XSelectInput(
                d,
                s.root,
                SubstructureNotifyMask | StructureNotifyMask | PropertyChangeMask,
            );
            Ok(s)
        }
    }
    pub unsafe fn trap<T>(&self, call: impl FnOnce() -> T) -> (T, bool) {
        XSync(self.d, 0);
        LAST_ERROR.store(0, Ordering::Relaxed);
        TRAPPING.store(true, Ordering::Relaxed);
        let result = call();
        XSync(self.d, 0);
        TRAPPING.store(false, Ordering::Relaxed);
        (result, LAST_ERROR.load(Ordering::Relaxed) == 0)
    }
    pub unsafe fn property(&self, w: Window, atom: Atom, ty: Atom) -> Vec<c_ulong> {
        let (mut actual, mut format, mut count, mut left, mut data) = (0, 0, 0, 0, ptr::null_mut());
        let status = XGetWindowProperty(
            self.d,
            w,
            atom,
            0,
            64,
            0,
            ty,
            &mut actual,
            &mut format,
            &mut count,
            &mut left,
            &mut data,
        );
        if data.is_null() {
            return vec![];
        }
        let result = if status == 0 && actual == ty && format == 32 {
            std::slice::from_raw_parts(data.cast(), count as usize).to_vec()
        } else {
            vec![]
        };
        XFree(data.cast());
        result
    }
    pub unsafe fn strings(&self, window: Window, property: Atom) -> Vec<String> {
        let mut value: XTextProperty = mem::zeroed();
        if XGetTextProperty(self.d, window, &mut value, property) == 0 || value.value.is_null() {
            return Vec::new();
        }
        let strings = if value.format == 8 {
            std::slice::from_raw_parts(value.value, value.nitems as usize)
                .split(|&b| b == 0)
                .map(|s| String::from_utf8_lossy(s).into_owned())
                .collect()
        } else {
            Vec::new()
        };
        XFree(value.value.cast());
        strings
    }
    pub unsafe fn request(&self, window: Window, message: Atom, value: usize, time: Time) {
        let mut event: XEvent = mem::zeroed();
        let msg = &mut event.client_message;
        msg.type_ = ClientMessage;
        msg.window = window;
        msg.message_type = message;
        msg.format = 32;
        msg.data.set_long(0, value as _);
        msg.data.set_long(1, time as _);
        XSendEvent(
            self.d,
            self.root,
            False,
            SubstructureRedirectMask | SubstructureNotifyMask,
            &mut event,
        );
    }
    pub unsafe fn attrs(&self, w: Window) -> Option<XWindowAttributes> {
        let mut a = mem::zeroed();
        (XGetWindowAttributes(self.d, w, &mut a) != 0).then_some(a)
    }
    pub unsafe fn children(&self) -> Vec<Window> {
        let (mut root, mut parent, mut children, mut count) = (0, 0, ptr::null_mut(), 0);
        XQueryTree(
            self.d,
            self.root,
            &mut root,
            &mut parent,
            &mut children,
            &mut count,
        );
        if children.is_null() {
            return vec![];
        }
        let result = std::slice::from_raw_parts(children, count as usize).to_vec();
        XFree(children.cast());
        result
    }
    pub unsafe fn dimensions(&self) -> (i32, i32) {
        (
            XDisplayWidth(self.d, self.screen),
            XDisplayHeight(self.d, self.screen),
        )
    }
    pub unsafe fn backend(&self, accelerated: bool) {
        let name = if accelerated { "glx" } else { "xrender" };
        XChangeProperty(
            self.d,
            self.owner,
            self.atoms.backend,
            XA_STRING,
            8,
            PropModeReplace,
            name.as_ptr(),
            name.len() as _,
        );
    }
}
impl Drop for Screen {
    fn drop(&mut self) {
        // SAFETY: other compositor resources have already been destroyed before closing this display.
        unsafe {
            if self.redirected {
                XCompositeUnredirectSubwindows(self.d, self.root, 1);
            }
            if self.overlay != 0 {
                XCompositeReleaseOverlayWindow(self.d, self.root);
            }
            if self.owner != 0 {
                if XGetSelectionOwner(self.d, self.selection) == self.owner {
                    XSetSelectionOwner(self.d, self.selection, 0, CurrentTime);
                }
                XDestroyWindow(self.d, self.owner);
            }
            XSync(self.d, 0);
            XCloseDisplay(self.d);
        }
    }
}
