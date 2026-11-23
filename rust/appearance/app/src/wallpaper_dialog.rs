// SPDX-License-Identifier: GPL-3.0-only
use crate::ui::{self, Window};
use anyhow::{Context, Result};
use emwm_appearance::{files, images, settings::Settings};
use gtk::prelude::*;
use serde_json::{json, Value};
use std::{
    cell::Cell,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    rc::Rc,
    time::Duration,
};

#[derive(Clone)]
struct Options {
    file: gtk::FileChooserButton,
    mode: gtk::ComboBoxText,
    color: gtk::ColorButton,
    width: gtk::SpinButton,
    height: gtk::SpinButton,
}
impl Options {
    fn selection(&self) -> Result<(PathBuf, String, u32, u32, String)> {
        Ok((
            self.file
                .filename()
                .context("Selecione uma imagem primeiro.")?,
            ui::id(&self.mode),
            self.width.value_as_int() as u32,
            self.height.value_as_int() as u32,
            ui::hex(&self.color.rgba()),
        ))
    }
    fn preview(&self, image: &gtk::Image, status: &gtk::Label) -> Result<()> {
        if self.file.filename().is_none() {
            return Ok(());
        }
        let (path, mode, w, h, color) = self.selection()?;
        let src = images::decode(image::ImageReader::open(path)?, false)?;
        let preview = images::compose(src, &mode, w, h, &color)?;
        let (w, h) = preview.dimensions();
        let preview = if w > 480 || h > 200 {
            image::DynamicImage::ImageRgb8(preview)
                .thumbnail(480, 200)
                .into_rgb8()
        } else {
            preview
        };
        let (w, h) = preview.dimensions();
        let bytes = glib::Bytes::from_owned(preview.into_raw());
        let pixbuf = gdk_pixbuf::Pixbuf::from_bytes(
            &bytes,
            gdk_pixbuf::Colorspace::Rgb,
            false,
            8,
            w as i32,
            h as i32,
            (w * 3) as i32,
        );
        image.set_from_pixbuf(Some(&pixbuf));
        status.set_text("Prévia; nenhuma alteração aplicada.");
        Ok(())
    }
}

fn submit(helper: &Path, mode: &str, data: Vec<u8>) -> Result<String> {
    let mut proc = Command::new("pkexec")
        .arg(helper)
        .arg("--background")
        .arg(mode)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    // Authentication can be cancelled before pkexec reads its stdin.
    let input = proc
        .stdin
        .take()
        .context("Falha ao abrir o canal da imagem")?
        .write_all(&data);
    let output = proc.wait_with_output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "Fundo não alterado: {}",
            if stderr.trim().is_empty() {
                "autenticação cancelada."
            } else {
                stderr.trim()
            }
        );
    }
    input?;
    Ok("Fundo do login instalado. Será usado no próximo login; a imagem original pode ser movida ou removida.".into())
}

