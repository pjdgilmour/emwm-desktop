// SPDX-License-Identifier: GPL-3.0-only
use anyhow::{Context, Result};
use emwm_appearance::{
    files, lightdm, runtime,
    settings::{self, Settings, COLORS, RANGES, THEME_KINDS},
    text,
};
use gtk::prelude::*;
use serde_json::{json, Value};
use std::{
    cell::Cell,
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
    rc::Rc,
    time::Duration,
};

fn label(text: &str) -> gtk::Label {
    let label = gtk::Label::new(Some(text));
    label.set_xalign(0.0);
    label
}
pub fn note(text: &str) -> gtk::Label {
    let l = label(text);
    l.set_line_wrap(true);
    l.set_max_width_chars(76);
    l
}
pub fn vbox(spacing: i32, border: u32) -> gtk::Box {
    let b = gtk::Box::new(gtk::Orientation::Vertical, spacing);
    b.set_border_width(border);
    b
}
pub fn grid() -> gtk::Grid {
    let g = gtk::Grid::new();
    g.set_column_spacing(16);
    g.set_row_spacing(8);
    g
}
pub fn attach(g: &gtk::Grid, row: i32, text: &str, w: &impl IsA<gtk::Widget>) {
    g.attach(&label(text), 0, row, 1, 1);
    g.attach(w, 1, row, 1, 1);
}
fn frame(name: &str, widget: &impl IsA<gtk::Widget>) -> gtk::Frame {
    let f = gtk::Frame::new(Some(name));
    f.add(widget);
    f
}
fn scroll(widget: &impl IsA<gtk::Widget>) -> gtk::ScrolledWindow {
    let w = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    w.add(widget);
    w
}
pub fn combo(opts: &[(&str, &str)]) -> gtk::ComboBoxText {
    let c = gtk::ComboBoxText::new();
    for (k, v) in opts {
        c.append(Some(k), v);
    }
    c
}
pub fn id(combo: &gtk::ComboBoxText) -> String {
    combo.active_id().map(|s| s.to_string()).unwrap_or_default()
}
pub fn hex(rgba: &gdk::RGBA) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        (rgba.red() * 255.).round_ties_even() as u8,
        (rgba.green() * 255.).round_ties_even() as u8,
        (rgba.blue() * 255.).round_ties_even() as u8
    )
}
pub fn rgba(s: &str, default: &str) -> gdk::RGBA {
    gdk::RGBA::parse(s)
        .or_else(|_| gdk::RGBA::parse(default))
        .unwrap_or(gdk::RGBA::BLACK)
}
fn set(c: &gtk::ComboBoxText, s: &str) {
    if !c.set_active_id(Some(s)) {
        c.set_active(Some(0));
    }
}
pub fn helper() -> Option<PathBuf> {
    let path = Path::new("/usr/lib/emwm-desktop/appearance/emwm-appearance-login");
    (path.is_file() && files::safe_private_path(path)).then(|| path.to_owned())
}

