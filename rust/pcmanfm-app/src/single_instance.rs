// SPDX-License-Identifier: GPL-2.0-or-later
// PCManFM single-instance IPC. Original protocol by Hong Jen Yee (2010)
// and Andriy Grytsenko (2012–2018).
use glib::{ffi, ControlFlow, IOCondition, SourceId};
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{c_char, c_int, CStr, CString, OsString},
    fs::{self, DirBuilder, File},
    io::{self, Read, Write},
    os::{
        fd::{AsRawFd, RawFd},
        unix::{
            ffi::{OsStrExt, OsStringExt},
            fs::{DirBuilderExt, FileTypeExt, MetadataExt},
            net::{UnixListener, UnixStream},
        },
    },
    path::PathBuf,
    ptr,
    rc::Rc,
};

type Callback = Option<unsafe extern "C" fn(*const c_char, c_int)>;

#[repr(C)]
pub struct Instance {
    name: *const c_char,
    callback: Callback,
    entries: *const ffi::GOptionEntry,
    screen: c_int,
    server: *mut Server,
    socket: c_int,
    watch: u32,
}

struct Server {
    path: PathBuf,
    inode: u64,
    listener: SourceId,
    clients: Rc<RefCell<HashMap<RawFd, SourceId>>>,
}

fn socket_path(name: &CStr) -> PathBuf {
    let display = std::env::var_os("DISPLAY").unwrap_or_default();
    let display = display.as_bytes();
    let (host, number) = display
        .iter()
        .rposition(|&c| c == b':')
        .map(|colon| (&display[..colon], &display[colon + 1..]))
        .unwrap_or_default();
    let number: Vec<_> = number
        .iter()
        .copied()
        .take_while(u8::is_ascii_digit)
        .collect();
    let number = std::str::from_utf8(&number)
        .ok()
        .and_then(|n| n.parse::<i32>().ok())
        .unwrap_or(0);
    let file = [
        name.to_bytes(),
        b"-socket-",
        host,
        b"-",
        number.to_string().as_bytes(),
    ]
    .concat();
    glib::user_runtime_dir().join(OsString::from_vec(file))
}

fn escaped(s: &CStr) -> Vec<u8> {
    // SAFETY: g_strescape accepts a NUL-terminated string and returns a GLib-owned copy.
    unsafe {
        let copy = ffi::g_strescape(s.as_ptr(), ptr::null());
        let bytes = CStr::from_ptr(copy).to_bytes().to_vec();
        ffi::g_free(copy.cast());
        bytes
    }
}

fn line(out: &mut Vec<u8>, s: &[u8]) {
    out.extend_from_slice(s);
    out.push(b'\n');
}

fn file_arg(name: &CStr, cwd: &[u8]) -> Vec<u8> {
    let bytes = name.to_bytes();
    if bytes == b"~" || bytes.starts_with(b"/") {
        return escaped(name);
    }
    if bytes.starts_with(b"~/") {
        if let Some(home) = std::env::var_os("HOME").and_then(|v| CString::new(v.into_vec()).ok()) {
            return [escaped(&home), escaped(name)[1..].to_vec()].concat();
        }
        return escaped(name);
    }
    // SAFETY: g_uri_peek_scheme borrows a valid NUL-terminated name.
    if !unsafe { ffi::g_uri_peek_scheme(name.as_ptr()) }.is_null() {
        return escaped(name);
    }
    [cwd, b"/", &escaped(name)].concat()
}

