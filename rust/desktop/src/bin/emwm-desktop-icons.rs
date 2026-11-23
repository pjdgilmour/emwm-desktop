// SPDX-License-Identifier: GPL-3.0-only
use emwm_desktop::{home, stopping, timed, watch_signals, xdg, OwnedChild};
use sha2::{Digest, Sha256};
use std::env;
use std::error::Error;
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{self, Read, Seek, Write};
use std::os::fd::AsRawFd;
use std::os::unix::fs::{DirBuilderExt, FileTypeExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};
use x11rb::connection::Connection;
use x11rb::connection::RequestConnection;
use x11rb::protocol::{randr, xproto::*, Event};
use x11rb::rust_connection::RustConnection;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const PROFILE: &str = "[*]\nwallpaper_mode=color\nwallpaper_common=1\ndesktop_bg=#516286\ndesktop_fg=#ffffff\ndesktop_shadow=#000000\ndesktop_font=Sans 10\nshow_wm_menu=0\nshow_trash=1\nshow_documents=0\nshow_mounts=1\n";
const SHORT: Duration = Duration::from_secs(5);

fn resource_path(value: &str) -> Option<String> {
    let value = value.trim_start();
    let mut chars = value.strip_prefix('"').unwrap_or(value).chars();
    let quoted = value.starts_with('"');
    let mut result = String::new();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted => return Some(result),
            '#' if !quoted => break,
            ch if !quoted && ch.is_whitespace() => break,
            '\\' => result.push(chars.next()?),
            ch => result.push(ch),
        }
    }
    (!quoted).then_some(result)
}

fn wallpaper(resources: &str, workspace: u32) -> Option<PathBuf> {
    resources
        .lines()
        .filter_map(|line| {
            let (key, value) = line.split_once(':')?;
            let mut fields = key.split(['*', '.']);
            if fields.next()? != "Emwm" {
                return None;
            }
            let ws = fields.next()?.strip_prefix("ws")?;
            if !ws.bytes().all(|b| b.is_ascii_digit())
                || ws.parse::<u32>().ok()? != workspace
                || !fields.eq(["backdrop", "image"])
            {
                return None;
            }
            resource_path(value)
        })
        .next_back()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
}

