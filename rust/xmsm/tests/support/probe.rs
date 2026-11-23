// SPDX-License-Identifier: MIT
//! Small XTest client for the isolated integration tests.
use std::ffi::{CStr, CString};
use std::mem::zeroed;
use std::process::ExitCode;
use std::ptr::{null, null_mut};
use x11::{xlib::*, xtest::*};

unsafe fn dialog(display: *mut Display, window: Window) -> Option<Window> {
    let (mut root, mut parent, mut children, mut count) = (0, 0, null_mut(), 0);
    if XQueryTree(
        display,
        window,
        &mut root,
        &mut parent,
        &mut children,
        &mut count,
    ) == 0
        || children.is_null()
    {
        return None;
    }
    let windows = std::slice::from_raw_parts(children, count as usize);
    let mut found = None;
    for &child in windows {
        let mut name = null_mut();
        if XFetchName(display, child, &mut name) != 0 && !name.is_null() {
            let matches = CStr::from_ptr(name) == c"Leaving Session";
            XFree(name.cast());
            let mut attrs = zeroed();
            if matches
                && XGetWindowAttributes(display, child, &mut attrs) != 0
                && attrs.map_state == IsViewable
            {
                found = Some(child);
                break;
            }
        }
        found = dialog(display, child);
        if found.is_some() {
            break;
        }
    }
    XFree(children.cast());
    found
}

unsafe fn send_request(display: *mut Display, root: Window, request: &str) -> bool {
    let (mut ty, mut fmt, mut count, mut remaining, mut data) = (0, 0, 0, 0, null_mut());
    let property = XInternAtom(display, c"_XM_SESSION_MANAGER".as_ptr(), False);
    XGetWindowProperty(
        display,
        root,
        property,
        0,
        1,
        False,
        XA_WINDOW,
        &mut ty,
        &mut fmt,
        &mut count,
        &mut remaining,
        &mut data,
    );
    if data.is_null() {
        return false;
    }
    let valid = count == 1 && ty == XA_WINDOW && fmt == 32;
    let target = if valid { *data.cast::<Window>() } else { 0 };
    XFree(data.cast());
    if target == 0 {
        return false;
    }
    let mut message: XClientMessageEvent = zeroed();
    message.type_ = ClientMessage;
    message.window = target;
    message.format = 8;
    message.message_type = XInternAtom(display, c"_XM_SESSION_MANAGER_CMD".as_ptr(), False);
    for (dst, src) in message
        .data
        .as_bytes_mut()
        .iter_mut()
        .take(19)
        .zip(request.bytes())
    {
        *dst = src as _;
    }
    XSendEvent(
        display,
        target,
        False,
        NoEventMask,
        &mut XEvent::from(message),
    ) != 0
}

unsafe fn probe(display: *mut Display, op: &str) -> bool {
    let root = XDefaultRootWindow(display);
    match op {
        "dialog" | "cancel" | "ok" => {
            let Some(w) = dialog(display, root) else {
                return false;
            };
            if op == "dialog" {
                return true;
            }
            let (mut attrs, mut x, mut y, mut child) = (zeroed(), 0, 0, 0);
            if XGetWindowAttributes(display, w, &mut attrs) == 0 {
                return false;
            }
            let fraction = if op == "ok" { 3 } else { 7 };
            if XTranslateCoordinates(
                display,
                w,
                root,
                attrs.width * fraction / 10,
                attrs.height - 13,
                &mut x,
                &mut y,
                &mut child,
            ) == 0
            {
                return false;
            }
            XTestFakeMotionEvent(display, XDefaultScreen(display), x, y, CurrentTime);
            XTestFakeButtonEvent(display, 1, True, CurrentTime);
            XTestFakeButtonEvent(display, 1, False, CurrentTime);
            true
        }
        "key" => {
            let name = std::env::args().nth(2).and_then(|s| CString::new(s).ok());
            let Some(name) = name else { return false };
            let key = XKeysymToKeycode(display, XStringToKeysym(name.as_ptr()));
            if key == 0 {
                return false;
            }
            XTestFakeKeyEvent(display, key.into(), True, CurrentTime);
            XTestFakeKeyEvent(display, key.into(), False, CurrentTime);
            true
        }
        "grab" => {
            let status = XGrabKeyboard(
                display,
                root,
                False,
                GrabModeAsync,
                GrabModeAsync,
                CurrentTime,
            );
            if status == GrabSuccess {
                XUngrabKeyboard(display, CurrentTime);
            }
            status == AlreadyGrabbed
        }
        "LOCK" | "LOGOUT" | "SUSPEND" => send_request(display, root, op),
        _ => false,
    }
}

fn main() -> ExitCode {
    let Some(op) = std::env::args().nth(1) else {
        return ExitCode::FAILURE;
    };
    // SAFETY: all requests use this live connection. Xlib-owned arrays and
    // strings are read with their reported size and released before closing it.
    unsafe {
        let display = XOpenDisplay(null());
        if display.is_null() {
            return ExitCode::FAILURE;
        }
        let ok = probe(display, &op);
        XSync(display, False);
        XCloseDisplay(display);
        ExitCode::from(u8::from(!ok))
    }
}