enum Control {
    Choice(gtk::ComboBoxText),
    Toggle(gtk::CheckButton),
    Number(gtk::SpinButton),
}
struct Effects {
    frame: gtk::Frame,
    widgets: BTreeMap<&'static str, Control>,
}
impl Effects {
    fn new() -> Self {
        let g = grid();
        g.set_border_width(12);
        let mut widgets = BTreeMap::new();
        for (r, (k, text)) in [
            ("backend", "Renderização"),
            ("effect", "Minimizar, restaurar e maximizar"),
            ("duration_ms", "Duração da animação (ms)"),
            ("fade_ms", "Aparecimento das janelas (ms)"),
            ("shadows", "Sombras das janelas"),
            ("shadow_radius", "Suavidade da sombra (px)"),
            ("shadow_offset", "Deslocamento da sombra (px)"),
            ("shadow_opacity", "Intensidade da sombra (%)"),
            ("vsync", "Sincronização vertical (OpenGL)"),
            ("max_fps", "Limite de quadros por segundo"),
        ]
        .into_iter()
        .enumerate()
        {
            let ctrl = match k {
                "backend" | "effect" => {
                    let options: &[(&str, &str)] = if k == "backend" {
                        &[
                            ("auto", "Automática (OpenGL quando disponível)"),
                            ("glx", "OpenGL / GPU"),
                            ("xrender", "XRender (compatibilidade)"),
                        ]
                    } else {
                        &[
                            ("none", "Sem animação"),
                            ("zoom", "Zoom — contornos clássicos"),
                            ("twist", "Twist — giro"),
                            ("flip", "Flip — dobra"),
                        ]
                    };
                    let w = combo(options);
                    attach(&g, r as i32, text, &w);
                    Control::Choice(w)
                }
                "shadows" | "vsync" => {
                    let w = gtk::CheckButton::new();
                    attach(&g, r as i32, text, &w);
                    Control::Toggle(w)
                }
                _ => {
                    let (_, lo, hi) = RANGES
                        .iter()
                        .copied()
                        .find(|(key, _, _)| *key == k)
                        .unwrap_or((k, 0, 100));
                    let w = gtk::SpinButton::with_range(lo as f64, hi as f64, 1.);
                    attach(&g, r as i32, text, &w);
                    Control::Number(w)
                }
            };
            widgets.insert(k, ctrl);
        }
        g.attach(&note("Os efeitos lembram o Window Maker. Aplicar atualiza o compositor em execução.\nOpenGL depende do driver; a sessão usa XRender se não puder iniciá-lo."),0,10,2,1);
        let f = frame("Composição, sombras e animações", &g);
        f.set_valign(gtk::Align::Start);
        Self { frame: f, widgets }
    }
    fn reload(&self, v: &Value) {
        for (k, w) in &self.widgets {
            match w {
                Control::Choice(c) => set(c, v[*k].as_str().unwrap_or_default()),
                Control::Toggle(b) => b.set_active(v[*k].as_bool().unwrap_or(false)),
                Control::Number(n) => n.set_value(
                    v[*k].as_f64().unwrap_or_default()
                        * if *k == "shadow_opacity" { 100. } else { 1. },
                ),
            }
        }
    }
    fn values(&self) -> Value {
        let mut v = json!({});
        for (k, w) in &self.widgets {
            v[*k] = match w {
                Control::Choice(c) => json!(id(c)),
                Control::Toggle(b) => json!(b.is_active()),
                Control::Number(n) if *k == "shadow_opacity" => {
                    json!(n.value_as_int() as f64 / 100.)
                }
                Control::Number(n) => json!(n.value_as_int()),
            };
        }
        v
    }
}

