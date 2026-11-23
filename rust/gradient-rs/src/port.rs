// SPDX-License-Identifier: GPL-3.0-only
use crate::model::{Gradient, COLORS};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

fn read(values: &Bound<'_, PyAny>) -> PyResult<Gradient> {
    let dict = values
        .cast::<PyDict>()
        .map_err(|_| PyValueError::new_err("Configuração de gradiente inválida."))?;
    Gradient::read(
        |key| {
            Ok(dict
                .get_item(key)?
                .map(|item| item.extract::<String>().unwrap_or_default()))
        },
        |message| PyValueError::new_err(message.to_owned()),
    )
}

#[pyfunction]
pub fn validate<'py>(values: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyDict>> {
    let gradient = read(values)?;
    let result = PyDict::new(values.py());
    result.set_item("mode", gradient.mode)?;
    for ((key, _, _), color) in COLORS.iter().zip(gradient.colors) {
        result.set_item(key, color)?;
    }
    Ok(result)
}

#[pyfunction]
pub fn resources(values: &Bound<'_, PyAny>) -> PyResult<String> {
    Ok(read(values)?.resources())
}
