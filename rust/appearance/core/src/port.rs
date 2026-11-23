// SPDX-License-Identifier: GPL-3.0-only
#[path = "../../shared/text.rs"]
mod text;
use text::{Setting, FIELDS};

use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

#[pyfunction]
pub fn palette_colors<'py>(name: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyDict>> {
    let Some(colors) = name
        .extract::<String>()
        .ok()
        .and_then(|n| text::palette(&n))
    else {
        name.hash()?;
        return Err(PyKeyError::new_err(name.clone().unbind()));
    };
    let result = PyDict::new(name.py());
    for (key, value) in FIELDS.into_iter().zip(colors) {
        result.set_item(key, value)?;
    }
    Ok(result)
}

#[pyfunction]
pub fn valid_name<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    let invalid = || PyValueError::new_err("Nome de tema inválido.");
    let name = value.cast::<PyString>().map_err(|_| invalid())?;
    let text = name.to_string_lossy();
    if !text::valid_name(&text) {
        return Err(invalid());
    }
    Ok(value.clone())
}

#[pyfunction]
pub fn edit_ini(
    text: &Bound<'_, PyString>,
    section: &str,
    values: &Bound<'_, PyDict>,
) -> PyResult<String> {
    let mut settings = Vec::new();
    for (k, v) in values {
        let key: String = k.extract()?;
        settings.push(Setting {
            lowercase: key.to_lowercase(),
            key,
            value: v.str()?.to_str()?.to_owned(),
        });
    }
    let lines: Vec<String> = text.call_method1("splitlines", (true,))?.extract()?;
    Ok(text::edit_ini(lines, section, &settings))
}
#[pyfunction(signature = (text, name, content, comment="#"))]
pub fn managed_block(text: &str, name: &str, content: &str, comment: &str) -> String {
    text::managed_block(text, name, content, comment)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn managed_sections_replace_in_place_with_one_footer() {
        assert_eq!(
            managed_block("x\n# BEGIN N\nold\n# END N\n", "N", "new\n", "#"),
            "x\n\n# BEGIN N\nnew\n# END N\n"
        );
    }
}
