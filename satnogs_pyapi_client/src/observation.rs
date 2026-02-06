use pyo3::prelude::*;
use satnogs_libapiclient::json::*;

#[pyclass(name = "Observation")]
pub struct PyObservation {
    pub(super) i: Observation,
}

#[pymethods]
impl PyObservation {
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.i)
    }
}
