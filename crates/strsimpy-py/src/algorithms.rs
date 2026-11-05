//! Python wrappers for the non-SIFT4 algorithms.

use std::collections::HashMap;

use pyo3::exceptions::{PyAttributeError, PyValueError};
use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;

use strsimpy_core as core;

use crate::utils;

/// Builds a shingle profile. Exposed for the pure-Python `ShingleBased` base
/// class.
#[pyfunction]
pub(crate) fn shingle_profile(string: &str, k: usize) -> HashMap<String, usize> {
    core::shingle::profile(string, k)
}

#[pyclass(module = "strsimpy.levenshtein")]
#[derive(Debug, Default, Clone, Copy)]
pub struct Levenshtein;

#[pymethods]
impl Levenshtein {
    #[new]
    fn new() -> Self {
        Self
    }

    fn distance(&self, py: Python<'_>, s0: &str, s1: &str) -> PyResult<Py<PyAny>> {
        utils::zero_or_int(py, s0 == s1, core::levenshtein::distance(s0, s1))
    }
}

#[pyclass(module = "strsimpy.normalized_levenshtein")]
#[derive(Debug, Default, Clone, Copy)]
pub struct NormalizedLevenshtein;

#[pymethods]
impl NormalizedLevenshtein {
    #[new]
    fn new() -> Self {
        Self
    }

    fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        if name == "levenshtein" {
            Ok(Py::new(py, Levenshtein)?.into_any())
        } else {
            Err(PyAttributeError::new_err(name.to_string()))
        }
    }

    fn distance(&self, s0: &str, s1: &str) -> f64 {
        core::normalized_levenshtein::distance(s0, s1)
    }

    fn similarity(&self, s0: &str, s1: &str) -> f64 {
        core::normalized_levenshtein::similarity(s0, s1)
    }
}

#[pyclass(module = "strsimpy.damerau")]
#[derive(Debug, Default, Clone, Copy)]
pub struct Damerau;

#[pymethods]
impl Damerau {
    #[new]
    fn new() -> Self {
        Self
    }

    fn distance(&self, py: Python<'_>, s0: &str, s1: &str) -> PyResult<Py<PyAny>> {
        utils::zero_or_int(py, s0 == s1, core::damerau::distance(s0, s1))
    }
}

#[pyclass(module = "strsimpy.optimal_string_alignment")]
#[derive(Debug, Default, Clone, Copy)]
pub struct OptimalStringAlignment;

#[pymethods]
impl OptimalStringAlignment {
    #[new]
    fn new() -> Self {
        Self
    }

    fn distance(&self, py: Python<'_>, s0: &str, s1: &str) -> PyResult<Py<PyAny>> {
        utils::zero_or_int(
            py,
            s0 == s1,
            core::optimal_string_alignment::distance(s0, s1),
        )
    }
}

#[pyclass(module = "strsimpy.jaro_winkler")]
#[derive(Debug, Clone, Copy)]
pub struct JaroWinkler {
    threshold: f64,
}

impl Default for JaroWinkler {
    fn default() -> Self {
        Self {
            threshold: core::jaro_winkler::DEFAULT_THRESHOLD,
        }
    }
}

#[pymethods]
impl JaroWinkler {
    #[new]
    #[pyo3(signature = (threshold=0.7))]
    fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    fn get_threshold(&self) -> f64 {
        self.threshold
    }

    fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        if name == "threshold" {
            self.threshold.into_py_any(py)
        } else {
            Err(PyAttributeError::new_err(name.to_string()))
        }
    }

    #[getter]
    fn three(&self) -> usize {
        3
    }

    #[getter]
    fn jw_coef(&self) -> f64 {
        0.1
    }

    fn similarity(&self, s0: &str, s1: &str) -> f64 {
        core::jaro_winkler::similarity_with_threshold(s0, s1, self.threshold)
    }

    fn distance(&self, s0: &str, s1: &str) -> f64 {
        core::jaro_winkler::distance_with_threshold(s0, s1, self.threshold)
    }
}

