// SPDX-License-Identifier: GPL-3.0-only
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt};

const RANGES: [(&str, i64, i64); 5] = [
    ("duration_ms", 50, 2000),
    ("fade_ms", 0, 1000),
    ("shadow_radius", 1, 64),
    ("shadow_offset", -32, 64),
    ("max_fps", 15, 240),
];

fn bounded(value: &Bound<'_, PyAny>, low: i64, high: i64) -> bool {
    value.is_instance_of::<PyInt>()
        && !value.is_instance_of::<PyBool>()
        && value
            .extract::<i64>()
            .is_ok_and(|n| (low..=high).contains(&n))
}

#[pyfunction]
pub fn validate<'py>(values: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyDict>> {
    // dict() accepts both a mapping and an iterable of pairs, as does dict.update().
    let supplied_object;
    let supplied = if let Ok(dict) = values.cast::<PyDict>() {
        dict
    } else {
        supplied_object = values.py().get_type::<PyDict>().call1((values,))?;
        supplied_object.cast::<PyDict>()?
    };
    let result = PyDict::new(values.py());
    result.set_item("backend", "auto")?;
    result.set_item("effect", "zoom")?;
    result.set_item("duration_ms", 220)?;
    result.set_item("fade_ms", 120)?;
    result.set_item("shadows", true)?;
    result.set_item("shadow_radius", 16)?;
    result.set_item("shadow_offset", 6)?;
    result.set_item("shadow_opacity", 0.45)?;
    result.set_item("vsync", true)?;
    result.set_item("max_fps", 60)?;
    for (key, options, message) in [
        (
            "backend",
            &["auto", "glx", "xrender"][..],
            "Backend de composição inválido.",
        ),
        (
            "effect",
            &["none", "zoom", "twist", "flip"][..],
            "Efeito de animação inválido.",
        ),
    ] {
        if let Some(v) = supplied.get_item(key)? {
            if !v
                .extract::<String>()
                .is_ok_and(|s| options.contains(&s.as_str()))
            {
                return Err(PyValueError::new_err(message));
            }
            result.set_item(key, v)?;
        }
    }
    for (key, low, high) in RANGES {
        if let Some(v) = supplied.get_item(key)? {
            if !bounded(&v, low, high) {
                return Err(PyValueError::new_err(format!(
                    "{key}: use um valor entre {low} e {high}."
                )));
            }
            result.set_item(key, v)?;
        }
    }
    for key in ["shadows", "vsync"] {
        if let Some(v) = supplied.get_item(key)? {
            if !v.is_instance_of::<PyBool>() {
                return Err(PyValueError::new_err(format!("{key}: valor inválido.")));
            }
            result.set_item(key, v)?;
        }
    }
    if let Some(v) = supplied.get_item("shadow_opacity")? {
        if v.is_instance_of::<PyBool>()
            || !(v.is_instance_of::<PyInt>() || v.is_instance_of::<PyFloat>())
            || !(0.0..=1.0).contains(&v.extract::<f64>()?)
        {
            return Err(PyValueError::new_err(
                "A intensidade da sombra deve estar entre 0 e 100%.",
            ));
        }
        result.set_item("shadow_opacity", v)?;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::RANGES;
    #[test]
    fn shadow_offset_accepts_both_signs() {
        assert!(RANGES.contains(&("shadow_offset", -32, 64)));
    }
}
