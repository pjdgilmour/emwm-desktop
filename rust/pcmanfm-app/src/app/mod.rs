// SPDX-License-Identifier: GPL-2.0-or-later
// PCManFM 1.4.0, GTK 3 / libfm 1.4 X11 backend.
use c2rust_bitfields::BitfieldStruct;
use core::ffi::*;
use std::{mem, ptr};
extern crate emwm_pcmanfm as _;

include!("ffi.rs");
include!("app_config.rs");
include!("connect_server.rs");
include!("pref.rs");
include!("desktop_actions.rs");
include!("window_actions.rs");
include!("desktop.rs");
include!("main_win.rs");
include!("application.rs");
include!("volume_manager.rs");
include!("tab_page.rs");

pub fn run() -> i32 {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStringExt;
    // Unix argv elements cannot contain embedded NUL. Keep them writable and
    // retain every allocation while GTK parses and reorders the argv array.
    let mut args: Vec<Vec<u8>> = std::env::args_os()
        .map(|arg| {
            CString::new(arg.into_vec())
                .expect("NUL in execve argv")
                .into_bytes_with_nul()
        })
        .collect();
    let mut argv: Vec<_> = args.iter_mut().map(|s| s.as_mut_ptr().cast()).collect();
    let argc = argv.len() as c_int;
    argv.push(ptr::null_mut());
    // SAFETY: argc excludes the null sentinel and GTK runs on the main thread.
    unsafe { main_0(argc, argv.as_mut_ptr()) }
}
