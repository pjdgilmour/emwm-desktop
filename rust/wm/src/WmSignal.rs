/*
 * Motif
 *
 * Copyright (c) 1987-2012, The Open Group. All rights reserved.
 * Copyright (c) 2018-2026, alx@fastestcode.org
 *
 * These libraries and programs are free software; you can
 * redistribute them and/or modify them under the terms of the GNU
 * Lesser General Public License as published by the Free Software
 * Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * These libraries and programs are distributed in the hope that
 * they will be useful, but WITHOUT ANY WARRANTY; without even the
 * implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 * PURPOSE. See the GNU Lesser General Public License for more
 * details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with these librararies and programs; if not, write
 * to the Free Software Foundation, Inc., 51 Franklin Street, Fifth
 * Floor, Boston, MA 02110-1301 USA
*/
use crate::abi::{Boolean, Display, Window, WmGlobalData, WmScreenData};
use libc::{c_int, sigaction, sighandler_t};
use std::{mem, ptr};

extern "C" {
    static mut wmGD: WmGlobalData;
    fn ConfirmAction(screen: *mut WmScreenData, action: c_int);
    fn Do_Quit_Mwm(restarting: Boolean);
    fn XCloseDisplay(display: *mut Display) -> c_int;
    fn XFlush(display: *mut Display) -> c_int;
    fn XSetInputFocus(
        display: *mut Display,
        window: Window,
        revert: c_int,
        time: libc::c_ulong,
    ) -> c_int;
    fn XSync(display: *mut Display, discard: c_int) -> c_int;
}

const FATAL: [c_int; 5] = [
    libc::SIGILL,
    libc::SIGFPE,
    libc::SIGBUS,
    libc::SIGSEGV,
    libc::SIGSYS,
];

fn install(signal: c_int, handler: sighandler_t, flags: c_int) {
    // SAFETY: Linux sigaction accepts an initialized action with an empty mask.
    unsafe {
        let mut action: sigaction = mem::zeroed();
        action.sa_sigaction = handler;
        action.sa_flags = flags;
        libc::sigemptyset(&mut action.sa_mask);
        libc::sigaction(signal, &action, ptr::null_mut());
    }
}

fn termination_handlers(handler: sighandler_t) {
    for signal in [libc::SIGINT, libc::SIGHUP, libc::SIGQUIT, libc::SIGTERM] {
        if matches!(signal, libc::SIGINT | libc::SIGHUP) {
            // SAFETY: a null action only queries the current process disposition.
            let mut previous = mem::MaybeUninit::<sigaction>::uninit();
            if unsafe {
                libc::sigaction(signal, ptr::null(), previous.as_mut_ptr()) == 0
                    && previous.assume_init().sa_sigaction == libc::SIG_IGN
            } {
                continue;
            }
        }
        install(signal, handler, 0);
    }
}

#[no_mangle]
pub extern "C" fn RestoreDefaultSignalHandlers() {
    termination_handlers(libc::SIG_DFL);
    for signal in [libc::SIGPIPE, libc::SIGCHLD].into_iter().chain(FATAL) {
        install(signal, libc::SIG_DFL, 0);
    }
}

#[no_mangle]
pub extern "C" fn SetupWmSignalHandlers(_: c_int) {
    termination_handlers(QuitWmSignalHandler as *const () as sighandler_t);
    install(libc::SIGCHLD, libc::SIG_IGN, libc::SA_NOCLDWAIT);
    for signal in FATAL {
        install(signal, abort_handler as *const () as sighandler_t, 0);
    }
}

extern "C" fn abort_handler(signal: c_int) {
    // Preserve EMWM's X11 focus handoff and the original terminating signal.
    // SAFETY: these handlers are installed after initializing the WM display.
    unsafe {
        XSetInputFocus(wmGD.display, 1, 1, 0);
        XSync(wmGD.display, 0);
        XCloseDisplay(wmGD.display);
        install(signal, libc::SIG_DFL, 0);
        libc::kill(libc::getpid(), signal);
    }
}

#[no_mangle]
pub extern "C" fn QuitWmSignalHandler(_: c_int) {
    const SHOW_QUIT_DIALOG: i64 = 1 << 6;
    const QUIT_ACTION: c_int = 3;
    // SAFETY: the active screen and display belong to the initialized Xt application.
    unsafe {
        if i64::from(wmGD.showFeedback) & SHOW_QUIT_DIALOG != 0 {
            ConfirmAction(wmGD.pActiveSD, QUIT_ACTION);
            XFlush(wmGD.display);
        } else {
            Do_Quit_Mwm(0);
        }
    }
}

#[no_mangle]
pub extern "C" fn ChildProcSignalHandler(_: c_int) {
    // SAFETY: signal returns the disposition to reinstall. wait accepts null
    // when the caller only needs to reap the child.
    unsafe {
        let interrupt = libc::signal(libc::SIGINT, libc::SIG_IGN);
        let quit = libc::signal(libc::SIGQUIT, libc::SIG_IGN);
        libc::wait(ptr::null_mut());
        libc::signal(
            libc::SIGCHLD,
            ChildProcSignalHandler as *const () as sighandler_t,
        );
        libc::signal(libc::SIGINT, interrupt);
        libc::signal(libc::SIGQUIT, quit);
    }
}
