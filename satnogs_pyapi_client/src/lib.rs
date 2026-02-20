use pyo3::prelude::*;
use std::time::Duration;
use ureq::Agent;

use satnogs_apiclient::api_client::APIClient;

static REQUEST_TIMEOUT: u64 = 30;

mod station;
use crate::station::*;
mod station_filter;
use crate::station_filter::*;

mod job;
use crate::job::*;
mod job_filter;
use crate::job_filter::*;

mod observation;
use crate::observation::*;
mod observation_filter;
use crate::observation_filter::*;

#[pyclass(name = "APIClient")]
pub struct PyAPIClient {
    i: APIClient,
}

#[pymethods]
impl PyAPIClient {
    #[new]
    fn new(mut api_url: String) -> Self {
        if !api_url.ends_with("/") {
            api_url.push('/');
        }

        let conf = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(REQUEST_TIMEOUT)));

        Self {
            i: APIClient::new(conf, api_url)
        }
    }

    fn get_station(&self, id: u64) -> PyStation {
        PyStation {
            i: self.i.get_station(id).unwrap(),
        }
    }

    fn get_stations(&self, f: PyStationFilter) -> PyResult<Vec<PyStation>> {
        let stations = self.i.get_stations(f.into()).unwrap();

        Ok(stations.into_iter().map(|i| PyStation { i }).collect())
    }

    fn get_job(&self, id: u64) -> PyJob {
        PyJob {
            i: self.i.get_job(id).unwrap(),
        }
    }

    fn get_jobs(&self, f: PyJobFilter) -> PyResult<Vec<PyJob>> {
        let jobs = self.i.get_jobs(f.into()).unwrap();

        Ok(jobs.into_iter().map(|i| PyJob { i }).collect())
    }

    fn get_observation(&self, id: u64) -> PyObservation {
        PyObservation {
            i: self.i.get_observation(id).unwrap(),
        }
    }

    fn get_observations(&self, f: PyObservationFilter) -> PyResult<Vec<PyObservation>> {
        let obs = self.i.get_observations(f.into()).unwrap();

        Ok(obs.into_iter().map(|i| PyObservation { i }).collect())
    }
}

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
    m.add_class::<PyAPIClient>()?;

    m.add_class::<PyAntenna>()?;
    m.add_class::<PyJob>()?;
    m.add_class::<PyObservation>()?;
    m.add_class::<PyStation>()?;

    m.add_class::<PyJobFilter>()?;
    m.add_class::<PyObservationFilter>()?;
    m.add_class::<PyStationFilter>()?;

    Ok(())
}
