use crate::filters::*;
use crate::json::*;
use log::debug;
use regex::Regex;
use serde::Serialize;
use std::fmt::Debug;
use std::path::Path;
use ureq::Agent;
use ureq::Error;
use ureq::typestate::AgentScope;
use ureq::unversioned::multipart;

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
#[derive(Debug, Clone)]
pub struct APIClient {
    pub agent: ureq::Agent,
    pub api_url: String,
    pub api_token: Option<String>,
}

/// A struct containing station information, that get sent with every heartbeat
/// to the network
#[derive(Debug, Serialize, Clone)]
pub struct BasicStationInfo {
    pub ground_station: u32,
    pub lat: f32,
    pub lon: f32,
    pub alt: u32,
}

impl BasicStationInfo {
    pub fn into_vec(self) -> Vec<(String, String)> {
        let value = serde_json::to_value(self).unwrap();

        value
            .as_object()
            .unwrap()
            .iter()
            .filter_map(|(k, v)| {
                if v.is_null() {
                    None
                } else {
                    Some((k.clone(), v.to_string().trim_matches('"').to_string()))
                }
            })
            .collect()
    }
}

pub enum UploadType {
    Demoddata,
    Waterfall,
    Audio,
}

impl ToString for UploadType {
    fn to_string(&self) -> String {
        match self {
            UploadType::Demoddata => "demoddata".to_string(),
            UploadType::Waterfall => "waterfall".to_string(),
            UploadType::Audio => "payload".to_string(),
        }
    }
}

// Design decision: get_somethingS enforce the usage of filters, to
// incentify reduction of load on server
impl APIClient {
    pub fn new(
        conf: ureq::config::ConfigBuilder<AgentScope>,
        mut api_url: String,
        api_token: Option<String>,
    ) -> APIClient {
        // Append trailing '/' if necessary
        if !api_url.ends_with("/") {
            api_url.push('/');
        }

        let config = conf.http_status_as_error(false).build();
        let agent: Agent = config.into();

        APIClient {
            agent,
            api_url: api_url,
            api_token,
        }
    }

    pub fn get_station(&self, id: u64) -> Result<Station, Error> {
        self.get(&format!("stations/{id}"))
    }

    pub fn get_stations(&self, f: StationFilter) -> Result<Vec<Station>, Error> {
        self.get_with_query("stations/", f)
    }

    pub fn get_job(&self, id: u64) -> Result<Job, Error> {
        self.get(&format!("jobs/{id}"))
    }

    pub fn get_jobs(&self, f: JobFilter) -> Result<Vec<Job>, Error> {
        self.get_with_query("jobs/", f)
    }

    /// Same result as in ```get_jobs()```, though this get function performs
    /// the heartbeat feature for the network. In the GET-request, there are
    /// basic information included about the station. In networks standard
    /// configuration, a heartbeat needs to be performed once a minute.
    pub fn get_jobs_heartbeat(
        &self,
        f: JobFilter,
        info: BasicStationInfo,
    ) -> Result<Vec<Job>, Error> {
        let mut json_aggregator: Vec<Job> = Vec::new();
        let mut next_page = "https://next-cursor-link.example.org".to_string();

        if self.api_token.is_none() {
            eprintln!("No API-Token given on initialisation. Can not perform this operation");
            // ToDo: Make reason more clear by not mis-using this generic error type
            return Err(Error::ConnectionFailed);
        }

        let mut resp = self
            .agent
            .get(format!("{}{}", self.api_url, "jobs/"))
            .query_pairs(f.into_vec())
            .query_pairs(info.into_vec())
            .header(
                "Authorization",
                format!("Token {}", self.api_token.clone().unwrap()),
            )
            .call()?;

        while !next_page.is_empty() {
            let b = resp.body_mut();
            let s = b.read_to_string()?;
            debug!("{s}");
            let json: Vec<Job> = serde_json::from_str(&s)?;
            json_aggregator.extend(json);

            //search for link-header
            match resp.headers().get("Link") {
                Some(s) => {
                    // was there a next-link?
                    match self.get_next_cursor_url(s.to_str().unwrap()) {
                        Some(link) => {
                            next_page = link;
                            resp = self.agent.get(next_page.clone()).call()?;
                        }
                        None => {
                            next_page.clear();
                        }
                    }
                }
                None => {
                    next_page.clear();
                }
            }
        }

        Ok(json_aggregator)
    }

