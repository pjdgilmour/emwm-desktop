// Copyright (C) 2018–2026 alx@fastestcode.org. SPDX-License-Identifier: MIT
//! Commands written in toolboxrc use the historical Toolchest quoting rules.
use crate::command_line::split_command;
use std::ffi::{c_char, c_int, CStr, CString, OsString};
use std::os::unix::{ffi::OsStringExt, process::CommandExt};
use std::process::Command;
use std::ptr::null_mut;

type Handler = Option<unsafe extern "C" fn(c_int)>;
pub(crate) unsafe fn rsignal(sig: c_int, handler: Handler) {
    // SAFETY: called at program start with a handler valid for the lifetime of
    // the process. A zeroed sigaction has an empty mask and no restorer.
    let mut action: libc::sigaction = std::mem::zeroed();
    action.sa_sigaction = handler.map_or(0, |h| h as usize);
    action.sa_flags = libc::SA_NOCLDSTOP | libc::SA_RESTART;
    libc::sigaction(sig, &action, null_mut());
}
pub(crate) unsafe fn print_version(name: *const c_char) {
    // SAFETY: the call site supplies the static Xt application name.
    println!("{} v2.0.0", CStr::from_ptr(name).to_string_lossy());
}
pub(crate) unsafe fn get_login() -> *mut c_char {
    // SAFETY: this Xt startup callback is single-threaded. tbmain copies the
    // resulting pw_name into its caption before any other passwd lookup.
    let pwd = libc::getpwuid(libc::getuid());
    if pwd.is_null() {
        null_mut()
    } else {
        (*pwd).pw_name
    }
}

fn expand(input: &[u8]) -> Result<Vec<u8>, c_int> {
    let mut out = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if input[i] != b'$' {
            out.push(input[i]);
            i += 1;
            continue;
        }
        i += 1;
        if input.get(i) == Some(&b'$') {
            out.push(b'$');
            i += 1;
            continue;
        }
        let name = if input.get(i) == Some(&b'{') {
            i += 1;
            let start = i;
            let end = input[i..]
                .iter()
                .position(|&b| b == b'}')
                .ok_or(libc::EINVAL)?
                + i;
            if end == start {
                break;
            }
            i = end + 1;
            &input[start..end]
        } else {
            let start = i;
            while i < input.len() && !b" \"'\t.\\/".contains(&input[i]) {
                i += 1;
            }
            if start == i {
                return Err(libc::EINVAL);
            }
            &input[start..i]
        };
        let name = OsString::from_vec(name.to_vec());
        if let Some(value) = std::env::var_os(&name) {
            out.extend(value.into_vec());
        } else {
            eprintln!("Undefined environment variable {}", name.to_string_lossy());
        }
        // The C parser consumes the first delimiter (or first byte following
        // a braced name) as literal text before looking for another expansion.
        if let Some(&b) = input.get(i) {
            out.push(b);
            i += 1;
        }
    }
    Ok(out)
}

pub(crate) unsafe fn expand_env_vars(input: *const c_char, out: *mut *mut c_char) -> c_int {
    // SAFETY: tbmain provides a C string and writable storage. Its calls to
    // libc::free require that the result also uses libc's allocator.
    let result = match expand(CStr::from_ptr(input).to_bytes()) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Ok(result) = CString::new(result) else {
        return libc::EINVAL;
    };
    *out = libc::strdup(result.as_ptr());
    if (*out).is_null() {
        libc::ENOMEM
    } else {
        0
    }
}

pub(crate) unsafe fn exec_command(command: *const c_char) -> c_int {
    // SAFETY: the Motif callback owns the input buffer throughout this call.
    let words = match split_command(CStr::from_ptr(command).to_bytes()) {
        Ok(words) => words,
        Err(()) => return libc::EINVAL,
    };
    let mut child = Command::new(&words[0]);
    child.args(&words[1..]);
    // SAFETY: setsid is async-signal-safe and only called in the forked child.
    child.pre_exec(|| {
        if libc::setsid() < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    });
    match child.spawn() {
        Ok(_) => 0,
        Err(e) => e.raw_os_error().unwrap_or(libc::EIO),
    }
}
