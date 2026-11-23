// SPDX-License-Identifier: GPL-3.0-only
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

fn valid_monitor(monitor: &str) -> bool {
    !monitor.is_empty()
        && monitor.chars().count() <= 256
        && !monitor
            .chars()
            .any(|c| c < ' ' || matches!(c, '\\' | '[' | ']'))
}

#[pyfunction]
pub fn validate<'py>(values: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyDict>> {
    let result = PyDict::new(values.py());
    // {**values} accepts mappings and rejects a list of pairs.
    let extra;
    let supplied = if let Ok(dict) = values.cast::<PyDict>() {
        dict
    } else {
        let kwargs = PyDict::new(values.py());
        let Ok(keys) = values.getattr("keys") else {
            return Err(PyTypeError::new_err(format!(
                "'{}' object is not a mapping",
                values.get_type().name()?
            )));
        };
        for key in keys.call0()?.try_iter()? {
            let key = key?;
            kwargs.set_item(&key, values.get_item(&key)?)?;
        }
        extra = kwargs;
        &extra
    };
    match supplied.get_item("monitor")? {
        Some(monitor) => {
            if !monitor
                .cast::<PyString>()
                .is_ok_and(|s| valid_monitor(&s.to_string_lossy()))
            {
                return Err(PyValueError::new_err("Monitor inválido."));
            }
            result.set_item("monitor", monitor)?;
        }
        None => result.set_item("monitor", "primary")?,
    }
    match supplied.get_item("anchor")? {
        Some(anchor) => {
            if !anchor.eq("left")? && !anchor.eq("right")? {
                return Err(PyValueError::new_err(
                    "Escolha o alinhamento à esquerda ou à direita.",
                ));
            }
            result.set_item("anchor", anchor)?;
        }
        None => result.set_item("anchor", "left")?,
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monitor_length_counts_characters() {
        assert!(valid_monitor(&"á".repeat(256)));
        assert!(!valid_monitor(&"á".repeat(257)));
        assert!(!valid_monitor("DP-1\nother=1"));
    }
}
