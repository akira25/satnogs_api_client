use pyo3::prelude::*;
use satnogs_libapiclient::filters::*;

#[pyclass(name = "StationFilter")]
#[derive(Clone)]
pub struct PyStationFilter {
    pub(crate) i: StationFilter,
}

#[pymethods]
impl PyStationFilter {
    #[new]
    pub fn new(
        status: Option<String>,
        name: Option<String>,
        client_version: Option<String>,
    ) -> Self {
        Self {
            i: StationFilter {
                status,
                name,
                client_version,
            },
        }
    }

    #[getter]
    fn get_status(&self) -> Option<String> {
        self.i.status.clone()
    }
    #[setter]
    fn set_status(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.status = v;
        Ok(())
    }

    #[getter]
    fn get_name(&self) -> Option<String> {
        self.i.name.clone()
    }
    #[setter]
    fn set_name(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.name = v;
        Ok(())
    }

    #[getter]
    fn get_client_version(&self) -> Option<String> {
        self.i.name.clone()
    }
    #[setter]
    fn set_client_version(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.name = v;
        Ok(())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.i)
    }
}

impl From<PyStationFilter> for StationFilter {
    fn from(py: PyStationFilter) -> Self {
        py.i
    }
}
