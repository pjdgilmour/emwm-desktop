use crate::backdrops::{resource_file, valid_color, valid_mode, Choice, Result};
use crate::layout::Layout;
use gdk_pixbuf::{Colorspace, InterpType, Pixbuf, PixbufRotation};
use std::fs;
use std::path::{Path, PathBuf};

fn dimensions(w: i32, h: i32) -> Result<()> {
    if !(1..=32768).contains(&w)
        || !(1..=32768).contains(&h)
        || i64::from(w) * i64::from(h) > 100_000_000
    {
        return Err("Resolução ou imagem inválida (máximo de 100 megapixels).".into());
    }
    Ok(())
}
fn canvas(w: i32, h: i32, color: &str) -> Result<Pixbuf> {
    dimensions(w, h)?;
    if !valid_color(color) {
        return Err("Imagem, posicionamento ou cor de fundo inválidos.".into());
    }
    let canvas = Pixbuf::new(Colorspace::Rgb, false, 8, w, h)
        .ok_or("Não foi possível criar a imagem do fundo.")?;
    canvas.fill(u32::from_str_radix(&color[1..], 16)? << 8 | 0xff);
    Ok(canvas)
}

fn compose(source: &Pixbuf, mode: &str, width: i32, height: i32, color: &str) -> Result<Pixbuf> {
    let canvas = canvas(width, height, color)?;
    if !valid_mode(mode) {
        return Err("Imagem, posicionamento ou cor de fundo inválidos.".into());
    }
    let (sw, sh) = (source.width(), source.height());
    dimensions(sw, sh)?;
    if mode == "tile" {
        let (mut w, mut h) = (sw.min(width), sh.min(height));
        source.composite(
            &canvas,
            0,
            0,
            w,
            h,
            0.,
            0.,
            1.,
            1.,
            InterpType::Nearest,
            255,
        );
        while w < width {
            let n = w.min(width - w);
            canvas.copy_area(0, 0, n, h, &canvas, w, 0);
            w += n;
        }
        while h < height {
            let n = h.min(height - h);
            canvas.copy_area(0, 0, width, n, &canvas, 0, h);
            h += n;
        }
        return Ok(canvas);
    }
    let (dw, dh) = match mode {
        "stretch" => (width, height),
        "fit" | "zoom" => {
            let (sx, sy) = (
                f64::from(width) / f64::from(sw),
                f64::from(height) / f64::from(sh),
            );
            let scale = if mode == "fit" {
                sx.min(sy)
            } else {
                sx.max(sy)
            };
            (
                (f64::from(sw) * scale).round().max(1.) as i32,
                (f64::from(sh) * scale).round().max(1.) as i32,
            )
        }
        _ => (sw, sh),
    };
    let (x, y) = ((width - dw) / 2, (height - dh) / 2);
    source.composite(
        &canvas,
        0.max(x),
        0.max(y),
        width.min(dw),
        height.min(dh),
        f64::from(x),
        f64::from(y),
        f64::from(dw) / f64::from(sw),
        f64::from(dh) / f64::from(sh),
        if mode == "center" {
            InterpType::Nearest
        } else {
            InterpType::Bilinear
        },
        255,
    );
    Ok(canvas)
}

fn load(path: &Path) -> Result<Pixbuf> {
    let (_, w, h) = Pixbuf::file_info(path)
        .ok_or_else(|| format!("Não foi possível ler a imagem: {}", path.display()))?;
    dimensions(w, h)?;
    Pixbuf::from_file(path)
        .map_err(|e| format!("Falha ao abrir a imagem: {e}"))?
        .apply_embedded_orientation()
        .ok_or_else(|| "Não foi possível orientar a imagem.".into())
}

fn desktop(choice: &Choice, layout: &Layout) -> Result<Pixbuf> {
    let (w, h) = (layout.width, layout.height);
    dimensions(w, h)?;
    if choice.monitors.is_empty() {
        return compose(&load(&choice.source)?, &choice.mode, w, h, &choice.color);
    }
    if !valid_color(&choice.color) || layout.monitors.is_empty() {
        return Err("Configuração de tela inválida.".into());
    }
    let dest = canvas(w, h, &choice.color)?;
    for m in &layout.monitors {
        dimensions(m.width, m.height)?;
        if m.x < 0 || m.y < 0 || m.x > w - m.width || m.y > h - m.height {
            return Err(format!("Monitor fora da área da tela: {}", m.name).into());
        }
        let (source, mode, color, rotation) =
            match choice.monitors.iter().find(|c| c.name == m.name) {
                Some(c) => (&c.source, &c.mode, &c.color, c.rotation.as_str()),
                None => (&choice.source, &choice.mode, &choice.color, "auto"),
            };
        if source.as_os_str().is_empty() {
            return Err(format!("Selecione uma imagem para o monitor {}.", m.name).into());
        }
        let source = load(source)?;
        let rotated = match rotation {
            "auto" | "0" => source,
            angle => source
                .rotate_simple(match angle {
                    "90" => PixbufRotation::Clockwise,
                    "180" => PixbufRotation::Upsidedown,
                    "270" => PixbufRotation::Counterclockwise,
                    _ => return Err("Rotação inválida.".into()),
                })
                .ok_or("Não foi possível girar a imagem.")?,
        };
        let tile = compose(&rotated, mode, m.width, m.height, color)?;
        tile.copy_area(0, 0, m.width, m.height, &dest, m.x, m.y);
    }
    Ok(dest)
}

pub fn prepare(home: &Path, choice: &Choice, layout: &Layout) -> Result<PathBuf> {
    // The Motif image loader needs opaque RGBA to preserve RGB channels.
    let encoded = desktop(choice, layout)?
        .add_alpha(false, 0, 0, 0)
        .map_err(|_| "Não foi possível preparar a imagem para o EMWM.")?;
    let bytes = encoded
        .save_to_bufferv("png", &[])
        .map_err(|e| format!("Falha ao preparar o fundo: {e}"))?;
    let hash = glib::compute_checksum_for_data(glib::ChecksumType::Sha256, &bytes)
        .ok_or("Falha ao preparar o fundo.")?;
    let directory = resource_file(home)?.with_file_name("wallpapers");
    fs::create_dir_all(&directory)?;
    let output = directory.join(format!("{hash}.png"));
    glib::file_set_contents_full(
        &output,
        &bytes,
        glib::FileSetContentsFlags::CONSISTENT,
        0o600,
    )
    .map_err(|e| format!("Falha ao salvar o fundo preparado: {e}"))?;
    Ok(output)
}
