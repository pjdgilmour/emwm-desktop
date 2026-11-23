// Copyright (C) 2018–2026 alx@fastestcode.org. See emwm-utils/COPYING (MIT).
// xinit/xdm entry point from emwm-utils. This is distinct from the desktop supervisor.
use std::env;
use std::fs;
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::process::CommandExt;
use std::process::{Command, ExitCode};

// A .Xprofile can define shell functions, change directory, and reassign these paths.
// Keep its continuation in the same POSIX shell when a profile is present.
const PROFILE: &str = r#"
LOGFILE=$1 XPROFILE=$2 XDEFAULTS=$3 XRESOURCES=$4
emwm_xmsm=$5
shift 5
echo "Sourcing $XPROFILE"
. "$XPROFILE"
if [ -f "$XDEFAULTS" ]; then echo "Merging $XDEFAULTS"; xrdb -merge "$XDEFAULTS"; fi
if [ -f "$XRESOURCES" ]; then echo "Merging $XRESOURCES"; xrdb -merge "$XRESOURCES"; fi
echo "Starting the session manager"
exec "$emwm_xmsm"
"#;

fn run() -> io::Result<()> {
    let home = emwm_desktop::home()?;
    let log = home.join(".xmsession.log");
    if log.is_file() {
        if let Err(e) = fs::copy(&log, home.join(".xmsession.log.old")) {
            eprintln!("xmsession: {e}");
        }
    }
    let file = fs::File::create(&log)?;
    for fd in [libc::STDOUT_FILENO, libc::STDERR_FILENO] {
        // SAFETY: file is open throughout both dup2 calls and the target is a standard descriptor.
        if unsafe { libc::dup2(file.as_raw_fd(), fd) } < 0 {
            return Err(io::Error::last_os_error());
        }
    }
    let manager = env::current_exe()?.with_file_name("xmsm");
    let profile = home.join(".Xprofile");
    let defaults = home.join(".Xdefaults");
    let resources = home.join(".Xresources");
    if profile.is_file() {
        let argv0 = env::args_os().next().unwrap_or_else(|| "xmsession".into());
        return Err(Command::new("/bin/sh")
            .arg("-c")
            .arg(PROFILE)
            .arg(argv0)
            .args([log, profile, defaults, resources, manager])
            .args(env::args_os().skip(1))
            .exec());
    }
    for resource in [defaults, resources] {
        if resource.is_file() {
            println!("Merging {}", resource.display());
            if let Err(err) = Command::new("xrdb").arg("-merge").arg(&resource).status() {
                eprintln!("xmsession: xrdb: {err}");
            }
        }
    }
    println!("Starting the session manager");
    Err(Command::new(manager).exec())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("xmsession: {err}");
            ExitCode::FAILURE
        }
    }
}
