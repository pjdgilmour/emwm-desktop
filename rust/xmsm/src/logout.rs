// SPDX-License-Identifier: MIT
use super::*;

pub struct Logout {
    shades: Vec<W>,
    shell: W,
    form: W,
    shutdown: W,
    reboot: W,
    ok: W,
    visible: Cell<bool>,
}
unsafe extern "C" fn activate(w: W, p: *mut c_void, _: *mut c_void) {
    // SAFETY: callbacks refer to the boxed App and this dialog's immutable handles.
    let app = &*p.cast::<App>();
    if let Some(dialog) = app.logout.get() {
        dialog.answer(app, w == dialog.ok);
    }
}
#[repr(C)]
struct AnyCallback {
    reason: i32,
    event: *mut XEvent,
}
unsafe extern "C" fn offset(w: W, p: *mut c_void, call: *mut c_void) {
    let app = &*p.cast::<App>();
    let data = &*call.cast::<AnyCallback>();
    if data.event.is_null() {
        return;
    }
    let off = &(*data.event).client_message.data;
    app.center(w, off.get_long(1) as i32, off.get_long(2) as i32);
}
impl Logout {
    pub fn new(app: &App) -> Self {
        // SAFETY: all widgets belong to app's single Xt event loop. Callback data points
        // to its stable address until the widgets and context are destroyed.
        unsafe {
            let d = app.dpy;
            let mut shades = Vec::new();
            if app.res.shade != 0 {
                for i in 0..XScreenCount(d) {
                    let shade = lock::screen_shell(app, i, c"shade", false);
                    xt::XtRealizeWidget(shade);
                    shades.push(shade);
                }
            }
            let parent = shades
                .get(XScreenNumberOfScreen(xt::XtScreen(app.shell)) as usize)
                .copied()
                .unwrap_or(app.shell);
            let shell = popup(
                parent,
                c"confirmExitDialog",
                xmDialogShellWidgetClass,
                &mut args![c"allowShellResize"=>1,c"mwmDecorations"=>10,c"mwmFunctions"=>0,c"useAsyncGeometry"=>0,c"mappedWhenManaged"=>0,c"deleteResponse"=>2],
            );
            let title = Label::new(c"Leaving Session");
            let mut attrs = args![c"marginHeight"=>5,c"verticalSpacing"=>2,c"dialogTitle"=>title.ptr(),c"fractionBase"=>5,c"dialogStyle"=>3,c"autoUnmanage"=>1];
            let form = xt::XtCreateWidget(
                c"confirmExit".as_ptr(),
                xmFormWidgetClass,
                shell,
                attrs.as_mut_ptr().cast(),
                attrs.len() as _,
            );
            let label=label_child(form,c"label",xmLabelGadgetClass,c"Choose an action to proceed with:",&mut args![c"alignment"=>0,c"topAttachment"=>1,c"leftAttachment"=>5,c"leftPosition"=>1,c"rightAttachment"=>5,c"rightPosition"=>4].to_vec());
            let row = child(
                form,
                c"rowColumn",
                xmRowColumnWidgetClass,
                &mut args![c"orientation"=>1,c"radioBehavior"=>1,c"topAttachment"=>3,c"topWidget"=>label,c"leftAttachment"=>5,c"leftPosition"=>1,c"rightAttachment"=>5,c"rightPosition"=>4,c"marginHeight"=>1,c"radioAlwaysOne"=>1],
            );
            label_child(
                row,
                c"logOut",
                xmToggleButtonGadgetClass,
                c"Log out",
                &mut args![c"indicatorType"=>2,c"set"=>1].to_vec(),
            );
            let shutdown = label_child(
                row,
                c"shutDown",
                xmToggleButtonGadgetClass,
                c"Shut down",
                &mut args![c"indicatorType"=>2].to_vec(),
            );
            let reboot = label_child(
                row,
                c"reboot",
                xmToggleButtonGadgetClass,
                c"Reboot",
                &mut args![c"indicatorType"=>2].to_vec(),
            );
            if app.res.shutdown == 0 {
                xt::XtUnmanageChild(shutdown);
            }
            if app.res.reboot == 0 {
                xt::XtUnmanageChild(reboot);
            }
            let sep = child(
                form,
                c"separator",
                xmSeparatorGadgetClass,
                &mut args![c"topAttachment"=>3,c"topWidget"=>row,c"leftAttachment"=>1,c"rightAttachment"=>1],
            );
            let button = |name: &CStr, title: &CStr, start: i32| {
                label_child(form,name,xmPushButtonGadgetClass,title,&mut args![c"topAttachment"=>3,c"topWidget"=>sep,c"leftAttachment"=>5,c"leftPosition"=>start,c"rightAttachment"=>5,c"rightPosition"=>start+1,c"bottomAttachment"=>1].to_vec())
            };
            let ok = button(c"ok", c"OK", 1);
            let cancel = button(c"cancel", c"Cancel", 3);
            for w in [cancel, ok] {
                xt::XtAddCallback(w, c"activateCallback".as_ptr(), Some(activate), app.data());
            }
            set(
                form,
                &mut args![c"initialFocus"=>ok,c"defaultButton"=>ok,c"cancelButton"=>cancel],
            );
            xt::XtRealizeWidget(shell);
            XmAddProtocolCallback(shell, app.wm_messages, app.wm_offset, offset, app.data());
            let protocols = XInternAtom(d, c"WM_PROTOCOLS".as_ptr(), False);
            let delete = XInternAtom(d, c"WM_DELETE_WINDOW".as_ptr(), False);
            XmAddProtocolCallback(shell, protocols, delete, activate, app.data());
            Self {
                shades,
                shell,
                form,
                shutdown,
                reboot,
                ok,
                visible: Cell::new(false),
            }
        }
    }
    pub fn resize(&self, app: &App) {
        unsafe {
            for &shade in &self.shades {
                let screen = xt::XtScreen(shade);
                xt::XtConfigureWidget(
                    shade,
                    0,
                    0,
                    XWidthOfScreen(screen) as _,
                    XHeightOfScreen(screen) as _,
                    0,
                );
            }
            if self.visible.get() {
                app.center(self.shell, 0, 0);
            }
        }
    }
    pub fn show(&self, app: &App) {
        unsafe {
            if self.visible.replace(true) {
                XRaiseWindow(app.dpy, xt::XtWindow(self.shell));
                return;
            }
            self.resize(app);
            for &shade in &self.shades {
                let d = app.dpy;
                let s = xt::XtScreen(shade);
                let root = XRootWindowOfScreen(s);
                let w = XWidthOfScreen(s) as u32;
                let h = XHeightOfScreen(s) as u32;
                let mut bits = [0x88u8, 0x22, 0x44, 0x11];
                let stipple =
                    XCreatePixmapFromBitmapData(d, root, bits.as_mut_ptr().cast(), 4, 4, 0, 1, 1);
                let pm = XCreatePixmap(d, root, w, h, XDefaultDepthOfScreen(s) as _);
                let mut values: XGCValues = std::mem::zeroed();
                values.function = GXcopy;
                values.fill_style = FillStippled;
                values.subwindow_mode = IncludeInferiors;
                values.stipple = stipple;
                values.background = XBlackPixelOfScreen(s);
                let gc = XCreateGC(
                    d,
                    xt::XtWindow(shade),
                    (GCFunction | GCBackground | GCFillStyle | GCStipple | GCSubwindowMode) as _,
                    &mut values,
                );
                XCopyArea(d, root, pm, gc, 0, 0, w, h, 0, 0);
                XFillRectangle(d, pm, gc, 0, 0, w, h);
                set(shade, &mut args![c"backgroundPixmap"=>pm]);
                // Immediately assign it to the server window, which retains its own reference.
                XSetWindowBackgroundPixmap(d, xt::XtWindow(shade), pm);
                XClearWindow(d, xt::XtWindow(shade));
                XFreeGC(d, gc);
                XFreePixmap(d, stipple);
                XFreePixmap(d, pm);
                xt::XtMapWidget(shade);
                XRaiseWindow(d, xt::XtWindow(shade));
            }
            xt::XtManageChild(self.form);
            xt::XtMapWidget(self.shell);
            app.center(self.shell, 0, 0);
            XRaiseWindow(app.dpy, xt::XtWindow(self.shell));
            XFlush(app.dpy);
        }
    }
    fn answer(&self, app: &App, confirm: bool) {
        if !self.visible.replace(false) {
            return;
        }
        unsafe {
            xt::XtUnmanageChild(self.form);
            xt::XtUnmapWidget(self.shell);
            for &shade in &self.shades {
                xt::XtUnmapWidget(shade);
                set(shade, &mut args![c"backgroundPixmap"=>2]);
            }
            if !confirm || app.lock.get().is_some_and(|l| l.active.get()) {
                return;
            }
            let op = if app.res.shutdown != 0 && XmToggleButtonGadgetGetState(self.shutdown) != 0 {
                Some(Request::Shutdown)
            } else if app.res.reboot != 0 && XmToggleButtonGadgetGetState(self.reboot) != 0 {
                Some(Request::Reboot)
            } else {
                None
            };
            if op.is_some_and(|op| !app.access.request(op, &[])) {
                app.bell();
                app.error_dialog();
                return;
            }
            XDeleteProperty(app.dpy, app.root, app.mgr);
            XSync(app.dpy, False);
            xt::XtAppSetExitFlag(app.ctx);
        }
    }
}
