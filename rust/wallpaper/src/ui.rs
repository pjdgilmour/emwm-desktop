use crate::backdrops::{self, Choice, Entries, MonitorChoice, Result, WORKSPACES};
use crate::layout;
use gtk::prelude::*;
use gtk::{
    gdk, Align, Application, ApplicationWindow, Box as GtkBox, Button, ButtonsType, ColorButton,
    ComboBoxText, Dialog, DialogFlags, FileChooserAction, FileChooserButton, FileFilter, Frame,
    Grid, Label, MessageDialog, MessageType, Orientation, RadioButton, ResponseType,
    ScrolledWindow, Window,
};
use std::cell::RefCell;
use std::rc::Rc;

fn label(text: &str) -> Label {
    let label = Label::new(Some(text));
    label.set_xalign(0.);
    label.set_line_wrap(true);
    label
}

fn finish_dialog(dialog: impl IsA<Dialog>) {
    // SAFETY: the modal run has returned. We take the last caller-owned handle
    // and never query this dialog or its children after destruction.
    unsafe {
        dialog.as_ref().destroy();
    }
}

fn message(parent: &impl IsA<Window>, kind: MessageType, title: &str, detail: &str) {
    let dialog = MessageDialog::new(
        Some(parent),
        DialogFlags::MODAL,
        kind,
        ButtonsType::Ok,
        title,
    );
    if !detail.is_empty() {
        dialog.set_secondary_text(Some(detail));
    }
    dialog.run();
    finish_dialog(dialog);
}

#[derive(Clone)]
struct Placement {
    row: GtkBox,
    mode: ComboBoxText,
    color: ColorButton,
}

impl Placement {
    fn new() -> Self {
        let row = GtkBox::new(Orientation::Horizontal, 8);
        let mode = ComboBoxText::new();
        for (key, text) in [
            ("fit", "Ajustar à tela (sem recortar)"),
            ("zoom", "Preencher a tela (recortar)"),
            ("stretch", "Esticar"),
            ("center", "Centralizar (tamanho original)"),
            ("tile", "Mosaico (tile)"),
        ] {
            mode.append(Some(key), text);
        }
        mode.set_active_id(Some("tile"));
        mode.set_tooltip_text(Some("Ajustar mantém a imagem inteira; preencher recorta as bordas. Esticar altera as proporções; centralizar e mosaico mantêm o tamanho original."));
        let color = ColorButton::with_rgba(&gdk::RGBA::new(0., 0., 0., 1.));
        color.set_use_alpha(false);
        color.set_title("Cor de fundo do papel de parede");
        color.set_tooltip_text(Some("Cor das margens e das áreas transparentes da imagem."));
        row.pack_start(&label("Posicionamento:"), false, false, 0);
        row.pack_start(&mode, true, true, 0);
        row.pack_start(&label("Cor de fundo:"), false, false, 0);
        row.pack_start(&color, false, false, 0);
        Self { row, mode, color }
    }
    fn restore(&self, mode: &str, color: &str) {
        self.mode.set_active_id(Some(mode));
        if let Ok(color) = gdk::RGBA::parse(color) {
            self.color.set_rgba(&color);
        }
    }
    fn values(&self) -> (String, String) {
        let color = self.color.rgba();
        let channel = |c: f64| ((c * 65535.).round() as u32) / 257;
        let color = format!(
            "#{:02x}{:02x}{:02x}",
            channel(color.red()),
            channel(color.green()),
            channel(color.blue())
        );
        (self.mode.active_id().unwrap_or_default().into(), color)
    }
}