pub fn show(parent: &Rc<Window>, settings: &Settings) -> Result<()> {
    let dialog = gtk::Dialog::with_buttons(
        Some("Fundo da tela de login"),
        Some(&parent.widget),
        gtk::DialogFlags::MODAL,
        &[
            ("Fechar", gtk::ResponseType::Close),
            ("Aplicar ao login", gtk::ResponseType::Apply),
        ],
    );
    dialog.set_default_size(660, 560);
    let content = dialog.content_area();
    content.set_spacing(10);
    content.set_border_width(12);
    let grid = ui::grid();
    content.pack_start(&grid, false, false, 0);
    let file = gtk::FileChooserButton::new("Escolher imagem", gtk::FileChooserAction::Open);
    file.set_hexpand(true);
    let filter = gtk::FileFilter::new();
    filter.set_name(Some("Imagens (PNG, JPEG, WebP, BMP, TIFF, GIF)"));
    for pattern in [
        "*.png", "*.jpg", "*.jpeg", "*.webp", "*.bmp", "*.tif", "*.tiff", "*.gif",
    ] {
        filter.add_pattern(pattern);
        filter.add_pattern(&pattern.to_ascii_uppercase());
    }
    file.add_filter(filter);
    let mode = ui::combo(&[
        ("zoom", "Preencher (recortar)"),
        ("stretch", "Esticar (expandir)"),
        ("fit", "Ajustar (sem recortar)"),
        ("center", "Centralizar"),
        ("tile", "Mosaico (tile)"),
    ]);
    mode.set_active_id(Some("zoom"));
    let color = gtk::ColorButton::new();
    color.set_rgba(&gdk::RGBA::BLACK);
    let width = gtk::SpinButton::with_range(1., 16384., 1.);
    let height = gtk::SpinButton::with_range(1., 16384., 1.);
    if let Some(display) = gdk::Display::default() {
        if let Some(screen) = display.primary_monitor().or_else(|| display.monitor(0)) {
            let geo = screen.geometry();
            let scale = screen.scale_factor();
            width.set_value((geo.width() * scale) as f64);
            height.set_value((geo.height() * scale) as f64);
        }
    }
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    row.pack_start(&width, true, true, 0);
    row.pack_start(&gtk::Label::new(Some("×")), false, false, 0);
    row.pack_start(&height, true, true, 0);
    ui::attach(&grid, 0, "Imagem", &file);
    ui::attach(&grid, 1, "Posicionamento", &mode);
    ui::attach(&grid, 2, "Cor das margens", &color);
    ui::attach(&grid, 3, "Resolução de referência", &row);
    let pref = settings.directory.join("login-background.json");
    if let Ok(saved) = serde_json::from_str::<Value>(&files::read(&pref)?) {
        if let Some(p) = saved["path"].as_str().filter(|s| Path::new(s).is_file()) {
            file.set_filename(p);
        }
        if let Some(s) = saved["mode"].as_str().filter(|s| images::MODES.contains(s)) {
            mode.set_active_id(Some(s));
        }
        if let Some(c) = saved["color"]
            .as_str()
            .and_then(|c| gdk::RGBA::parse(c).ok())
        {
            color.set_rgba(&c);
        }
        if let Some(n) = saved["width"].as_f64() {
            width.set_value(n);
        }
        if let Some(n) = saved["height"].as_f64() {
            height.set_value(n);
        }
    }
    let preview = gtk::Image::new();
    preview.set_size_request(320, 180);
    content.pack_start(&preview, true, true, 0);
    let button = gtk::Button::with_label("Atualizar prévia");
    content.pack_start(&button, false, false, 0);
    content.pack_start(&ui::note("Centralizar, ajustar e mosaico usam a resolução de referência. Em monitores com outra resolução, o resultado será redimensionado. Preencher e esticar se adaptam a cada monitor.\nA cópia para o sistema é feita ao aplicar, com senha administrativa."),false,false,0);
    let status = ui::note("");
    content.pack_start(&status, false, false, 0);
    let opts = Options {
        file: file.clone(),
        mode: mode.clone(),
        color,
        width,
        height,
    };
    let update = {
        let opts = opts.clone();
        let label = status.clone();
        move || {
            if let Err(e) = opts.preview(&preview, &label) {
                label.set_text(&e.to_string());
            }
        }
    };
    let cb = update.clone();
    file.connect_file_set(move |_| cb());
    let cb = update.clone();
    mode.connect_changed(move |_| cb());
    let cb = update.clone();
    button.connect_clicked(move |_| cb());
    let busy = Rc::new(Cell::new(false));
    let lock = busy.clone();
    dialog.connect_delete_event(move |_, _| {
        if lock.get() {
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    let root = parent.clone();
    dialog.connect_response(move |dialog,response|{
        if busy.get(){return;}
        if response!=gtk::ResponseType::Apply{dialog.close();return;}
        let start=||->Result<()>{
            let helper=ui::helper().context("Atualize o componente administrativo: sudo ./apps/emwm-appearance/install-system.sh (no repositório emwm-desktop).")?;
            let (path,mode,width,height,color)=opts.selection()?;
            let data=images::prepare(&path,&mode,width,height,&color)?;
            let selection=json!({"path":path,"mode":mode,"width":width,"height":height,"color":color});
            busy.set(true);root.busy.set(true);dialog.set_response_sensitive(gtk::ResponseType::Apply,false);grid.set_sensitive(false);status.set_text("Aguardando autenticação e instalação do fundo…");
            let (sender,receiver)=std::sync::mpsc::channel();std::thread::spawn(move||{let _=sender.send(submit(&helper,&mode,data));});
            let busy=busy.clone();let parent=root.clone();let status=status.clone();let grid=grid.clone();let dlg=dialog.clone();let pref=pref.clone();
            glib::timeout_add_local(Duration::from_millis(80),move||{
                let Ok(done)=receiver.try_recv()else{return glib::ControlFlow::Continue;};
                busy.set(false);parent.busy.set(false);grid.set_sensitive(true);dlg.set_response_sensitive(gtk::ResponseType::Apply,true);
                let message=match done{Err(e)=>e.to_string(),Ok(mut message)=>{
                    let written=serde_json::to_string_pretty(&selection).map_err(anyhow::Error::from).and_then(|s|files::atomic(&pref,(s+"\n").as_bytes(),None,0o600));
                    if let Err(e)=written{message+=&format!(" Não foi possível salvar a seleção local: {e}");}message
                }};status.set_text(&message);glib::ControlFlow::Break
            });Ok(())
        };
        if let Err(e)=start(){status.set_text(&e.to_string());}
    });
    dialog.show_all();
    update();
    Ok(())
}
