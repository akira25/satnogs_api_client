use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use satnogs_libapiclient::filters::*;

#[pyclass(name = "ObservationFilter")]
#[derive(Clone)]
pub struct PyObservationFilter {
    pub(crate) i: ObservationFilter,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyObservationFilter {
    #[new]
    #[pyo3(signature = (
        status=None,
        ground_station=None,
        start=None,
        end=None,
        transmitter_uuid=None,
        transmitter_mode=None,
        transmitter_type=None,
        waterfall_status=None,
        vetted_status=None,
        vetted_user=None,
        observer=None,
        sat_id=None,
        start__lt=None,
        end__gt=None,
        observation_id=None,
        norad_cat_id=None,
    ))]
    #[allow(non_snake_case)]
    pub fn new(
        status: Option<String>,
        ground_station: Option<u32>,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        transmitter_uuid: Option<String>,
        transmitter_mode: Option<String>,
        transmitter_type: Option<String>,
        waterfall_status: Option<String>,
        vetted_status: Option<String>,
        vetted_user: Option<String>,
        observer: Option<String>,
        sat_id: Option<String>,
        start__lt: Option<DateTime<Utc>>,
        end__gt: Option<DateTime<Utc>>,
        observation_id: Option<u64>,
        norad_cat_id: Option<u32>,
    ) -> Self {
        Self {
            i: ObservationFilter {
                status,
                ground_station,
                start,
                end,
                transmitter_uuid,
                transmitter_mode,
                transmitter_type,
                waterfall_status,
                vetted_status,
                vetted_user,
                observer,
                sat_id,
                start__lt,
                end__gt,
                observation_id,
                norad_cat_id,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "ObservationFilter(status={:?}, ground_station={:?}, start={:?}, end={:?}, transmitter_uuid={:?}, transmitter_mode={:?}, transmitter_type={:?}, waterfall_status={:?}, vetted_status={:?}, vetted_user={:?}, observer={:?}, sat_id={:?}, start__lt={:?}, end__gt={:?}, observation_id={:?}, norad_cat_id={:?})",
            self.i.status,
            self.i.ground_station,
            self.i.start,
            self.i.end,
            self.i.transmitter_uuid,
            self.i.transmitter_mode,
            self.i.transmitter_type,
            self.i.waterfall_status,
            self.i.vetted_status,
            self.i.vetted_user,
            self.i.observer,
            self.i.sat_id,
            self.i.start__lt,
            self.i.end__gt,
            self.i.observation_id,
            self.i.norad_cat_id,
        )
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
    fn get_ground_station(&self) -> Option<u32> {
        self.i.ground_station.clone()
    }
    #[setter]
    fn set_ground_station(&mut self, v: Option<u32>) -> PyResult<()> {
        self.i.ground_station = v;
        Ok(())
    }

    #[getter]
    fn get_start(&self) -> Option<DateTime<Utc>> {
        self.i.start.clone()
    }
    #[setter]
    fn set_start(&mut self, v: Option<DateTime<Utc>>) -> PyResult<()> {
        self.i.start = v;
        Ok(())
    }

    #[getter]
    fn get_end(&self) -> Option<DateTime<Utc>> {
        self.i.end.clone()
    }
    #[setter]
    fn set_end(&mut self, v: Option<DateTime<Utc>>) -> PyResult<()> {
        self.i.end = v;
        Ok(())
    }

    #[getter]
    fn get_transmitter_uuid(&self) -> Option<String> {
        self.i.transmitter_uuid.clone()
    }
    #[setter]
    fn set_transmitter_uuid(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.transmitter_uuid = v;
        Ok(())
    }

    #[getter]
    fn get_transmitter_mode(&self) -> Option<String> {
        self.i.transmitter_mode.clone()
    }
    #[setter]
    fn set_transmitter_mode(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.transmitter_mode = v;
        Ok(())
    }

    #[getter]
    fn get_transmitter_type(&self) -> Option<String> {
        self.i.transmitter_type.clone()
    }
    #[setter]
    fn set_transmitter_type(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.transmitter_type = v;
        Ok(())
    }

    #[getter]
    fn get_waterfall_status(&self) -> Option<String> {
        self.i.waterfall_status.clone()
    }
    #[setter]
    fn set_waterfall_status(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.waterfall_status = v;
        Ok(())
    }

    #[getter]
    fn get_vetted_status(&self) -> Option<String> {
        self.i.vetted_status.clone()
    }
    #[setter]
    fn set_vetted_status(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.vetted_status = v;
        Ok(())
    }

    #[getter]
    fn get_vetted_user(&self) -> Option<String> {
        self.i.vetted_user.clone()
    }
    #[setter]
    fn set_vetted_user(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.vetted_user = v;
        Ok(())
    }

    #[getter]
    fn get_observer(&self) -> Option<String> {
        self.i.observer.clone()
    }
    #[setter]
    fn set_observer(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.observer = v;
        Ok(())
    }

    #[getter]
    fn get_sat_id(&self) -> Option<String> {
        self.i.sat_id.clone()
    }
    #[setter]
    fn set_sat_id(&mut self, v: Option<String>) -> PyResult<()> {
        self.i.sat_id = v;
        Ok(())
    }

    #[getter]
    fn get_start__lt(&self) -> Option<DateTime<Utc>> {
        self.i.start__lt.clone()
    }
    #[setter]
    fn set_start__lt(&mut self, v: Option<DateTime<Utc>>) -> PyResult<()> {
        self.i.start__lt = v;
        Ok(())
    }

    #[getter]
    fn get_end__gt(&self) -> Option<DateTime<Utc>> {
        self.i.end__gt.clone()
    }
    #[setter]
    fn set_end__gt(&mut self, v: Option<DateTime<Utc>>) -> PyResult<()> {
        self.i.end__gt = v;
        Ok(())
    }

    #[getter]
    fn get_observation_id(&self) -> Option<u64> {
        self.i.observation_id.clone()
    }
    #[setter]
    fn set_observation_id(&mut self, v: Option<u64>) -> PyResult<()> {
        self.i.observation_id = v;
        Ok(())
    }

    #[getter]
    fn get_norad_cat_id(&self) -> Option<u32> {
        self.i.norad_cat_id.clone()
    }
    #[setter]
    fn set_norad_cat_id(&mut self, v: Option<u32>) -> PyResult<()> {
        self.i.norad_cat_id = v;
        Ok(())
    }
}

impl From<PyObservationFilter> for ObservationFilter {
    fn from(py: PyObservationFilter) -> Self {
        py.i
    }
}