fn dialog_monitors(parent: &Window, choice: &Choice) -> Result<Option<Vec<MonitorChoice>>> {
    let layout = layout::detect()?;
    let dialog = Dialog::with_buttons(
        Some("Papel de parede por monitor"),
        Some(parent),
        DialogFlags::MODAL,
        &[
            ("Cancelar", ResponseType::Cancel),
            ("Usar estes fundos", ResponseType::Ok),
        ],
    );
    dialog.set_default_size(780, 560);
    let scroll = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    let rows = GtkBox::new(Orientation::Vertical, 12);
    rows.set_border_width(12);
    scroll.add(&rows);
    dialog.content_area().pack_start(&scroll, true, true, 0);
    rows.pack_start(&label("A orientação e a resolução vêm do monitor. A rotação manual gira apenas a imagem.\nMonitores espelhados compartilham o mesmo fundo. As escolhas serão salvas em Aplicar."),false,false,0);
    let mut controls = Vec::new();
    for m in layout.monitors {
        let orientation = if m.height > m.width {
            "vertical"
        } else {
            "horizontal"
        };
        let frame = Frame::new(Some(&format!(
            "{} — {}×{} · {} · {}°",
            m.name, m.width, m.height, orientation, m.rotation
        )));
        let column = GtkBox::new(Orientation::Vertical, 6);
        column.set_border_width(10);
        frame.add(&column);
        rows.pack_start(&frame, false, false, 0);
        let file = FileChooserButton::new("Selecionar imagem", FileChooserAction::Open);
        let filter = FileFilter::new();
        filter.set_name(Some("Imagens"));
        filter.add_pixbuf_formats();
        file.add_filter(filter);
        let placement = Placement::new();
        let rotation = ComboBoxText::new();
        for (key, text) in [
            ("auto", "Automática — respeitar orientação do monitor"),
            ("0", "Imagem sem giro (0°)"),
            ("90", "Girar imagem 90° à direita"),
            ("180", "Girar imagem 180°"),
            ("270", "Girar imagem 90° à esquerda"),
        ] {
            rotation.append(Some(key), text);
        }
        let (path, mode, color, angle) = match choice.monitors.iter().find(|s| s.name == m.name) {
            Some(s) => (&s.source, &s.mode, &s.color, s.rotation.as_str()),
            None => (&choice.source, &choice.mode, &choice.color, "auto"),
        };
        if !path.as_os_str().is_empty() {
            file.set_filename(path);
        }
        placement.restore(mode, color);
        rotation.set_active_id(Some(angle));
        column.pack_start(&file, false, false, 0);
        column.pack_start(&placement.row, false, false, 0);
        column.pack_start(&rotation, false, false, 0);
        controls.push((m.name, file, placement, rotation));
    }
    dialog.show_all();
    let updated = loop {
        match dialog.run() {
            ResponseType::Ok => {
                let mut updated = choice.monitors.clone();
                let mut complete = true;
                for (name, file, placement, rotation) in &controls {
                    let Some(source) = file.filename() else {
                        complete = false;
                        break;
                    };
                    let (mode, color) = placement.values();
                    let selection = MonitorChoice {
                        name: name.clone(),
                        source,
                        mode,
                        color,
                        rotation: rotation.active_id().unwrap_or_default().into(),
                    };
                    if let Some(m) = updated.iter_mut().find(|m| m.name == *name) {
                        *m = selection;
                    } else {
                        updated.push(selection);
                    }
                }
                if complete {
                    break Some(updated);
                }
                message(
                    &dialog,
                    MessageType::Error,
                    "Selecione uma imagem para cada monitor.",
                    "",
                );
            }
            _ => break None,
        }
    };
    finish_dialog(dialog);
    Ok(updated)
}

fn monitor_label(count: usize) -> String {
    if count == 0 {
        "Configurar fundo por monitor…".into()
    } else {
        format!("Fundos por monitor ({count})…")
    }
}
struct Editor {
    button: Button,
    choice: Rc<RefCell<Choice>>,
}
impl Editor {
    fn new() -> Self {
        let choice = Rc::new(RefCell::new(Choice::default()));
        let button = Button::with_label(&monitor_label(0));
        let saved = choice.clone();
        button.connect_clicked(move |button| {
            let Some(parent) = button.toplevel().and_downcast::<Window>() else {
                return;
            };
            let result = dialog_monitors(&parent, &saved.borrow());
            match result {
                Ok(Some(monitors)) => {
                    button.set_label(&monitor_label(monitors.len()));
                    saved.borrow_mut().monitors = monitors;
                }
                Ok(None) => (),
                Err(e) => message(&parent, MessageType::Error, &e.to_string(), ""),
            }
        });
        Self { button, choice }
    }
    fn restore(&self, choice: &Choice) {
        self.choice.replace(choice.clone());
        self.button.set_label(&monitor_label(choice.monitors.len()));
    }
    fn selection(&self) -> Choice {
        self.choice.borrow().clone()
    }
}

