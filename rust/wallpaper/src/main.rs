mod backdrops;
mod layout;
mod render;
mod ui;

use backdrops::{Choice, Entries, Result};
use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn home() -> Result<PathBuf> {
    env::var_os("HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .ok_or_else(|| "Variável de ambiente não encontrada: HOME".into())
}
fn command_exists(name: &str) -> bool {
    env::var_os("PATH").is_some_and(|paths| {
        env::split_paths(&paths)
            .filter(|p| !p.as_os_str().is_empty())
            .any(|p| {
                p.join(name)
                    .metadata()
                    .is_ok_and(|m| m.permissions().mode() & 0o111 != 0)
            })
    })
}
fn merge(resources: &Path) -> Result<()> {
    if !command_exists("xrdb") {
        return Err("xrdb não encontrado. Os fundos foram salvos, mas os recursos da sessão não foram atualizados.".into());
    }
    let out = Command::new("xrdb")
        .args(["-merge", "-nocpp"])
        .arg(resources)
        .output()?;
    if out.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Os fundos foram salvos, mas não foi possível atualizar os recursos da sessão:\n{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        )
        .into())
    }
}
fn log(home: &Path, message: &str) {
    if let (Ok(mut f), Ok(now)) = (
        fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(home.join(".emwm-wallpaper-tool.log")),
        glib::DateTime::now_local(),
    ) {
        if let Ok(time) = now.format("%Y-%m-%d %H:%M:%S") {
            let _ = writeln!(f, "{time} {message}");
        }
    }
}
fn notify(home: &Path, updates: &[(usize, PathBuf, Choice)]) -> Result<String> {
    if !command_exists("tellmwm") {
        return Ok(
            "tellmwm não encontrado; o fundo será aplicado no próximo login do EMWM.".into(),
        );
    }
    let mut failures = Vec::new();
    for (i, path, _) in updates {
        let args: [OsString; 3] = [
            "backdrop".into(),
            format!("ws{i}").into(),
            backdrops::tellmwm_argument(path)?,
        ];
        let quoted = std::iter::once(OsStr::new("tellmwm"))
            .chain(args.iter().map(OsString::as_os_str))
            .map(|arg| format!("'{}'", arg.to_string_lossy().replace('\'', "'\\''")))
            .collect::<Vec<_>>()
            .join(" ");
        log(home, &format!("Executando: {quoted}"));
        let out = Command::new("tellmwm").args(args).output()?;
        let details = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        log(
            home,
            &format!(
                "Resultado tellmwm ws{i}: exit={} output=[{}]",
                out.status.code().unwrap_or(1),
                details
            ),
        );
        if !out.status.success() {
            failures.push(format!(
                "Falha ao aplicar ws{i} com tellmwm.{}",
                if details.is_empty() {
                    String::new()
                } else {
                    format!("\n{details}")
                }
            ));
        }
    }
    if failures.is_empty() {
        Ok("O fundo foi aplicado imediatamente com tellmwm.".into())
    } else {
        Ok(format!(
            "Não foi possível enviar todos os fundos ao tellmwm.\n{}",
            failures.join("\n")
        ))
    }
}

fn prepare(
    home: &Path,
    choices: Vec<(usize, Choice)>,
    layout: &layout::Layout,
    old: Option<&Entries>,
) -> Result<Vec<(usize, PathBuf, Choice)>> {
    let mut cache = HashMap::new();
    let mut updates = Vec::new();
    for (i, choice) in choices {
        let path = match cache.entry(choice.clone()) {
            std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(render::prepare(home, &choice, layout)?)
            }
        };
        if old
            .and_then(|e| e[i].as_ref())
            .is_some_and(|(p, _)| p == path)
        {
            continue;
        }
        updates.push((i, path.clone(), choice));
    }
    Ok(updates)
}
fn refresh() -> Result<()> {
    let home = home()?;
    let saved = backdrops::read(&home)?;
    let choices = saved
        .iter()
        .enumerate()
        .filter_map(|(i, entry)| entry.as_ref().map(|(_, c)| (i, c.clone())))
        .collect();
    let updates = prepare(&home, choices, &layout::detect()?, Some(&saved))?;
    if !updates.is_empty() {
        merge(&backdrops::save(&home, &updates)?)?;
        notify(&home, &updates)?;
    }
    Ok(())
}
fn migrate() -> Result<()> {
    let file = backdrops::save(&home()?, &[])?;
    println!("Fundos salvos em \"{}\"", file.display());
    merge(&file)
}
fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let (prefix, result) = match args.as_slice() {
        [flag] if flag == "--refresh-layout" => ("Atualização dos fundos: ", refresh()),
        [flag] if flag == "--migrate" => ("", migrate()),
        _ => return ui::run(),
    };
    if let Err(e) = result {
        eprintln!("{prefix}{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
