// Transform Motif resource values using Xrm itself so wildcards, octal quoting
// and resource precedence match what the window manager sees.
use std::collections::HashSet;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::fs;
use std::io::{ErrorKind, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::PermissionsExt;
use std::path::{absolute, Path, PathBuf};
use std::ptr;
use x11::xlib;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub const WORKSPACES: usize = 4;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct MonitorChoice {
    pub name: String,
    pub source: PathBuf,
    pub mode: String,
    pub color: String,
    pub rotation: String,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Choice {
    pub source: PathBuf,
    pub mode: String,
    pub color: String,
    pub monitors: Vec<MonitorChoice>,
}

impl Default for Choice {
    fn default() -> Self {
        Self {
            source: PathBuf::new(),
            mode: "tile".into(),
            color: "#000000".into(),
            monitors: vec![],
        }
    }
}

pub type Entries = [Option<(PathBuf, Choice)>; WORKSPACES];

pub fn resource_file(home: &Path) -> Result<PathBuf> {
    Ok(absolute(home)?.join(".config/emwm/backdrops.resources"))
}

pub fn valid_mode(mode: &str) -> bool {
    matches!(mode, "fit" | "zoom" | "stretch" | "center" | "tile")
}
pub fn valid_color(color: &str) -> bool {
    color.len() == 7
        && color.starts_with('#')
        && color.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit)
}
pub fn valid_rotation(rotation: &str) -> bool {
    matches!(rotation, "auto" | "0" | "90" | "180" | "270")
}

fn encode(value: &OsStr) -> Result<Vec<u8>> {
    let bytes = value.as_bytes();
    if bytes.last().is_none_or(u8::is_ascii_whitespace)
        || bytes.iter().any(|c| matches!(c, b'\n' | b'\r' | 0))
    {
        return Err(format!(
            "O caminho da imagem não pode conter quebras de linha nem terminar em espaço: {}",
            value.to_string_lossy()
        )
        .into());
    }
    let mut encoded = vec![b'"'];
    for &byte in bytes {
        match byte {
            b'\\' => encoded.extend_from_slice(br"\134\134"),
            b'"' => encoded.extend_from_slice(br"\134\042"),
            other => encoded.push(other),
        }
    }
    encoded.push(b'"');
    Ok(encoded)
}

pub fn tellmwm_argument(path: &Path) -> Result<OsString> {
    fn escape(input: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        for &c in input {
            if matches!(c, b'"' | b'\\') {
                result.push(b'\\');
            }
            result.push(c);
        }
        result
    }
    let mut motif = vec![b'"'];
    motif.extend(escape(absolute(path)?.as_os_str().as_bytes()));
    motif.push(b'"');
    Ok(OsString::from_vec(escape(&motif)))
}

fn decode(raw: &[u8]) -> Result<OsString> {
    let mut chars = raw
        .iter()
        .copied()
        .skip_while(|&c| c == b' ' || c == b'\t')
        .peekable();
    let first = chars.peek().ok_or("Recurso de fundo vazio.")?;
    let quoted = *first == b'"';
    if quoted {
        chars.next();
    }
    let mut closed = !quoted;
    let mut value = Vec::new();
    while let Some(c) = chars.next() {
        if quoted && c == b'"' {
            closed = true;
            break;
        }
        if !quoted && (c.is_ascii_whitespace() || c == b'#') {
            break;
        }
        value.push(if c == b'\\' {
            chars
                .next()
                .ok_or("Escape incompleto no recurso de fundo.")?
        } else {
            c
        });
    }
    if !closed || value.is_empty() {
        return Err("Recurso de fundo inválido.".into());
    }
    Ok(OsString::from_vec(value))
}

struct Database(xlib::XrmDatabase);

impl Database {
    fn new(contents: &[u8]) -> Result<Self> {
        let text = CString::new(contents.split(|&c| c == 0).next().unwrap_or_default())?;
        // SAFETY: Xrm copies the NUL-terminated input and returns an owned database.
        let raw = unsafe {
            xlib::XrmInitialize();
            xlib::XrmGetStringDatabase(text.as_ptr())
        };
        if raw.is_null() {
            return Err("Não foi possível interpretar os recursos.".into());
        }
        Ok(Self(raw))
    }
    fn get(&self, key: &str) -> Option<&[u8]> {
        let key = CString::new(key).ok()?;
        let mut kind = ptr::null_mut();
        let mut value = xlib::XrmValue {
            size: 0,
            addr: ptr::null_mut(),
        };
        // SAFETY: the database and both C strings live for the call. Xrm owns the
        // returned string until the database is destroyed. The borrow enforces that.
        unsafe {
            if xlib::XrmGetResource(self.0, key.as_ptr(), key.as_ptr(), &mut kind, &mut value) == 0
                || value.addr.is_null()
            {
                return None;
            }
            Some(CStr::from_ptr(value.addr).to_bytes())
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        // SAFETY: this wrapper exclusively owns the non-null database.
        unsafe {
            xlib::XrmDestroyDatabase(self.0);
        }
    }
}

fn read_legacy(path: &Path) -> Result<Entries> {
    let mut entries = Entries::default();
    let data = match fs::read(path) {
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(entries),
        result => result?,
    };
    // Read the legacy shell syntax as data, including shell-quoted single quotes.
    for line in data.split(|&c| c == b'\n') {
        let mut rest = line.trim_ascii();
        let mut token = || {
            let end = rest
                .iter()
                .position(u8::is_ascii_whitespace)
                .unwrap_or(rest.len());
            let word = &rest[..end];
            rest = rest[end..].trim_ascii_start();
            word
        };
        if !matches!(token(), b"tellmwm" | b"/usr/bin/tellmwm") || token() != b"backdrop" {
            continue;
        }
        let [b'w', b's', index @ b'0'..=b'3'] = token() else {
            continue;
        };
        if rest.is_empty() {
            continue;
        }
        let mut value = Vec::new();
        if let Some(mut s) = rest.strip_prefix(b"'").and_then(|s| s.strip_suffix(b"'")) {
            while let Some(index) = s.windows(4).position(|v| v == br"'\''") {
                value.extend(&s[..index]);
                value.push(b'\'');
                s = &s[index + 4..];
            }
            value.extend(s);
        } else {
            value.extend(rest);
        }
        let path = PathBuf::from(OsString::from_vec(value));
        entries[(index - b'0') as usize] = Some((
            path.clone(),
            Choice {
                source: path,
                ..Choice::default()
            },
        ));
    }
    Ok(entries)
}

pub fn read(home: &Path) -> Result<Entries> {
    let file = resource_file(home)?;
    let data = match fs::read(&file) {
        Err(e) if e.kind() == ErrorKind::NotFound => {
            return read_legacy(&home.join(".config/emwm/apply-backdrops.sh"))
        }
        Err(_) => return Err(format!("Não foi possível ler {}", file.display()).into()),
        Ok(bytes) => bytes,
    };
    let db = Database::new(&data)?;
    let mut entries = Entries::default();
    for (i, slot) in entries.iter_mut().enumerate() {
        let Some(value) = db.get(&format!("Emwm.ws{i}.backdrop.image")) else {
            continue;
        };
        let output = PathBuf::from(decode(value)?);
        let mut selected = Choice {
            source: output.clone(),
            ..Choice::default()
        };
        let get = |field: &str| {
            db.get(&format!("EmwmWallpaperTool.ws{i}.{field}"))
                .unwrap_or_default()
        };
        let text = |field: &str| std::str::from_utf8(get(field)).unwrap_or_default();
        let count = get("monitorCount");
        if !count.is_empty() {
            if count.len() > 2 || !count.iter().all(u8::is_ascii_digit) {
                return Err("Lista de monitores inválida.".into());
            }
            let count = text("monitorCount").parse::<usize>()?;
            if count > 64 {
                return Err("Monitores demais na configuração.".into());
            }
            if count > 0 {
                selected.source.clear();
            }
            for m in 0..count {
                let field = |key: &str| format!("monitor{m}.{key}");
                selected.monitors.push(MonitorChoice {
                    name: decode(get(&field("name")))?
                        .into_string()
                        .map_err(|_| "Configuração de monitor inválida.")?,
                    source: decode(get(&field("source")))?.into(),
                    mode: text(&field("mode")).into(),
                    color: text(&field("color")).into(),
                    rotation: text(&field("rotation")).into(),
                });
            }
            validate_monitors(&selected.monitors)?;
        }
        if !get("source").is_empty() {
            selected.source = decode(get("source"))?.into();
        }
        if valid_mode(text("mode")) {
            selected.mode = text("mode").into();
        }
        if valid_color(text("color")) {
            selected.color = text("color").into();
        }
        *slot = Some((output, selected));
    }
    Ok(entries)
}

fn validate_monitors(monitors: &[MonitorChoice]) -> Result<()> {
    if monitors.len() > 64 {
        return Err("Monitores demais.".into());
    }
    let mut names = HashSet::new();
    for m in monitors {
        if m.name.is_empty()
            || m.source.as_os_str().is_empty()
            || !valid_mode(&m.mode)
            || !valid_color(&m.color)
            || !valid_rotation(&m.rotation)
            || !names.insert(&m.name)
        {
            return Err("Configuração de monitor inválida.".into());
        }
    }
    Ok(())
}

fn write_atomic(path: &Path, data: &[u8]) -> Result<()> {
    let destination = if path.is_symlink() {
        fs::canonicalize(path)?
    } else {
        path.to_owned()
    };
    let parent = destination.parent().ok_or("Diretório inválido.")?;
    fs::create_dir_all(parent)?;
    let mut mode = 0o600;
    match fs::read(&destination) {
        Ok(contents) => {
            if contents == data {
                return Ok(());
            }
            mode = destination.metadata()?.permissions().mode() & 0o777;
            let mut backup = destination.as_os_str().to_owned();
            backup.push(".before-wallpaper-resources");
            // Publish a complete backup exactly once, including concurrent saves.
            let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
            tmp.write_all(&contents)?;
            tmp.as_file()
                .set_permissions(fs::Permissions::from_mode(mode))?;
            if let Err(e) = tmp.persist_noclobber(backup) {
                if e.error.kind() != ErrorKind::AlreadyExists {
                    return Err(e.error.into());
                }
            }
        }
        Err(e) if e.kind() == ErrorKind::NotFound => (),
        Err(e) => return Err(e.into()),
    }
    let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
    tmp.write_all(data)?;
    tmp.as_file()
        .set_permissions(fs::Permissions::from_mode(mode))?;
    tmp.as_file().sync_all()?;
    tmp.persist(destination)?;
    Ok(())
}

pub fn save(home: &Path, updates: &[(usize, PathBuf, Choice)]) -> Result<PathBuf> {
    let mut entries = read(home)?;
    for (i, path, choice) in updates {
        if choice.source.as_os_str().is_empty() && choice.monitors.is_empty()
            || !valid_mode(&choice.mode)
            || !valid_color(&choice.color)
        {
            return Err("Imagem, posicionamento ou cor de fundo inválidos.".into());
        }
        validate_monitors(&choice.monitors)?;
        *entries.get_mut(*i).ok_or("Workspace inválido.")? =
            Some((absolute(path)?, choice.clone()));
    }
    if entries.iter().all(Option::is_none) {
        return Err("Nenhum fundo salvo para migrar.".into());
    }
    let mut resources = b"! Gerenciado pelo emwm-wallpaper-tool.\n".to_vec();
    for (i, entry) in entries.into_iter().enumerate() {
        let Some((image, selected)) = entry else {
            continue;
        };
        let mut line = |key: &str, value: &[u8]| {
            resources.extend(key.as_bytes());
            resources.extend(b": ");
            resources.extend(value);
            resources.push(b'\n');
        };
        line(
            &format!("Emwm*ws{i}*backdrop*image"),
            &encode(absolute(image)?.as_os_str())?,
        );
        let prefix = format!("EmwmWallpaperTool.ws{i}.");
        if !selected.source.as_os_str().is_empty() {
            line(
                &(prefix.clone() + "source"),
                &encode(absolute(selected.source)?.as_os_str())?,
            );
        }
        line(&(prefix.clone() + "mode"), selected.mode.as_bytes());
        line(&(prefix.clone() + "color"), selected.color.as_bytes());
        line(
            &(prefix.clone() + "monitorCount"),
            selected.monitors.len().to_string().as_bytes(),
        );
        for (id, m) in selected.monitors.iter().enumerate() {
            let key = format!("{prefix}monitor{id}.");
            line(&(key.clone() + "name"), &encode(m.name.as_ref())?);
            line(
                &(key.clone() + "source"),
                &encode(absolute(&m.source)?.as_os_str())?,
            );
            line(&(key.clone() + "mode"), m.mode.as_bytes());
            line(&(key.clone() + "color"), m.color.as_bytes());
            line(&(key + "rotation"), m.rotation.as_bytes());
        }
    }
    let file = resource_file(home)?;
    let name = file.as_os_str().as_bytes();
    if name
        .iter()
        .any(|c| matches!(c, b'"' | b'\\' | b'\r' | b'\n'))
    {
        return Err(
            "O caminho HOME não pode conter aspas, barras invertidas ou quebras de linha.".into(),
        );
    }
    let xresources = home.join(".Xresources");
    let mut config = match fs::read(&xresources) {
        Err(e) if e.kind() == ErrorKind::NotFound => vec![],
        result => result?,
    };
    let include = [b"#include \"".as_slice(), name, b"\""].concat();
    if !config.split(|&c| c == b'\n').any(|line| line == include) {
        if !config.is_empty() && !config.ends_with(b"\n") {
            config.push(b'\n');
        }
        config.extend(b"\n! Fundos dos workspaces do EMWM (emwm-wallpaper-tool).\n");
        config.extend(include);
        config.push(b'\n');
    }
    write_atomic(&file, &resources)?;
    write_atomic(&xresources, &config)?;
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrates_legacy_quoting_and_preserves_other_workspaces() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let home = temp.path();
        let old = home.join(".config/emwm/apply-backdrops.sh");
        fs::create_dir_all(old.parent().unwrap())?;
        let image = home.join("imagem 'aspas' e \"#1\" \\ espaço.xpm");
        let quoted = image.to_string_lossy().replace('\'', "'\\''");
        fs::write(
            &old,
            format!("#!/bin/sh\n/usr/bin/tellmwm backdrop ws0 '{quoted}'\n"),
        )?;
        fs::write(home.join("dotfile"), b"XTerm*faceSize: 13")?;
        std::os::unix::fs::symlink("dotfile", home.join(".Xresources"))?;
        let file = save(home, &[])?;
        assert_eq!(read(home)?[0].as_ref().unwrap().0, image);
        assert!(home.join(".Xresources").is_symlink());
        assert_eq!(
            fs::read(home.join("dotfile.before-wallpaper-resources"))?,
            b"XTerm*faceSize: 13"
        );
        let new = home.join("outra.png");
        save(
            home,
            &[(
                2,
                new.clone(),
                Choice {
                    source: new.clone(),
                    ..Choice::default()
                },
            )],
        )?;
        fs::write(old, "tellmwm backdrop ws0 wrong\n")?;
        let entries = read(home)?;
        assert_eq!(entries[0].as_ref().unwrap().0, image);
        assert_eq!(entries[2].as_ref().unwrap().0, new);
        let saved = fs::read(&file)?;
        let invalid = home.join("image.png ");
        assert!(save(
            home,
            &[(
                1,
                invalid.clone(),
                Choice {
                    source: invalid,
                    ..Choice::default()
                }
            )]
        )
        .is_err());
        assert_eq!(saved, fs::read(file)?);
        Ok(())
    }
}
