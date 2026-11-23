// SPDX-License-Identifier: MIT
use super::*;
use std::cell::RefCell;
use std::ffi::c_long;
struct Cover {
    shell: W,
    area: W,
}
pub struct Lock {
    covers: Vec<Cover>,
    panel: W,
    field: W,
    message: W,
    password: RefCell<Vec<u8>>,
    max: usize,
    pub active: Cell<bool>,
    visible: Cell<bool>,
    hide_timer: Cell<c_ulong>,
    hosts: Cell<*mut XHostAddress>,
    host_count: Cell<i32>,
    acl_enabled: Cell<bool>,
}
#[repr(C)]
struct Block {
    ptr: *mut c_char,
    len: i32,
    format: Atom,
}
#[repr(C)]
struct Verify {
    reason: i32,
    event: *mut XEvent,
    allow: u8,
    curr: c_long,
    next: c_long,
    start: c_long,
    end: c_long,
    text: *mut Block,
}
unsafe extern "C" fn edit(w: W, p: *mut c_void, data: *mut c_void) {
    // SAFETY: Motif supplies XmTextVerifyCallbackStruct; text is initialized only on modification.
    let app = &*p.cast::<App>();
    let Some(lock) = app.lock.get() else { return };
    let v = &mut *data.cast::<Verify>();
    if v.reason == 21 {
        if v.next != XmTextFieldGetLastPosition(w) {
            v.allow = 0;
        }
        return;
    }
    if v.reason != 20 || v.text.is_null() {
        return;
    }
    let text = &mut *v.text;
    let mut secret = lock.password.borrow_mut();
    if text.len == 0 {
        let pos = usize::try_from(v.start).unwrap_or(0).min(secret.len());
        auth::wipe(&mut secret[pos..]);
        secret.truncate(pos);
    } else if text.len == 1
        && !text.ptr.is_null()
        && v.curr >= 0
        && v.curr as usize == secret.len()
        && secret.len() < lock.max
        && *text.ptr != 0
    {
        secret.push(*text.ptr as u8);
        *text.ptr = b'*' as c_char;
    } else {
        v.allow = 0;
    }
}
unsafe extern "C" fn enter(_: W, p: *mut c_void, _: *mut c_void) {
    let app = &*p.cast::<App>();
    let Some(lock) = app.lock.get() else { return };
    if !lock.active.get() {
        return;
    }
    let ok = app.access.request(Request::Verify, &lock.password.borrow());
    lock.clear();
    if ok {
        lock.unlock(app);
        text(lock.message, c" ");
    } else {
        app.bell();
        text(lock.message, c"Password Incorrect");
    }
}
unsafe extern "C" fn hide(p: *mut c_void, _: *mut c_ulong) {
    let app = &*p.cast::<App>();
    if let Some(lock) = app.lock.get() {
        lock.hide_timer.set(0);
        lock.visible.set(false);
        lock.clear();
        text(lock.message, c" ");
        if lock.active.get() {
            xt::XtUnmapWidget(lock.panel);
            XFlush(app.dpy);
        }
    }
}
unsafe extern "C" fn visibility(w: W, p: *mut c_void, ev: *mut XEvent, _: *mut c_char) {
    let app = &*p.cast::<App>();
    if app.lock.get().is_some_and(|l| l.active.get())
        && (*ev).type_ == VisibilityNotify
        && (*ev).visibility.state == VisibilityFullyObscured
    {
        XRaiseWindow(app.dpy, xt::XtWindow(w));
    }
}
pub(super) unsafe fn screen_shell(app: &App, index: i32, name: &CStr, modal: bool) -> W {
    let d = app.dpy;
    let screen = XScreenOfDisplay(d, index);
    let mut attrs=args![c"mwmDecorations"=>0,c"mwmFunctions"=>0,c"overrideRedirect"=>1,
        c"x"=>0,c"y"=>0,c"width"=>XWidthOfScreen(screen),c"height"=>XHeightOfScreen(screen),
        c"useAsyncGeometry"=>1,c"mappedWhenManaged"=>0,c"resizePolicy"=>0,c"depth"=>XDefaultDepth(d,index),
        c"colormap"=>XDefaultColormap(d,index),c"screen"=>screen].to_vec();
    if modal {
        attrs.push(arg(c"mwmInputMode", 2));
    }
    popup(app.shell, name, topLevelShellWidgetClass, &mut attrs)
}
impl Lock {
    pub fn new(app: &App) -> Self {
        // SAFETY: Xt initialization has completed; all children and callback targets
        // are owned for the lifetime of this single-threaded application's display.
        unsafe {
            let d = app.dpy;
            let mut color: XColor = std::mem::zeroed();
            color.pixel = app.res.color;
            XQueryColor(d, XDefaultColormap(d, XDefaultScreen(d)), &mut color);
            let motion = xt::XtParseTranslationTable(c"<Motion>:DrawingAreaInput()\n".as_ptr());
            let mut covers = Vec::new();
            for i in 0..XScreenCount(d) {
                let cm = XDefaultColormap(d, i);
                let screen = XScreenOfDisplay(d, i);
                XAllocColor(d, cm, &mut color);
                let mut bg = 2;
                if !app.res.image.is_null() {
                    let (mut fg, mut top, mut shadow, mut sel) = (0, 0, 0, 0);
                    XmGetColors(
                        screen,
                        cm,
                        color.pixel,
                        &mut fg,
                        &mut top,
                        &mut shadow,
                        &mut sel,
                    );
                    bg = XmGetPixmap(screen, app.res.image, shadow, color.pixel);
                    if bg == 2 {
                        eprintln!(
                            "[XMSM] Failed to load pixmap: {}",
                            CStr::from_ptr(app.res.image).to_string_lossy()
                        );
                    }
                }
                let shell = screen_shell(app, i, c"coverShell", i == 0);
                let area = child(
                    shell,
                    c"coverBackdrop",
                    xmDrawingAreaWidgetClass,
                    &mut args![c"background"=>color.pixel,c"backgroundPixmap"=>bg],
                );
                xt::XtAugmentTranslations(area, motion);
                xt::XtRealizeWidget(shell);
                xt::XtAddEventHandler(
                    shell,
                    VisibilityChangeMask as _,
                    0,
                    Some(visibility),
                    app.data(),
                );
                covers.push(Cover { shell, area });
            }
            let panel = child(
                covers[0].area,
                c"unlock",
                xmFrameWidgetClass,
                &mut args![c"shadowThickness"=>2,c"shadowType"=>8,c"mappedWhenManaged"=>0],
            );
            let row = child(
                panel,
                c"rowColumn",
                xmRowColumnWidgetClass,
                &mut args![c"minWidth"=>420,c"minHeight"=>240,c"orientation"=>1,c"entryAlignment"=>1,c"marginHeight"=>6,c"marginWidth"=>6],
            );
            let mut hostname = [0 as c_char; 256];
            libc::gethostname(hostname.as_mut_ptr(), 255);
            let login = libc::getpwuid(libc::getuid())
                .as_ref()
                .and_then(|p| (!p.pw_name.is_null()).then(|| CStr::from_ptr(p.pw_name)))
                .unwrap_or(c"(unknown)");
            let title = CString::new(
                [
                    login.to_bytes(),
                    b"@",
                    CStr::from_ptr(hostname.as_ptr()).to_bytes(),
                ]
                .concat(),
            )
            .unwrap_or_default();
            label_child(row, c"lockedBy", xmLabelGadgetClass, &title, &mut vec![]);
            let translations=xt::XtParseTranslationTable(c"<FocusIn>:focusIn()\n<FocusOut>:focusOut()\n<Key>osfActivate:activate()\n<Key>Return:activate()\n<Key>osfBackSpace:delete-previous-character()\n<Key>BackSpace:delete-previous-character()\n<Key>osfDelete:delete-previous-character()\n<Key>Delete:delete-previous-character()\n<Key>:self-insert()\n".as_ptr());
            let max = usize::try_from(libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX))
                .unwrap_or(255)
                .clamp(1, auth::PASSWORD_LIMIT);
            let field = child(
                row,
                c"password",
                xmTextFieldWidgetClass,
                &mut args![c"translations"=>translations,c"maxLength"=>max],
            );
            for (name, cb) in [
                (c"modifyVerifyCallback", edit as Callback),
                (c"motionVerifyCallback", edit),
                (c"activateCallback", enter),
            ] {
                xt::XtAddCallback(field, name.as_ptr(), Some(cb), app.data());
            }
            let message = child(
                row,
                c"message",
                xmLabelGadgetClass,
                &mut args![c"alignment"=>1],
            );
            text(message, c" ");
            app.center(panel, 0, 0);
            Self {
                covers,
                panel,
                field,
                message,
                password: RefCell::new(Vec::with_capacity(max)),
                max,
                active: Cell::new(false),
                visible: Cell::new(false),
                hide_timer: Cell::new(0),
                hosts: Cell::new(null_mut()),
                host_count: Cell::new(0),
                acl_enabled: Cell::new(true),
            }
        }
    }
    fn clear(&self) {
        {
            let mut pass = self.password.borrow_mut();
            auth::wipe(&mut pass);
            pass.clear();
        }
        unsafe {
            XmTextFieldSetString(self.field, c"".as_ptr());
        }
    }
    pub fn resize(&self, app: &App) {
        unsafe {
            for cover in &self.covers {
                let s = xt::XtScreen(cover.shell);
                xt::XtResizeWidget(
                    cover.shell,
                    XWidthOfScreen(s) as _,
                    XHeightOfScreen(s) as _,
                    0,
                );
            }
            app.center(self.panel, 0, 0);
        }
    }
    pub fn start(&self, app: &App) {
        if self.active.get() || app.lock_disabled.get() {
            return;
        }
        if !app.access.request(Request::CanLock, &[]) {
            eprintln!("[XMSM] Cannot authenticate. Screen locking disabled!");
            app.lock_disabled.set(true);
            app.config();
            app.bell();
            app.error_dialog();
            return;
        }
        unsafe {
            for cover in &self.covers {
                xt::XtMapWidget(cover.shell);
                XRaiseWindow(app.dpy, xt::XtWindow(cover.shell));
            }
            XSync(app.dpy, False);
            let owner = self.covers[0].shell;
            let pointer = xt::XtGrabPointer(
                owner,
                1,
                (ButtonPressMask | PointerMotionMask) as _,
                GrabModeAsync,
                GrabModeAsync,
                0,
                0,
                CurrentTime,
            ) == GrabSuccess;
            let keyboard = xt::XtGrabKeyboard(owner, 1, GrabModeAsync, GrabModeAsync, CurrentTime)
                == GrabSuccess;
            if !pointer || !keyboard {
                if pointer {
                    xt::XtUngrabPointer(owner, CurrentTime);
                }
                if keyboard {
                    xt::XtUngrabKeyboard(owner, CurrentTime);
                }
                for cover in &self.covers {
                    xt::XtUnmapWidget(cover.shell);
                }
                eprintln!("[XMSM] Cannot lock! Failed to grab input devices.");
                app.bell();
                app.error_dialog();
                return;
            }
            let (mut n, mut enabled) = (0, 0);
            let hosts = XListHosts(app.dpy, &mut n, &mut enabled);
            self.hosts.set(hosts);
            self.host_count.set(n);
            self.acl_enabled.set(enabled != 0);
            if !hosts.is_null() && n > 0 {
                XRemoveHosts(app.dpy, hosts, n);
            }
            if enabled == 0 {
                XEnableAccessControl(app.dpy);
            }
            self.active.set(true);
            self.show(app);
        }
    }
    fn show(&self, app: &App) {
        unsafe {
            self.clear();
            self.visible.set(true);
            xt::XtMapWidget(self.panel);
            XmProcessTraversal(self.panel, 0);
            self.reset(app);
            XFlush(app.dpy);
        }
    }
    fn reset(&self, app: &App) {
        unsafe {
            let old = self.hide_timer.replace(0);
            if old != 0 {
                xt::XtRemoveTimeOut(old);
            }
            self.hide_timer.set(xt::XtAppAddTimeOut(
                app.ctx,
                c_ulong::from(app.res.unlock_timeout).saturating_mul(1000),
                Some(hide),
                app.data(),
            ));
        }
    }
    pub fn on_input(&self, app: &App) -> bool {
        if self.visible.get() {
            self.reset(app);
            false
        } else {
            self.show(app);
            true
        }
    }
    fn unlock(&self, app: &App) {
        if !self.active.replace(false) {
            return;
        }
        unsafe {
            let timer = self.hide_timer.replace(0);
            if timer != 0 {
                xt::XtRemoveTimeOut(timer);
            }
            self.visible.set(false);
            let hosts = self.hosts.replace(null_mut());
            if !hosts.is_null() {
                XAddHosts(app.dpy, hosts, self.host_count.get());
                XFree(hosts.cast());
            }
            if !self.acl_enabled.get() {
                XDisableAccessControl(app.dpy);
            }
            xt::XtUngrabPointer(self.covers[0].shell, CurrentTime);
            xt::XtUngrabKeyboard(self.covers[0].shell, CurrentTime);
            for cover in &self.covers {
                xt::XtUnmapWidget(cover.shell);
            }
            XFlush(app.dpy);
        }
    }
}
