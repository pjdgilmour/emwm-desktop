// SPDX-License-Identifier: MIT
// Rust port of xmsm by alx@fastestcode.org; see LICENSE.
mod auth;
mod resources;
#[macro_use]
mod motif;
mod lock;
mod logout;
use auth::{Access, Request};
use lock::Lock;
use logout::Logout;
use motif::*;
use resources::Resources;
use std::{
    cell::{Cell, OnceCell},
    ffi::{c_char, c_int, c_ulong, c_void, CStr, CString, OsStr},
    io,
    os::unix::{ffi::OsStrExt, process::CommandExt},
    path::PathBuf,
    process::{Command, ExitCode},
    ptr::null_mut,
    sync::atomic::{AtomicUsize, Ordering},
};
use x11::{xinerama, xlib::*, xrandr, xss, xt};

struct App {
    shell: W,
    ctx: xt::XtAppContext,
    dpy: *mut Display,
    root: Window,
    res: Resources,
    access: Access,
    mgr: Atom,
    pid: Atom,
    cmd: Atom,
    cfg: Atom,
    wm_messages: Atom,
    wm_offset: Atom,
    lock: OnceCell<Lock>,
    logout: OnceCell<Logout>,
    error: Cell<W>,
    lock_disabled: Cell<bool>,
    lock_timer: Cell<c_ulong>,
    screensaver: Cell<Option<i32>>,
    randr: Cell<Option<i32>>,
}
static SIGNAL_LOCK: AtomicUsize = AtomicUsize::new(0);
static SIGNAL_LOGOUT: AtomicUsize = AtomicUsize::new(0);
unsafe extern "C" fn notice(sig: c_int) {
    let n = if sig == libc::SIGUSR1 {
        SIGNAL_LOCK.load(Ordering::Relaxed)
    } else {
        SIGNAL_LOGOUT.load(Ordering::Relaxed)
    };
    if n != 0 {
        xt::XtNoticeSignal(n as c_ulong);
    }
}
unsafe extern "C" fn reap(_: c_int) {
    let errno = libc::__errno_location();
    let saved = *errno;
    while libc::waitpid(-1, null_mut(), libc::WNOHANG) > 0 {}
    *errno = saved;
}
unsafe extern "C" fn signal_lock(p: *mut c_void, _: *mut c_ulong) {
    let app = &*(p as *const App);
    app.request_lock(true);
}
unsafe extern "C" fn signal_logout(p: *mut c_void, _: *mut c_ulong) {
    let app = &*(p as *const App);
    app.show_logout();
}
unsafe extern "C" fn lock_timeout(p: *mut c_void, _: *mut c_ulong) {
    let app = &*(p as *const App);
    app.lock_timer.set(0);
    if let Some(lock) = app.lock.get() {
        lock.start(app);
    }
}
unsafe extern "C" fn blank_timeout(p: *mut c_void, _: *mut c_ulong) {
    let app = &*(p as *const App);
    if app.lock.get().is_some_and(|l| l.active.get()) {
        XForceScreenSaver(app.dpy, ScreenSaverActive);
    }
}
fn signals() -> io::Result<()> {
    // SAFETY: the three handlers only use async-signal-safe system/Xt signal APIs.
    for (sig, handler) in [
        (libc::SIGUSR1, notice as unsafe extern "C" fn(_)),
        (libc::SIGUSR2, notice),
        (libc::SIGCHLD, reap),
    ] {
        unsafe {
            let mut action: libc::sigaction = std::mem::zeroed();
            action.sa_sigaction = handler as usize;
            action.sa_flags = libc::SA_RESTART | libc::SA_NOCLDSTOP;
            libc::sigemptyset(&mut action.sa_mask);
            if libc::sigaction(sig, &action, null_mut()) != 0 {
                return Err(io::Error::last_os_error());
            }
        }
    }
    Ok(())
}
unsafe extern "C" fn ignore_stale(_: *mut Display, _: *mut XErrorEvent) -> c_int {
    0
}
fn property(d: *mut Display, w: Window, atom: Atom, kind: Atom) -> Option<c_ulong> {
    let (mut actual, mut format, mut n, mut left, mut data) = (0, 0, 0, 0, null_mut());
    // SAFETY: initialized output addresses; Xlib returns native longs for 32-bit properties.
    unsafe {
        let rc = XGetWindowProperty(
            d,
            w,
            atom,
            0,
            1,
            False,
            kind,
            &mut actual,
            &mut format,
            &mut n,
            &mut left,
            &mut data,
        );
        let value = if rc == 0 && actual == kind && format == 32 && n == 1 && !data.is_null() {
            Some(*data.cast::<c_ulong>())
        } else {
            None
        };
        if !data.is_null() {
            XFree(data.cast());
        }
        value
    }
}
fn launch(bytes: &[u8]) -> io::Result<()> {
    let mut argv = bytes
        .split(|c| *c == b' ')
        .filter(|s| !s.is_empty())
        .map(OsStr::from_bytes);
    let path = argv
        .next()
        .ok_or_else(|| io::Error::from_raw_os_error(libc::ENOENT))?;
    // execv in the original resolves a bare executable against the current directory.
    let path = if path.as_bytes().contains(&b'/') {
        PathBuf::from(path)
    } else {
        PathBuf::from(".").join(path)
    };
    Command::new(path).args(argv).process_group(0).spawn()?;
    Ok(())
}
fn sessionetc() {
    let Some(home) = std::env::var_os("HOME") else {
        eprintln!("[XMSM] HOME is not set");
        return;
    };
    let path = PathBuf::from(home).join(".sessionetc");
    match std::fs::File::open(&path) {
        Ok(_) => {
            if let Err(e) = Command::new("/bin/sh").arg(&path).process_group(0).spawn() {
                eprintln!("[XMSM] shell execution failed with: {e}");
            }
        }
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            eprintln!("[XMSM] No read permission for {}", path.display())
        }
        Err(_) => {}
    }
}
impl App {
    fn data(&self) -> *mut c_void {
        (self as *const Self).cast_mut().cast()
    }
    fn bell(&self) {
        if self.res.silent == 0 {
            unsafe {
                XBell(self.dpy, 100);
            }
        }
    }
    fn window(&self) -> Window {
        unsafe { xt::XtWindow(self.shell) }
    }
    fn publish(&self, w: Window, key: Atom, kind: Atom, value: c_ulong) {
        unsafe {
            XChangeProperty(
                self.dpy,
                w,
                key,
                kind,
                32,
                PropModeReplace,
                (&value as *const c_ulong).cast(),
                1,
            );
        }
    }
    fn config(&self) {
        let lock = if self.res.locking != 0 && !self.lock_disabled.get() {
            2
        } else {
            0
        };
        self.publish(
            self.root,
            self.cfg,
            XA_INTEGER,
            lock | c_ulong::from(self.res.suspend != 0),
        );
    }
    fn claim(&self, program: &str) -> bool {
        // SAFETY: the server grab makes checking the manager property and publishing
        // our window atomic across concurrent launches. All work under it uses Xlib.
        unsafe {
            XGrabServer(self.dpy);
            let existing = property(self.dpy, self.root, self.mgr, XA_WINDOW).and_then(|old| {
                XSync(self.dpy, False);
                let prev = XSetErrorHandler(Some(ignore_stale));
                let result = property(self.dpy, old, self.pid, XA_INTEGER);
                XSync(self.dpy, False);
                XSetErrorHandler(prev);
                result
            });
            if existing.is_none() {
                self.publish(self.root, self.mgr, XA_WINDOW, self.window());
                self.publish(self.window(), self.pid, XA_INTEGER, libc::getpid() as _);
            }
            XUngrabServer(self.dpy);
            XSync(self.dpy, False);
            if let Some(pid) = existing {
                eprintln!("[XMSM] {program} is already running as PID {pid}");
                false
            } else {
                true
            }
        }
    }
    fn geometry(&self, screen: *mut Screen) -> (i32, i32, i32, i32) {
        unsafe {
            if xinerama::XineramaIsActive(self.dpy) != 0 {
                let mut n = 0;
                let infos = xinerama::XineramaQueryScreens(self.dpy, &mut n);
                if !infos.is_null() {
                    if n > 0 {
                        let index = if self.res.primary < n as u32 {
                            self.res.primary as usize
                        } else {
                            0
                        };
                        let info = *infos.add(index);
                        XFree(infos.cast());
                        return (
                            info.x_org.into(),
                            info.y_org.into(),
                            info.width.into(),
                            info.height.into(),
                        );
                    }
                    XFree(infos.cast());
                }
            }
            (0, 0, XWidthOfScreen(screen), XHeightOfScreen(screen))
        }
    }
    fn center(&self, w: W, dx: i32, dy: i32) {
        unsafe {
            let (x, y, width, height) = self.geometry(xt::XtScreen(w));
            let (ww, wh) = dims(w);
            xt::XtMoveWidget(
                w,
                (x + (width - i32::from(ww)) / 2 - dx) as i16,
                (y + (height - i32::from(wh)) / 2 - dy) as i16,
            );
        }
    }
    fn init_input(&self) {
        // SAFETY: screen numbers come from this open display.
        unsafe {
            let cursor = XCreateFontCursor(self.dpy, 68);
            for i in 0..XScreenCount(self.dpy) {
                XDefineCursor(self.dpy, XRootWindow(self.dpy, i), cursor);
            }
            XFreeCursor(self.dpy, cursor);
            if self.res.numlock.is_null() {
                return;
            }
            let state = CStr::from_ptr(self.res.numlock);
            if state.to_bytes().eq_ignore_ascii_case(b"keep") {
                return;
            }
            let on = state.to_bytes().eq_ignore_ascii_case(b"on");
            if !on && !state.to_bytes().eq_ignore_ascii_case(b"off") {
                eprintln!(
                    "[XMSM] Illegal numLockState value: {} Must be either ON, OFF or KEEP.",
                    state.to_string_lossy()
                );
                return;
            }
            let mask = XkbKeysymToModifiers(self.dpy, x11::keysym::XK_Num_Lock.into());
            if mask == 0 || XkbLockModifiers(self.dpy, 0x100, mask, if on { mask } else { 0 }) == 0
            {
                eprintln!("[XMSM] Cannot set NumLock state!");
            }
        }
    }
    fn extensions(&self) {
        unsafe {
            let (mut base, mut err) = (0, 0);
            if xrandr::XRRQueryExtension(self.dpy, &mut base, &mut err) != 0 {
                self.randr.set(Some(base));
                xrandr::XRRSelectInput(self.dpy, self.window(), xrandr::RRScreenChangeNotifyMask);
            }
            if self.res.locking == 0 {
                return;
            }
            if xss::XScreenSaverQueryExtension(self.dpy, &mut base, &mut err) == 0 {
                eprintln!("[XMSM] No XScreenSaver extension available. Timed locking disabled.");
                return;
            }
            self.screensaver.set(Some(base));
            for i in 0..XScreenCount(self.dpy) {
                xss::XScreenSaverRegister(self.dpy, i, self.window(), XA_WINDOW);
                xss::XScreenSaverSelectInput(
                    self.dpy,
                    XRootWindow(self.dpy, i),
                    xss::ScreenSaverNotifyMask,
                );
            }
            XSetScreenSaver(
                self.dpy,
                self.res.blank_timeout as _,
                0,
                PreferBlanking,
                DefaultExposures,
            );
        }
    }
    fn request_lock(&self, signal: bool) {
        if self.lock_disabled.get() || self.lock.get().is_none() {
            eprintln!("[XMSM] Can't lock. Locking is disabled");
            if !signal {
                self.bell();
                self.error_dialog();
            }
            return;
        }
        let lock = self.lock.get().unwrap();
        lock.start(self);
        if self.res.blank_on_lock != 0 && lock.active.get() {
            unsafe {
                if signal {
                    XForceScreenSaver(self.dpy, ScreenSaverActive);
                } else {
                    xt::XtAppAddTimeOut(self.ctx, 1000, Some(blank_timeout), self.data());
                }
            }
        }
    }
    fn handle(&self, ev: &mut XEvent) -> bool {
        unsafe {
            let t = ev.type_;
            if [KeyPress, ButtonPress, MotionNotify].contains(&t) {
                if let Some(lock) = self.lock.get() {
                    if lock.active.get() && lock.on_input(self) {
                        return true;
                    }
                }
            } else if Some(t) == self.screensaver.get() {
                let state = ev.xss_notify.state;
                if state == xss::ScreenSaverOn
                    && self.lock_timer.get() == 0
                    && self.res.lock_timeout != 0
                    && !self.lock_disabled.get()
                    && self.lock.get().is_some_and(|l| !l.active.get())
                {
                    self.lock_timer.set(xt::XtAppAddTimeOut(
                        self.ctx,
                        c_ulong::from(self.res.lock_timeout).saturating_mul(1000),
                        Some(lock_timeout),
                        self.data(),
                    ));
                } else if state == xss::ScreenSaverOff {
                    let timer = self.lock_timer.replace(0);
                    if timer != 0 {
                        xt::XtRemoveTimeOut(timer);
                    }
                }
            } else if Some(t) == self.randr.get() {
                xrandr::XRRUpdateConfiguration(ev);
                if let Some(lock) = self.lock.get() {
                    lock.resize(self);
                }
                if let Some(dialog) = self.logout.get() {
                    dialog.resize(self);
                }
            } else if t == ClientMessage
                && ev.client_message.message_type == self.cmd
                && ev.client_message.format == 8
            {
                let bytes = ev.client_message.data.as_bytes();
                let cmd = bytes.split(|c| *c == 0).next().unwrap_or_default();
                if cmd
                    .iter()
                    .copied()
                    .map(|b| b as u8)
                    .eq(b"LOGOUT".iter().copied())
                {
                    self.show_logout();
                } else if cmd
                    .iter()
                    .copied()
                    .map(|b| b as u8)
                    .eq(b"LOCK".iter().copied())
                {
                    self.request_lock(false);
                } else if cmd
                    .iter()
                    .copied()
                    .map(|b| b as u8)
                    .eq(b"SUSPEND".iter().copied())
                {
                    if self.res.lock_on_suspend != 0 {
                        self.request_lock(false);
                        if !self.lock.get().is_some_and(|l| l.active.get()) {
                            return true;
                        }
                    }
                    if self.res.suspend == 0 {
                        eprintln!("[XMSM] Can't suspend. Command is disabled");
                    } else if !self.access.request(Request::Suspend, &[]) {
                        self.bell();
                        self.error_dialog();
                    }
                }
            }
        }
        false
    }
    fn error_dialog(&self) {
        if !self.error.get().is_null() {
            return;
        }
        let msg=Label::new(c"The session manager has encountered an error,\nmost likely due to improper configuration.\nSee the log file for details.");
        let title = Label::new(c"XmSm");
        let dismiss = Label::new(c"Dismiss");
        let mut attrs = args![c"dialogTitle"=>title.ptr(),c"defaultButtonType"=>4,c"messageString"=>msg.ptr(),c"okLabelString"=>dismiss.ptr()];
        unsafe {
            let dialog = XmCreateErrorDialog(
                self.shell,
                c"messageDialog".as_ptr(),
                attrs.as_mut_ptr().cast(),
                attrs.len() as _,
            );
            for which in [2, 7] {
                xt::XtUnmanageChild(XmMessageBoxGetChild(dialog, which));
            }
            self.error.set(dialog);
            xt::XtManageChild(dialog);
        }
    }
    fn show_logout(&self) {
        if self.lock.get().is_some_and(|l| l.active.get()) {
            return;
        }
        self.logout.get_or_init(|| Logout::new(self)).show(self);
    }
}
fn run(open_access: impl FnOnce(&str) -> io::Result<Access>) -> io::Result<()> {
    let mut argv_text = std::env::args_os()
        .map(|a| CString::new(a.as_bytes()))
        .collect::<Result<Vec<_>, _>>()?;
    let program = argv_text
        .first()
        .map_or("xmsm".into(), |p| p.to_string_lossy());
    let access = open_access(&program)?;
    let mut argv = argv_text
        .iter()
        .map(|s| s.as_ptr().cast_mut())
        .collect::<Vec<_>>();
    let mut argc = argv.len() as c_int;
    argv.push(null_mut());
    // SAFETY: argv, argc and the Xt context remain valid for the application lifetime.
    unsafe {
        xt::XtSetLanguageProc(null_mut(), None, null_mut());
        xt::XtToolkitInitialize();
        let mut ctx = null_mut();
        let mut attrs = args![c"iconName"=>c"XmSm".as_ptr(),c"mwmFunctions"=>0,c"mwmDecorations"=>0,c"mappedWhenManaged"=>0,c"overrideRedirect"=>1];
        let shell = xt::XtAppInitialize(
            &mut ctx,
            c"XmSm".as_ptr(),
            null_mut(),
            0,
            &mut argc,
            argv.as_mut_ptr(),
            null_mut(),
            attrs.as_mut_ptr().cast(),
            attrs.len() as _,
        );
        if argv
            .iter()
            .take(argc as usize)
            .skip(1)
            .any(|p| CStr::from_ptr(*p) == c"-version")
        {
            println!("xmsm v2.0.0");
            xt::XtDestroyApplicationContext(ctx);
            return Ok(());
        }
        let dpy = xt::XtDisplay(shell);
        let atom = |name: &CStr| XInternAtom(dpy, name.as_ptr(), False);
        let app = Box::new(App {
            shell,
            ctx,
            dpy,
            root: XDefaultRootWindow(dpy),
            res: Resources::read(shell),
            access,
            mgr: atom(c"_XM_SESSION_MANAGER"),
            pid: atom(c"_XM_SESSION_MANAGER_PID"),
            cmd: atom(c"_XM_SESSION_MANAGER_CMD"),
            cfg: atom(c"_XM_SESSION_MANAGER_CFG"),
            wm_messages: atom(c"_MOTIF_WM_MESSAGES"),
            wm_offset: atom(c"_MOTIF_WM_OFFSET"),
            lock: OnceCell::new(),
            logout: OnceCell::new(),
            error: Cell::new(null_mut()),
            lock_disabled: Cell::new(false),
            lock_timer: Cell::new(0),
            randr: Cell::new(None),
            screensaver: Cell::new(None),
        });
        let (width, height) = dims(shell);
        if width == 0 || height == 0 {
            set(
                shell,
                &mut args![c"width"=>width.max(1),c"height"=>height.max(1)],
            );
        }
        xt::XtRealizeWidget(shell);
        XDeleteProperty(dpy, app.window(), XA_WM_COMMAND);
        SIGNAL_LOCK.store(
            xt::XtAppAddSignal(ctx, Some(signal_lock), app.data()) as usize,
            Ordering::Relaxed,
        );
        SIGNAL_LOGOUT.store(
            xt::XtAppAddSignal(ctx, Some(signal_logout), app.data()) as usize,
            Ordering::Relaxed,
        );
        signals()?;
        if !app.claim(&program) {
            return Ok(());
        }
        app.init_input();
        app.extensions();
        app.config();
        if app.res.locking != 0 {
            app.lock.get_or_init(|| Lock::new(&app));
        }
        for (ptr, kind) in [
            (app.res.wm, "window manager"),
            (app.res.launcher, "launcher"),
        ] {
            if ptr.is_null() && kind == "launcher" {
                continue;
            }
            let path = if ptr.is_null() {
                c""
            } else {
                CStr::from_ptr(ptr)
            };
            if let Err(e) = launch(path.to_bytes()) {
                eprintln!(
                    "[XMSM] Failed to exec the {kind} ({}): {}",
                    path.to_string_lossy(),
                    CStr::from_ptr(libc::strerror(e.raw_os_error().unwrap_or(libc::EINVAL)))
                        .to_string_lossy()
                );
                return Err(io::ErrorKind::Interrupted.into());
            }
        }
        sessionetc();
        reap(0);
        XmAddProtocols(shell, app.wm_messages, &mut app.wm_offset.clone(), 1);
        while xt::XtAppGetExitFlag(ctx) == 0 {
            let mut event = std::mem::MaybeUninit::uninit();
            xt::XtAppNextEvent(ctx, event.as_mut_ptr());
            let mut event = event.assume_init();
            if !app.handle(&mut event) {
                xt::XtDispatchEvent(&mut event);
            }
        }
        SIGNAL_LOCK.store(0, Ordering::Relaxed);
        SIGNAL_LOGOUT.store(0, Ordering::Relaxed);
        xt::XtDestroyApplicationContext(ctx);
    }
    argv_text.clear();
    Ok(())
}
fn main() -> ExitCode {
    match run(Access::new) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            if e.kind() != io::ErrorKind::Interrupted {
                eprintln!("[XMSM] {e}");
            }
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "exercised by tests/lock.sh on its own Xvfb"]
    fn isolated_lock_session() {
        assert_eq!(unsafe { libc::geteuid() }, unsafe { libc::getuid() });
        assert_ne!(unsafe { libc::getuid() }, 0);
        assert_eq!(std::env::var("XMSM_TEST_SYNTHETIC").as_deref(), Ok("1"));
        super::run(|_| super::Access::synthetic()).unwrap();
    }
}
