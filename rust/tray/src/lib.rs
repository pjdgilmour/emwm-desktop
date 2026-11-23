// SPDX-License-Identifier: MIT
mod host;
mod ui;
use host::Host;
use std::cell::{Cell, RefCell};
use std::ffi::{c_char, c_int, c_long, c_ulong, c_void, CStr};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicUsize, Ordering};
use ui::Ui;
use x11::{xlib::*, xrender, xt, xt::Widget};

type ErrorHandler = Option<unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> c_int>;
thread_local! {
    static HOST: RefCell<Option<Host>> = const {RefCell::new(None)};
    static PREVIOUS: Cell<ErrorHandler> = const {Cell::new(None)};
    static TRAP: Cell<Option<u8>> = const {Cell::new(None)};
}
static SIGNAL: AtomicUsize = AtomicUsize::new(0);

// Xt can deliver a synchronous resize callback inside a layout operation. That
// operation positions the icons again using the final, negotiated widget size.
fn with_host(f: impl FnOnce(&mut Host)) {
    HOST.with(|cell| {
        if let Ok(mut slot) = cell.try_borrow_mut() {
            if let Some(host) = slot.as_mut() {
                f(host)
            }
        }
    });
}
fn set_host(host: Host) {
    HOST.with_borrow_mut(|slot| *slot = Some(host));
}

unsafe extern "C" fn events_cb(_: Widget, _: *mut c_void, ev: *mut XEvent, _: *mut c_char) {
    // SAFETY: Xt provides the active, initialized event for this callback.
    with_host(|h| h.event(&*ev));
}
unsafe extern "C" fn resize_cb(_: Widget, _: *mut c_void, _: *mut c_void) {
    // SAFETY: callbacks run on the host's Xt thread.
    with_host(|h| {
        if xt::XtIsRealized(h.ui.area) != 0 {
            h.position()
        }
    });
}
unsafe extern "C" fn x_error(d: *mut Display, ev: *mut XErrorEvent) -> i32 {
    // SAFETY: Xlib supplies a valid error event and previous handler here.
    let code = (*ev).error_code;
    if TRAP.get().is_some() {
        TRAP.set(Some(code));
        return 0;
    }
    if code == BadWindow || code == BadDrawable {
        return 0;
    }
    PREVIOUS.get().map_or(0, |handler| handler(d, ev))
}
fn trap<T>(d: *mut Display, operation: impl FnOnce() -> T) -> (T, bool) {
    // SAFETY: d is this thread's live display; XSync flushes any preceding
    // errors before recording only errors from operation.
    unsafe { XSync(d, 0) };
    TRAP.set(Some(0));
    let value = operation();
    // SAFETY: complete all requests while the trap still owns their errors.
    unsafe { XSync(d, 0) };
    (value, TRAP.replace(None) == Some(0))
}
unsafe extern "C" fn signal_cb(_: *mut c_void, _: *mut c_ulong) {
    // SAFETY: Xt dispatches this on its event loop, after the signal handler.
    with_host(|h| {
        if h.ui.embedded {
            h.release();
            XSync(h.d, 0);
            xt::XtDestroyWidget(h.ui.shell);
            XCloseDisplay(h.d);
            std::process::exit(0);
        } else {
            xt::XtAppSetExitFlag(xt::XtWidgetToApplicationContext(h.ui.shell));
        }
    });
}
extern "C" fn signal_handler(_: c_int) {
    // SAFETY: XtNoticeSignal is Xt's async-signal-safe notification API.
    unsafe { xt::XtNoticeSignal(SIGNAL.load(Ordering::Relaxed) as _) };
}
fn install_signals(app: xt::XtAppContext) {
    // SAFETY: the context and the static callback live through the event loop.
    unsafe {
        SIGNAL.store(
            xt::XtAppAddSignal(app, Some(signal_cb), null_mut()) as usize,
            Ordering::Relaxed,
        );
        libc::signal(
            libc::SIGINT,
            signal_handler as *const () as libc::sighandler_t,
        );
        libc::signal(
            libc::SIGTERM,
            signal_handler as *const () as libc::sighandler_t,
        );
    }
}

#[no_mangle]
unsafe extern "C" fn TrayCreate(parent: Widget, shell: Widget) -> Widget {
    // SAFETY: Toolchest supplies its live shell and footer widgets.
    let Ok(ui) = Ui::new(parent, shell, true) else {
        return null_mut();
    };
    let frame = ui.frame;
    set_host(Host::new(ui));
    frame
}
#[no_mangle]
extern "C" fn TrayStart() {
    // SAFETY: Toolchest calls after realizing the shell and its children.
    with_host(|h| unsafe {
        if xt::XtIsRealized(h.ui.area) == 0 {
            return;
        }
        if h.claim() {
            install_signals(xt::XtWidgetToApplicationContext(h.ui.shell));
            h.layout();
        } else {
            xt::XtUnmanageChild(h.ui.frame);
        }
    });
}
pub fn standalone() -> i32 {
    ui::run()
}