struct Desktop {
    page: gtk::Box,
    monitor: gtk::ComboBoxText,
    anchor: gtk::ComboBoxText,
    detected: gtk::Label,
}
impl Desktop {
    fn new() -> Self {
        let page = vbox(14, 14);
        page.pack_start(
            &label("Ícones, arquivos e lixeira em um único monitor"),
            false,
            false,
            0,
        );
        let g = grid();
        let monitor = combo(&[]);
        monitor.set_hexpand(true);
        let anchor = combo(&[("left", "Esquerda"), ("right", "Direita")]);
        attach(&g, 0, "Monitor dos ícones", &monitor);
        attach(&g, 1, "Ancorar ícones à", &anchor);
        page.pack_start(&g, false, false, 0);
        let detected = note("");
        page.pack_start(&detected, false, false, 0);
        page.pack_start(&note("Seguir o sistema respeita o monitor principal informado pelo X11/driver. Se ele ou a saída escolhida estiver desconectado, será usado o principal disponível ou o primeiro monitor ativo. A escolha é lembrada quando o monitor voltar.\n\nOs outros monitores continuam com o papel de parede. Ao trocar o lado, as posições manuais são espelhadas horizontalmente, mantendo a altura. Esta escolha afeta apenas os ícones; não altera a disposição dos monitores no driver."),false,false,0);
        let wallpaper = gtk::Button::with_label("Configurar papel de parede…");
        wallpaper.connect_clicked(|_| {
            let _ = Command::new("emwm-wallpaper-tool").spawn();
        });
        page.pack_start(&wallpaper, false, false, 0);
        Self {
            page,
            monitor,
            anchor,
            detected,
        }
    }
    fn populate(&self, selected: &str) {
        self.monitor.remove_all();
        self.monitor
            .append(Some("primary"), "Seguir o principal do sistema");
        let mut main = None;
        if let (Some(screen), Some(display)) = (gdk::Screen::default(), gdk::Display::default()) {
            let primary = display.primary_monitor();
            for i in 0..display.n_monitors() {
                use glib::translate::{from_glib_full, ToGlibPtr};
                let Some(monitor) = display.monitor(i) else {
                    continue;
                };
                // SAFETY: screen is live on the GTK thread and i is a current monitor index.
                // GDK returns an owned string; from_glib_full releases it on drop.
                let name: Option<glib::GString> = unsafe {
                    from_glib_full(gdk::ffi::gdk_screen_get_monitor_plug_name(
                        screen.to_glib_none().0,
                        i,
                    ))
                };
                if let Some(name) = name {
                    let geo = monitor.geometry();
                    let mut tag = format!(
                        "{name} — {}×{} ({}, {})",
                        geo.width(),
                        geo.height(),
                        geo.x(),
                        geo.y()
                    );
                    if primary.as_ref() == Some(&monitor) {
                        tag += " · principal do sistema";
                        main = Some(name.to_string());
                    }
                    self.monitor.append(Some(name.as_str()), &tag);
                }
            }
        }
        if !self.monitor.set_active_id(Some(selected)) {
            self.monitor
                .append(Some(selected), &format!("{selected} · desconectado"));
            set(&self.monitor, selected);
        }
        self.detected.set_text(&format!(
            "Principal disponível informado pelo X11: {}",
            main.as_deref()
                .unwrap_or("não informado; será usado o primeiro ativo.")
        ));
    }
    fn reload(&self, v: &Value) {
        self.populate(v["monitor"].as_str().unwrap_or("primary"));
        set(&self.anchor, v["anchor"].as_str().unwrap_or("left"));
    }
    fn values(&self) -> Value {
        json!({"monitor":id(&self.monitor),"anchor":id(&self.anchor)})
    }
}

struct Gradient {
    frame: gtk::Frame,
    mode: gtk::ComboBoxText,
    colors: BTreeMap<&'static str, gtk::ColorButton>,
}
impl Gradient {
    fn new() -> Self {
        let g = grid();
        g.set_border_width(12);
        let mode = combo(&[
            ("none", "Sem gradiente (cores do tema)"),
            ("horizontal", "Horizontal — esquerda para direita"),
            ("vertical", "Vertical — cima para baixo"),
            (
                "diagonal",
                "Diagonal — canto superior esquerdo ao inferior direito",
            ),
        ]);
        g.attach(&label("Preenchimento"), 0, 0, 1, 1);
        g.attach(&mode, 1, 0, 3, 1);
        for (i, t) in ["Cor inicial", "Cor final", "Texto"].iter().enumerate() {
            g.attach(&label(t), i as i32 + 1, 1, 1, 1);
        }
        g.attach(&label("Janela ativa"), 0, 2, 1, 1);
        g.attach(&label("Janela inativa"), 0, 3, 1, 1);
        let mut colors = BTreeMap::new();
        for (i, (k, _, _)) in COLORS.iter().enumerate() {
            let button = gtk::ColorButton::new();
            button.set_title(&format!(
                "{} — {}",
                if i < 3 {
                    "Janela ativa"
                } else {
                    "Janela inativa"
                },
                ["cor inicial", "cor final", "texto"][i % 3]
            ));
            g.attach(&button, 1 + (i % 3) as i32, 2 + (i / 3) as i32, 1, 1);
            colors.insert(*k, button);
        }
        g.attach(&note("A prévia não altera as janelas. Aplicar atualiza o gradiente e o texto ao vivo. O relevo e os botões Motif são preservados."),0,4,4,1);
        Self {
            frame: frame("Gradiente da barra de título", &g),
            mode,
            colors,
        }
    }
    fn reload(&self, v: &Value) {
        set(&self.mode, v["mode"].as_str().unwrap_or("none"));
        for (k, _, d) in COLORS {
            self.colors[k].set_rgba(&rgba(v[k].as_str().unwrap_or(d), d));
        }
    }
    fn values(&self) -> Value {
        let mut v = json!({"mode":id(&self.mode)});
        for (k, _, _) in COLORS {
            v[k] = json!(hex(&self.colors[k].rgba()));
        }
        v
    }
}

