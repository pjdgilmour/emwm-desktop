// SPDX-License-Identifier: GPL-3.0-only
//! Process lifetime and environment conventions shared by the X11 session helpers.
use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

static STOP: AtomicBool = AtomicBool::new(false);

extern "C" fn stop(_: libc::c_int) {
    STOP.store(true, Ordering::Relaxed);
}

pub fn watch_signals() -> io::Result<()> {
    for sig in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
        // SAFETY: the signal handler only performs an atomic store and has C ABI.
        if unsafe { libc::signal(sig, stop as *const () as libc::sighandler_t) } == libc::SIG_ERR {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

pub fn stopping() -> bool {
    STOP.load(Ordering::Relaxed)
}

pub struct OwnedChild(pub Child);

impl OwnedChild {
    fn wait_until(
        &mut self,
        timeout: Duration,
        interruptible: bool,
    ) -> io::Result<Option<ExitStatus>> {
        let deadline = Instant::now() + timeout;
        loop {
            if let Some(status) = self.0.try_wait()? {
                return Ok(Some(status));
            }
            if interruptible && stopping() {
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "sessão encerrada",
                ));
            }
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return Ok(None);
            }
            std::thread::sleep(left.min(Duration::from_millis(10)));
        }
    }
}

impl Drop for OwnedChild {
    fn drop(&mut self) {
        if !matches!(self.0.try_wait(), Ok(None)) {
            return;
        }
        // SAFETY: a live owned child or unreaped zombie retains its PID.
        unsafe {
            libc::kill(self.0.id() as libc::pid_t, libc::SIGTERM);
        }
        if !matches!(self.wait_until(Duration::from_secs(5), false), Ok(Some(_))) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}

pub fn home() -> io::Result<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME"))
}

pub fn xdg(var: &str, default: &Path) -> PathBuf {
    env::var_os(var)
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| default.to_owned())
}

pub fn timed(command: &mut Command, timeout: Duration) -> io::Result<ExitStatus> {
    let mut child = OwnedChild(command.spawn()?);
    child
        .wait_until(timeout, true)?
        .ok_or_else(|| io::Error::new(io::ErrorKind::TimedOut, format!("{command:?}")))
}
