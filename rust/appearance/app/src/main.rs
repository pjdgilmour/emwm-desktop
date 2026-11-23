// SPDX-License-Identifier: GPL-3.0-only
use anyhow::{bail, Result};
use emwm_appearance::{lightdm, runtime, settings::Settings};
use serde_json::json;
use std::path::Path;
mod ui;
mod wallpaper_dialog;

const USAGE: &str =
    "usage: gui.py [-h] [--session] [--list] [--effects] [--desktop] [--decoration]";
const HELP: &str = "Aparência do EMWM e da tela de login

options:
  -h, --help    show this help message and exit
  --session     Ativar as preferências salvas, sem janela.
  --list        Listar temas e detectar o gerenciador de login.
  --effects     Abrir a aba de efeitos e renderização.
  --desktop     Abrir a aba da área de trabalho.
  --decoration  Abrir a aba de decoração e gradientes.";
fn run() -> Result<()> {
    let mut flags = Vec::new();
    let mut unknown = Vec::new();
    for arg in std::env::args().skip(1) {
        let (flag, explicit) = arg
            .split_once('=')
            .map_or((arg.as_str(), None), |(flag, value)| (flag, Some(value)));
        let options = [
            "--help",
            "--session",
            "--list",
            "--effects",
            "--desktop",
            "--decoration",
        ];
        let candidates: Vec<_> = if flag == "-h" {
            vec!["--help"]
        } else {
            options
                .into_iter()
                .filter(|o| flag.len() > 2 && flag.starts_with("--") && o.starts_with(flag))
                .collect()
        };
        if candidates.len() > 1 {
            eprintln!(
                "{USAGE}\ngui.py: error: ambiguous option: {flag} could match {}",
                candidates.join(", ")
            );
            std::process::exit(2);
        }
        if let Some(&option) = candidates.first() {
            if let Some(value) = explicit {
                eprintln!(
                    "{USAGE}\ngui.py: error: argument {}: ignored explicit argument '{}'",
                    if option == "--help" {
                        "-h/--help"
                    } else {
                        option
                    },
                    value
                );
                std::process::exit(2);
            }
            if option == "--help" {
                println!("{USAGE}\n\n{HELP}");
                return Ok(());
            }
            flags.push(option);
        } else {
            unknown.push(arg);
        }
    }
    if !unknown.is_empty() {
        eprintln!(
            "{USAGE}\ngui.py: error: unrecognized arguments: {}",
            unknown.join(" ")
        );
        std::process::exit(2);
    }
    let settings = Settings::from_env()?;
    if flags.contains(&"--list") {
        let mut info = json!({"login":lightdm::detect(Path::new("/"),&std::env::var("XDG_SEAT").unwrap_or_else(|_|"seat0".into()))?});
        for kind in ["gtk", "icons", "cursor"] {
            info[kind] = json!(settings
                .discover(kind)
                .into_iter()
                .map(|t| t.name)
                .collect::<Vec<_>>());
        }
        println!("{}", serde_json::to_string_pretty(&info)?);
        return Ok(());
    }
    if flags.contains(&"--session") {
        if settings.directory.join("selection.json").is_file() {
            let _ = gtk::init();
            for note in runtime::apply(&settings, &settings.current()?)? {
                eprintln!("{note}");
            }
        }
        return Ok(());
    }
    let original = settings.current()?;
    std::env::remove_var("GTK_THEME");
    if gtk::init().is_err() {
        bail!("Abra a ferramenta dentro de uma sessão gráfica.");
    }
    let page = if flags.contains(&"--decoration") {
        3
    } else if flags.contains(&"--desktop") {
        2
    } else if flags.contains(&"--effects") {
        1
    } else {
        0
    };
    let _window = ui::Window::new(settings, &original, page)?;
    gtk::main();
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
