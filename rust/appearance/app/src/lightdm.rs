// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    files::{atomic, backup_path, edit, ini, ini_value, read},
    images, text,
};
use anyhow::{bail, ensure, Context, Result};
use regex::Regex;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

fn seat_matches(pattern: &str, seat: &str) -> bool {
    use std::ffi::CString;
    let (Ok(p), Ok(s)) = (CString::new(pattern), CString::new(seat)) else {
        return false;
    };
    // SAFETY: both strings live across the call to POSIX fnmatch.
    unsafe { libc::fnmatch(p.as_ptr(), s.as_ptr(), 0) == 0 }
}
pub fn detect(root: &Path, seat: &str) -> Result<Value> {
    let service = root.join("etc/systemd/system/display-manager.service");
    let dm = read(&root.join("etc/X11/default-display-manager"))?;
    let mut manager = Path::new(dm.trim())
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    if let Ok(target) = fs::read_link(service) {
        if let Some(name) = target.file_name() {
            let n = name.to_string_lossy();
            manager = n.trim_end_matches(".service").into();
        }
    }
    if manager.is_empty() {
        manager = "desconhecido".into();
    }
    let mut result = json!({"manager":manager,"greeter":"desconhecido","supported":false});
    if manager != "lightdm" {
        return Ok(result);
    }
    let mut files = Vec::new();
    for dir in [
        "usr/share/lightdm/lightdm.conf.d",
        "usr/local/share/lightdm/lightdm.conf.d",
        "etc/lightdm/lightdm.conf.d",
    ] {
        if let Ok(entries) = fs::read_dir(root.join(dir)) {
            let mut paths: Vec<_> = entries
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|x| x == "conf"))
                .collect();
            paths.sort();
            files.extend(paths);
        }
    }
    files.push(root.join("etc/lightdm/lightdm.conf"));
    let mut greeter = "lightdm-greeter".to_owned();
    for file in files {
        let source = read(&file)?;
        for (name, _) in ini(&source).unwrap_or_default() {
            if name
                .strip_prefix("Seat:")
                .is_some_and(|pattern| seat_matches(pattern, seat))
            {
                greeter = ini_value(&source, &name, "greeter-session", &greeter);
            }
        }
    }
    if greeter.is_empty()
        || !greeter
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_.-".contains(&c))
    {
        return Ok(result);
    }
    let desktop = read(
        &root
            .join("usr/share/xgreeters")
            .join(format!("{greeter}.desktop")),
    )?;
    let command = ini_value(&desktop, "Desktop Entry", "Exec", "");
    let parsed = glib::shell_parse_argv(&command).unwrap_or_default();
    let executable = parsed
        .first()
        .and_then(|s| Path::new(s).file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("");
    result["greeter"] = json!(if executable.is_empty() {
        &greeter
    } else {
        executable
    });
    result["supported"] = json!(executable == "lightdm-gtk-greeter");
    Ok(result)
}
fn supported(root: &Path) -> Result<bool> {
    Ok(detect(
        root,
        &std::env::var("XDG_SEAT").unwrap_or_else(|_| "seat0".into()),
    )?["supported"]
        == true)
}
pub fn theme(root: &Path, theme: &str, icons: &str, cursor: &str, size: u32) -> Result<PathBuf> {
    ensure!(
        supported(root)?,
        "Gerenciador/greeter detectado não suporta temas GTK pelo método desta ferramenta."
    );
    ensure!(
        [theme, icons, cursor].into_iter().all(text::valid_name),
        "Nome de tema inválido."
    );
    ensure!((8..=128).contains(&size), "Tamanho de cursor inválido.");
    let paths =
        |relative: PathBuf| ["usr/share", "usr/local/share"].map(|d| root.join(d).join(&relative));
    ensure!(matches!(theme,"Adwaita"|"Adwaita-dark")||paths(Path::new("themes").join(theme).join("gtk-3.0/gtk.css")).iter().any(|p|p.is_file()),"O tema GTK {theme} precisa estar instalado em /usr/share/themes ou /usr/local/share/themes.");
    ensure!(
        paths(Path::new("icons").join(icons).join("index.theme"))
            .iter()
            .any(|p| p.is_file()),
        "Os ícones {icons} precisam estar instalados para todo o sistema."
    );
    ensure!(
        paths(Path::new("icons").join(cursor).join("cursors"))
            .iter()
            .any(|p| p.is_dir()),
        "O cursor {cursor} precisa estar instalado para todo o sistema."
    );
    let path = root.join("etc/lightdm/lightdm-gtk-greeter.conf");
    let result = edit(
        &read(&path)?,
        "greeter",
        &[
            ("theme-name", theme),
            ("icon-theme-name", icons),
            ("cursor-theme-name", cursor),
            ("cursor-theme-size", &size.to_string()),
        ],
    );
    let backup = backup_path(&root.join("var/backups/emwm-appearance"));
    atomic(&path, result.as_bytes(), Some(&backup), 0o644)?;
    Ok(backup)
}
pub fn wallpaper(root: &Path, data: &[u8], mode: &str) -> Result<PathBuf> {
    ensure!(
        supported(root)?,
        "O fundo de login requer LightDM GTK Greeter."
    );
    ensure!(images::MODES.contains(&mode), "Modo de fundo inválido.");
    let image = images::normalize_png(data)?;
    let dir = root.join("usr/share/backgrounds/emwm-desktop");
    for path in dir.ancestors() {
        if let Ok(meta) = fs::symlink_metadata(path) {
            ensure!(
                !meta.is_symlink(),
                "Destino de fundo não pode conter links simbólicos."
            );
            if root == Path::new("/") {
                ensure!(meta.uid()==0 && meta.mode()&0o022==0,"O diretório de fundos deve pertencer ao administrador e não permitir escrita por outros usuários.");
            }
        }
        if path == root {
            break;
        }
    }
    fs::create_dir_all(&dir)?;
    let name = format!("lightdm-{:x}.png", Sha256::digest(&image));
    let target = dir.join(&name);
    ensure!(!target.is_symlink(), "Destino de imagem inválido.");
    let prefix = if mode == "zoom" {
        "#zoomed:"
    } else {
        "#stretched:"
    };
    let value = format!("{prefix}/usr/share/backgrounds/emwm-desktop/{name}");
    let path = root.join("etc/lightdm/lightdm-gtk-greeter.conf");
    let old = read(&path)?;
    let changes = [("background", value.as_str()), ("user-background", "false")];
    let mut result = edit(&old, "greeter", &changes);
    for cap in Regex::new(r"(?m)^\s*\[(monitor:[^\]]+)\]")?.captures_iter(&old) {
        let section = cap.get(1).context("Seção de monitor inválida")?.as_str();
        if ini_value(&old, section, "background", "") != "#skip" {
            result = edit(&result, section, &changes);
        }
    }
    let existed = target.exists();
    crate::files::replace(&target, &image, 0o644)?;
    let backup = backup_path(&root.join("var/backups/emwm-appearance"));
    if let Err(e) = atomic(&path, result.as_bytes(), Some(&backup), 0o644) {
        if !existed {
            fs::remove_file(&target)?;
        }
        bail!(e);
    }
    Ok(backup)
}
