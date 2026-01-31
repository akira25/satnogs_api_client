use crate::filters::*;
use crate::json::*;

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
pub struct APIClient {
    pub agent: ureq::Agent,
    pub api_url: String,
}

// Design decision: get_somethingS enforce the usage of filters, to
// incentify reduction of load on server
impl APIClient {
    pub fn new(agent: ureq::Agent, mut api_url: String) -> APIClient {
        // Append trailing '/' if necessary
        if !api_url.ends_with("/") {
            api_url.push('/');
        }

        APIClient { agent, api_url: api_url }
    }

    pub fn get_station(&self, id: u64) -> Result<Station, ureq::Error> {
        let station =  self
            .agent
            .get(self.api_url.clone() + format!("stations/{id}").as_str())
            .call()?
            .body_mut()
            .read_json::<Station>()?;
        Ok(station)
    }

    pub fn get_stations(&self, f: StationFilter) -> Result<Vec<Station>, ureq::Error> {
        return self
            .agent
            .get(self.api_url.clone() + "stations/")
            .query_pairs(f.into_vec())
            .call()?
            .body_mut()
            .read_json::<Vec<Station>>();
    }

    pub fn get_job(&self, id: u64) -> Result<Job, ureq::Error> {
        return self
            .agent
            .get(self.api_url.clone() + format!("jobs/{id}").as_str())
            .call()?
            .body_mut()
            .read_json::<Job>();
    }

    pub fn get_jobs(&self) -> Result<Vec<Job>, ureq::Error> {
        return self
            .agent
            .get(self.api_url.clone() + "jobs/")
            .call()?
            .body_mut()
            .read_json::<Vec<Job>>();
    }

    pub fn get_observation(&self, id: u64) -> Result<Observation, ureq::Error> {
        return self
            .agent
            .get(self.api_url.clone() + format!("observations/{id}").as_str())
            .call()?
            .body_mut()
            .read_json::<Observation>();
    }

    pub fn get_observations(&self) -> Result<Vec<Observation>, ureq::Error> {
        return self
            .agent
            .get(self.api_url.clone() + "observations/")
            .call()?
            .body_mut()
            .read_json::<Vec<Observation>>();
    }

    pub fn get_observations_paginated(&self) -> Result<Vec<Observation>, ureq::Error> {
        todo!()
    }
}
