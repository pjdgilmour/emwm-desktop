// SPDX-License-Identifier: MIT
//! The Xt process permanently drops its saved IDs before opening a display.
//! Its peer receives fixed opcodes over an inherited close-on-exec socket.
use std::ffi::{CStr, CString};
use std::io::{self, Read, Write};
use std::os::fd::{FromRawFd, IntoRawFd};
use std::os::unix::{net::UnixStream, process::CommandExt};
use std::process::{Command, Stdio};

#[link(name = "crypt")]
extern "C" {
    fn crypt(pass: *const libc::c_char, salt: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    fn explicit_bzero(p: *mut libc::c_void, n: usize);
}
pub fn wipe(bytes: &mut [u8]) {
    // SAFETY: the slice owns exactly this many writable bytes.
    unsafe {
        explicit_bzero(bytes.as_mut_ptr().cast(), bytes.len());
    }
}
pub const PASSWORD_LIMIT: usize = 4096;
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Request {
    CanLock = 1,
    Verify,
    Suspend,
    Shutdown,
    Reboot,
}

fn hash(uid: libc::uid_t) -> Option<CString> {
    // SAFETY: NSS owns each returned record. The single-threaded broker copies
    // the result before making another NSS call. Only its caller's uid is used.
    unsafe {
        let record = libc::getpwuid(uid).as_ref()?;
        if record.pw_passwd.is_null() || record.pw_name.is_null() {
            return None;
        }
        let mut value = CStr::from_ptr(record.pw_passwd);
        if value.to_bytes() == b"x" {
            let protected = libc::getspnam(record.pw_name).as_ref()?;
            if protected.sp_pwdp.is_null() {
                return None;
            }
            value = CStr::from_ptr(protected.sp_pwdp);
        }
        if matches!(value.to_bytes().first(), None | Some(b'*' | b'!')) {
            return None;
        }
        Some(value.to_owned())
    }
}
fn check_password(password: &CStr, hash: &CStr) -> bool {
    // SAFETY: this broker alone calls crypt, with two valid NUL-terminated strings.
    unsafe {
        let result = crypt(password.as_ptr(), hash.as_ptr());
        if result.is_null() {
            return false;
        }
        let result = CStr::from_ptr(result).to_bytes();
        !result.starts_with(b"*")
            && result.len() == hash.to_bytes().len()
            && result
                .iter()
                .zip(hash.to_bytes())
                .fold(0u8, |diff, (a, b)| diff | (a ^ b))
                == 0
    }
}
fn power(op: u8) -> io::Result<()> {
    let (bin, args): (&str, &[&str]) = match op {
        3 => ("/usr/bin/systemctl", &["suspend"]),
        4 => ("/sbin/poweroff", &[]),
        5 => ("/sbin/reboot", &[]),
        _ => return Err(io::Error::from(io::ErrorKind::InvalidInput)),
    };
    let mut command = Command::new(bin);
    command
        .args(args)
        .current_dir("/")
        .env_clear()
        .env("PATH", "/usr/sbin:/usr/bin:/sbin:/bin")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .gid(0)
        .uid(0)
        .process_group(0);
    // SAFETY: after fork this hook only makes a raw syscall and reads errno.
    // Preserve the spawn error pipe until exec, and prevent NSS descriptors
    // opened after broker startup from reaching a system command.
    unsafe {
        command.pre_exec(|| {
            if libc::syscall(
                libc::SYS_close_range,
                3u32,
                u32::MAX,
                libc::CLOSE_RANGE_CLOEXEC,
            ) == 0
            {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        });
    }
    command.spawn()?;
    Ok(())
}
fn serve(mut stream: UnixStream, uid: libc::uid_t) -> io::Result<()> {
    let mut header = [0u8; 3];
    while stream.read_exact(&mut header).is_ok() {
        let count = u16::from_be_bytes([header[1], header[2]]) as usize;
        if count > PASSWORD_LIMIT || (header[0] != 2 && count != 0) {
            break;
        }
        let ok = match header[0] {
            kind @ (1 | 2) => {
                let mut secret = vec![0u8; count + 1];
                stream.read_exact(&mut secret[..count])?;
                let hash = hash(uid);
                let ok = match hash {
                    Some(h) => {
                        let result = kind == 1
                            || CStr::from_bytes_with_nul(&secret)
                                .is_ok_and(|p| check_password(p, &h));
                        wipe(&mut h.into_bytes_with_nul());
                        result
                    }
                    None => false,
                };
                wipe(&mut secret);
                ok
            }
            kind @ 3..=5 => power(kind).is_ok(),
            _ => break,
        };
        stream.write_all(&[ok as u8])?;
    }
    Ok(())
}
pub struct Access(Option<UnixStream>);
impl Access {
    #[cfg(test)]
    pub(super) fn synthetic() -> io::Result<Self> {
        let (client, mut fake) = UnixStream::pair()?;
        // This only exists in cargo test binaries. No real account is read.
        std::thread::spawn(move || {
            let mut header = [0u8; 3];
            while fake.read_exact(&mut header).is_ok() {
                let n = u16::from_be_bytes([header[1], header[2]]) as usize;
                let mut password = vec![0; n];
                if fake.read_exact(&mut password).is_err() {
                    break;
                }
                let ok = match header[0] {
                    1 => true,
                    2 => password == b"demo",
                    _ => false,
                };
                if fake.write_all(&[u8::from(ok)]).is_err() {
                    break;
                }
            }
        });
        Ok(Self(Some(client)))
    }
    pub fn new(program: &str) -> io::Result<Self> {
        // SAFETY: IDs are read from the kernel. Fork occurs before Xt, threads
        // or application callbacks. The child never returns to GUI startup.
        unsafe {
            if libc::geteuid() != 0 {
                eprintln!(
                    "[XMSM] {program} must be setuid root to enable screen locking capabilities."
                );
                return Ok(Self(None));
            }
            let uid = libc::getuid();
            let gid = libc::getgid();
            let (client, server) = UnixStream::pair()?;
            match libc::fork() {
                -1 => return Err(io::Error::last_os_error()),
                0 => {
                    drop(client);
                    // This child is the only process which retains the saved root UID.
                    // Detach it from caller-controlled files, directories and environment
                    // before NSS/libcrypt is invoked. Keep only the close-on-exec socket.
                    if libc::clearenv() != 0
                        || libc::chdir(c"/".as_ptr()) != 0
                        || libc::prctl(libc::PR_SET_DUMPABLE, 0) != 0
                    {
                        libc::_exit(1);
                    }
                    libc::umask(0o077);
                    let fd = server.into_raw_fd();
                    if fd != 3 && libc::dup3(fd, 3, libc::O_CLOEXEC) < 0 {
                        libc::_exit(1);
                    }
                    if libc::syscall(libc::SYS_close_range, 4u32, u32::MAX, 0u32) != 0 {
                        libc::_exit(1);
                    }
                    let devnull = libc::open(c"/dev/null".as_ptr(), libc::O_RDWR | libc::O_CLOEXEC);
                    if devnull < 0 {
                        libc::_exit(1);
                    }
                    for dest in 0..=2 {
                        if libc::dup2(devnull, dest) < 0 {
                            libc::_exit(1);
                        }
                    }
                    if devnull > 3 {
                        libc::close(devnull);
                    }
                    let server = UnixStream::from_raw_fd(3);
                    libc::signal(libc::SIGCHLD, libc::SIG_IGN);
                    let code = if serve(server, uid).is_ok() { 0 } else { 1 };
                    libc::_exit(code);
                }
                _ => drop(server),
            }
            if libc::setresgid(gid, gid, gid) != 0 || libc::setresuid(uid, uid, uid) != 0 {
                libc::_exit(1);
            }
            if libc::prctl(libc::PR_SET_DUMPABLE, 0) != 0 {
                libc::_exit(1);
            }
            Ok(Self(Some(client)))
        }
    }
    pub fn request(&self, op: Request, value: &[u8]) -> bool {
        let Some(mut stream) = self.0.as_ref() else {
            return false;
        };
        if value.len() > PASSWORD_LIMIT {
            return false;
        }
        let len = (value.len() as u16).to_be_bytes();
        let mut result = [0];
        stream
            .write_all(&[op as u8, len[0], len[1]])
            .and_then(|()| stream.write_all(value))
            .and_then(|()| stream.read_exact(&mut result))
            .is_ok()
            && result == [1]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn synthetic_password() {
        let hash = c"abJnggxhB/yWI"; // public DES-crypt test vector for "password"
        assert!(super::check_password(c"password", hash));
        assert!(!super::check_password(c"incorrect", hash));
        assert!(!super::check_password(c"incorrect", c"*0"));
    }
}
