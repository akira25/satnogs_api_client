use pyo3::prelude::*;
use satnogs_apiclient::api_client::{BasicStationInfo, UploadType};

#[pyclass(name = "BasicStationInfo")]
#[derive(Clone)]
pub struct PyBasicStationInfo {
    pub(crate) i: BasicStationInfo,
}

#[pymethods]
impl PyBasicStationInfo {
    #[new]
    #[pyo3(signature = (ground_station, lat = 0.0, lon = 0.0, alt = 0))]
    pub fn new(ground_station: u32, lat: f32, lon: f32, alt: u32) -> Self {
        Self {
            i: BasicStationInfo {
                ground_station,
                lat,
                lon,
                alt,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "BasicStationInfo(ground_station={:?}, lat={:?}, lon={:?}, alt={:?})",
            self.i.ground_station, self.i.lat, self.i.lon, self.i.alt
        )
    }

    #[getter]
    fn get_ground_station(&self) -> u32 {
        self.i.ground_station
    }
    #[setter]
    fn set_ground_station(&mut self, v: u32) -> PyResult<()> {
        self.i.ground_station = v;
        Ok(())
    }

    #[getter]
    fn get_lat(&self) -> f32 {
        self.i.lat
    }
    #[setter]
    fn set_lat(&mut self, v: f32) -> PyResult<()> {
        self.i.lat = v;
        Ok(())
    }

    #[getter]
    fn get_lon(&self) -> f32 {
        self.i.lon
    }
    #[setter]
    fn set_lon(&mut self, v: f32) -> PyResult<()> {
        self.i.lon = v;
        Ok(())
    }

    #[getter]
    fn get_alt(&self) -> u32 {
        self.i.alt
    }
    #[setter]
    fn set_alt(&mut self, v: u32) -> PyResult<()> {
        self.i.alt = v;
        Ok(())
    }
}

impl From<PyBasicStationInfo> for BasicStationInfo {
    fn from(py: PyBasicStationInfo) -> Self {
        py.i
    }
}
