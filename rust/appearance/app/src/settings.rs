// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    files::{atomic, backup_path, edit, ini_value, read},
    text,
};
use anyhow::{anyhow, ensure, Context, Result};
use regex::Regex;
use serde_json::{json, Map, Value};
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};

pub use crate::gradient::COLORS;
pub const THEME_KINDS: [(&str, &str); 3] =
    [("theme", "gtk"), ("icons", "icons"), ("cursor", "cursor")];
pub const RANGES: [(&str, i64, i64); 5] = [
    ("duration_ms", 50, 2000),
    ("fade_ms", 0, 1000),
    ("shadow_radius", 1, 64),
    ("shadow_offset", -32, 64),
    ("max_fps", 15, 240),
];
const XRES_INCLUDE: &str = r#"^\s*#include\s+"([^"\n]+)"\s*$"#;
fn read_gradient(input: &Value) -> Result<crate::gradient::Gradient> {
    ensure!(input.is_object(), "Configuração de gradiente inválida.");
    crate::gradient::Gradient::read(
        |key| Ok(input.get(key).map(|v| v.as_str().unwrap_or("").to_owned())),
        |message| anyhow!(message.to_owned()),
    )
}
pub fn gradient(input: &Value) -> Result<Value> {
    let gradient = read_gradient(input)?;
    let mut result = json!({"mode":gradient.mode});
    for ((key, _, _), color) in COLORS.iter().zip(gradient.colors) {
        result[*key] = json!(color);
    }
    Ok(result)
}
fn strings<'a>(values: &'a Value, key: &str) -> Result<&'a str> {
    values[key].as_str().context("Seleção inválida.")
}
pub fn selection(input: &Value) -> Result<Value> {
    let mut v = json!({});
    for (key, _) in THEME_KINDS {
        let s = input[key]
            .as_str()
            .filter(|s| text::valid_name(s))
            .context("Nome de tema inválido.")?;
        v[key] = json!(s);
    }
    let size = input["size"]
        .as_i64()
        .or_else(|| input["size"].as_str().and_then(|s| s.trim().parse().ok()))
        .or_else(|| input["size"].as_f64().map(|n| n as i64))
        .context("Tamanho de cursor deve estar entre 8 e 128.")?;
    ensure!(
        (8..=128).contains(&size),
        "Tamanho de cursor deve estar entre 8 e 128."
    );
    v["size"] = json!(size);
    let pal = input
        .get("palette")
        .map(|v| v.as_str().unwrap_or(""))
        .unwrap_or("keep");
    ensure!(text::palette(pal).is_some(), "Paleta inválida.");
    v["palette"] = json!(pal);
    if let Some(g) = input.get("gradient") {
        v["gradient"] = gradient(g)?;
    }
    if input.get("colors").is_some() || pal == "custom" {
        let colors = input["colors"]
            .as_object()
            .filter(|c| c.len() == 5 && text::FIELDS.iter().all(|k| c.contains_key(*k)))
            .context("Selecione todas as cores do tema personalizado.")?;
        let mut out = Map::new();
        for (k, v) in colors {
            let s = v
                .as_str()
                .filter(|s| crate::gradient::is_color(s))
                .context("Cor inválida: use o formato #RRGGBB.")?;
            out.insert(k.clone(), json!(s.to_ascii_lowercase()));
        }
        v["colors"] = Value::Object(out);
    }
    Ok(v)
}