unsafe fn write_options(inst: &Instance, mut stream: UnixStream) -> io::Result<()> {
    let cwd = CString::new(std::env::current_dir()?.as_os_str().as_bytes())?;
    let cwd = escaped(&cwd);
    let mut message = Vec::new();
    line(&mut message, &cwd);
    line(&mut message, inst.screen.to_string().as_bytes());
    let mut entries = inst.entries;
    // SAFETY: the GTK option table has a null-name sentinel and each arg_data
    // points to the storage type indicated by its arg, for the instance's lifetime.
    unsafe {
        while !entries.is_null() && !(*entries).long_name.is_null() {
            let e = &*entries;
            entries = entries.add(1);
            let name = CStr::from_ptr(e.long_name).to_bytes();
            let flag = [b"--".as_slice(), name].concat();
            if e.arg_data.is_null() {
                continue;
            }
            match e.arg {
                ffi::G_OPTION_ARG_NONE if *e.arg_data.cast::<c_int>() != 0 => {
                    line(&mut message, &flag)
                }
                ffi::G_OPTION_ARG_INT => {
                    let n = *e.arg_data.cast::<c_int>();
                    if n >= 0 {
                        line(&mut message, &flag);
                        line(&mut message, n.to_string().as_bytes());
                    }
                }
                ffi::G_OPTION_ARG_STRING | ffi::G_OPTION_ARG_FILENAME => {
                    let val = *e.arg_data.cast::<*const c_char>();
                    if !val.is_null() {
                        let s = CStr::from_ptr(val);
                        if !s.is_empty() {
                            line(&mut message, &flag);
                            if s.to_bytes().starts_with(b"--") {
                                line(&mut message, b"--");
                            }
                            line(&mut message, &escaped(s));
                        }
                    }
                }
                ffi::G_OPTION_ARG_STRING_ARRAY | ffi::G_OPTION_ARG_FILENAME_ARRAY => {
                    let mut vals = *e.arg_data.cast::<*const *const c_char>();
                    if !vals.is_null() && !(*vals).is_null() {
                        if !name.is_empty() {
                            line(&mut message, &flag);
                        }
                        while !(*vals).is_null() {
                            line(&mut message, &file_arg(CStr::from_ptr(*vals), &cwd));
                            vals = vals.add(1);
                        }
                    }
                }
                ffi::G_OPTION_ARG_DOUBLE => {
                    line(&mut message, &flag);
                    line(
                        &mut message,
                        format!("{:.6}", *e.arg_data.cast::<f64>()).as_bytes(),
                    );
                }
                ffi::G_OPTION_ARG_INT64 => {
                    line(&mut message, &flag);
                    line(
                        &mut message,
                        (*e.arg_data.cast::<i64>()).to_string().as_bytes(),
                    );
                }
                _ => {}
            }
        }
    }
    stream.write_all(&message)
}

fn parse_message(bytes: &[u8], entries: *const ffi::GOptionEntry, callback: Callback) {
    let mut args = Vec::new();
    for line in bytes.split_inclusive(|&b| b == b'\n') {
        let Some(line) = line.strip_suffix(b"\n") else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        if line.iter().any(|&b| b < b' ') {
            return;
        }
        let Ok(line) = CString::new(line) else { return };
        // SAFETY: g_strcompress returns a new GLib C string.
        unsafe {
            let val = ffi::g_strcompress(line.as_ptr());
            args.push(CStr::from_ptr(val).to_owned());
            ffi::g_free(val.cast());
        }
    }
    if args.len() < 2 {
        return;
    }
    // SAFETY: entries and callback are borrowed from the live instance on the GTK thread.
    // GOption rewrites the pointer vector and allocates values in the caller's option storage.
    unsafe {
        let screen = libc::atoi(args[1].as_ptr()).max(0);
        let mut argv = vec![ffi::g_get_prgname().cast_mut()];
        argv.extend(args[2..].iter().map(|s| s.as_ptr().cast_mut()));
        let Ok(mut argc) = c_int::try_from(argv.len()) else {
            return;
        };
        argv.push(ptr::null_mut());
        let mut ptr = argv.as_mut_ptr();
        let ctx = ffi::g_option_context_new(c"".as_ptr());
        ffi::g_option_context_add_main_entries(ctx, entries, ptr::null());
        ffi::g_option_context_parse(ctx, &mut argc, &mut ptr, ptr::null_mut());
        ffi::g_option_context_free(ctx);
        if let Some(cb) = callback {
            cb(args[0].as_ptr(), screen);
        }
    }
}

