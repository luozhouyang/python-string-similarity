//! Python wrapper for weighted Levenshtein distance.

use pyo3::prelude::*;
use strsimpy_core::weighted_levenshtein::{self as core, WeightedCosts};

fn module_function(py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
    Ok(py.import("strsimpy._strsimpy")?.getattr(name)?.unbind())
}

#[pyfunction]
fn _default_insertion_cost(_c: &str) -> f64 {
    1.0
}

#[pyfunction]
fn _default_deletion_cost(_c: &str) -> f64 {
    1.0
}

#[pyfunction]
fn _default_substitution_cost(_a: &str, _b: &str) -> f64 {
    1.0
}

struct PythonCosts<'a, 'py> {
    py: Python<'py>,
    substitution: Option<&'a Py<PyAny>>,
    insertion: Option<&'a Py<PyAny>>,
    deletion: Option<&'a Py<PyAny>>,
}

fn call_char_cost(py: Python<'_>, function: &Py<PyAny>, c: char) -> PyResult<f64> {
    function.bind(py).call1((c.to_string(),))?.extract::<f64>()
}

fn call_substitution_cost(py: Python<'_>, function: &Py<PyAny>, a: char, b: char) -> PyResult<f64> {
    function
        .bind(py)
        .call1((a.to_string(), b.to_string()))?
        .extract::<f64>()
}

impl WeightedCosts for PythonCosts<'_, '_> {
    type Error = PyErr;

    fn insertion_cost(&self, c: char) -> Result<f64, Self::Error> {
        match self.insertion {
            Some(function) => call_char_cost(self.py, function, c),
            None => Ok(1.0),
        }
    }

    fn deletion_cost(&self, c: char) -> Result<f64, Self::Error> {
        match self.deletion {
            Some(function) => call_char_cost(self.py, function, c),
            None => Ok(1.0),
        }
    }

    fn substitution_cost(&self, a: char, b: char) -> Result<f64, Self::Error> {
        match self.substitution {
            Some(function) => call_substitution_cost(self.py, function, a, b),
            None => Ok(1.0),
        }
    }
}

/// Levenshtein distance with user-defined insertion, deletion and substitution
/// costs.
#[pyclass(module = "strsimpy.weighted_levenshtein")]
#[derive(Default)]
pub struct WeightedLevenshtein {
    substitution_cost_fn: Option<Py<PyAny>>,
    insertion_cost_fn: Option<Py<PyAny>>,
    deletion_cost_fn: Option<Py<PyAny>>,
}

#[pymethods]
impl WeightedLevenshtein {
    #[new]
    #[pyo3(signature = (substitution_cost_fn=None, insertion_cost_fn=None, deletion_cost_fn=None))]
    fn new(
        substitution_cost_fn: Option<Py<PyAny>>,
        insertion_cost_fn: Option<Py<PyAny>>,
        deletion_cost_fn: Option<Py<PyAny>>,
    ) -> Self {
        Self {
            substitution_cost_fn,
            insertion_cost_fn,
            deletion_cost_fn,
        }
    }

    fn distance(&self, py: Python<'_>, s0: &str, s1: &str) -> PyResult<f64> {
        let costs = PythonCosts {
            py,
            substitution: self.substitution_cost_fn.as_ref(),
            insertion: self.insertion_cost_fn.as_ref(),
            deletion: self.deletion_cost_fn.as_ref(),
        };
        core::distance_with_costs(s0, s1, &costs)
    }

    #[getter]
    fn substitution_cost_fn(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match &self.substitution_cost_fn {
            Some(function) => Ok(function.clone_ref(py)),
            None => module_function(py, "_default_substitution_cost"),
        }
    }

    #[setter]
    fn set_substitution_cost_fn(&mut self, function: Py<PyAny>) {
        self.substitution_cost_fn = Some(function);
    }

    #[getter]
    fn insertion_cost_fn(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match &self.insertion_cost_fn {
            Some(function) => Ok(function.clone_ref(py)),
            None => module_function(py, "_default_insertion_cost"),
        }
    }

    #[setter]
    fn set_insertion_cost_fn(&mut self, function: Py<PyAny>) {
        self.insertion_cost_fn = Some(function);
    }

    #[getter]
    fn deletion_cost_fn(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match &self.deletion_cost_fn {
            Some(function) => Ok(function.clone_ref(py)),
            None => module_function(py, "_default_deletion_cost"),
        }
    }

    #[setter]
    fn set_deletion_cost_fn(&mut self, function: Py<PyAny>) {
        self.deletion_cost_fn = Some(function);
    }
}

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(_default_insertion_cost, module)?)?;
    module.add_function(wrap_pyfunction!(_default_deletion_cost, module)?)?;
    module.add_function(wrap_pyfunction!(_default_substitution_cost, module)?)?;
    module.add_class::<WeightedLevenshtein>()?;
    Ok(())
}