fn prepare(config: &Path, monitors: usize) -> io::Result<()> {
    let dir = config.join("pcmanfm/emwm-desktop");
    fs::create_dir_all(&dir)?;
    for i in 0..monitors.max(1) {
        match File::create_new(dir.join(format!("desktop-items-{i}.conf"))) {
            Ok(mut f) => f.write_all(PROFILE.as_bytes())?,
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn active_socket(runtime: &Path, display: &str) -> Result<bool> {
    let (host, number) = display.rsplit_once(':').ok_or("Invalid X11 display")?;
    let screen: u32 = number.split('.').next().unwrap_or("").parse()?;
    let path = runtime.join(format!("pcmanfm-socket-{host}-{screen}"));
    let check = || -> io::Result<bool> {
        if !fs::metadata(&path)?.file_type().is_socket() {
            return Ok(false);
        }
        let text = fs::read_to_string("/proc/net/unix")?;
        Ok(text.lines().skip(1).any(|mut line| {
            for _ in 0..7 {
                let Some(i) = line.find(char::is_whitespace) else {
                    return false;
                };
                line = line[i..].trim_start();
            }
            Path::new(line) == path
        }))
    };
    match check() {
        Ok(found) => Ok(found),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => Ok(true),
        Err(e) => Err(e.into()),
    }
}

fn output(program: &str, args: &[&str]) -> Result<String> {
    let mut file = tempfile::tempfile()?;
    let status = timed(
        Command::new(program)
            .args(args)
            .stdout(file.try_clone()?)
            .stderr(Stdio::null()),
        SHORT,
    )?;
    if !status.success() {
        return Err(format!("{program} {args:?}: {status}").into());
    }
    file.rewind()?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn bundled(name: &str) -> Result<PathBuf> {
    let path = env::current_exe()?.with_file_name(name);
    Ok(if path.is_file() { path } else { name.into() })
}

fn sync_wallpaper(
    file_manager: &Path,
    file: &Path,
    workspace: u32,
    last: &mut Option<(Option<PathBuf>, Option<SystemTime>)>,
) -> Result<()> {
    let mut text = output("xrdb", &["-query"])?;
    match fs::read_to_string(file) {
        Ok(s) => {
            text.push('\n');
            text.push_str(&s);
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => (),
        Err(e) => return Err(e.into()),
    }
    let path = wallpaper(&text, workspace).filter(|p| p.is_file());
    let stamp = path
        .as_ref()
        .map(|p| fs::metadata(p)?.modified())
        .transpose()?;
    let current = (path, stamp);
    if last.as_ref() != Some(&current) {
        let mut cmd = Command::new(file_manager);
        cmd.arg("--profile=emwm-desktop");
        if let Some(path) = &current.0 {
            cmd.arg("--set-wallpaper")
                .arg(path)
                .arg("--wallpaper-mode=screen");
        } else {
            cmd.arg("--wallpaper-mode=color");
        }
        if !timed(&mut cmd, SHORT)?.success() {
            return Err("PCManFM wallpaper command failed".into());
        }
        *last = Some(current);
    }
    Ok(())
}

fn workspace(conn: &RustConnection, root: Window, atom: Atom) -> Result<u32> {
    let reply = conn
        .get_property(false, root, atom, AtomEnum::CARDINAL, 0, 1)?
        .reply()?;
    Ok(reply.value32().and_then(|mut it| it.next()).unwrap_or(0))
}

fn run() -> Result<()> {
    let display = env::var("DISPLAY")
        .ok()
        .filter(|d| !d.is_empty())
        .ok_or("EMWM desktop icons require X11.")?;
    let home = home()?;
    let config = xdg("XDG_CONFIG_HOME", &home.join(".config"));
    let runtime = xdg(
        "XDG_RUNTIME_DIR",
        &xdg("XDG_CACHE_HOME", &home.join(".cache")),
    );
    let state = xdg("XDG_STATE_HOME", &home.join(".local/state")).join("emwm-desktop");
    fs::create_dir_all(&state)?;
    let digest = format!("{:x}", Sha256::digest(display.as_bytes()));
    let lock = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(state.join(format!("icons-{}.lock", &digest[..16])))?;
    // SAFETY: lock holds this descriptor until run returns.
    if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        let e = io::Error::last_os_error();
        if e.kind() == io::ErrorKind::WouldBlock {
            return Ok(());
        }
        return Err(e.into());
    }
    if active_socket(&runtime, &display)? {
        return Err("PCManFM is already running on this display. Start the EMWM desktop before other PCManFM instances.".into());
    }
    watch_signals()?;
    let mut ready = false;
    for _ in 0..100 {
        if stopping() {
            return Ok(());
        }
        if output("xprop", &["-root", "_NET_SUPPORTING_WM_CHECK"])?.contains("0x") {
            ready = true;
            break;
        }
        sleep(Duration::from_millis(100));
    }
    if !ready {
        return Err("No X11 window manager became ready.".into());
    }
    let monitors = output("xrandr", &["--listmonitors"])
        .ok()
        .and_then(|s| s.lines().next()?.split_once(':')?.1.trim().parse().ok())
        .unwrap_or(1);
    prepare(&config, monitors)?;
    let (conn, screen) = x11rb::connect(None)?;
    let root = conn.setup().roots[screen].root;
    conn.change_window_attributes(
        root,
        &ChangeWindowAttributesAux::new().event_mask(EventMask::PROPERTY_CHANGE),
    )?
    .check()?;
    if conn.extension_information("RANDR")?.is_some() {
        randr::query_version(&conn, 1, 5)?.reply()?;
        randr::select_input(
            &conn,
            root,
            randr::NotifyMask::SCREEN_CHANGE
                | randr::NotifyMask::CRTC_CHANGE
                | randr::NotifyMask::OUTPUT_CHANGE,
        )?
        .check()?;
    }
    let desktop_atom = conn
        .intern_atom(false, b"_NET_CURRENT_DESKTOP")?
        .reply()?
        .atom;
    let trash = xdg("XDG_DATA_HOME", &home.join(".local/share")).join("Trash");
    fs::create_dir_all(trash.parent().ok_or("Invalid trash directory")?)?;
    let mut builder = DirBuilder::new();
    builder.mode(0o700);
    for dir in [&trash, &trash.join("files"), &trash.join("info")] {
        if let Err(e) = builder.create(dir) {
            if e.kind() != io::ErrorKind::AlreadyExists || !dir.is_dir() {
                return Err(e.into());
            }
        }
    }
    let desktop = output("xdg-user-dir", &["DESKTOP"])?;
    if !desktop.trim().is_empty() && Path::new(desktop.trim()) != home {
        fs::create_dir_all(desktop.trim())?;
    }
    let fm = bundled("pcmanfm")?;
    let mut child = OwnedChild(
        Command::new(&fm)
            .args(["--profile=emwm-desktop", "--desktop"])
            .env("EMWM_DESKTOP_LAYOUT", "1")
            .spawn()?,
    );
    sleep(Duration::from_millis(400));
    if child.0.try_wait()?.is_some() {
        return Err("PCManFM already running on this display or failed to start; wallpaper synchronization was not attached.".into());
    }
    let file = home.join(".config/emwm/backdrops.resources");
    let tool = bundled("emwm-wallpaper-tool")?;
    let mut layout = Some(Instant::now());
    let mut next_check = Instant::now();
    let mut refresh = true;
    let mut file_stamp = None;
    let mut last = None;
    while !stopping() && child.0.try_wait()?.is_none() {
        while let Some(event) = conn.poll_for_event()? {
            match event {
                Event::PropertyNotify(e)
                    if e.atom == desktop_atom || e.atom == AtomEnum::RESOURCE_MANAGER.into() =>
                {
                    refresh = true
                }
                Event::RandrNotify(_) | Event::RandrScreenChangeNotify(_) => {
                    layout = Some(Instant::now() + Duration::from_millis(500))
                }
                _ => (),
            }
        }
        if layout.is_some_and(|t| t <= Instant::now()) {
            if !timed(
                Command::new(&tool).arg("--refresh-layout"),
                Duration::from_secs(60),
            )
            .is_ok_and(|s| s.success())
            {
                eprintln!(
                    "emwm-desktop-icons: mantendo o fundo anterior após falha na recomposição."
                );
            }
            layout = None;
            refresh = true;
        }
        if Instant::now() >= next_check {
            let stamp = fs::metadata(&file).and_then(|m| m.modified()).ok();
            if stamp != file_stamp {
                refresh = true;
                file_stamp = stamp;
            }
            next_check = Instant::now() + Duration::from_secs(1);
        }
        if refresh {
            sync_wallpaper(&fm, &file, workspace(&conn, root, desktop_atom)?, &mut last)?;
            refresh = false;
        }
        let mut poll = libc::pollfd {
            fd: conn.stream().as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        // SAFETY: poll points at a single initialized descriptor, which conn owns.
        let n = unsafe { libc::poll(&mut poll, 1, 500) };
        if n < 0 && io::Error::last_os_error().kind() != io::ErrorKind::Interrupted {
            return Err(io::Error::last_os_error().into());
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("emwm-desktop-icons: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::net::UnixListener;
    #[test]
    fn paths_and_workspace_precedence() {
        let text = "Emwm*ws0*backdrop*image: no\nEmwm.ws0.backdrop.image: \"/Área de trabalho/a \\\"b\\\".png\"\nEmwm*ws0*backdrop*image: \"unfinished";
        assert_eq!(
            wallpaper(text, 0),
            Some(PathBuf::from("/Área de trabalho/a \"b\".png"))
        );
        assert_eq!(wallpaper(text, 1), None);
        assert_eq!(
            resource_path("one\\ two#comment").as_deref(),
            Some("one two")
        );
    }
    #[test]
    fn profiles_preserve_existing_positions() -> io::Result<()> {
        let tmp = tempfile::tempdir()?;
        prepare(tmp.path(), 0)?;
        let file = tmp.path().join("pcmanfm/emwm-desktop/desktop-items-0.conf");
        assert_eq!(fs::read_to_string(&file)?, PROFILE);
        fs::write(&file, "[file]\nx=42\n")?;
        prepare(tmp.path(), 2)?;
        assert_eq!(fs::read_to_string(&file)?, "[file]\nx=42\n");
        assert!(file.with_file_name("desktop-items-1.conf").exists());
        Ok(())
    }
    #[test]
    fn distinguishes_live_and_stale_socket() -> Result<()> {
        let tmp = tempfile::tempdir()?;
        let path = tmp.path().join("pcmanfm-socket--17");
        let server = UnixListener::bind(path)?;
        assert!(active_socket(tmp.path(), ":17.0")?);
        drop(server);
        assert!(!active_socket(tmp.path(), ":17")?);
        Ok(())
    }
}
