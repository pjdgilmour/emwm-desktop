// SPDX-License-Identifier: GPL-3.0-only
use emwm_desktop::{home, stopping, watch_signals, xdg, OwnedChild};
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{self, OpenOptions};
use std::io;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

// These files are user-authored POSIX shell programs. Source them in the same shell
// so assignments, exports, functions and working-directory changes reach session.conf.
const PROFILE: &str = r#"
emwm_session_exe=$1
shift
[ ! -f "$HOME/.Xprofile" ] || . "$HOME/.Xprofile"
config_dir=${XDG_CONFIG_HOME:-"$HOME/.config"}/emwm-desktop
EMWM_DESKTOP_COMPOSITOR=1 EMWM_DESKTOP_ICONS=1 EMWM_DESKTOP_TRAY=1
EMWM_DESKTOP_NETWORK=1 EMWM_DESKTOP_KEYRING=1 EMWM_DESKTOP_POLKIT=1
[ ! -f "$config_dir/session.conf" ] || . "$config_dir/session.conf"
if [ -f "$config_dir/appearance/session.sh" ]; then
    . "$config_dir/appearance/session.sh"
    "$EMWM_DESKTOP_PREFIX/bin/emwm-appearance" --session
fi
export PATH="$EMWM_DESKTOP_PREFIX/bin:$PATH" EMWM_DESKTOP_TRAY
exec "$emwm_session_exe" --supervise "$EMWM_DESKTOP_COMPOSITOR" "$EMWM_DESKTOP_ICONS" "$EMWM_DESKTOP_NETWORK" "$EMWM_DESKTOP_KEYRING" "$EMWM_DESKTOP_POLKIT"
"#;

fn prepend(value: &OsStr, tail: Option<OsString>) -> OsString {
    let mut result = value.to_owned();
    if let Some(tail) = tail.filter(|v| !v.is_empty()) {
        result.push(":");
        result.push(tail);
    }
    result
}

fn initialize(prefix: &Path, data: &Path, args: &[OsString]) -> io::Result<i32> {
    if env::var_os("DISPLAY").filter(|v| !v.is_empty()).is_none() {
        eprintln!("EMWM Desktop requires an X11 display.");
        return Ok(1);
    }
    let exe = env::current_exe()?;
    if env::var_os("DBUS_SESSION_BUS_ADDRESS")
        .filter(|v| !v.is_empty())
        .is_none()
    {
        // SAFETY: getuid takes no arguments and has no preconditions.
        let default = format!("/run/user/{}", unsafe { libc::getuid() });
        let bus = xdg("XDG_RUNTIME_DIR", Path::new(&default)).join("bus");
        if fs::metadata(&bus).is_ok_and(|m| m.file_type().is_socket()) {
            let mut address = OsString::from("unix:path=");
            address.push(bus);
            env::set_var("DBUS_SESSION_BUS_ADDRESS", address);
        } else {
            return Err(Command::new("dbus-run-session")
                .arg("--")
                .arg(&exe)
                .args(args)
                .exec());
        }
    }
    let home = home()?;
    let state = xdg("XDG_STATE_HOME", &home.join(".local/state")).join("emwm-desktop");
    fs::create_dir_all(&state)?;
    let log = state.join("session.log");
    if log.is_file() {
        fs::rename(&log, state.join("session.log.old"))?;
    }
    let file = OpenOptions::new().append(true).create(true).open(log)?;
    let path = prepend(home.join(".local/bin").as_os_str(), env::var_os("PATH"));
    let data_dirs = env::var_os("XDG_DATA_DIRS")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "/usr/local/share:/usr/share".into());
    Err(Command::new("/bin/sh")
        .args(["-c", PROFILE, "emwm-desktop-session"])
        .arg(exe)
        .args(args)
        .env("EMWM_DESKTOP_PREFIX", prefix)
        .env("EMWM_DESKTOP_DATA", data)
        .env("PATH", prepend(prefix.join("bin").as_os_str(), Some(path)))
        .env("XDG_CURRENT_DESKTOP", "EMWM")
        .env("XDG_SESSION_DESKTOP", "emwm-desktop")
        .env("XDG_SESSION_TYPE", "x11")
        .env(
            "XFILESEARCHPATH",
            prepend(
                data.join("app-defaults/%N").as_os_str(),
                env::var_os("XFILESEARCHPATH"),
            ),
        )
        .env("XDG_DATA_DIRS", prepend(data.as_os_str(), Some(data_dirs)))
        .stdout(file.try_clone()?)
        .stderr(file)
        .exec())
}

