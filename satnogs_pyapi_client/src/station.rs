use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use satnogs_libapiclient::json::*;

#[pyclass(name = "Station")]
pub struct PyStation {
    pub(crate) i: Station,
}

#[pymethods]
impl PyStation {
    //fn __repr__(&self) -> String {
    //    format!(
    //        "Station(name={!r}, latitude={}, longitude={})",
    //        self.i.name,
    //        self.i.latitude,
    //        self.i.longitude
    //    )
    //}
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.i)
    }

    #[new]
    pub fn new(
        id: u32,
        name: String,
        altitude: i32,
        min_horizon: u32,
        lat: f32,
        lng: f32,
        qthlocator: String,
        // antenna: Vec<Antenna>,
        created: DateTime<Utc>,
        last_seen: Option<DateTime<Utc>>,
        status: String,
        observations: u64,
        future_observations: u32,
        description: String,
        client_version: String,
        target_utilization: Option<u32>,
        image: String,
        // success_rate: SuccessRate,
        owner: String,
    ) -> Self {
        Self {
            i: Station {
                id,
                name,
                altitude,
                min_horizon,
                lat,
                lng,
                qthlocator,
                antenna: Vec::new(),
                created,
                last_seen,
                status,
                observations,
                future_observations,
                description,
                client_version,
                target_utilization,
                image,
                success_rate: SuccessRate::Int(50),
                owner,
            },
        }
    }

    // ToDo: Use the great hack from filters.rs: create tuples of fieldname + value and print then.
    //fn __str__(&self) -> String {
    //    format!(
    //        "Station ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
    //        self.i.id,
    //        self.i.name,
    //        self.i.altitude,
    //        self.i.min_horizon,
    //        self.i.lat,
    //        self.i.lng,
    //        self.i.qthlocator,
    //        // self.i.antenna,
    //        self.i.created,
    //        // self.i.last_seen,
    //        self.i.status,
    //        self.i.observations,
    //        self.i.future_observations,
    //        self.i.description,
    //        self.i.client_version,
    //        // self.i.target_utilization,
    //        self.i.image,
    //        // self.i.success_rate,
    //        self.i.owner,
    //    )
    //}

    #[getter]
    fn get_id(&self) -> u32 {
        self.i.id
    }
    #[getter]
    fn get_name(&self) -> String {
        self.i.name.clone()
    }
    #[getter]
    fn get_altitude(&self) -> i32 {
        self.i.altitude
    }
    #[getter]
    fn get_min_horizon(&self) -> u32 {
        self.i.min_horizon
    }
    #[getter]
    fn get_lat(&self) -> f32 {
        self.i.lat
    }
    #[getter]
    fn get_lng(&self) -> f32 {
        self.i.lng
    }
    #[getter]
    fn get_qthlocator(&self) -> String {
        self.i.qthlocator.clone()
    }
    //#[getter]
    //fn get_antenna(&self) -> Vec<Antenna> {
    //    self.i.antenna.clone()
    //}
    #[getter]
    fn get_created(&self) -> DateTime<Utc> {
        self.i.created.clone()
    }
    #[getter]
    fn get_last_seen(&self) -> Option<DateTime<Utc>> {
        self.i.last_seen.clone()
    }
    #[getter]
    fn get_status(&self) -> String {
        self.i.status.clone()
    }
    #[getter]
    fn get_observations(&self) -> u64 {
        self.i.observations
    }
    #[getter]
    fn get_future_observations(&self) -> u32 {
        self.i.future_observations
    }
    #[getter]
    fn get_description(&self) -> String {
        self.i.description.clone()
    }
    #[getter]
    fn get_client_version(&self) -> String {
        self.i.client_version.clone()
    }
    #[getter]
    fn get_target_utilization(&self) -> Option<u32> {
        self.i.target_utilization
    }
    #[getter]
    fn get_image(&self) -> String {
        self.i.image.clone()
    }
    //#[getter]
    //fn get_success_rate(&self) -> Option<u64> {
    //    self.i.success_rate
    //}
    #[getter]
    fn get_owner(&self) -> String {
        self.i.owner.clone()
    }

    #[setter]
    fn set_id(&mut self, v: u32) -> PyResult<()> {
        self.i.id = v;
        Ok(())
    }
    #[setter]
    fn set_name(&mut self, v: String) -> PyResult<()> {
        self.i.name = v;
        Ok(())
    }
    #[setter]
    fn set_altitude(&mut self, v: i32) -> PyResult<()> {
        self.i.altitude = v;
        Ok(())
    }
    #[setter]
    fn set_min_horizon(&mut self, v: u32) -> PyResult<()> {
        self.i.min_horizon = v;
        Ok(())
    }
    #[setter]
    fn set_lat(&mut self, v: f32) -> PyResult<()> {
        self.i.lat = v;
        Ok(())
    }
    #[setter]
    fn set_lng(&mut self, v: f32) -> PyResult<()> {
        self.i.lng = v;
        Ok(())
    }
    #[setter]
    fn set_qthlocator(&mut self, v: String) -> PyResult<()> {
        self.i.qthlocator = v;
        Ok(())
    }
    //#[setter]
    //fn set_antenna(&mut self, v: Vec<Antenna>) -> PyResult<()> {
    //    self.i.antenna = v;
    //    Ok(())
    //}
    #[setter]
    fn set_created(&mut self, v: DateTime<Utc>) -> PyResult<()> {
        self.i.created = v;
        Ok(())
    }
    #[setter]
    fn set_last_seen(&mut self, v: Option<DateTime<Utc>>) -> PyResult<()> {
        self.i.last_seen = v;
        Ok(())
    }
    #[setter]
    fn set_status(&mut self, v: String) -> PyResult<()> {
        self.i.status = v;
        Ok(())
    }
    #[setter]
    fn set_observations(&mut self, v: u64) -> PyResult<()> {
        self.i.observations = v;
        Ok(())
    }
    #[setter]
    fn set_future_observations(&mut self, v: u32) -> PyResult<()> {
        self.i.future_observations = v;
        Ok(())
    }
    #[setter]
    fn set_description(&mut self, v: String) -> PyResult<()> {
        self.i.description = v;
        Ok(())
    }
    #[setter]
    fn set_client_version(&mut self, v: String) -> PyResult<()> {
        self.i.client_version = v;
        Ok(())
    }
    #[setter]
    fn set_target_utilization(&mut self, v: Option<u32>) -> PyResult<()> {
        self.i.target_utilization = v;
        Ok(())
    }
    #[setter]
    fn set_image(&mut self, v: String) -> PyResult<()> {
        self.i.image = v;
        Ok(())
    }
    //#[setter]
    //fn set_success_rate(&mut self, v: SuccessRate) -> PyResult<()> {
    //    self.i.success_rate = v;
    //    Ok(())
    //}
    #[setter]
    fn set_owner(&mut self, v: String) -> PyResult<()> {
        self.i.owner = v;
        Ok(())
    }
}