pub fn default_effects() -> Value {
    json!({"backend":"auto","effect":"zoom","duration_ms":220,"fade_ms":120,"shadows":true,"shadow_radius":16,"shadow_offset":6,"shadow_opacity":0.45,"vsync":true,"max_fps":60})
}
fn valid_effect(key: &str, v: &Value) -> bool {
    match key {
        "backend" => v
            .as_str()
            .is_some_and(|s| matches!(s, "auto" | "glx" | "xrender")),
        "effect" => v
            .as_str()
            .is_some_and(|s| matches!(s, "none" | "zoom" | "twist" | "flip")),
        "shadows" | "vsync" => v.is_boolean(),
        "shadow_opacity" => v.as_f64().is_some_and(|v| (0.0..=1.0).contains(&v)),
        _ => RANGES
            .iter()
            .any(|&(k, lo, hi)| k == key && v.as_i64().is_some_and(|n| (lo..=hi).contains(&n))),
    }
}
fn valid_monitor(s: &str) -> bool {
    !s.is_empty()
        && s.chars().count() <= 256
        && !s.chars().any(|c| c < ' ' || matches!(c, '\\' | '[' | ']'))
}

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub capabilities: Vec<String>,
    pub system: bool,
    pub system_caps: Vec<String>,
}
#[derive(Clone)]
pub struct Settings {
    pub home: PathBuf,
    pub config: PathBuf,
    pub data: PathBuf,
    pub state: PathBuf,
    pub directory: PathBuf,
}
fn xdg(key: &str, default: PathBuf) -> PathBuf {
    env::var_os(key)
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .unwrap_or(default)
}
impl Settings {
    pub fn from_env() -> Result<Self> {
        let home = PathBuf::from(env::var_os("HOME").context("HOME não definido.")?);
        let config = xdg("XDG_CONFIG_HOME", home.join(".config"));
        let data = xdg("XDG_DATA_HOME", home.join(".local/share"));
        let state = xdg("XDG_STATE_HOME", home.join(".local/state"));
        let directory = config.join("emwm-desktop/appearance");
        Ok(Self {
            home,
            config,
            data,
            state,
            directory,
        })
    }
    pub fn discover(&self, kind: &str) -> Vec<Theme> {
        let leaf = if kind == "gtk" { "themes" } else { "icons" };
        let system = ["/usr/local/share", "/usr/share"].map(|d| Path::new(d).join(leaf));
        let mut roots = vec![self.data.join(leaf), self.home.join(format!(".{leaf}"))];
        let dirs =
            env::var_os("XDG_DATA_DIRS").unwrap_or_else(|| "/usr/local/share:/usr/share".into());
        roots.extend(
            env::split_paths(&dirs)
                .filter(|p| p.is_absolute())
                .map(|p| p.join(leaf)),
        );
        if kind == "cursor" {
            roots.push("/usr/share/pixmaps".into());
        }
        roots.extend(system.clone());
        let mut found = BTreeMap::new();
        let mut visited = Vec::new();
        for root in roots {
            if visited.contains(&root) {
                continue;
            }
            visited.push(root.clone());
            let Ok(entries) = fs::read_dir(&root) else {
                continue;
            };
            let mut paths: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
            paths.sort();
            for path in paths {
                if !path.is_dir() {
                    continue;
                }
                let Some(name) = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .filter(|s| text::valid_name(s))
                else {
                    continue;
                };
                let Ok(info) = read(&path.join("index.theme")) else {
                    continue;
                };
                let caps: Vec<String> = match kind {
                    "gtk" => [
                        ("GTK 2", "gtk-2.0/gtkrc"),
                        ("GTK 3", "gtk-3.0/gtk.css"),
                        ("GTK 4", "gtk-4.0/gtk.css"),
                    ]
                    .into_iter()
                    .filter(|(_, f)| path.join(f).is_file())
                    .map(|(s, _)| s.into())
                    .collect(),
                    "icons"
                        if ["Directories", "ScaledDirectories"]
                            .iter()
                            .any(|k| !ini_value(&info, "Icon Theme", k, "").is_empty()) =>
                    {
                        vec!["Ícones".into()]
                    }
                    "cursor" if path.join("cursors").is_dir() => vec!["Cursor".into()],
                    _ => continue,
                };
                if caps.is_empty() {
                    continue;
                }
                let sys = system.contains(&root);
                let t = found.entry(name.to_owned()).or_insert_with(|| Theme {
                    name: name.into(),
                    capabilities: caps.clone(),
                    system: sys,
                    system_caps: vec![],
                });
                if sys {
                    for cap in caps {
                        if !t.system_caps.contains(&cap) {
                            t.system_caps.push(cap);
                        }
                    }
                }
            }
        }
        if kind == "gtk" {
            for name in ["Adwaita", "Adwaita-dark"] {
                let t = found.entry(name.into()).or_insert_with(|| Theme {
                    name: name.into(),
                    capabilities: vec!["GTK 3".into()],
                    system: true,
                    system_caps: vec![],
                });
                if !t.system_caps.iter().any(|s| s == "GTK 3") {
                    t.system_caps.push("GTK 3".into());
                }
            }
        }
        let mut out: Vec<_> = found.into_values().collect();
        out.sort_by_cached_key(|t| glib::casefold(&t.name).to_string());
        out
    }
    pub fn check_themes(&self, values: &Value, login: bool) -> Result<()> {
        for (key, kind) in THEME_KINDS {
            let name = strings(values, key)?;
            let theme = self.discover(kind).into_iter().find(|t| t.name == name);
            if login {
                let t=theme.filter(|t|!t.system_caps.is_empty()).ok_or_else(||anyhow!("{name} não está disponível para o login. Instale o tema também em /usr/share/themes ou /usr/share/icons; ou desmarque a opção de login."))?;
                ensure!(
                    key != "theme" || t.system_caps.iter().any(|s| s == "GTK 3"),
                    "O LightDM GTK Greeter precisa de um tema GTK 3 instalado para todo o sistema."
                );
            } else {
                ensure!(theme.is_some(), "Tema não encontrado: {name}");
            }
        }
        Ok(())
    }
    pub fn motif_colors(&self) -> Result<Value> {
        static INCLUDE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(XRES_INCLUDE).expect("constant regular expression"));
        let mut colors = json!({});
        for (k, v) in text::FIELDS
            .into_iter()
            .zip(text::palette("keep").unwrap_or_default())
        {
            colors[k] = json!(v);
        }
        fn visit(
            path: &Path,
            stack: &mut Vec<PathBuf>,
            colors: &mut Value,
            home: &Path,
        ) -> Result<()> {
            let p = fs::canonicalize(path).unwrap_or_else(|_| path.to_owned());
            if stack.len() >= 16 || stack.contains(&p) {
                return Ok(());
            }
            stack.push(p.clone());
            for line in read(&p)?.lines() {
                if let Some(c) = INCLUDE.captures(line) {
                    let child = Path::new(&c[1]);
                    let child = child
                        .strip_prefix("~")
                        .map(|c| home.join(c))
                        .unwrap_or_else(|_| p.parent().unwrap_or(home).join(child));
                    visit(&child, stack, colors, home)?;
                } else if let Some((k, v)) = line.split_once(':') {
                    if text::FIELDS.contains(&k.trim()) && !v.trim().is_empty() {
                        colors[k.trim()] = json!(v.trim());
                    }
                }
            }
            stack.pop();
            Ok(())
        }
        for f in [".Xdefaults", ".Xresources"] {
            visit(&self.home.join(f), &mut vec![], &mut colors, &self.home)?;
        }
        Ok(colors)
    }
    pub fn current(&self) -> Result<Value> {
        let saved = read(&self.directory.join("selection.json"))?;
        if let Ok(v) = serde_json::from_str(&saved) {
            if let Ok(v) = selection(&v) {
                return Ok(v);
            }
        }
        let gtk = read(&self.config.join("gtk-3.0/settings.ini"))?;
        let xr = read(&self.home.join(".Xresources"))?;
        let resource = |key: &str, default: &str| {
            xr.lines()
                .find_map(|l| {
                    l.split_once(':')
                        .filter(|(k, v)| k.trim_end() == key && !v.trim().is_empty())
                        .map(|(_, v)| v.trim().to_owned())
                })
                .unwrap_or_else(|| default.to_owned())
        };
        let theme = env::var("GTK_THEME").unwrap_or_else(|_| "Adwaita".into());
        Ok(
            json!({"theme":ini_value(&gtk,"Settings","gtk-theme-name",theme.split(':').next().unwrap_or("Adwaita")),
            "icons":ini_value(&gtk,"Settings","gtk-icon-theme-name","Adwaita"),
            "cursor":ini_value(&gtk,"Settings","gtk-cursor-theme-name",&resource("Xcursor.theme","Adwaita")),
            "size":ini_value(&gtk,"Settings","gtk-cursor-theme-size",&resource("Xcursor.size","24")).trim().parse::<i64>()?, "palette":"keep"}),
        )
    }
    pub fn desktop(&self) -> Result<Value> {
        let file = read(&self.config.join("emwm-desktop/desktop.conf"))?;
        let monitor = ini_value(&file, "desktop", "monitor", "primary");
        let anchor = ini_value(&file, "desktop", "anchor", "left");
        Ok(
            json!({"monitor":if valid_monitor(&monitor){&monitor}else{"primary"},"anchor":if matches!(anchor.as_str(),"left"|"right"){&anchor}else{"left"}}),
        )
    }
    pub fn effects(&self) -> Result<Value> {
        let file = read(&self.config.join("emwm-desktop/compositor.conf"))?;
        let mut values = default_effects();
        for (k, default) in values.as_object_mut().context("Padrões inválidos")? {
            let s = ini_value(&file, "compositor", k, "");
            let v = match default {
                Value::String(_) => json!(s),
                Value::Bool(_) => match s.to_lowercase().as_str() {
                    "1" | "on" | "yes" | "true" => json!(true),
                    "0" | "off" | "no" | "false" => json!(false),
                    _ => continue,
                },
                _ if k == "shadow_opacity" => match s.parse::<f64>() {
                    Ok(n) => json!(n),
                    Err(_) => continue,
                },
                _ => match s.parse::<i64>() {
                    Ok(n) => json!(n),
                    Err(_) => continue,
                },
            };
            if valid_effect(k, &v) {
                *default = v;
            }
        }
        Ok(values)
    }

    pub fn apply(&self, values: &Value, effects: &Value, desktop: &Value) -> Result<PathBuf> {
        let v = selection(values)?;
        self.check_themes(&v, false)?;
        let mut changes = Vec::new();
        let monitor = strings(desktop, "monitor")?;
        let anchor = strings(desktop, "anchor")?;
        ensure!(valid_monitor(monitor), "Monitor inválido.");
        ensure!(
            matches!(anchor, "left" | "right"),
            "Escolha o alinhamento à esquerda ou à direita."
        );
        let path = self.config.join("emwm-desktop/desktop.conf");
        changes.push((
            path.clone(),
            edit(
                &read(&path)?,
                "desktop",
                &[("monitor", monitor), ("anchor", anchor)],
            ),
        ));
        let mut raw = Vec::new();
        for key in default_effects()
            .as_object()
            .context("Padrões inválidos")?
            .keys()
        {
            let value = &effects[key];
            ensure!(
                valid_effect(key, value),
                "Configuração de composição inválida: {key}."
            );
            raw.push((
                key.clone(),
                value
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| value.to_string()),
            ));
        }
        let opts: Vec<_> = raw.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        let path = self.config.join("emwm-desktop/compositor.conf");
        changes.push((path.clone(), edit(&read(&path)?, "compositor", &opts)));
        let size = v["size"].to_string();
        let theme = strings(&v, "theme")?;
        let icons = strings(&v, "icons")?;
        let cursor = strings(&v, "cursor")?;
        let settings = [
            ("gtk-theme-name", theme),
            ("gtk-icon-theme-name", icons),
            ("gtk-cursor-theme-name", cursor),
            ("gtk-cursor-theme-size", &size),
        ];
        for ver in ["3.0", "4.0"] {
            let p = self.config.join(format!("gtk-{ver}/settings.ini"));
            changes.push((p.clone(), edit(&read(&p)?, "Settings", &settings)));
        }
        let block = |path: &Path, content: &str, comment: &str| -> Result<(PathBuf, String)> {
            Ok((
                path.to_owned(),
                text::managed_block(&read(path)?, "EMWM APPEARANCE", content, comment),
            ))
        };
        let mut content = Vec::new();
        for (k, s) in settings {
            content.push(format!(
                "{k} = {}",
                if k.ends_with("size") {
                    s.to_owned()
                } else {
                    json!(s).to_string()
                }
            ));
        }
        changes.push(block(
            &self.home.join(".gtkrc-2.0"),
            &content.join("\n"),
            "#",
        )?);
        let res = self.directory.join("appearance.resources");
        let mut resources = format!("Xcursor.theme: {cursor}\nXcursor.size: {size}\n");
        match strings(&v, "palette")? {
            "keep" => {
                for l in read(&res)?.lines().filter(|l| {
                    !l.is_empty()
                        && !l.starts_with("Xcursor.theme:")
                        && !l.starts_with("Xcursor.size:")
                }) {
                    resources.push_str(l);
                    resources.push('\n');
                }
            }
            "custom" => {
                for k in text::FIELDS {
                    resources.push_str(&format!("{k}: {}\n", strings(&v["colors"], k)?));
                }
            }
            palette => {
                for (k, s) in text::FIELDS
                    .into_iter()
                    .zip(text::palette(palette).context("Paleta inválida.")?)
                {
                    resources.push_str(&format!("{k}: {s}\n"));
                }
            }
        }
        if let Some(g) = v.get("gradient") {
            resources = resources
                .lines()
                .filter(|l| !l.starts_with("Emwm*titleGradient"))
                .map(|l| format!("{l}\n"))
                .collect();
            resources.push_str(&read_gradient(g)?.resources());
        }
        let incl = res.to_string_lossy();
        ensure!(
            !incl.contains(['\n', '\r', '"', '\\']),
            "Caminho de configuração incompatível com Xresources."
        );
        let include = format!("#include \"{incl}\"");
        changes.push((res, resources));
        changes.push(block(&self.home.join(".Xresources"), &include, "!")?);
        let session = self.directory.join("session.sh");
        changes.push((session.clone(),format!("# Generated by EMWM Appearance\nunset GTK_THEME\nexport XCURSOR_THEME={}\nexport XCURSOR_SIZE={size}\n",quote(cursor))));
        let hook=format!("# The selected appearance is activated only in an EMWM session.\ncase \"${{XDG_CURRENT_DESKTOP:-}}:${{DESKTOP_SESSION:-}}:${{XDG_SESSION_DESKTOP:-}}\" in\n  *EMWM*|*emwm*)\n    . {}\n    if command -v emwm-appearance >/dev/null 2>&1; then emwm-appearance --session; fi\n    ;;\nesac",quote(&session.to_string_lossy()));
        changes.push(block(&self.home.join(".Xprofile"), &hook, "#")?);
        changes.push((self.directory.join("xsettingsd.conf"),format!("Net/ThemeName {}\nNet/IconThemeName {}\nGtk/CursorThemeName {}\nGtk/CursorThemeSize {size}\n",json!(theme),json!(icons),json!(cursor))));
        for ver in ["5", "6"] {
            let p = self.config.join(format!("qt{ver}ct/qt{ver}ct.conf"));
            if p.is_file() {
                changes.push((
                    p.clone(),
                    edit(&read(&p)?, "Appearance", &[("icon_theme", icons)]),
                ));
            }
        }
        changes.push((
            self.directory.join("selection.json"),
            serde_json::to_string_pretty(&v)? + "\n",
        ));
        let old: Vec<Option<Vec<u8>>> = changes
            .iter()
            .map(|(p, _)| {
                if p.exists() {
                    fs::read(p).map(Some)
                } else {
                    Ok(None)
                }
            })
            .collect::<std::io::Result<_>>()?;
        let backup = backup_path(&self.state.join("emwm-desktop/appearance-backups"));
        for (i, (path, data)) in changes.iter().enumerate() {
            if let Err(e) = atomic(path, data.as_bytes(), Some(&backup), 0o600) {
                for j in (0..i).rev() {
                    let p = &changes[j].0;
                    if let Some(original) = &old[j] {
                        atomic(p, original, None, 0o600)?;
                    } else if p.exists() {
                        fs::remove_file(p)?;
                    }
                }
                return Err(e);
            }
        }
        Ok(backup)
    }
}

fn quote(input: &str) -> String {
    if !input.is_empty()
        && input
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_@%+=:,./-".contains(&c))
    {
        return input.into();
    }
    format!("'{}'", input.replace('\'', "'\"'\"'"))
}
