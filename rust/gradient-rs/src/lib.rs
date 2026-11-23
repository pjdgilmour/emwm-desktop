// SPDX-License-Identifier: GPL-3.0-only
#![forbid(unsafe_code)]
use pyo3::prelude::*;

mod model;
mod port;

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(port::validate, module)?)?;
    module.add_function(wrap_pyfunction!(port::resources, module)?)?;
    Ok(())
}
