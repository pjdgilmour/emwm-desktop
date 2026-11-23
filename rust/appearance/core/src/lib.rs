// SPDX-License-Identifier: GPL-3.0-only
#![forbid(unsafe_code)]
use pyo3::prelude::*;

mod port;

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(port::palette_colors, module)?)?;
    module.add_function(wrap_pyfunction!(port::valid_name, module)?)?;
    module.add_function(wrap_pyfunction!(port::edit_ini, module)?)?;
    module.add_function(wrap_pyfunction!(port::managed_block, module)?)?;
    Ok(())
}
