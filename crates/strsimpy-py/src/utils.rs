use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;

/// Converts any Python mapping (a `dict`, `Counter`, `MappingProxyType`,
/// `UserDict`, ...) into a `HashMap`.
///
/// The profile-based methods used to accept arbitrary mappings; extracting a
/// concrete `HashMap` directly would reject read-only and user-defined mapping
/// wrappers, so iterate the mapping protocol instead.
pub(crate) fn extract_profile_usize(
    profile: &Bound<'_, PyAny>,
) -> PyResult<HashMap<String, usize>> {
    extract_profile(profile)
}

/// `f64` counterpart of [`extract_profile_usize`].
pub(crate) fn extract_profile_f64(profile: &Bound<'_, PyAny>) -> PyResult<HashMap<String, f64>> {
    extract_profile(profile)
}

fn extract_profile<T>(profile: &Bound<'_, PyAny>) -> PyResult<HashMap<String, T>>
where
    for<'a, 'py> T: FromPyObject<'a, 'py>,
{
    let mut result = HashMap::new();
    for item in profile.call_method0("items")?.try_iter()? {
        let (key, value) = item?.extract::<(String, T)>()?;
        result.insert(key, value);
    }
    Ok(result)
}

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
