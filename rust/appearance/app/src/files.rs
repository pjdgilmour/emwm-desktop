// SPDX-License-Identifier: GPL-3.0-only
use crate::text;
use anyhow::{Context, Result};
use chrono::Local;
use std::fs::{self, File, FileTimes, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn read(path: &Path) -> Result<String> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text.replace("\r\n", "\n").replace('\r', "\n")),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(e).with_context(|| path.display().to_string()),
    }
}

pub fn edit(text_in: &str, section: &str, pairs: &[(&str, &str)]) -> String {
    let fields: Vec<_> = pairs
        .iter()
        .map(|(k, v)| text::Setting {
            key: (*k).into(),
            lowercase: k.to_lowercase(),
            value: (*v).into(),
        })
        .collect();
    text::edit_ini(text_in.split_inclusive('\n'), section, &fields)
}

/// Read ConfigParser options, including DEFAULT, repeated sections and continuation lines.
type Section = (String, Vec<(String, String)>);

pub fn ini(text: &str) -> Option<Vec<Section>> {
    let mut sections: Vec<Section> = Vec::new();
    let mut current = None;
    let mut previous: Option<usize> = None;
    let mut indent = 0;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(['#', ';']) {
            continue;
        }
        let leading = raw.chars().take_while(|c| c.is_whitespace()).count();
        if let (Some(section), Some(index)) = (current, previous) {
            if leading > indent {
                let (_, options): &mut Section = &mut sections[section];
                options[index].1.push('\n');
                options[index].1.push_str(line);
                continue;
            }
        }
        indent = leading;
        if let Some(rest) = line.strip_prefix('[') {
            if let Some((name, _)) = rest.rsplit_once(']') {
                if name.is_empty() {
                    return None;
                }
                let index = sections
                    .iter()
                    .position(|(k, _)| k == name)
                    .unwrap_or_else(|| {
                        sections.push((name.to_owned(), Vec::new()));
                        sections.len() - 1
                    });
                current = Some(index);
                previous = None;
                continue;
            }
        }
        let (key, value) = line.split_once(['=', ':'])?;
        let key = key.trim().to_lowercase();
        if key.is_empty() {
            return None;
        }
        let options = &mut sections[current?].1;
        let index = options
            .iter()
            .position(|(k, _)| k == &key)
            .unwrap_or_else(|| {
                options.push((key, String::new()));
                options.len() - 1
            });
        options[index].1 = value.trim().to_owned();
        previous = Some(index);
    }
    Some(sections)
}

pub fn ini_value(text: &str, section: &str, key: &str, default: &str) -> String {
    let get = || {
        let entries = ini(text)?;
        let find = |group: &str| entries.iter().find(|(s, _)| s == group);
        let lookup = |list: &[(String, String)]| {
            list.iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(key))
                .map(|(_, v)| v.clone())
        };
        let (_, options) = find(section)?;
        lookup(options).or_else(|| find("DEFAULT").and_then(|(_, opt)| lookup(opt)))
    };
    get().unwrap_or_else(|| default.to_owned())
}

fn resolved(path: &Path) -> Result<PathBuf> {
    if let Ok(real) = fs::canonicalize(path) {
        return Ok(real);
    }
    let mut path = path.to_path_buf();
    for _ in 0..40 {
        match fs::read_link(&path) {
            Ok(target) => {
                path = if target.is_absolute() {
                    target
                } else {
                    path.parent().context("Destino inválido")?.join(target)
                }
            }
            Err(e)
                if e.kind() == io::ErrorKind::NotFound
                    || e.kind() == io::ErrorKind::InvalidInput =>
            {
                return Ok(path)
            }
            Err(e) => return Err(e.into()),
        }
    }
    anyhow::bail!("Link simbólico circular.")
}

pub fn atomic(path: &Path, data: &[u8], backup: Option<&Path>, mode: u32) -> Result<()> {
    let path = resolved(path)?;
    let parent = path.parent().context("Destino inválido")?;
    fs::create_dir_all(parent)?;
    let mut mode = mode;
    match fs::read(&path) {
        Ok(old) => {
            if old == data {
                return Ok(());
            }
            mode = fs::metadata(&path)?.permissions().mode() & 0o7777;
            if let Some(dir) = backup {
                fs::create_dir_all(dir)?;
                let count = fs::read_dir(dir)?.count();
                let name = format!(
                    "{}-{count}",
                    path.file_name()
                        .context("Destino inválido")?
                        .to_string_lossy()
                );
                let destination = dir.join(&name);
                fs::copy(&path, &destination)?;
                let metadata = fs::metadata(&path)?;
                let times = FileTimes::new()
                    .set_accessed(metadata.accessed()?)
                    .set_modified(metadata.modified()?);
                File::open(&destination)?.set_times(times)?;
                let mut manifest = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(dir.join("manifest.txt"))?;
                writeln!(manifest, "{name}\t{}", path.display())?;
            }
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.into()),
    }
    replace(path.as_path(), data, mode)
}

/// Replace an administrator-installed image even when an older copy has different permissions.
pub fn replace(path: &Path, data: &[u8], mode: u32) -> Result<()> {
    let parent = path.parent().context("Destino inválido")?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    temp.write_all(data)?;
    temp.as_file().sync_all()?;
    temp.as_file()
        .set_permissions(fs::Permissions::from_mode(mode))?;
    temp.persist(path)?;
    Ok(())
}

pub fn backup_path(base: &Path) -> PathBuf {
    base.join(Local::now().format("%Y%m%d-%H%M%S-%6f").to_string())
}

pub fn safe_private_path(path: &Path) -> bool {
    use std::os::unix::fs::MetadataExt;
    path.ancestors().all(|p| {
        fs::symlink_metadata(p)
            .is_ok_and(|m| !m.is_symlink() && m.uid() == 0 && m.mode() & 0o022 == 0)
    })
}
