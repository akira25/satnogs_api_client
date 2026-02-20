use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use satnogs_apiclient::json::*;

#[pyclass(name = "Job")]
pub struct PyJob {
    pub(crate) i: Job,
}

#[pymethods]
impl PyJob {
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.i)
    }

    #[new]
    fn new(
        id: u64,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        ground_station: u64,
        tle0: String,
        tle1: String,
        tle2: String,
        frequency: u64,
        mode: Option<String>,
        transmitter: String,
        baud: Option<f32>,
        max_altitude: f32,
        norad_cat_id: u32,
    ) -> Self {
        Self {
            i: Job {
                id,
                start,
                end,
                ground_station,
                tle0,
                tle1,
                tle2,
                frequency,
                mode,
                transmitter,
                baud,
                max_altitude,
                norad_cat_id,
            },
        }
    }

    #[getter]
    fn get_id(&self) -> u64 {
        self.i.id
    }
    #[getter]
    fn get_start(&self) -> DateTime<Utc> {
        self.i.start
    }
    #[getter]
    fn get_end(&self) -> DateTime<Utc> {
        self.i.end
    }
    #[getter]
    fn get_ground_station(&self) -> u64 {
        self.i.ground_station
    }
    #[getter]
    fn get_tle0(&self) -> String {
        self.i.tle0.clone()
    }
    #[getter]
    fn get_tle1(&self) -> String {
        self.i.tle1.clone()
    }
    #[getter]
    fn get_tle2(&self) -> String {
        self.i.tle2.clone()
    }
    #[getter]
    fn get_frequency(&self) -> u64 {
        self.i.frequency
    }
    #[getter]
    fn get_mode(&self) -> Option<String> {
        self.i.mode.clone()
    }
    #[getter]
    fn get_transmitter(&self) -> String {
        self.i.transmitter.clone()
    }
    #[getter]
    fn get_baud(&self) -> Option<f32> {
        self.i.baud
    }
    #[getter]
    fn get_max_altitude(&self) -> f32 {
        self.i.max_altitude
    }
    #[getter]
    fn get_norad_cat_id(&self) -> u32 {
        self.i.norad_cat_id
    }

    #[setter]
    fn set_id(&mut self, v: u64) -> PyResult<()> {
        self.i.id = v;
        Ok(())
    }
    #[setter]
    fn set_start(&mut self, v: DateTime<Utc>) -> PyResult<()> {
        self.i.start = v;
        Ok(())
    }
    #[setter]
    fn set_end(&mut self, v: DateTime<Utc>) -> PyResult<()> {
        self.i.end = v;
        Ok(())
    }
    #[setter]
    fn set_ground_station(&mut self, v: u64) -> PyResult<()> {
        self.i.ground_station = v;
        Ok(())
    }
    #[setter]
    fn set_tle0(&mut self, v: String) -> PyResult<()> {
        self.i.tle0 = v;
        Ok(())
    }
    #[setter]
    fn set_tle1(&mut self, v: String) -> PyResult<()> {
        self.i.tle1 = v;
        Ok(())
    }
    #[setter]
    fn set_tle2(&mut self, v: String) -> PyResult<()> {
        self.i.tle2 = v;
        Ok(())
    }
    #[setter]
    fn set_frequency(&mut self, v: u64) -> PyResult<()> {
        self.i.frequency = v;
        Ok(())
    }
    #[setter]
    fn set_mode(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.mode = v;
        Ok(())
    }
    #[setter]
    fn set_transmitter(&mut self, v: String) -> PyResult<()> {
        self.i.transmitter = v;
        Ok(())
    }
    #[setter]
    fn set_baud(&mut self, v: Option<f32>) -> PyResult<()> {
        self.i.baud = v;
        Ok(())
    }
    #[setter]
    fn set_max_altitude(&mut self, v: f32) -> PyResult<()> {
        self.i.max_altitude = v;
        Ok(())
    }
    #[setter]
    fn set_norad_cat_id(&mut self, v: u32) -> PyResult<()> {
        self.i.norad_cat_id = v;
        Ok(())
    }
}
