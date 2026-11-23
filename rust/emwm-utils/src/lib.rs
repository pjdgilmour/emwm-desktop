// SPDX-License-Identifier: MIT
mod parser;

use std::cell::RefCell;
use std::ffi::{c_char, c_int, CStr, CString, OsStr};
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

#[repr(C)]
pub struct TbEntry {
    kind: parser::Kind,
    level: c_int,
    title: *mut c_char,
    mnemonic: c_char,
    command: *mut c_char,
    next: *mut TbEntry,
}

#[derive(Default)]
struct Menu {
    records: Vec<TbEntry>,
    strings: Vec<Vec<u8>>,
}

impl Menu {
    fn string(&mut self, mut bytes: Vec<u8>) -> *mut c_char {
        bytes.push(0);
        let ptr = bytes.as_mut_ptr().cast();
        self.strings.push(bytes);
        ptr
    }

    fn new(parsed: Vec<parser::Entry>) -> Self {
        let mut menu = Self::default();
        for e in parsed {
            let title = if e.kind == parser::Kind::Separator {
                ptr::null_mut()
            } else {
                menu.string(e.title)
            };
            let command = if e.kind == parser::Kind::Command {
                menu.string(e.command)
            } else {
                ptr::null_mut()
            };
            menu.records.push(TbEntry {
                kind: e.kind,
                level: e.level,
                mnemonic: e.mnemonic as c_char,
                title,
                command,
                next: ptr::null_mut(),
            });
        }
        let mut next = ptr::null_mut();
        for record in menu.records.iter_mut().rev() {
            record.next = next;
            next = record;
        }
        menu
    }
}

#[derive(Default)]
struct Parser {
    menu: Menu,
    error: Option<CString>,
}

// Parsing and menu construction run on the same Xt event-loop thread.
// A failed reload keeps all records from the last successful call alive.
thread_local! { static STATE: RefCell<Parser> = RefCell::default(); }

fn read_config(filename: &Path) -> Result<Vec<u8>, c_int> {
    let errno = |e: std::io::Error| e.raw_os_error().unwrap_or(libc::EIO);
    if fs::metadata(filename).map_err(errno)?.len() == 0 {
        return Err(libc::EIO);
    }
    fs::read(filename).map_err(errno)
}

/// Parse a toolboxrc and publish its entries for the current Xt thread.
///
/// # Safety
/// `filename` must address a NUL-terminated pathname and `root` a writable
/// pointer. Returned entries belong to the parser until the next successful call.
#[no_mangle]
pub unsafe extern "C" fn tb_parse_config(
    filename: *const c_char,
    root: *mut *mut TbEntry,
) -> c_int {
    STATE.with_borrow_mut(|state| {
        state.error = None;
        if filename.is_null() || root.is_null() {
            return libc::EINVAL;
        }
        // SAFETY: the caller supplies a live C string as documented above.
        let bytes = unsafe { CStr::from_ptr(filename) }.to_bytes();
        let data = match read_config(Path::new(OsStr::from_bytes(bytes))) {
            Ok(bytes) => bytes,
            Err(errno) => return errno,
        };
        match parser::parse(&data) {
            Err(message) => {
                state.error = CString::new(message).ok();
                -1
            }
            Ok(parsed) => {
                state.menu = Menu::new(parsed);
                // SAFETY: root is writable and records retain a fixed address
                // in the owning thread-local menu until the next successful parse.
                unsafe {
                    *root = state
                        .menu
                        .records
                        .first_mut()
                        .map_or(ptr::null_mut(), |r| r);
                }
                0
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn tb_parser_error_string() -> *mut c_char {
    STATE.with_borrow(|state| {
        state
            .error
            .as_ref()
            .map_or(ptr::null_mut(), |e| e.as_ptr().cast_mut())
    })
}