// Older versions could store a single image for the whole X screen. Applying
// in the monitor editor gives each connected output an explicit choice.
fn fill_missing_outputs(choice: &mut Choice, layout: &layout::Layout) -> Result<()> {
    for output in &layout.monitors {
        if choice.monitors.iter().any(|m| m.name == output.name) {
            continue;
        }
        if choice.source.as_os_str().is_empty() {
            return Err(format!("Selecione uma imagem para o monitor {}.", output.name).into());
        }
        choice.monitors.push(MonitorChoice {
            name: output.name.clone(),
            source: choice.source.clone(),
            mode: choice.mode.clone(),
            color: choice.color.clone(),
            rotation: "auto".into(),
        });
    }
    Ok(())
}

struct Editors {
    all: Editor,
    one: Editor,
    four: [Editor; WORKSPACES],
    workspace: ComboBoxText,
    saved: RefCell<Entries>,
}
impl Editors {
    fn selected(&self) -> usize {
        self.workspace.active().unwrap_or(0) as usize
    }
    fn restore_one(&self) {
        let saved = self.saved.borrow();
        self.one.restore(
            &saved[self.selected()]
                .as_ref()
                .map(|(_, c)| c.clone())
                .unwrap_or_default(),
        );
    }
    fn reload(&self) -> Result<()> {
        self.saved.replace(backdrops::read(&crate::home()?)?);
        let saved = self.saved.borrow();
        if let Some((_, c)) = &saved[0] {
            self.all.restore(c);
        }
        self.restore_one();
        for (panel, entry) in self.four.iter().zip(saved.iter()) {
            if let Some((_, c)) = entry {
                panel.restore(c);
            }
        }
        Ok(())
    }
    fn apply(&self, mut choices: Vec<(usize, Choice)>, resolution: &Label) -> Result<String> {
        let home = crate::home()?;
        let layout = layout::detect()?;
        resolution.set_text(&format!(
            "Área total da tela: {}x{}",
            layout.width, layout.height
        ));
        for (_, choice) in &mut choices {
            fill_missing_outputs(choice, &layout)?;
        }
        let updates = crate::prepare(&home, choices, &layout, None)?;
        let resources = backdrops::save(&home, &updates)?;
        self.reload()?;
        let warning = crate::merge(&resources).err();
        for (i, path, _) in &updates {
            crate::log(&home, &format!("Fundo salvo ws{i}={}", path.display()));
        }
        let details = crate::notify(&home, &updates)?;
        let mut message = format!(
            "Os fundos foram salvos e serão restaurados ao entrar na sessão EMWM.\n{details}"
        );
        if let Some(error) = warning {
            message.push('\n');
            message.push_str(&error.to_string());
        }
        Ok(message)
    }
}

