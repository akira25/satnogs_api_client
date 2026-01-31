use pyo3::prelude::*;
use std::time::Duration;
use ureq::Agent;

use satnogs_libapiclient::{api_client::APIClient, filters::*, json::*};

static REQUEST_TIMEOUT: u64 = 30;

#[pyclass(name = "APIClient")]
pub struct PyAPIClient {
    i: APIClient,
}

#[pymethods]
impl PyAPIClient {
    #[new]
    fn new(mut api_url: String) -> Self {
        let config = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(REQUEST_TIMEOUT)))
            .build();

        let agent: Agent = config.into();

        if !api_url.ends_with("/") {
            api_url.push('/');
        }

        Self {
            i: APIClient { agent, api_url: api_url },
        }
    }

    // Example for properties-method. Results in accessing field directly in python
    //#[getter]
    //fn a(&self) -> i32 {
    //    self.inner.a
    //}

    fn get_station(&self, id: u64) -> PyStation {
        PyStation {
            i: self.i.get_station(id).unwrap(),
        }
    }

    fn get_job(&self, id: u64) -> PyJob {
        PyJob {
            i: self.i.get_job(id).unwrap(),
        }
    }

    fn get_observation(&self, id: u64) -> PyObservation {
        PyObservation {
            i: self.i.get_observation(id).unwrap(),
        }
    }
}

#[pyclass(name = "Station")]
pub struct PyStation {
    i: Station,
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
    fn __repr__(&self) -> String {
        format!("{:?}", self.i)
    }
}

#[pyclass(name = "Job")]
pub struct PyJob {
    i: Job,
}

#[pymethods]
impl PyJob {
    fn __repr__(&self) -> String {
        format!("{:?}", self.i)
    }
}

#[pyclass(name = "Observation")]
pub struct PyObservation {
    i: Observation,
}

#[pymethods]
impl PyObservation {
    fn __repr__(&self) -> String {
        format!("{:?}", self.i)
    }
}


// #[pyfunction]
// fn hello() {
//     println!("Hello from the satnogs API client! This is wirtten in rust.");
// }

/// Python-Interface for the SatNOGs-Network-API. Implemented in Rust.
///
/// This module provides you an easy-to-use API-Client for usage with the SatNOGs
/// open groundstation network. It abstracts the whole process of retrieving the
/// data from you, so you can focus on getting the information you are looking
/// for.
///
/// You can request Stations, Jobs and Observations. For each category, there are
/// two kind of functions: Those without trailing 's' and those with. E.g. with
/// `get_job(id)`, you can request all data related to a specific job. With
/// `get_jobs(filter)`, you will get ALL jobs in the network, unless you filter
/// them.
#[pymodule]
fn satnogs_pyapi_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_class::<PyAPIClient>()?;
    m.add_class::<PyStation>()?;
    m.add_class::<PyJob>()?;
    m.add_class::<PyObservation>()?;

    Ok(())
}