fn start_once(pattern: &str, executable: &str) {
    // SAFETY: getuid takes no arguments and has no preconditions.
    let uid = unsafe { libc::getuid() }.to_string();
    if !Command::new("pgrep")
        .args(["-u", &uid, "-f", pattern])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
    {
        let _ = Command::new(executable).stdin(Stdio::null()).spawn();
    }
}

fn owned(path: &Path) -> io::Result<OwnedChild> {
    Command::new(path)
        .stdin(Stdio::null())
        .spawn()
        .map(OwnedChild)
}

fn supervise(prefix: &Path, data: &Path, flags: &[OsString]) -> io::Result<i32> {
    let home = home()?;
    for path in [
        data.join("defaults.Xresources"),
        home.join(".Xdefaults"),
        home.join(".Xresources"),
    ] {
        if path.is_file() || path == data.join("defaults.Xresources") {
            let _ = Command::new("xrdb").arg("-merge").arg(path).status();
        }
    }
    let _ = Command::new("dbus-update-activation-environment")
        .args([
            "--systemd",
            "DISPLAY",
            "XAUTHORITY",
            "XDG_CURRENT_DESKTOP",
            "XDG_SESSION_DESKTOP",
            "XDG_SESSION_TYPE",
            "PATH",
            "XFILESEARCHPATH",
            "XDG_DATA_DIRS",
        ])
        .status();
    watch_signals()?;
    let bin = prefix.join("bin");
    let _compositor = if flags[0] == "1" {
        owned(&bin.join("emwm-compositor")).ok()
    } else {
        None
    };
    if flags[4] == "1" {
        start_once(
            "^(/[^ ]*/)?polkit-mate-authentication-agent-1([[:space:]]|$)",
            "/usr/libexec/polkit-mate-authentication-agent-1",
        );
    }
    if flags[3] == "1" {
        let _ = Command::new("gnome-keyring-daemon")
            .args(["--start", "--components=secrets,pkcs11"])
            .status();
    }
    if flags[2] == "1" {
        start_once("^(/[^ ]*/)?nm-applet([[:space:]]|$)", "nm-applet");
    }
    let mut manager = owned(&bin.join("xmsm"))?;
    let _icons = if flags[1] == "1" {
        owned(&bin.join("emwm-desktop-icons")).ok()
    } else {
        None
    };
    while !stopping() {
        if let Some(status) = manager.0.try_wait()? {
            return Ok(status
                .code()
                .unwrap_or_else(|| 128 + status.signal().unwrap_or(0)));
        }
        sleep(Duration::from_millis(50));
    }
    Ok(0)
}

fn main() {
    let prefix = xdg("EMWM_DESKTOP_PREFIX", Path::new("/usr/lib/emwm-desktop"));
    let data = xdg("EMWM_DESKTOP_DATA", Path::new("/usr/share/emwm-desktop"));
    let args: Vec<_> = env::args_os().skip(1).collect();
    let result = if args.len() == 6 && args[0] == "--supervise" {
        supervise(&prefix, &data, &args[1..])
    } else {
        initialize(&prefix, &data, &args)
    };
    std::process::exit(result.unwrap_or_else(|e| {
        eprintln!("emwm-desktop-session: {e}");
        1
    }));
}