pub struct Window {
    pub widget: gtk::Window,
    settings: Settings,
    pages: gtk::Notebook,
    combos: BTreeMap<&'static str, gtk::ComboBoxText>,
    size: gtk::SpinButton,
    palette: gtk::ComboBoxText,
    colors: BTreeMap<&'static str, gtk::ColorButton>,
    gradient: Gradient,
    effects: Effects,
    desktop: Desktop,
    drawing: gtk::DrawingArea,
    login: gtk::CheckButton,
    wallpaper: gtk::Button,
    status: gtk::Label,
    reset: gtk::Button,
    apply: gtk::Button,
    pub busy: Cell<bool>,
}
const LABELS: [&str; 5] = [
    "Fundo dos aplicativos Motif",
    "Texto dos aplicativos Motif",
    "Decoração da janela inativa",
    "Decoração da janela ativa",
    "Cor de seleção do ícone",
];
impl Window {
    pub fn new(settings: Settings, initial: &Value, page: u32) -> Result<Rc<Self>> {
        let widget = gtk::Window::new(gtk::WindowType::Toplevel);
        widget.set_title("Aparência do EMWM");
        widget.set_default_size(680, 700);
        widget.set_border_width(14);
        let layout = vbox(12, 0);
        widget.add(&layout);
        let pages = gtk::Notebook::new();
        layout.pack_start(&pages, true, true, 0);
        let outer = vbox(12, 0);
        let s = scroll(&outer);
        s.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        pages.append_page(&s, Some(&label("Temas e cores")));
        outer.pack_start(
            &label("Temas dos aplicativos, ícones e ponteiro do mouse"),
            false,
            false,
            0,
        );
        let g = grid();
        outer.pack_start(&g, false, false, 0);
        let mut combos = BTreeMap::new();
        for (i, (k, title)) in [
            ("theme", "Tema dos aplicativos"),
            ("icons", "Tema de ícones"),
            ("cursor", "Tema do cursor"),
        ]
        .into_iter()
        .enumerate()
        {
            let c = combo(&[]);
            c.set_hexpand(true);
            attach(&g, i as i32, title, &c);
            combos.insert(k, c);
        }
        let size = gtk::SpinButton::with_range(8., 128., 1.);
        attach(&g, 3, "Tamanho do cursor", &size);
        let palette = combo(&[
            ("keep", "Manter as cores atuais"),
            ("custom", "Cores personalizadas"),
            ("irix", "IRIX clássico"),
            ("classic", "Motif cinza"),
        ]);
        attach(&g, 4, "Cores das janelas Motif", &palette);
        let cg = grid();
        cg.set_border_width(10);
        let decoration = vbox(12, 0);
        decoration.pack_start(&frame("Cores do tema e da decoração", &cg), false, false, 0);
        let mut colors = BTreeMap::new();
        for (i, k) in text::FIELDS.iter().enumerate() {
            let b = gtk::ColorButton::new();
            b.set_title(LABELS[i]);
            attach(&cg, i as i32, LABELS[i], &b);
            colors.insert(*k, b);
        }
        let drawing = gtk::DrawingArea::new();
        drawing.set_size_request(420, 100);
        cg.attach(&drawing, 0, 5, 2, 1);
        let gradient = Gradient::new();
        decoration.pack_start(&gradient.frame, false, false, 0);
        let demo = vbox(8, 12);
        let entry = gtk::Entry::new();
        entry.set_text("Texto de exemplo");
        demo.pack_start(&entry, false, false, 0);
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        for icon in ["folder", "text-x-generic", "preferences-desktop-theme"] {
            row.pack_start(
                &gtk::Image::from_icon_name(Some(icon), gtk::IconSize::Dialog),
                false,
                false,
                0,
            );
        }
        row.pack_end(
            &gtk::Button::with_label("Botão de exemplo"),
            false,
            false,
            0,
        );
        demo.pack_start(&row, false, false, 0);
        let ck = gtk::CheckButton::with_label("Opção de exemplo");
        ck.set_active(true);
        demo.pack_start(&ck, false, false, 0);
        outer.pack_start(
            &frame(
                "Prévia local — nenhuma preferência é salva até Aplicar",
                &demo,
            ),
            false,
            false,
            0,
        );
        let effects = Effects::new();
        pages.append_page(
            &scroll(&effects.frame),
            Some(&label("Efeitos e renderização")),
        );
        let desktop = Desktop::new();
        pages.append_page(&desktop.page, Some(&label("Área de trabalho")));
        pages.append_page(&scroll(&decoration), Some(&label("Decoração")));
        let detected = lightdm::detect(
            Path::new("/"),
            &std::env::var("XDG_SEAT").unwrap_or_else(|_| "seat0".into()),
        )?;
        outer.pack_start(
            &label(&format!(
                "Tela de login detectada: {} / {}",
                detected["manager"].as_str().unwrap_or_default(),
                detected["greeter"].as_str().unwrap_or_default()
            )),
            false,
            false,
            0,
        );
        let supported = detected["supported"] == true;
        let login = gtk::CheckButton::with_label(
            "Aplicar também à tela de login (solicita senha administrativa)",
        );
        login.set_sensitive(supported);
        outer.pack_start(&login, false, false, 0);
        let wallpaper = gtk::Button::with_label("Fundo da tela de login…");
        wallpaper.set_sensitive(supported);
        outer.pack_start(&wallpaper, false, false, 0);
        let mut details="A tela de login usa os temas instalados para todo o sistema. Quando há duas cópias, a sessão prioriza a pessoal e o login usa a do sistema. As mudanças aparecem no próximo login. Aplicativos Qt/libadwaita podem manter aparência própria; GTK 2/4 precisam de arquivos compatíveis no tema. As cores Motif entram em vigor ao reiniciar o WM; o formato das decorações é preservado.".to_owned();
        if !supported {
            details+=" Este gerenciador/greeter foi detectado, mas não possui integração de tema nesta versão.";
        }
        outer.pack_start(&note(&details), false, false, 0);
        let status = note("");
        layout.pack_start(&status, false, false, 0);
        let buttons = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
        buttons.set_layout(gtk::ButtonBoxStyle::End);
        let reset = gtk::Button::with_label("Recarregar");
        let apply = gtk::Button::with_label("Aplicar");
        let close = gtk::Button::with_label("Fechar");
        for b in [&reset, &close, &apply] {
            buttons.add(b);
        }
        layout.pack_end(&buttons, false, false, 0);
        let window = Rc::new(Self {
            widget,
            settings,
            pages,
            combos,
            size,
            palette,
            colors,
            gradient,
            effects,
            desktop,
            drawing,
            login,
            wallpaper,
            status,
            reset,
            apply,
            busy: Cell::new(false),
        });
        window.reload(initial)?;
        window.connect(&close);
        window.preview();
        window.widget.show_all();
        window.pages.set_current_page(Some(page));
        Ok(window)
    }
    fn values(&self) -> Value {
        let mut v = json!({});
        for (key, _) in THEME_KINDS {
            v[key] = json!(id(&self.combos[key]));
        }
        v["size"] = json!(self.size.value_as_int());
        v["palette"] = json!(id(&self.palette));
        let mut colors = json!({});
        for k in text::FIELDS {
            colors[k] = json!(hex(&self.colors[k].rgba()));
        }
        v["colors"] = colors;
        v["gradient"] = self.gradient.values();
        v
    }
    fn palette_changed(&self) -> Result<()> {
        let pal = id(&self.palette);
        if pal != "custom" {
            let defaults = text::palette(&pal).context("Paleta inválida.")?;
            let colors = if pal == "keep" {
                self.settings.motif_colors()?
            } else {
                Value::Null
            };
            for (k, d) in text::FIELDS.into_iter().zip(defaults) {
                self.colors[k].set_rgba(&rgba(colors[k].as_str().unwrap_or(d), d));
            }
        }
        self.drawing.queue_draw();
        Ok(())
    }
    fn reload(&self, v: &Value) -> Result<()> {
        self.effects.reload(&self.settings.effects()?);
        self.desktop.reload(&self.settings.desktop()?);
        self.gradient.reload(&v["gradient"]);
        for (key, kind) in THEME_KINDS {
            let c = &self.combos[key];
            c.remove_all();
            for t in self.settings.discover(kind) {
                let caps = if key == "theme" {
                    format!(" · {}", t.capabilities.join(", "))
                } else {
                    String::new()
                };
                let src = if t.system {
                    "sistema"
                } else if t.system_caps.is_empty() {
                    "pessoal"
                } else {
                    "pessoal + sistema"
                };
                c.append(Some(&t.name), &format!("{}{caps} · {src}", t.name));
            }
            set(c, v[key].as_str().unwrap_or_default());
        }
        self.size.set_value(v["size"].as_f64().unwrap_or(24.));
        set(&self.palette, v["palette"].as_str().unwrap_or("keep"));
        if v["palette"] == "custom" {
            for (k, d) in text::FIELDS
                .into_iter()
                .zip(text::palette("keep").unwrap_or_default())
            {
                self.colors[k].set_rgba(&rgba(v["colors"][k].as_str().unwrap_or(d), d));
            }
        } else {
            self.palette_changed()?;
        }
        self.drawing.queue_draw();
        self.preview();
        Ok(())
    }
    fn preview(&self) {
        if let Some(prefs) = gtk::Settings::default() {
            for (k, prop) in [
                ("theme", "gtk-theme-name"),
                ("icons", "gtk-icon-theme-name"),
                ("cursor", "gtk-cursor-theme-name"),
            ] {
                if let Some(s) = self.combos[k].active_id() {
                    prefs.set_property(prop, s.as_str());
                }
            }
            prefs.set_property("gtk-cursor-theme-size", self.size.value_as_int());
        }
        if let Some(w) = self.widget.window() {
            w.set_cursor(gdk::Cursor::from_name(&w.display(), "default").as_ref());
        }
    }
    fn draw(&self, cr: &cairo::Context, width: f64) {
        let fill = |key: &str, x: f64, y: f64, w: f64, h: f64| {
            let c = self.colors[key].rgba();
            cr.set_source_rgb(c.red(), c.green(), c.blue());
            cr.rectangle(x, y, w, h);
            let _ = cr.fill();
        };
        let g = self.gradient.values();
        let mode = g["mode"].as_str().unwrap_or("none");
        for (x, key, state, title) in [
            (4., "Emwm*client*background", "inactive", "Janela inativa"),
            (
                width / 2. + 4.,
                "Emwm*client*activeBackground",
                "active",
                "Janela ativa",
            ),
        ] {
            let w = width / 2. - 12.;
            fill(key, x, 4., w, 88.);
            if mode != "none" && w > 2. {
                let pattern = cairo::LinearGradient::new(
                    0.,
                    0.,
                    if mode == "vertical" { 0. } else { 1. },
                    if mode == "horizontal" { 0. } else { 1. },
                );
                pattern.set_matrix(cairo::Matrix::new(
                    1. / (w - 2.),
                    0.,
                    0.,
                    1. / 25.,
                    -(x + 1.) / (w - 2.),
                    -5. / 25.,
                ));
                for (i, part) in ["start", "end"].into_iter().enumerate() {
                    let c = rgba(
                        g[format!("{state}_{part}")].as_str().unwrap_or("#000000"),
                        "#000000",
                    );
                    pattern.add_color_stop_rgb(i as f64, c.red(), c.green(), c.blue());
                }
                let _ = cr.set_source(&pattern);
                cr.rectangle(x + 1., 5., w - 2., 25.);
                let _ = cr.fill();
            }
            fill("*background", x + 5., 31., w - 10., 56.);
            let c = self.colors[key].rgba();
            let shade = if 0.299 * c.red() + 0.587 * c.green() + 0.114 * c.blue() > 0.5 {
                0.
            } else {
                1.
            };
            if mode == "none" {
                cr.set_source_rgb(shade, shade, shade);
            } else {
                let c = rgba(
                    g[format!("{state}_text")].as_str().unwrap_or("#000000"),
                    "#000000",
                );
                cr.set_source_rgb(c.red(), c.green(), c.blue());
            }
            cr.move_to(x + 8., 23.);
            let _ = cr.show_text(title);
            let c = self.colors["*foreground"].rgba();
            cr.set_source_rgb(c.red(), c.green(), c.blue());
            cr.move_to(x + 12., 54.);
            let _ = cr.show_text("Texto Motif");
            fill("Emwm*icon*activeBackground", x + 12., 64., 24., 16.);
        }
    }
    fn connect(self: &Rc<Self>, close: &gtk::Button) {
        let weak = Rc::downgrade(self);
        close.connect_clicked(move |_| {
            if let Some(w) = weak.upgrade() {
                w.widget.close();
            }
        });
        let weak = Rc::downgrade(self);
        self.widget.connect_delete_event(move |_, _| {
            if weak.upgrade().is_some_and(|w| w.busy.get()) {
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        self.widget.connect_destroy(|_| gtk::main_quit());
        for c in self.combos.values() {
            let weak = Rc::downgrade(self);
            c.connect_changed(move |_| {
                if let Some(w) = weak.upgrade() {
                    w.preview();
                }
            });
        }
        let weak = Rc::downgrade(self);
        self.size.connect_value_changed(move |_| {
            if let Some(w) = weak.upgrade() {
                w.preview();
            }
        });
        let weak = Rc::downgrade(self);
        self.palette.connect_changed(move |_| {
            if let Some(w) = weak.upgrade() {
                if let Err(e) = w.palette_changed() {
                    w.status.set_text(&e.to_string());
                }
            }
        });
        for b in self.colors.values() {
            let weak = Rc::downgrade(self);
            b.connect_color_set(move |_| {
                if let Some(w) = weak.upgrade() {
                    set(&w.palette, "custom");
                    w.drawing.queue_draw();
                }
            });
        }
        for b in self.gradient.colors.values() {
            let d = self.drawing.clone();
            b.connect_color_set(move |_| d.queue_draw());
        }
        let d = self.drawing.clone();
        self.gradient.mode.connect_changed(move |_| d.queue_draw());
        let weak = Rc::downgrade(self);
        self.drawing.connect_draw(move |area, cr| {
            if let Some(w) = weak.upgrade() {
                w.draw(cr, area.allocated_width() as f64);
            }
            glib::Propagation::Proceed
        });
        if let Some(screen) = gdk::Screen::default() {
            let weak = Rc::downgrade(self);
            let handler = screen.connect_monitors_changed(move |_| {
                if let Some(w) = weak.upgrade() {
                    w.desktop.populate(&id(&w.desktop.monitor));
                }
            });
            let handler = std::cell::RefCell::new(Some(handler));
            self.widget.connect_destroy(move |_| {
                if let Some(id) = handler.take() {
                    screen.disconnect(id);
                }
            });
        }
        let weak = Rc::downgrade(self);
        self.reset.connect_clicked(move |_| {
            if let Some(w) = weak.upgrade() {
                let r = w.settings.current().and_then(|v| w.reload(&v));
                w.status.set_text(
                    &r.map(|_| "Temas e preferências recarregados.".to_owned())
                        .unwrap_or_else(|e| e.to_string()),
                );
            }
        });
        let weak = Rc::downgrade(self);
        self.apply.connect_clicked(move |_| {
            if let Some(w) = weak.upgrade() {
                if let Err(e) = w.apply() {
                    w.status.set_text(&e.to_string());
                }
            }
        });
        let weak = Rc::downgrade(self);
        self.wallpaper.connect_clicked(move |_| {
            if let Some(w) = weak.upgrade() {
                if let Err(e) = crate::wallpaper_dialog::show(&w, &w.settings) {
                    w.status.set_text(&e.to_string());
                }
            }
        });
    }
    fn apply(self: &Rc<Self>) -> Result<()> {
        let values = settings::selection(&self.values())?;
        let admin = if self.login.is_active() {
            let path=helper().context("Instale o componente administrativo primeiro: sudo ./apps/emwm-appearance/install-system.sh (no repositório emwm-desktop).")?;
            self.settings.check_themes(&values, true)?;
            Some(path)
        } else {
            None
        };
        let backup =
            self.settings
                .apply(&values, &self.effects.values(), &self.desktop.values())?;
        let mut notes = runtime::apply(&self.settings, &values)?;
        notes.extend(["Composição: o compositor ativo lê os efeitos em até um segundo; serão usados também no próximo login.","Área de trabalho: monitor e alinhamento aplicados automaticamente; uma operação de arrastar em andamento termina primeiro.","Decoração: gradiente e cor do texto atualizados nas janelas abertas."].map(str::to_owned));
        self.status.set_text(&format!("Aplicado à sessão. Backup: {}\n{}\nCores Motif: reinicie o EMWM para atualizar as decorações e reabra os aplicativos Motif.\nSe a sessão foi iniciada com GTK_THEME fixado, saia e entre novamente uma vez para liberar a troca de tema em todos os aplicativos.",backup.display(),notes.join("\n")));
        if let Some(path) = admin {
            self.busy.set(true);
            self.apply.set_sensitive(false);
            self.reset.set_sensitive(false);
            let mut cmd = Command::new("pkexec");
            cmd.arg(path);
            for (k, _) in THEME_KINDS {
                cmd.arg(values[k].as_str().context("Tema inválido.")?);
            }
            cmd.arg(values["size"].to_string());
            let weak = Rc::downgrade(self);
            let (send, recv) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let _ = send.send(cmd.output());
            });
            glib::timeout_add_local(Duration::from_millis(80), move || {
                let Ok(done) = recv.try_recv() else {
                    return glib::ControlFlow::Continue;
                };
                if let Some(w) = weak.upgrade() {
                    w.busy.set(false);
                    w.apply.set_sensitive(true);
                    w.reset.set_sensitive(true);
                    let message = match done {
                        Ok(o) if o.status.success() => {
                            "Tela de login configurada. Será usada no próximo login.".into()
                        }
                        Ok(o) => format!(
                            "A sessão foi configurada, mas a tela de login não foi alterada: {}",
                            if o.stderr.is_empty() {
                                "autenticação cancelada.".into()
                            } else {
                                String::from_utf8_lossy(&o.stderr).trim().to_owned()
                            }
                        ),
                        Err(e) => format!("Sessão configurada; falha na integração do login: {e}"),
                    };
                    w.status
                        .set_text(&format!("{}\n{message}", w.status.text()));
                }
                glib::ControlFlow::Break
            });
        }
        Ok(())
    }
}
