// SPDX-License-Identifier: GPL-3.0-only
mod effects;
mod gl;
mod native;
#[cfg(test)]
mod pixel_tests;
mod renderer;
mod scene;
mod screen;
mod settings;
mod surface;
use std::{env, os::unix::ffi::OsStrExt, path::PathBuf, process::ExitCode};

fn run() -> Result<(), u8> {
    let mut config = env::var_os("XDG_CONFIG_HOME")
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env::var_os("HOME").unwrap_or_else(|| ".".into())).join(".config")
        })
        .join("emwm-desktop/compositor.conf");
    let mut backend = String::new();
    let mut args = env::args_os().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_bytes() {
            b"--help" => {
                println!("emwm-compositor [--backend auto|glx|xrender] [--config FILE]\nReload: SIGHUP/SIGUSR1, or edit compositor.conf.");
                return Ok(());
            }
            b"--config" | b"--backend" => {
                if let Some(value) = args.next() {
                    if arg == "--config" {
                        config = value.into();
                    } else {
                        backend = value.to_string_lossy().into_owned();
                    }
                    continue;
                }
            }
            _ => {}
        }
        use std::io::Write;
        let stderr = std::io::stderr();
        let mut stderr = stderr.lock();
        let _ = stderr.write_all(b"Invalid argument: ");
        let _ = stderr.write_all(arg.as_bytes());
        let _ = stderr.write_all(b"\n");
        return Err(2);
    }
    if !backend.is_empty() && !settings::backend(&backend) {
        return Err(2);
    }
    scene::Scene::start(config, backend).map_err(|e| {
        eprintln!("emwm-compositor: {e}");
        1
    })
}
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => code.into(),
    }
}