fn window(app: &Application) -> Result<()> {
    let layout = layout::detect()?;
    let win = ApplicationWindow::new(app);
    win.set_title("Configurar Fundo do EMWM");
    win.set_default_size(540, 380);
    win.set_border_width(16);
    let root = GtkBox::new(Orientation::Vertical, 12);
    win.add(&root);
    let resolution = label(&format!(
        "Área total da tela: {}x{}",
        layout.width, layout.height
    ));
    root.pack_start(&resolution, false, false, 0);
    let panels = GtkBox::new(Orientation::Vertical, 12);
    let scroll = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scroll.add(&panels);
    root.pack_start(&scroll, true, true, 0);
    let modes = GtkBox::new(Orientation::Vertical, 6);
    panels.pack_start(&modes, false, false, 0);
    let radios = [
        "Usar os mesmos fundos em todos os workspaces",
        "Alterar um workspace",
        "Configurar cada workspace separadamente",
    ]
    .map(RadioButton::with_label);
    radios[1].join_group(Some(&radios[0]));
    radios[2].join_group(Some(&radios[0]));
    radios[0].set_active(true);
    for r in &radios {
        modes.pack_start(r, false, false, 0);
    }
    let workspace = ComboBoxText::new();
    for i in 1..=WORKSPACES {
        workspace.append(Some(&i.to_string()), &format!("Workspace {i}"));
    }
    workspace.set_active(Some(0));
    let editors = Rc::new(Editors {
        all: Editor::new(),
        one: Editor::new(),
        four: std::array::from_fn(|_| Editor::new()),
        workspace: workspace.clone(),
        saved: RefCell::default(),
    });
    let all = GtkBox::new(Orientation::Vertical, 6);
    all.pack_start(&editors.all.button, false, false, 0);
    panels.pack_start(&all, false, false, 0);
    let one = GtkBox::new(Orientation::Vertical, 6);
    one.pack_start(&workspace, false, false, 0);
    one.pack_start(&editors.one.button, false, false, 0);
    panels.pack_start(&one, false, false, 0);
    workspace.connect_changed(glib::clone!(@weak editors => move |_| editors.restore_one()));
    let four = Grid::new();
    four.set_row_spacing(8);
    four.set_column_spacing(8);
    for (i, editor) in editors.four.iter().enumerate() {
        let y = i as i32;
        four.attach(&label(&format!("Workspace {}", i + 1)), 0, y, 1, 1);
        four.attach(&editor.button, 1, y, 1, 1);
    }
    panels.pack_start(&four, false, false, 0);
    let buttons = GtkBox::new(Orientation::Horizontal, 8);
    buttons.set_halign(Align::End);
    let apply = Button::with_label("Aplicar");
    let quit = Button::with_label("Sair");
    buttons.pack_start(&apply, false, false, 0);
    buttons.pack_start(&quit, false, false, 0);
    root.pack_end(&buttons, false, false, 0);
    quit.connect_clicked(glib::clone!(@weak win => move |_| win.close()));
    let one_radio = radios[1].clone();
    let four_radio = radios[2].clone();
    apply.connect_clicked(glib::clone!(@strong editors, @weak win => move |_| {
        let choices=if one_radio.is_active(){vec![(editors.selected(),editors.one.selection())]}
                    else if four_radio.is_active(){editors.four.iter().enumerate().map(|(i,e)|(i,e.selection())).collect()}
                    else {let choice=editors.all.selection();(0..WORKSPACES).map(|i|(i,choice.clone())).collect()};
        match editors.apply(choices,&resolution) {
            Ok(details)=>message(&win,MessageType::Info,"Concluído",&details),
            Err(error)=>message(&win,MessageType::Error,"Erro ao aplicar papel de parede",&error.to_string()),
        }
    }));
    win.show_all();
    for (r, p) in radios
        .iter()
        .zip([all.upcast::<gtk::Widget>(), one.upcast(), four.upcast()])
    {
        p.set_visible(r.is_active());
        r.connect_toggled(move |r| p.set_visible(r.is_active()));
    }
    if let Err(error) = editors.reload() {
        message(
            &win,
            MessageType::Error,
            "Erro ao carregar papéis de parede",
            &error.to_string(),
        );
    }
    Ok(())
}

