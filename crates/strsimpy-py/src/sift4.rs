//! Python bindings for SIFT4 and its option system.
//!
//! SIFT4 accepts user-supplied tokenizers and evaluators, so the option object
//! stores Python callables. The default (no options) path stays entirely in
//! Rust; the configurable path reuses the same generic Rust algorithm while
//! invoking the configured Python callables.

use pyo3::exceptions::{PyNotImplementedError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyInt};
use pyo3::IntoPyObjectExt;
use strsimpy_core::shingle::split_whitespace;
use strsimpy_core::sift4::{self as core, Sift4Options as Sift4OptionsTrait};

use crate::utils::to_py_number;

fn module_function(py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
    Ok(py.import("strsimpy._strsimpy")?.getattr(name)?.unbind())
}

fn static_method(py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
    Ok(py.get_type::<SIFT4Options>().getattr(name)?.unbind())
}

fn allowed_names(names: &[(&str, &str)]) -> String {
    names
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>()
        .join(", ")
}

fn resolve_option(
    py: Python<'_>,
    key: &str,
    value: &Bound<'_, PyAny>,
    names: &[(&str, &str)],
) -> PyResult<Py<PyAny>> {
    if value.is_callable() {
        return Ok(value.clone().unbind());
    }

    let error = || {
        PyValueError::new_err(format!(
            "Option {} should be callable or one of [{}]",
            key,
            allowed_names(names)
        ))
    };

    let name: String = value.extract().map_err(|_| error())?;
    for (option_name, method_name) in names {
        if *option_name == name {
            return static_method(py, method_name);
        }
    }
    Err(error())
}

/// SIFT4 options, matching the Python `SIFT4Options` API.
#[pyclass(module = "strsimpy.sift4")]
pub struct SIFT4Options {
    maxdistance: i64,
    tokenizer: Py<PyAny>,
    tokenmatcher: Py<PyAny>,
    matchingevaluator: Py<PyAny>,
    locallengthevaluator: Py<PyAny>,
    transpositioncostevaluator: Py<PyAny>,
    transpositionsevaluator: Py<PyAny>,
}

impl SIFT4Options {
    fn defaults(py: Python<'_>) -> PyResult<Self> {
        Ok(Self {
            maxdistance: 0,
            tokenizer: module_function(py, "_sift4_char_tokenizer")?,
            tokenmatcher: module_function(py, "_sift4_token_equals")?,
            matchingevaluator: module_function(py, "_sift4_one")?,
            locallengthevaluator: module_function(py, "_sift4_identity")?,
            transpositioncostevaluator: module_function(py, "_sift4_one")?,
            transpositionsevaluator: module_function(py, "_sift4_lcss_minus_trans")?,
        })
    }
}

#[pymethods]
impl SIFT4Options {
    #[new]
    #[pyo3(signature = (options=None))]
    fn new(py: Python<'_>, options: Option<&Bound<'_, PyAny>>) -> PyResult<Self> {
        let mut resolved = Self::defaults(py)?;
        let Some(options) = options else {
            return Ok(resolved);
        };

        let dictionary = options
            .cast::<PyDict>()
            .map_err(|_| PyValueError::new_err("options should be a dictionary"))?;

        for (key, value) in dictionary.iter() {
            let key: String = key
                .extract()
                .unwrap_or_else(|_| key.repr().map(|r| r.to_string()).unwrap_or_default());
            match key.as_str() {
                "maxdistance" => {
                    if !value.is_instance_of::<PyInt>() {
                        return Err(PyValueError::new_err("Option maxdistance should be int"));
                    }
                    resolved.maxdistance = value.extract::<i64>()?;
                }
                "tokenizer" => {
                    resolved.tokenizer = resolve_option(
                        py,
                        &key,
                        &value,
                        &[
                            ("ngram", "ngramtokenizer"),
                            ("wordsplit", "wordsplittokenizer"),
                            ("characterfrequency", "characterfrequencytokenizer"),
                        ],
                    )?;
                }
                "tokenmatcher" => {
                    resolved.tokenmatcher = resolve_option(
                        py,
                        &key,
                        &value,
                        &[("sift4tokenmatcher", "sift4tokenmatcher")],
                    )?;
                }
                "matchingevaluator" => {
                    resolved.matchingevaluator = resolve_option(
                        py,
                        &key,
                        &value,
                        &[("sift4matchingevaluator", "sift4matchingevaluator")],
                    )?;
                }
                "locallengthevaluator" => {
                    resolved.locallengthevaluator = resolve_option(
                        py,
                        &key,
                        &value,
                        &[
                            ("rewardlengthevaluator", "rewardlengthevaluator"),
                            ("rewardlengthevaluator2", "rewardlengthevaluator2"),
                        ],
                    )?;
                }
                "transpositioncostevaluator" => {
                    resolved.transpositioncostevaluator = resolve_option(
                        py,
                        &key,
                        &value,
                        &[(
                            "longertranspositionsaremorecostly",
                            "longertranspositionsaremorecostly",
                        )],
                    )?;
                }
                "transpositionsevaluator" => {
                    resolved.transpositionsevaluator = resolve_option(py, &key, &value, &[])?;
                }
                _ => {
                    return Err(PyValueError::new_err(format!(
                        "Option {} not recognized.",
                        key
                    )));
                }
            }
        }

        Ok(resolved)
    }