    pub fn get_observation(&self, id: u64) -> Result<Observation, Error> {
        self.get(&format!("observations/{id}"))
    }

    pub fn get_observations(&self, f: ObservationFilter) -> Result<Vec<Observation>, Error> {
        self.get_with_query("observations/", f)
    }

    /// Generalised function call for retrieving objects via id
    fn get<T>(&self, path: &str) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        self.agent
            .get(format!("{}{}", self.api_url, path))
            .call()?
            .body_mut()
            .read_json()
    }

    /// Filters the next-cursor-url from the Link-Header
    fn get_next_cursor_url(&self, s: &str) -> Option<String> {
        let re = Regex::new(r#"<([^>]+)>; rel="next""#).ok()?;
        re.captures(s)?.get(1).map(|m| m.as_str().to_string())
    }

    /// Generalised function call for retrieving filtered objects
    fn get_with_query<T, F>(&self, path: &str, filter: F) -> Result<Vec<T>, Error>
    where
        T: serde::de::DeserializeOwned,
        F: QueryParameters + Debug,
    {
        let mut json_aggregator: Vec<T> = Vec::new();
        let mut next_page = "https://next-cursor-link.example.org".to_string();

        let mut resp = self
            .agent
            .get(format!("{}{}", self.api_url, path))
            .query_pairs(filter.into_vec())
            .call()?;

        while !next_page.is_empty() {
            let b = resp.body_mut();
            let json: Vec<T> = b.read_json()?;
            json_aggregator.extend(json);

            //search for link-header
            match resp.headers().get("Link") {
                Some(s) => {
                    // was there a next-link?
                    match self.get_next_cursor_url(s.to_str().unwrap()) {
                        Some(link) => {
                            next_page = link;
                            resp = self.agent.get(next_page.clone()).call()?;
                        }
                        None => {
                            next_page.clear();
                        }
                    }
                }
                None => {
                    next_page.clear();
                }
            }
        }

        Ok(json_aggregator)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::filters;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_something() {
        let api_url = "https://network.satnogs.org/api/".to_string();
        let conf = Agent::config_builder();
        let client = APIClient::new(conf, api_url, None);

        let f = filters::ObservationFilter {
            status: None,
            ground_station: None,
            start: Some(
                DateTime::parse_from_rfc3339("2026-02-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            end: Some(
                DateTime::parse_from_rfc3339("2026-02-01T00:20:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            transmitter_uuid: None,
            transmitter_mode: None,
            transmitter_type: None,
            waterfall_status: None,
            vetted_status: None,
            vetted_user: None,
            observer: None,
            sat_id: None,
            start__lt: None,
            end__gt: None,
            observation_id: None,
            norad_cat_id: None,
        };
        let obs = client.get_observations(f).unwrap();

        assert_eq!(obs.len(), 37)
    }

    #[test]
    fn test_get_next_cursor_url() {
        let header = r#"
        <https://network.satnogs.org/api/observations/?cursor=abc>; rel="next",
        <https://network.satnogs.org/api/observations/>; rel="prev"
        "#;

        let api_url = "https://network.satnogs.org/api/".to_string();

        let conf = Agent::config_builder();
        let client = APIClient::new(conf, api_url, None);

        let s = client.get_next_cursor_url(header).unwrap();

        assert_eq!(
            s,
            "https://network.satnogs.org/api/observations/?cursor=abc".to_string()
        )
    }
}
