use crate::filters::*;
use crate::json::*;
use regex::Regex;
use ureq::Error;

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

        APIClient {
            agent,
            api_url: api_url,
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

    fn get_next_cursor_url(&self, s: &str) -> Option<String> {
        let re = Regex::new(r#"<([^>]+)>; rel="next""#).ok()?;
        re.captures(s)?.get(1).map(|m| m.as_str().to_string())
    }

    /// Generalised function call for retrieving filtered objects
    fn get_with_query<T, F>(&self, path: &str, filter: F) -> Result<Vec<T>, Error>
    where
        T: serde::de::DeserializeOwned,
        F: QueryParameters,
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

    #[test]
    fn test_something() {
        let api_url = "https://network.satnogs.org/api/".to_string();
        let agent = ureq::Agent::new_with_defaults();
        let client = APIClient { agent, api_url };

        let f = filters::ObservationFilter {
            status: None,
            ground_station: None,
            start: Some("2026-02-01T00:00:00Z".to_string()),
            end: Some("2026-02-01T00:20:00Z".to_string()),
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
        let agent = ureq::Agent::new_with_defaults();
        let client = APIClient { agent, api_url };

        let s = client.get_next_cursor_url(header).unwrap();

        assert_eq!(
            s,
            "https://network.satnogs.org/api/observations/?cursor=abc".to_string()
        )
    }
}
