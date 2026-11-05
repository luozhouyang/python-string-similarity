//! PyO3 bindings backing the `strsimpy` Python package.

mod algorithms;
mod sift4;
mod utils;
mod weighted;

use pyo3::prelude::*;

#[pymodule(gil_used = true)]
fn _strsimpy(module: &Bound<'_, PyModule>) -> PyResult<()> {
    algorithms::register(module)?;
    weighted::register(module)?;
    sift4::register(module)?;
    Ok(())
}