    #[getter]
    fn maxdistance(&self) -> i64 {
        self.maxdistance
    }

    #[setter]
    fn set_maxdistance(&mut self, value: i64) {
        self.maxdistance = value;
    }

    #[getter]
    fn options(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let dictionary = PyDict::new(py);
        dictionary.set_item("maxdistance", self.maxdistance)?;
        dictionary.set_item("tokenizer", self.tokenizer.bind(py))?;
        dictionary.set_item("tokenmatcher", self.tokenmatcher.bind(py))?;
        dictionary.set_item("matchingevaluator", self.matchingevaluator.bind(py))?;
        dictionary.set_item("locallengthevaluator", self.locallengthevaluator.bind(py))?;
        dictionary.set_item(
            "transpositioncostevaluator",
            self.transpositioncostevaluator.bind(py),
        )?;
        dictionary.set_item(
            "transpositionsevaluator",
            self.transpositionsevaluator.bind(py),
        )?;
        Ok(dictionary.unbind())
    }

    #[getter]
    fn tokenizer(&self, py: Python<'_>) -> Py<PyAny> {
        self.tokenizer.clone_ref(py)
    }

    #[setter]
    fn set_tokenizer(&mut self, value: Py<PyAny>) {
        self.tokenizer = value;
    }

    #[getter]
    fn tokenmatcher(&self, py: Python<'_>) -> Py<PyAny> {
        self.tokenmatcher.clone_ref(py)
    }

    #[setter]
    fn set_tokenmatcher(&mut self, value: Py<PyAny>) {
        self.tokenmatcher = value;
    }

    #[getter]
    fn matchingevaluator(&self, py: Python<'_>) -> Py<PyAny> {
        self.matchingevaluator.clone_ref(py)
    }

    #[setter]
    fn set_matchingevaluator(&mut self, value: Py<PyAny>) {
        self.matchingevaluator = value;
    }

    #[getter]
    fn locallengthevaluator(&self, py: Python<'_>) -> Py<PyAny> {
        self.locallengthevaluator.clone_ref(py)
    }

    #[setter]
    fn set_locallengthevaluator(&mut self, value: Py<PyAny>) {
        self.locallengthevaluator = value;
    }

    #[getter]
    fn transpositioncostevaluator(&self, py: Python<'_>) -> Py<PyAny> {
        self.transpositioncostevaluator.clone_ref(py)
    }

    #[setter]
    fn set_transpositioncostevaluator(&mut self, value: Py<PyAny>) {
        self.transpositioncostevaluator = value;
    }

    #[getter]
    fn transpositionsevaluator(&self, py: Python<'_>) -> Py<PyAny> {
        self.transpositionsevaluator.clone_ref(py)
    }

    #[setter]
    fn set_transpositionsevaluator(&mut self, value: Py<PyAny>) {
        self.transpositionsevaluator = value;
    }

    /// Present for compatibility with the `MetricStringDistance` base class.
    fn distance(&self, _s0: &str, _s1: &str) -> PyResult<f64> {
        Err(PyNotImplementedError::new_err(
            "SIFT4Options does not implement distance()",
        ))
    }

    #[staticmethod]
    fn ngramtokenizer(s: &str, n: usize) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        if chars.is_empty() {
            return Vec::new();
        }
        let count = chars.len().saturating_sub(n).saturating_sub(1);
        (0..count)
            .map(|i| chars[i..i + n].iter().collect())
            .collect()
    }

    #[staticmethod]
    fn wordsplittokenizer(s: &str) -> Vec<String> {
        split_whitespace(s).into_iter().map(str::to_owned).collect()
    }

    #[staticmethod]
    fn characterfrequencytokenizer(s: &str) -> Vec<usize> {
        let mut counts = [0usize; 26];
        for c in s.chars() {
            for lower in c.to_lowercase() {
                if lower.is_ascii_lowercase() {
                    counts[(lower as u8 - b'a') as usize] += 1;
                }
            }
        }
        counts.to_vec()
    }

    #[staticmethod]
    fn sift4tokenmatcher(t1: &str, t2: &str) -> PyResult<bool> {
        let max_len = t1.chars().count().max(t2.chars().count());
        if max_len == 0 {
            return Err(PyZeroDivisionError::new_err("float division by zero"));
        }
        let similarity = 1.0 - core::distance(t1, t2, 5) / max_len as f64;
        Ok(similarity > 0.7)
    }

    #[staticmethod]
    fn sift4matchingevaluator(t1: &str, t2: &str) -> PyResult<f64> {
        let max_len = t1.chars().count().max(t2.chars().count());
        if max_len == 0 {
            return Err(PyZeroDivisionError::new_err("float division by zero"));
        }
        Ok(1.0 - core::distance(t1, t2, 5) / max_len as f64)
    }

    #[staticmethod]
    fn rewardlengthevaluator(l: f64) -> f64 {
        if l < 1.0 {
            l
        } else {
            l - 1.0 / (l + 1.0)
        }
    }

    #[staticmethod]
    fn rewardlengthevaluator2(l: f64) -> f64 {
        l.powf(1.5)
    }

    #[staticmethod]
    fn longertranspositionsaremorecostly(c1: f64, c2: f64) -> f64 {
        (c2 - c1).abs() / 9.0 + 1.0
    }
}