#[pyclass(name = "Antenna")]
pub struct PyAntenna {
    pub(crate) i: Antenna,
}

#[pymethods]
impl PyAntenna {
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.i)
    }

    #[new]
    pub fn new(
        frequency: u64,
        frequency_max: u64,
        band: String,
        antenna_type: String,
        antenna_type_name: String,
    ) -> Self {
        Self {
            i: Antenna {
                frequency,
                frequency_max,
                band,
                antenna_type,
                antenna_type_name,
            },
        }
    }

    #[getter]
    fn get_frequency(&self) -> u64 {
        self.i.frequency
    }
    #[getter]
    fn get_frequency_max(&self) -> u64 {
        self.i.frequency_max
    }
    #[getter]
    fn get_band(&self) -> String {
        self.i.band.clone()
    }
    #[getter]
    fn get_antenna_type(&self) -> String {
        self.i.antenna_type.clone()
    }
    #[getter]
    fn get_antenna_type_name(&self) -> String {
        self.i.antenna_type_name.clone()
    }

    #[setter]
    fn set_frequency(&mut self, v: u64) -> PyResult<()> {
        self.i.frequency = v;
        Ok(())
    }
    #[setter]
    fn set_frequency_max(&mut self, v: u64) -> PyResult<()> {
        self.i.frequency_max = v;
        Ok(())
    }
    #[setter]
    fn set_band(&mut self, v: String) -> PyResult<()> {
        self.i.band = v;
        Ok(())
    }
    #[setter]
    fn set_antenna_type(&mut self, v: String) -> PyResult<()> {
        self.i.antenna_type = v;
        Ok(())
    }
    #[setter]
    fn set_antenna_type_name(&mut self, v: String) -> PyResult<()> {
        self.i.antenna_type_name = v;
        Ok(())
    }
}

impl From<PyAntenna> for Antenna {
    fn from(py: PyAntenna) -> Self {
        py.i
    }
}

#[pyclass(name = "SuccessRate")]
pub struct PySuccessRate {
    pub(crate) i: SuccessRate,
}

#[pymethods]
impl PySuccessRate {
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.i)
    }
}

impl From<PySuccessRate> for SuccessRate {
    fn from(py: PySuccessRate) -> Self {
        py.i
    }
}
