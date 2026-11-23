// SPDX-License-Identifier: GPL-3.0-only
use gtk::gdk::RGBA;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, ColorChooserDialog, DialogFlags, DrawingArea, Grid,
    Label, MessageDialog, MessageType, ResponseType,
};
use std::{cell::RefCell, env, fs, path::PathBuf, rc::Rc};

const FIELDS: [(&str, &str); 3] = [
    ("Emwm*client*background", "Decoração da janela inativa"),
    ("Emwm*client*activeBackground", "Decoração da janela ativa"),
    ("Emwm*icon*activeBackground", "Cor de seleção do ícone"),
];
fn load(text: &str) -> [String; 3] {
    let mut colors = [String::new(), String::new(), String::new()];
    for line in text.lines() {
        for (i, (key, _)) in FIELDS.iter().enumerate() {
            if let Some((_, value)) = line.split_once(':').filter(|_| line.contains(key)) {
                colors[i] = value.trim().to_owned();
            }
        }
    }
    colors
}
fn save(text: &[u8], colors: &[String; 3]) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len());
    for raw in text.split_inclusive(|b| *b == b'\n') {
        let line = raw.strip_suffix(b"\n").unwrap_or(raw);
        if let Some((i, (key, _))) = FIELDS
            .iter()
            .enumerate()
            .find(|(_, (key, _))| line.windows(key.len()).any(|s| s == key.as_bytes()))
        {
            out.extend_from_slice(key.as_bytes());
            out.extend_from_slice(b": ");
            out.extend_from_slice(colors[i].as_bytes());
        } else {
            out.extend_from_slice(line);
        }
        out.push(b'\n');
    }
    out
}
fn rgba(value: &str) -> RGBA {
    RGBA::parse(value).unwrap_or_else(|_| RGBA::new(1., 1., 1., 1.))
}
fn message(parent: &ApplicationWindow, kind: MessageType, text: &str) {
    let dialog = MessageDialog::new(
        Some(parent),
        DialogFlags::MODAL,
        kind,
        gtk::ButtonsType::Ok,
        text,
    );
    if kind == MessageType::Info {
        dialog.set_title("Sucesso");
    }
    dialog.run();
    dialog.close();
}
fn build(app: &Application) {
    let win = ApplicationWindow::new(app);
    win.set_title("Ajustar o tema");
    win.set_border_width(10);
    win.set_resizable(false);
    let grid = Grid::new();
    grid.set_row_spacing(8);
    grid.set_column_spacing(10);
    win.add(&grid);
    let Some(home) = env::var_os("HOME").filter(|s| !s.is_empty()) else {
        message(
            &win,
            MessageType::Error,
            "Não foi possível encontrar a variável HOME.",
        );
        return;
    };
    let path = PathBuf::from(home).join(".Xresources");
    let colors = Rc::new(RefCell::new(load(&String::from_utf8_lossy(
        &fs::read(&path).unwrap_or_default(),
    ))));
    for (i, (_, title)) in FIELDS.iter().enumerate() {
        let label = Label::new(Some(title));
        label.set_halign(gtk::Align::Start);
        let square = DrawingArea::new();
        square.set_size_request(58, 22);
        let current = Rc::clone(&colors);
        square.connect_draw(move |_, cr| {
            let c = rgba(&current.borrow()[i]);
            cr.set_source_rgb(c.red(), c.green(), c.blue());
            let _ = cr.paint();
            gtk::glib::Propagation::Proceed
        });
        let select = Button::with_label("Selecionar");
        let parent = win.downgrade();
        let swatch = square.downgrade();
        let chosen = Rc::clone(&colors);
        select.connect_clicked(move |_| {
            let Some(parent) = parent.upgrade() else {
                return;
            };
            let dialog = ColorChooserDialog::new(Some("Selecionar cor"), Some(&parent));
            dialog.set_rgba(&rgba(&chosen.borrow()[i]));
            if dialog.run() == ResponseType::Ok {
                let c = dialog.rgba();
                let byte = |v: f64| (v * 255.).round().clamp(0., 255.) as u8;
                chosen.borrow_mut()[i] = format!(
                    "#{:02x}{:02x}{:02x}",
                    byte(c.red()),
                    byte(c.green()),
                    byte(c.blue())
                );
                if let Some(square) = swatch.upgrade() {
                    square.queue_draw();
                }
            }
            dialog.close();
        });
        grid.attach(&label, 0, i as i32, 1, 1);
        grid.attach(&square, 1, i as i32, 1, 1);
        grid.attach(&select, 2, i as i32, 1, 1);
    }
    let apply = Button::with_label("Salvar");
    let parent = win.downgrade();
    apply.connect_clicked(move |_| {
        let Some(parent) = parent.upgrade() else {
            return;
        };
        let Ok(source) = fs::read(&path) else {
            message(
                &parent,
                MessageType::Error,
                "Não foi possível abrir ~/.Xresources para leitura.",
            );
            return;
        };
        if fs::write(&path, save(&source, &colors.borrow())).is_err() {
            message(
                &parent,
                MessageType::Error,
                "Não foi possível abrir ~/.Xresources para escrita.",
            );
            return;
        }
        message(
            &parent,
            MessageType::Info,
            "As cores foram salvas com sucesso!",
        );
        parent.close();
    });
    grid.attach(&apply, 0, 3, 3, 1);
    win.show_all();
}
fn main() {
    let app = Application::builder()
        .application_id("br.local.trocacor")
        .build();
    app.connect_activate(build);
    std::process::exit(app.run().value());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserve_other_resources_and_use_last_matching_value() {
        let text =
            "XTerm*size: 12\nEmwm*client*background: #ffffff\nEmwm*client*background:   #00AA00\n";
        let mut colors = load(text);
        assert_eq!(colors[0], "#00AA00");
        colors[0] = "#123456".into();
        assert_eq!(
            save(text.as_bytes(), &colors),
            b"XTerm*size: 12\nEmwm*client*background: #123456\nEmwm*client*background: #123456\n"
        );
    }
}