#[pyclass(module = "strsimpy.longest_common_subsequence")]
#[derive(Debug, Default, Clone, Copy)]
pub struct LongestCommonSubsequence;

#[pymethods]
impl LongestCommonSubsequence {
    #[new]
    fn new() -> Self {
        Self
    }

    fn distance(&self, py: Python<'_>, s0: &str, s1: &str) -> PyResult<Py<PyAny>> {
        utils::zero_or_int(
            py,
            s0 == s1,
            core::longest_common_subsequence::distance(s0, s1),
        )
    }

    #[staticmethod]
    fn length(s0: &str, s1: &str) -> usize {
        core::longest_common_subsequence::length(s0, s1)
    }
}

#[pyclass(module = "strsimpy.metric_lcs")]
#[derive(Debug, Default, Clone, Copy)]
pub struct MetricLCS;

#[pymethods]
impl MetricLCS {
    #[new]
    fn new() -> Self {
        Self
    }

    fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        if name == "lcs" {
            Ok(Py::new(py, LongestCommonSubsequence)?.into_any())
        } else {
            Err(PyAttributeError::new_err(name.to_string()))
        }
    }

    fn distance(&self, s0: &str, s1: &str) -> f64 {
        core::metric_lcs::distance(s0, s1)
    }
}

#[pyclass(module = "strsimpy.ngram")]
#[derive(Debug, Clone, Copy)]
pub struct NGram {
    n: usize,
}

#[pymethods]
impl NGram {
    #[new]
    #[pyo3(signature = (n=2))]
    fn new(n: usize) -> PyResult<Self> {
        if n == 0 {
            return Err(PyValueError::new_err("n must be greater than 0"));
        }
        Ok(Self { n })
    }

    #[getter]
    fn n(&self) -> usize {
        self.n
    }

    #[setter]
    fn set_n(&mut self, n: usize) -> PyResult<()> {
        if n == 0 {
            return Err(PyValueError::new_err("n must be greater than 0"));
        }
        self.n = n;
        Ok(())
    }

    fn distance(&self, s0: &str, s1: &str) -> f64 {
        core::ngram::distance(s0, s1, self.n)
    }
}

#[pyclass(module = "strsimpy.qgram")]
#[derive(Debug, Clone, Copy)]
pub struct QGram {
    k: usize,
}

#[pymethods]
impl QGram {
    #[new]
    #[pyo3(signature = (k=3))]
    fn new(k: usize) -> Self {
        Self { k }
    }

    fn get_k(&self) -> usize {
        self.k
    }

    fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        if name == "k" {
            self.k.into_py_any(py)
        } else {
            Err(PyAttributeError::new_err(name.to_string()))
        }
    }

    fn get_profile(&self, string: &str) -> HashMap<String, usize> {
        core::shingle::profile(string, self.k)
    }

    fn distance(&self, py: Python<'_>, s0: &str, s1: &str) -> PyResult<Py<PyAny>> {
        utils::zero_or_int(py, s0 == s1, core::qgram::distance(s0, s1, self.k) as f64)
    }

    #[staticmethod]
    fn distance_profile(
        profile0: HashMap<String, usize>,
        profile1: HashMap<String, usize>,
    ) -> usize {
        core::qgram::distance_profile(&profile0, &profile1)
    }
}

#[pyclass(module = "strsimpy.cosine")]
#[derive(Debug, Clone, Copy)]
pub struct Cosine {
    k: usize,
}

#[pymethods]
impl Cosine {
    #[new]
    fn new(k: usize) -> Self {
        Self { k }
    }

    fn get_k(&self) -> usize {
        self.k
    }

    fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        if name == "k" {
            self.k.into_py_any(py)
        } else {
            Err(PyAttributeError::new_err(name.to_string()))
        }
    }

    fn get_profile(&self, string: &str) -> HashMap<String, usize> {
        core::shingle::profile(string, self.k)
    }

    fn similarity(&self, s0: &str, s1: &str) -> f64 {
        core::cosine::similarity(s0, s1, self.k)
    }

    fn distance(&self, s0: &str, s1: &str) -> f64 {
        core::cosine::distance(s0, s1, self.k)
    }

    fn similarity_profiles(
        &self,
        profile0: HashMap<String, f64>,
        profile1: HashMap<String, f64>,
    ) -> f64 {
        core::cosine::similarity_profiles(&profile0, &profile1)
    }
}

#[pyclass(module = "strsimpy.jaccard")]
#[derive(Debug, Clone, Copy)]
pub struct Jaccard {
    k: usize,
}

#[pymethods]
impl Jaccard {
    #[new]
    fn new(k: usize) -> Self {
        Self { k }
    }

    fn get_k(&self) -> usize {
        self.k
    }

    fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        if name == "k" {
            self.k.into_py_any(py)
        } else {
            Err(PyAttributeError::new_err(name.to_string()))
        }
    }

    fn get_profile(&self, string: &str) -> HashMap<String, usize> {
        core::shingle::profile(string, self.k)
    }

    fn similarity(&self, s0: &str, s1: &str) -> f64 {
        core::jaccard::similarity(s0, s1, self.k)
    }

    fn distance(&self, s0: &str, s1: &str) -> f64 {
        core::jaccard::distance(s0, s1, self.k)
    }
}

#[pyclass(module = "strsimpy.sorensen_dice")]
#[derive(Debug, Clone, Copy)]
pub struct SorensenDice {
    k: usize,
}

#[pymethods]
impl SorensenDice {
    #[new]
    #[pyo3(signature = (k=3))]
    fn new(k: usize) -> Self {
        Self { k }
    }

    fn get_k(&self) -> usize {
        self.k
    }

    fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        if name == "k" {
            self.k.into_py_any(py)
        } else {
            Err(PyAttributeError::new_err(name.to_string()))
        }
    }

    fn get_profile(&self, string: &str) -> HashMap<String, usize> {
        core::shingle::profile(string, self.k)
    }

    fn similarity(&self, s0: &str, s1: &str) -> f64 {
        core::sorensen_dice::similarity(s0, s1, self.k)
    }

    fn distance(&self, s0: &str, s1: &str) -> f64 {
        core::sorensen_dice::distance(s0, s1, self.k)
    }
}

#[pyclass(module = "strsimpy.overlap_coefficient")]
#[derive(Debug, Clone, Copy)]
pub struct OverlapCoefficient {
    k: usize,
}

#[pymethods]
impl OverlapCoefficient {
    #[new]
    #[pyo3(signature = (k=3))]
    fn new(k: usize) -> Self {
        Self { k }
    }

    fn get_k(&self) -> usize {
        self.k
    }

    fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        if name == "k" {
            self.k.into_py_any(py)
        } else {
            Err(PyAttributeError::new_err(name.to_string()))
        }
    }

    fn get_profile(&self, string: &str) -> HashMap<String, usize> {
        core::shingle::profile(string, self.k)
    }

    fn similarity(&self, s0: &str, s1: &str) -> f64 {
        core::overlap_coefficient::similarity(s0, s1, self.k)
    }

    fn distance(&self, s0: &str, s1: &str) -> f64 {
        core::overlap_coefficient::distance(s0, s1, self.k)
    }
}

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(shingle_profile, module)?)?;
    module.add_class::<Levenshtein>()?;
    module.add_class::<NormalizedLevenshtein>()?;
    module.add_class::<Damerau>()?;
    module.add_class::<OptimalStringAlignment>()?;
    module.add_class::<JaroWinkler>()?;
    module.add_class::<LongestCommonSubsequence>()?;
    module.add_class::<MetricLCS>()?;
    module.add_class::<NGram>()?;
    module.add_class::<QGram>()?;
    module.add_class::<Cosine>()?;
    module.add_class::<Jaccard>()?;
    module.add_class::<SorensenDice>()?;
    module.add_class::<OverlapCoefficient>()?;
    Ok(())
}