fn watch_client(
    mut stream: UnixStream,
    entries: *const ffi::GOptionEntry,
    callback: Callback,
    clients: Rc<RefCell<HashMap<RawFd, SourceId>>>,
) -> io::Result<()> {
    stream.set_nonblocking(true)?;
    let fd = stream.as_raw_fd();
    let tracked = Rc::clone(&clients);
    let mut message = Vec::new();
    let watch = glib::source::unix_fd_add_local(
        fd,
        IOCondition::IN | IOCondition::HUP | IOCondition::ERR,
        move |_, cond| {
            let mut chunk = [0; 8192];
            let complete = loop {
                match stream.read(&mut chunk) {
                    Ok(0) => break true,
                    Ok(n) => message.extend_from_slice(&chunk[..n]),
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e)
                        if e.kind() == io::ErrorKind::WouldBlock
                            && !cond.intersects(IOCondition::HUP | IOCondition::ERR) =>
                    {
                        return ControlFlow::Continue
                    }
                    Err(_) => break false,
                }
            };
            tracked.borrow_mut().remove(&fd);
            if complete {
                parse_message(&message, entries, callback);
            }
            ControlFlow::Break
        },
    );
    clients.borrow_mut().insert(fd, watch);
    Ok(())
}

fn init(inst: &mut Instance) -> io::Result<c_int> {
    // SAFETY: PCManFM sets name to a static C string before initializing.
    let path = socket_path(unsafe { CStr::from_ptr(inst.name) });
    let dir = path.parent().ok_or(io::ErrorKind::InvalidInput)?;
    DirBuilder::new().recursive(true).mode(0o700).create(dir)?;
    let lock = File::open(dir)?;
    // SAFETY: lock is a live directory fd. Dropping it releases this startup-only lock.
    if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX) } != 0 {
        return Err(io::Error::last_os_error());
    }
    match UnixStream::connect(&path) {
        Ok(stream) => {
            drop(lock);
            // SAFETY: option backing storage belongs to the initialized GTK instance.
            unsafe { write_options(inst, stream) }?;
            return Ok(1);
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) if e.kind() == io::ErrorKind::ConnectionRefused => {
            let meta = fs::symlink_metadata(&path)?;
            // SAFETY: getuid has no preconditions.
            if !meta.file_type().is_socket() || meta.uid() != unsafe { libc::getuid() } {
                return Err(e);
            }
            fs::remove_file(&path)?;
        }
        Err(e) => return Err(e),
    }
    let listener = UnixListener::bind(&path)?;
    let inode = fs::symlink_metadata(&path)?.ino();
    let (entries, callback) = (inst.entries, inst.callback);
    let clients = Rc::new(RefCell::new(HashMap::new()));
    let tracked = Rc::clone(&clients);
    let fd = listener.as_raw_fd();
    let source = glib::source::unix_fd_add_local(fd, IOCondition::IN, move |_, _| {
        if let Ok((stream, _)) = listener.accept() {
            let _ = watch_client(stream, entries, callback, Rc::clone(&tracked));
        }
        ControlFlow::Continue
    });
    inst.socket = fd;
    inst.watch = source.as_raw();
    inst.server = Box::into_raw(Box::new(Server {
        path,
        inode,
        listener: source,
        clients,
    }));
    Ok(0)
}

#[no_mangle]
pub unsafe extern "C" fn single_inst_init(inst: *mut Instance) -> c_int {
    // SAFETY: the caller initializes only the public fields in an otherwise
    // uninitialized stack record. Write all private fields before borrowing it.
    unsafe {
        if inst.is_null() {
            return 2;
        }
        ptr::addr_of_mut!((*inst).server).write(ptr::null_mut());
        ptr::addr_of_mut!((*inst).socket).write(-1);
        ptr::addr_of_mut!((*inst).watch).write(0);
        init(&mut *inst).unwrap_or(2)
    }
}

#[no_mangle]
pub unsafe extern "C" fn single_inst_finalize(inst: *mut Instance) {
    // SAFETY: unique live instance, previously initialized on this same GTK thread.
    unsafe {
        if inst.is_null() || (*inst).server.is_null() {
            return;
        }
        let server = Box::from_raw(std::mem::replace(&mut (*inst).server, ptr::null_mut()));
        if fs::symlink_metadata(&server.path).is_ok_and(|m| m.ino() == server.inode) {
            let _ = fs::remove_file(&server.path);
        }
        server.listener.remove();
        for (_, source) in server.clients.borrow_mut().drain() {
            source.remove();
        }
        (*inst).socket = -1;
        (*inst).watch = 0;
    }
}