#[pyfunction]
fn _sift4_char_tokenizer(s: &str) -> Vec<String> {
    s.chars().map(|c| c.to_string()).collect()
}

#[pyfunction]
fn _sift4_token_equals(token1: &Bound<'_, PyAny>, token2: &Bound<'_, PyAny>) -> PyResult<bool> {
    token1.eq(token2)
}

#[pyfunction]
fn _sift4_one(_token1: &Bound<'_, PyAny>, _token2: &Bound<'_, PyAny>) -> f64 {
    1.0
}

#[pyfunction]
fn _sift4_identity(value: &Bound<'_, PyAny>) -> PyResult<f64> {
    value.extract()
}

#[pyfunction]
fn _sift4_lcss_minus_trans(
    lcss: &Bound<'_, PyAny>,
    transpositions: &Bound<'_, PyAny>,
) -> PyResult<f64> {
    Ok(lcss.extract::<f64>()? - transpositions.extract::<f64>()?)
}

struct PyToken(Py<PyAny>);

struct PySift4Options<'a, 'py> {
    py: Python<'py>,
    options: &'a SIFT4Options,
}

impl Sift4OptionsTrait for PySift4Options<'_, '_> {
    type Token = PyToken;
    type Error = PyErr;

    fn max_distance(&self) -> f64 {
        self.options.maxdistance as f64
    }

    fn tokenize(&self, string: &str) -> PyResult<Vec<PyToken>> {
        let result = self.options.tokenizer.bind(self.py).call1((string,))?;
        let iterator = result.try_iter()?;
        let mut tokens = Vec::new();
        for item in iterator {
            tokens.push(PyToken(item?.unbind()));
        }
        Ok(tokens)
    }

    fn token_match(&self, token1: &PyToken, token2: &PyToken) -> PyResult<bool> {
        self.options
            .tokenmatcher
            .bind(self.py)
            .call1((
                token1.0.bind(self.py).clone(),
                token2.0.bind(self.py).clone(),
            ))?
            .extract::<bool>()
    }

    fn matching_evaluator(&self, token1: &PyToken, token2: &PyToken) -> PyResult<f64> {
        self.options
            .matchingevaluator
            .bind(self.py)
            .call1((
                token1.0.bind(self.py).clone(),
                token2.0.bind(self.py).clone(),
            ))?
            .extract::<f64>()
    }

    fn local_length_evaluator(&self, length: f64) -> PyResult<f64> {
        let argument = to_py_number(self.py, length)?;
        self.options
            .locallengthevaluator
            .bind(self.py)
            .call1((argument,))?
            .extract::<f64>()
    }

    fn transposition_cost_evaluator(&self, c1: f64, c2: f64) -> PyResult<f64> {
        let first = to_py_number(self.py, c1)?;
        let second = to_py_number(self.py, c2)?;
        self.options
            .transpositioncostevaluator
            .bind(self.py)
            .call1((first, second))?
            .extract::<f64>()
    }

    fn transpositions_evaluator(&self, lcss: f64, transpositions: f64) -> PyResult<f64> {
        let first = to_py_number(self.py, lcss)?;
        let second = to_py_number(self.py, transpositions)?;
        self.options
            .transpositionsevaluator
            .bind(self.py)
            .call1((first, second))?
            .extract::<f64>()
    }
}

/// SIFT4 string distance.
#[pyclass(module = "strsimpy.sift4")]
#[derive(Debug, Default, Clone, Copy)]
pub struct SIFT4;

#[pymethods]
impl SIFT4 {
    #[new]
    fn new() -> Self {
        Self
    }

    #[pyo3(signature = (s1, s2, maxoffset=5, options=None))]
    fn distance(
        &self,
        py: Python<'_>,
        s1: &str,
        s2: &str,
        maxoffset: i64,
        options: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Py<PyAny>> {
        let value = match options {
            None => core::distance(s1, s2, maxoffset),
            Some(options) => {
                let parsed = SIFT4Options::new(py, Some(options))?;
                let adapter = PySift4Options {
                    py,
                    options: &parsed,
                };
                core::distance_with_options(s1, s2, maxoffset, &adapter)?
            }
        };
        (value as i64).into_py_any(py)
    }
}

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<SIFT4Options>()?;
    module.add_class::<SIFT4>()?;
    module.add_function(wrap_pyfunction!(_sift4_char_tokenizer, module)?)?;
    module.add_function(wrap_pyfunction!(_sift4_token_equals, module)?)?;
    module.add_function(wrap_pyfunction!(_sift4_one, module)?)?;
    module.add_function(wrap_pyfunction!(_sift4_identity, module)?)?;
    module.add_function(wrap_pyfunction!(_sift4_lcss_minus_trans, module)?)?;
    Ok(())
}