pub fn run() -> std::process::ExitCode {
    use std::process::Termination;
    let app = Application::new(Some("br.com.paulo.emwm-wallpaper-tool"), Default::default());
    app.connect_activate(|app| {
        if let Some(win) = app.active_window() {
            win.present();
        } else if let Err(error) = window(app) {
            eprintln!("{error}");
            app.quit();
        }
    });
    app.run().report()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn children<T: IsA<gtk::Widget>>(widget: &gtk::Widget, items: &mut Vec<T>) {
        if let Ok(item) = widget.clone().dynamic_cast::<T>() {
            items.push(item);
        }
        if let Some(container) = widget.downcast_ref::<gtk::Container>() {
            for child in container.children() {
                children(&child, items);
            }
        }
    }

    #[test]
    fn legacy_settings_fill_only_outputs_without_a_saved_image() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let legacy_image = directory.path().join("antigo.png");
        let selected_image = directory.path().join("preferido.png");
        let output = |name: &str| layout::Monitor {
            name: name.into(),
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            rotation: 0,
        };
        let layout = layout::Layout {
            width: 1600,
            height: 600,
            monitors: vec![output("HDMI-1"), output("DP-1")],
        };
        let existing = |name: &str| MonitorChoice {
            name: name.into(),
            source: selected_image.clone(),
            mode: "zoom".into(),
            color: "#abcdef".into(),
            rotation: "90".into(),
        };
        let mut choice = Choice {
            source: legacy_image.clone(),
            mode: "fit".into(),
            color: "#123456".into(),
            monitors: vec![existing("HDMI-1"), existing("desconectado")],
        };
        fill_missing_outputs(&mut choice, &layout)?;
        assert_eq!(choice.monitors[0], existing("HDMI-1"));
        assert_eq!(choice.monitors[1], existing("desconectado"));
        assert_eq!(
            choice.monitors[2],
            MonitorChoice {
                name: "DP-1".into(),
                source: legacy_image,
                mode: "fit".into(),
                color: "#123456".into(),
                rotation: "auto".into(),
            }
        );
        fill_missing_outputs(&mut choice, &layout)?;
        assert_eq!(choice.monitors.len(), 3);
        backdrops::save(directory.path(), &[(1, selected_image, choice.clone())])?;
        assert_eq!(
            backdrops::read(directory.path())?[1].as_ref().unwrap().1,
            choice
        );
        assert!(fill_missing_outputs(&mut Choice::default(), &layout).is_err());
        Ok(())
    }

    #[test]
    #[ignore = "requer um servidor X11 privado"]
    fn monitor_dialog() -> Result<()> {
        gtk::init()?;
        let parent = Window::new(gtk::WindowType::Toplevel);
        parent.show();
        let temp = tempfile::tempdir()?;
        let image = temp.path().join("monitor-ui.png");
        let fixture = gdk_pixbuf::Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, false, 8, 4, 2).unwrap();
        fixture.fill(0xff0000ff);
        fixture.savev(&image, "png", &[])?;
        let mut choice = Choice {
            source: image.clone(),
            mode: "fit".into(),
            color: "#123456".into(),
            monitors: vec![],
        };
        for response in [ResponseType::Cancel, ResponseType::Ok] {
            let checked = Rc::new(RefCell::new(false));
            glib::timeout_add_local(
                std::time::Duration::from_millis(200),
                glib::clone!(@strong image,@strong checked => move || {
                    for window in Window::list_toplevels() {
                        let Ok(dialog)=window.downcast::<Dialog>() else {continue;};
                        if !dialog.is_mapped() || dialog.title().as_deref()!=Some("Papel de parede por monitor"){continue;}
                        let mut files=vec![];
                        children::<FileChooserButton>(dialog.upcast_ref(),&mut files);
                        let mut combos=vec![];
                        children::<ComboBoxText>(dialog.upcast_ref(),&mut combos);
                        let found = combos.iter().any(|c| c.set_active_id(Some("90")));
                        checked.replace(dialog.widget_for_response(ResponseType::Reject).is_none() && !files.is_empty() && files[0].filename().as_ref()==Some(&image) && found);
                        dialog.response(if *checked.borrow() {response} else {ResponseType::Cancel});
                        return glib::ControlFlow::Break;
                    }
                    glib::ControlFlow::Continue
                }),
            );
            let result = dialog_monitors(&parent, &choice)?;
            assert!(*checked.borrow());
            match response {
                ResponseType::Cancel => assert!(result.is_none()),
                ResponseType::Ok => {
                    choice.monitors = result.unwrap();
                    assert_eq!(choice.monitors[0].rotation, "90");
                    assert_eq!(choice.monitors[0].color, "#123456");
                    assert_eq!(choice.monitors[0].mode, "fit");
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}
