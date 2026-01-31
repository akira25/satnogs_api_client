use crate::filters::*;
use crate::json::*;
use pyo3::prelude::*;
use std::time::Duration;
use ureq::Agent;

/// JSON-API-Client for the SatNOGs Network
///
/// This struct provides an API-Client with handy methods to access SatNOGs
/// Network JSON-API.
///
/// Some general words on the plural functions (e.g. get_observation*s*):
/// It might happen, that an API-call does not give you the whole data set, as
/// SatNOGs uses pagination. Currently, this isn't implemented in the client.
///
/// With plural-functions, you can also hand in a filter-struct, that will
/// narrow down your search query to the server.
#[pyclass]
pub struct APIClient {
    agent: ureq::Agent,
    api_url: String,
}

// Design decision: get_somethingS enforce the usage of filters, to
// incentify reduction of load on server
#[pymethods]
impl APIClient {
    #[new]
    pub fn new(api_url: String) -> APIClient {
        // Append trailing '/' if necessary
        let mut api_url = api_url;
        if !api_url.ends_with("/") {
            api_url += "/";
        }

        let config = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(60)))
            .build();

        let agent: Agent = config.into();

        APIClient { agent, api_url }
    }

    pub fn get_station(&self, id: u64) -> Result<Station, PyUreqError> {
        let station =  self
            .agent
            .get(self.api_url.clone() + format!("stations/{id}").as_str())
            .call()?
            .body_mut()
            .read_json::<Station>()?;
        Ok(station)
    }

    /*pub fn get_stations(&self, f: StationFilter) -> Result<Vec<Station>, PyUreqError> {
        return self
            .agent
            .get(self.api_url.clone() + "stations/")
            .query_pairs(f.into_vec())
            .call()?
            .body_mut()
            .read_json::<Vec<Station>>();
    }

    pub fn get_job(&self, id: u64) -> Result<Job, PyUreqError> {
        return self
            .agent
            .get(self.api_url.clone() + format!("jobs/{id}").as_str())
            .call()?
            .body_mut()
            .read_json::<Job>();
    }

    pub fn get_jobs(&self) -> Result<Vec<Job>, PyUreqError> {
        return self
            .agent
            .get(self.api_url.clone() + "jobs/")
            .call()?
            .body_mut()
            .read_json::<Vec<Job>>();
    }

    pub fn get_observation(&self, id: u64) -> Result<Observation, PyUreqError> {
        return self
            .agent
            .get(self.api_url.clone() + format!("observations/{id}").as_str())
            .call()?
            .body_mut()
            .read_json::<Observation>();
    }

    pub fn get_observations(&self) -> Result<Vec<Observation>, PyUreqError> {
        return self
            .agent
            .get(self.api_url.clone() + "observations/")
            .call()?
            .body_mut()
            .read_json::<Vec<Observation>>();
    }

    pub fn get_observations_paginated(&self) -> Result<Vec<Observation>, PyUreqError> {
        todo!()
    }*/
}

use pyo3::exceptions::PyRuntimeError;
use pyo3::PyErr;

pub struct PyUreqError(ureq::Error);

impl From<ureq::Error> for PyUreqError {
    fn from(other: ureq::Error) -> Self {
        Self(other)
    }
    //fn from(err: ureq::Error) -> PyErr {
    //    PyRuntimeError::new_err(err.to_string())
    //}
}

impl From<PyUreqError> for PyErr {
    fn from(error: PyUreqError) -> Self {
        PyRuntimeError::new_err(error.0.to_string())
    }
}

// impl From<OtherError> for MyOtherError {
//     fn from(other: OtherError) -> Self {
//         Self(other)
//     }
// }
