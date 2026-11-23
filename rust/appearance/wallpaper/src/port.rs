// SPDX-License-Identifier: GPL-3.0-only
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyInt};

const MAX_PIXELS: usize = 40_000_000;

fn checked_area(width: usize, height: usize) -> Option<usize> {
    if !(1..=16384).contains(&width) || !(1..=16384).contains(&height) {
        return None;
    }
    width.checked_mul(height).filter(|n| *n <= MAX_PIXELS)
}

fn invalid_size() -> PyErr {
    PyValueError::new_err("Resolução inválida ou imagem maior que 40 megapixels.")
}

#[pyfunction]
pub fn dimensions(width: &Bound<'_, PyAny>, height: &Bound<'_, PyAny>) -> PyResult<(usize, usize)> {
    let int = width.py().get_type::<PyInt>();
    let converted_width = int.call1((width,))?;
    let converted_height = int.call1((height,))?;
    let w = converted_width.extract().map_err(|_| invalid_size())?;
    let h = converted_height.extract().map_err(|_| invalid_size())?;
    checked_area(w, h).ok_or_else(invalid_size)?;
    Ok((w, h))
}

fn blend(background: u8, source: u8, alpha: u8) -> u8 {
    let a = u32::from(alpha);
    ((u32::from(background) * (255 - a) + u32::from(source) * a + 127) / 255) as u8
}

/// Produce an opaque RGB image, matching Pillow paste(source, mask=source).
/// Source bytes are packed RGBA; target bytes are written once, row by row.
#[pyfunction]
pub fn paste_rgba<'py>(
    py: Python<'py>,
    source: &Bound<'_, PyBytes>,
    size: (usize, usize),
    target: (usize, usize),
    background: [u8; 3],
    tile: bool,
) -> PyResult<Bound<'py, PyBytes>> {
    let (sw, sh) = size;
    let (w, h) = target;
    let src = source.as_bytes();
    if checked_area(sw, sh).is_none_or(|a| a * 4 != src.len()) {
        return Err(invalid_size());
    }
    let area = checked_area(w, h).ok_or_else(invalid_size)?;
    let dx = (w as isize - sw as isize).div_euclid(2);
    let dy = (h as isize - sh as isize).div_euclid(2);
    PyBytes::new_with(py, area * 3, |out| {
        for (y, row) in out.chunks_exact_mut(w * 3).enumerate() {
            let sy = if tile {
                y % sh
            } else {
                (y as isize - dy) as usize
            };
            for (x, pixel) in row.chunks_exact_mut(3).enumerate() {
                let sx = if tile {
                    x % sw
                } else {
                    (x as isize - dx) as usize
                };
                if sx >= sw || sy >= sh {
                    pixel.copy_from_slice(&background);
                } else {
                    let i = 4 * (sy * sw + sx);
                    for c in 0..3 {
                        pixel[c] = blend(background[c], src[i + c], src[i + 3]);
                    }
                }
            }
        }
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alpha_is_rounded_like_pillow() {
        assert_eq!(blend(0, 255, 128), 128);
        assert_eq!(blend(255, 0, 128), 127);
        assert_eq!(blend(25, 90, 0), 25);
        assert_eq!(blend(25, 90, 255), 90);
    }
    #[test]
    fn image_limits() {
        assert_eq!(checked_area(8000, 5000), Some(MAX_PIXELS));
        assert!(checked_area(8000, 5001).is_none());
        assert!(checked_area(16385, 1).is_none());
    }
}
