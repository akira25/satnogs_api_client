use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use satnogs_apiclient::filters::*;

#[pyclass(name = "JobFilter")]
#[derive(Clone)]
pub struct PyJobFilter {
    pub(crate) i: JobFilter,
}

#[allow(non_snake_case)]
#[pymethods]
impl PyJobFilter {
    fn __repr__(&self) -> String {
        format!(
            "JobFilter(status={:?}, ground_station={:?}, start={:?}, end={:?}, transmitter_uuid={:?}, transmitter_mode={:?}, transmitter_type={:?}, observer={:?}, sat_id={:?}, start__lt={:?}, end__gt={:?}, observation_id={:?}, norad_cat_id={:?})",
            self.i.status,
            self.i.ground_station,
            self.i.start,
            self.i.end,
            self.i.transmitter_uuid,
            self.i.transmitter_mode,
            self.i.transmitter_type,
            self.i.observer,
            self.i.sat_id,
            self.i.start__lt,
            self.i.end__gt,
            self.i.observation_id,
            self.i.norad_cat_id,
        )
    }

    #[new]
    #[pyo3(signature = (
        status=None, ground_station=None, start=None, end=None, transmitter_uuid=None,
        transmitter_mode=None, transmitter_type=None, observer=None, sat_id=None,
        start__lt=None, end__gt=None, observation_id=None, norad_cat_id=None,))]
    pub fn new(
        status: Option<String>,
        ground_station: Option<u32>,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        transmitter_uuid: Option<String>,
        transmitter_mode: Option<String>,
        transmitter_type: Option<String>,
        observer: Option<String>,
        sat_id: Option<String>,
        start__lt: Option<DateTime<Utc>>,
        end__gt: Option<DateTime<Utc>>,
        observation_id: Option<u64>,
        norad_cat_id: Option<u32>,
    ) -> Self {
        Self {
            i: JobFilter {
                status,
                ground_station,
                start,
                end,
                transmitter_uuid,
                transmitter_mode,
                transmitter_type,
                observer,
                sat_id,
                start__lt,
                end__gt,
                observation_id,
                norad_cat_id,
            },
        }
    }

    #[getter]
    fn get_status(&self) -> Option<String> {
        self.i.status.clone()
    }
    #[getter]
    fn get_ground_station(&self) -> Option<u32> {
        self.i.ground_station
    }
    #[getter]
    fn get_start(&self) -> Option<DateTime<Utc>> {
        self.i.start
    }
    #[getter]
    fn get_end(&self) -> Option<DateTime<Utc>> {
        self.i.end
    }
    #[getter]
    fn get_transmitter_uuid(&self) -> Option<String> {
        self.i.transmitter_uuid.clone()
    }
    #[getter]
    fn get_transmitter_mode(&self) -> Option<String> {
        self.i.transmitter_mode.clone()
    }
    #[getter]
    fn get_transmitter_type(&self) -> Option<String> {
        self.i.transmitter_type.clone()
    }
    #[getter]
    fn get_observer(&self) -> Option<String> {
        self.i.observer.clone()
    }
    #[getter]
    fn get_sat_id(&self) -> Option<String> {
        self.i.sat_id.clone()
    }
    #[getter]
    fn get_start__lt(&self) -> Option<DateTime<Utc>> {
        self.i.start__lt
    }
    #[getter]
    fn get_end__gt(&self) -> Option<DateTime<Utc>> {
        self.i.end__gt
    }
    #[getter]
    fn get_observation_id(&self) -> Option<u64> {
        self.i.observation_id
    }
    #[getter]
    fn get_norad_cat_id(&self) -> Option<u32> {
        self.i.norad_cat_id
    }

    #[setter]
    fn set_status(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.status = v;
        Ok(())
    }
    #[setter]
    fn set_ground_station(&mut self, v: Option<u32>) -> PyResult<()> {
        self.i.ground_station = v;
        Ok(())
    }
    #[setter]
    fn set_start(&mut self, v: Option<DateTime<Utc>>) -> PyResult<()> {
        self.i.start = v;
        Ok(())
    }
    #[setter]
    fn set_end(&mut self, v: Option<DateTime<Utc>>) -> PyResult<()> {
        self.i.end = v;
        Ok(())
    }
    #[setter]
    fn set_transmitter_uuid(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.transmitter_uuid = v;
        Ok(())
    }
    #[setter]
    fn set_transmitter_mode(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.transmitter_mode = v;
        Ok(())
    }
    #[setter]
    fn set_transmitter_type(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.transmitter_type = v;
        Ok(())
    }
    #[setter]
    fn set_observer(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.observer = v;
        Ok(())
    }
    #[setter]
    fn set_sat_id(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.sat_id = v;
        Ok(())
    }
    #[setter]
    fn set_start__lt(&mut self, v: Option<DateTime<Utc>>) -> PyResult<()> {
        self.i.start__lt = v;
        Ok(())
    }
    #[setter]
    fn set_end__gt(&mut self, v: Option<DateTime<Utc>>) -> PyResult<()> {
        self.i.end__gt = v;
        Ok(())
    }
    #[setter]
    fn set_observation_id(&mut self, v: Option<u64>) -> PyResult<()> {
        self.i.observation_id = v;
        Ok(())
    }
    #[setter]
    fn set_norad_cat_id(&mut self, v: Option<u32>) -> PyResult<()> {
        self.i.norad_cat_id = v;
        Ok(())
    }
}

impl From<PyJobFilter> for JobFilter {
    fn from(py: PyJobFilter) -> Self {
        py.i
    }
}
