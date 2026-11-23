// SPDX-License-Identifier: GPL-3.0-only
use crate::settings::Settings;
use anyhow::{Context, Result};
use gdk::prelude::*;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::os::fd::AsRawFd;
use std::{
    env,
    ffi::OsStr,
    fs::{self, OpenOptions},
    os::unix::{ffi::OsStrExt, fs::MetadataExt, process::CommandExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::Duration,
};
use x11rb::protocol::xproto::ConnectionExt;

fn owned() -> bool {
    let probe = || -> Result<bool> {
        let (connection, screen) = x11rb::connect(None)?;
        let atom = connection
            .intern_atom(false, format!("_XSETTINGS_S{screen}").as_bytes())?
            .reply()?
            .atom;
        let owner = connection.get_selection_owner(atom)?.reply()?.owner;
        Ok(owner != 0)
    };
    probe().unwrap_or(true)
}
fn which(name: &str) -> Option<PathBuf> {
    env::split_paths(&env::var_os("PATH")?)
        .map(|d| d.join(name))
        .find(|p| {
            p.metadata()
                .is_ok_and(|m| m.is_file() && m.mode() & 0o111 != 0)
        })
}
fn reload(pidfile: &Path, config: &Path) -> bool {
    let check = || -> Option<bool> {
        let pid: i32 = fs::read_to_string(pidfile).ok()?.trim().parse().ok()?;
        if pid <= 0 {
            return None;
        }
        let path = Path::new("/proc").join(pid.to_string());
        let bytes = fs::read(path.join("cmdline")).ok()?;
        let argv: Vec<_> = bytes.split(|v| *v == 0).collect();
        // SAFETY: getuid has no arguments and does not access caller memory.
        if fs::metadata(path).ok()?.uid() != unsafe { libc::getuid() }
            || !argv.contains(&config.as_os_str().as_bytes())
            || Path::new(OsStr::from_bytes(argv.first()?)).file_name()? != "xsettingsd"
        {
            return None;
        }
        // SAFETY: pid is positive and the process identity above matched the owned daemon.
        Some(unsafe { libc::kill(pid, libc::SIGHUP) } == 0)
    };
    check() == Some(true)
}
fn xsettings(settings: &Settings) -> Result<Vec<String>> {
    let Some(display) = env::var_os("DISPLAY").filter(|d| !d.is_empty()) else {
        return Ok(vec!["Atualização ao vivo requer uma sessão X11.".into()]);
    };
    let Some(exe) = which("xsettingsd") else {
        return Ok(vec!["Instale xsettingsd para atualizar aplicativos GTK abertos. As preferências foram salvas.".into()]);
    };
    let config = settings.directory.join("xsettingsd.conf");
    if !config.is_file() {
        return Ok(vec![]);
    }
    let directory = settings.state.join("emwm-desktop");
    fs::create_dir_all(&directory)?;
    let hash = format!("{:x}", Sha256::digest(display.as_bytes()));
    let hash = &hash[..16];
    let pidfile = directory.join(format!("xsettingsd-{hash}.pid"));
    let lock = OpenOptions::new()
        .append(true)
        .create(true)
        .open(directory.join(format!("xsettingsd-{hash}.lock")))?;
    // SAFETY: the opened descriptor remains live throughout the locked section.
    if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX) } < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    if reload(&pidfile, &config) {
        return Ok(vec![]);
    }
    if owned() {
        return Ok(vec!["Outro serviço já controla a aparência nesta sessão. Ele foi preservado; reentre no EMWM para usar estas preferências.".into()]);
    }
    let log = OpenOptions::new()
        .append(true)
        .create(true)
        .open(directory.join("appearance-xsettingsd.log"))?;
    let mut cmd = Command::new(exe);
    cmd.arg("--config")
        .arg(config)
        .stdin(Stdio::null())
        .stderr(log.try_clone()?)
        .stdout(log);
    // SAFETY: the post-fork closure calls only the async-signal-safe setsid syscall.
    unsafe {
        cmd.pre_exec(|| {
            if libc::setsid() < 0 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }
    let mut child = cmd.spawn()?;
    thread::sleep(Duration::from_millis(120));
    if child.try_wait()?.is_some() {
        return Ok(vec![
            "xsettingsd não iniciou. Consulte appearance-xsettingsd.log.".into(),
        ]);
    }
    fs::write(pidfile, child.id().to_string())?;
    Ok(vec![])
}

pub fn apply(settings: &Settings, values: &Value) -> Result<Vec<String>> {
    let mut notes = xsettings(settings)?;
    let path = settings.directory.join("appearance.resources");
    if let Some(exe) = which("xrdb").filter(|_| path.is_file()) {
        let out = Command::new(exe).arg("-merge").arg(path).output()?;
        if !out.status.success() {
            notes.push(format!(
                "Não foi possível atualizar os recursos X: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
    }
    if let Err(error) = update_preferences(values) {
        notes.push(format!("Atualização complementar indisponível: {error}"));
    }
    Ok(notes)
}

fn update_preferences(values: &Value) -> Result<()> {
    if let Some(schema) = gio::SettingsSchemaSource::default()
        .and_then(|s| s.lookup("org.gnome.desktop.interface", true))
    {
        let prefs = gio::Settings::new_full(&schema, gio::SettingsBackend::NONE, None);
        for (key, field) in [
            ("gtk-theme", "theme"),
            ("icon-theme", "icons"),
            ("cursor-theme", "cursor"),
        ] {
            if schema.has_key(key) && prefs.is_writable(key) {
                prefs.set_string(key, values[field].as_str().context("Tema inválido")?)?;
            }
        }
        if schema.has_key("cursor-size") && prefs.is_writable("cursor-size") {
            prefs.set_int(
                "cursor-size",
                values["size"].as_i64().context("Tamanho inválido")? as i32,
            )?;
        }
        gio::Settings::sync();
    }
    if gtk::is_initialized_main_thread() {
        if let Some(display) = gdk::Display::default() {
            if let Some(screen) = gdk::Screen::default() {
                if let Some(root) = screen.root_window() {
                    root.set_cursor(gdk::Cursor::from_name(&display, "default").as_ref());
                }
            }
            display.flush();
        }
    }
    Ok(())
}
