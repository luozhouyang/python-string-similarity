use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;

/// Converts an `f64` into a Python number, using `int` for integral values.
///
/// The Python implementation accumulates several SIFT4 values as integers, and
/// some user-supplied evaluators may rely on receiving an `int`.
pub(crate) fn to_py_number(py: Python<'_>, value: f64) -> PyResult<Py<PyAny>> {
    if value.is_finite()
        && value.fract() == 0.0
        && value >= i64::MIN as f64
        && value <= i64::MAX as f64
    {
        (value as i64).into_py_any(py)
    } else {
        value.into_py_any(py)
    }
}

/// Mirrors the original Python return types for integral edit distances:
/// identical strings return the float `0.0`, everything else returns an `int`.
pub(crate) fn zero_or_int(py: Python<'_>, strings_equal: bool, value: f64) -> PyResult<Py<PyAny>> {
    if strings_equal {
        0.0f64.into_py_any(py)
    } else {
        (value as usize).into_py_any(py)
    }
}
