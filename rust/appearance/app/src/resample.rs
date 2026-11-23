// SPDX-License-Identifier: GPL-3.0-only
//! Separable Lanczos-3 sampling with the quantization used for GTK appearance
//! previews created with Pillow. Filter premultiplied channels before compositing.
use anyhow::{ensure, Result};
use image::{Rgba, RgbaImage};
use std::collections::VecDeque;
use std::f64::consts::PI;

const SCALE: i64 = 1 << 22;
const ROUND: i64 = SCALE / 2;

struct Filter {
    start: usize,
    coefficients: Vec<i64>,
}
fn sinc(value: f64) -> f64 {
    if value == 0. {
        1.
    } else {
        let phase = value * PI;
        phase.sin() / phase
    }
}
fn filters(source: u32, output: u32, from: f64, to: f64) -> Vec<Filter> {
    // Pillow accepts its crop rectangle as single-precision input coordinates.
    let from = f64::from(from as f32);
    let to = f64::from(to as f32);
    let ratio = (to - from) / f64::from(output);
    let stretch = ratio.max(1.);
    let radius = 3. * stretch;
    (0..output)
        .map(|i| {
            let center = from + (f64::from(i) + 0.5) * ratio;
            let left = (center - radius + 0.5).max(0.) as usize;
            let right = ((center + radius + 0.5) as usize).min(source as usize);
            let mut weights: Vec<f64> = (left..right)
                .map(|p| {
                    let d = (p as f64 - center + 0.5) / stretch;
                    if (-3. ..3.).contains(&d) {
                        sinc(d) * sinc(d / 3.)
                    } else {
                        0.
                    }
                })
                .collect();
            let total = weights.iter().sum::<f64>();
            if total != 0. {
                for weight in &mut weights {
                    *weight /= total;
                }
            }
            let coefficients = weights
                .iter()
                .map(|w| (w * SCALE as f64).round() as i64)
                .collect();
            Filter {
                start: left,
                coefficients,
            }
        })
        .collect()
}
fn clipped(sum: i64) -> u8 {
    ((sum + ROUND) >> 22).clamp(0, 255) as u8
}
fn premult(p: [u8; 4]) -> [u8; 4] {
    let a = u32::from(p[3]);
    [
        ((u32::from(p[0]) * a + 127) / 255) as u8,
        ((u32::from(p[1]) * a + 127) / 255) as u8,
        ((u32::from(p[2]) * a + 127) / 255) as u8,
        p[3],
    ]
}
fn unpremult(mut p: [u8; 4]) -> [u8; 4] {
    let a = u32::from(p[3]);
    if a != 0 && a != 255 {
        for c in &mut p[..3] {
            *c = (u32::from(*c) * 255 / a).min(255) as u8;
        }
    }
    p
}
fn horizontal(image: &RgbaImage, row: u32, table: &[Filter]) -> Vec<[u8; 4]> {
    table
        .iter()
        .map(|f| {
            let mut sum = [0; 4];
            for (dx, &k) in f.coefficients.iter().enumerate().filter(|(_, k)| **k != 0) {
                let pixel = premult(image.get_pixel((f.start + dx) as u32, row).0);
                for (v, p) in sum.iter_mut().zip(pixel) {
                    *v += i64::from(p) * k;
                }
            }
            sum.map(clipped)
        })
        .collect()
}

pub fn resize(image: RgbaImage, mode: &str, w: u32, h: u32) -> Result<RgbaImage> {
    let (iw, ih) = image.dimensions();
    let (sw, sh) = (f64::from(iw), f64::from(ih));
    let dst = f64::from(w) / f64::from(h);
    let (w, h) = if mode == "fit" && sw / sh > dst {
        (w, (sh / sw * f64::from(w)).round_ties_even() as u32)
    } else if mode == "fit" && sw / sh < dst {
        ((sw / sh * f64::from(h)).round_ties_even() as u32, h)
    } else {
        (w, h)
    };
    ensure!(w > 0 && h > 0, "height and width must be > 0");
    let (crop_w, crop_h) = if mode == "zoom" && sw / sh > dst {
        (dst * sh, sh)
    } else if mode == "zoom" && sw / sh < dst {
        (sw, sw / dst)
    } else {
        (sw, sh)
    };
    if iw == w && ih == h && sw == crop_w && sh == crop_h {
        return Ok(image);
    }
    let x = (sw - crop_w) / 2.;
    let y = (sh - crop_h) / 2.;
    let xs = filters(iw, w, x, x + crop_w);
    let ys = filters(ih, h, y, y + crop_h);
    let mut out = RgbaImage::new(w, h);
    // A bounded cache avoids allocating width × source-height when scaling a
    // very tall, narrow image onto a very wide target.
    let mut rows: VecDeque<(u32, Vec<[u8; 4]>)> = VecDeque::new();
    let mut accum = vec![[0i64; 4]; w as usize];
    for (y, filter) in ys.iter().enumerate() {
        accum.fill([0; 4]);
        for (dy, &k) in filter
            .coefficients
            .iter()
            .enumerate()
            .filter(|(_, k)| **k != 0)
        {
            let sy = (filter.start + dy) as u32;
            let index = rows.iter().position(|(i, _)| *i == sy).unwrap_or_else(|| {
                if rows.len() == 16 {
                    rows.pop_front();
                }
                rows.push_back((sy, horizontal(&image, sy, &xs)));
                rows.len() - 1
            });
            for (sums, pixel) in accum.iter_mut().zip(&rows[index].1) {
                for (sum, c) in sums.iter_mut().zip(pixel) {
                    *sum += i64::from(*c) * k;
                }
            }
        }
        for (x, sums) in accum.iter().enumerate() {
            out.put_pixel(x as u32, y as u32, Rgba(unpremult(sums.map(clipped))));
        }
    }
    Ok(out)
}
