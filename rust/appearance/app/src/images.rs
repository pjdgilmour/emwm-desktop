// SPDX-License-Identifier: GPL-3.0-only
use anyhow::{bail, ensure, Result};
use image::{DynamicImage, ImageDecoder, ImageFormat, ImageReader, Rgb, RgbImage, RgbaImage};
use std::io::{BufRead, Cursor, Seek};
use std::path::Path;

pub const MAX_BYTES: usize = 64 * 1024 * 1024;
pub const MODES: [&str; 5] = ["zoom", "stretch", "fit", "center", "tile"];

pub fn dimensions(w: u32, h: u32) -> Result<(u32, u32)> {
    ensure!(
        (1..=16384).contains(&w)
            && (1..=16384).contains(&h)
            && u64::from(w) * u64::from(h) <= 40_000_000,
        "Resolução inválida ou imagem maior que 40 megapixels."
    );
    Ok((w, h))
}
pub fn decode<R: BufRead + Seek>(reader: ImageReader<R>, png_only: bool) -> Result<RgbaImage> {
    let reader = reader.with_guessed_format()?;
    if png_only && reader.format() != Some(ImageFormat::Png) {
        bail!("O auxiliar aceita somente imagens PNG preparadas pela ferramenta.");
    }
    let mut decoder = reader
        .into_decoder()
        .map_err(|e| anyhow::anyhow!("Não foi possível ler a imagem: {e}"))?;
    let (w, h) = decoder.dimensions();
    dimensions(w, h)?;
    let orientation = decoder.orientation()?;
    let mut image = DynamicImage::from_decoder(decoder)?;
    image.apply_orientation(orientation);
    Ok(image.into_rgba8())
}
pub fn png(img: RgbImage) -> Result<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());
    DynamicImage::ImageRgb8(img).write_to(&mut buffer, ImageFormat::Png)?;
    let bytes = buffer.into_inner();
    ensure!(
        bytes.len() <= MAX_BYTES,
        "Imagem preparada maior que 64 MiB."
    );
    Ok(bytes)
}
pub fn normalize_png(data: &[u8]) -> Result<Vec<u8>> {
    ensure!(
        !data.is_empty() && data.len() <= MAX_BYTES,
        "Tamanho de imagem inválido (máximo 64 MiB)."
    );
    let rgba = decode(ImageReader::new(Cursor::new(data)), true)?;
    png(DynamicImage::ImageRgba8(rgba).into_rgb8())
}

fn background(color: &str) -> Result<[u8; 3]> {
    let raw = color
        .strip_prefix('#')
        .filter(|s| s.len() == 6 && s.bytes().all(|b| b.is_ascii_hexdigit()))
        .ok_or_else(|| anyhow::anyhow!("Cor de fundo inválida."))?;
    Ok([
        u8::from_str_radix(&raw[..2], 16)?,
        u8::from_str_radix(&raw[2..4], 16)?,
        u8::from_str_radix(&raw[4..], 16)?,
    ])
}
pub fn compose(img: RgbaImage, mode: &str, w: u32, h: u32, color: &str) -> Result<RgbImage> {
    ensure!(MODES.contains(&mode), "Modo de fundo inválido.");
    dimensions(w, h)?;
    let bg = background(color)?;
    let img = match mode {
        "stretch" | "zoom" | "fit" => crate::resample::resize(img, mode, w, h)?,
        _ => img,
    };
    let (sw, sh) = img.dimensions();
    let dx = (i64::from(w) - i64::from(sw)).div_euclid(2);
    let dy = (i64::from(h) - i64::from(sh)).div_euclid(2);
    Ok(RgbImage::from_fn(w, h, |x, y| {
        let (sx, sy) = if mode == "tile" {
            (x % sw, y % sh)
        } else {
            ((i64::from(x) - dx) as u32, (i64::from(y) - dy) as u32)
        };
        if sx >= sw || sy >= sh {
            return Rgb(bg);
        }
        let p = img.get_pixel(sx, sy).0;
        let a = u32::from(p[3]);
        Rgb(std::array::from_fn(|i| {
            ((u32::from(bg[i]) * (255 - a) + u32::from(p[i]) * a + 127) / 255) as u8
        }))
    }))
}
pub fn prepare(path: &Path, mode: &str, w: u32, h: u32, color: &str) -> Result<Vec<u8>> {
    ensure!(MODES.contains(&mode), "Modo de fundo inválido.");
    dimensions(w, h)?;
    let img = decode(ImageReader::open(path)?, false)?;
    let (w, h, mode) = if matches!(mode, "zoom" | "stretch") {
        (img.width(), img.height(), "center")
    } else {
        (w, h, mode)
    };
    png(compose(img, mode, w, h, color)?)
}
