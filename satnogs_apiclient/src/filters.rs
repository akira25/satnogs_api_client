use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json;
use serde_urlencoded::ser::Error;

#[derive(Debug, Serialize, Default, Clone)]
pub struct StationFilter {
    pub status: Option<String>,
    pub name: Option<String>,
    pub client_version: Option<String>,
}
impl QueryParameters for StationFilter {}

#[derive(Debug, Serialize, Default, Clone)]
#[allow(non_snake_case)]
pub struct JobFilter {
    pub status: Option<String>,
    pub ground_station: Option<u32>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub transmitter_uuid: Option<String>,
    pub transmitter_mode: Option<String>,
    pub transmitter_type: Option<String>,
    pub observer: Option<String>,
    pub sat_id: Option<String>,
    pub start__lt: Option<DateTime<Utc>>,
    pub end__gt: Option<DateTime<Utc>>,
    pub observation_id: Option<u64>,
    pub norad_cat_id: Option<u32>,
}
impl QueryParameters for JobFilter {}

#[derive(Debug, Serialize, Default, Clone)]
#[allow(non_snake_case)]
pub struct ObservationFilter {
    pub status: Option<String>,
    pub ground_station: Option<u32>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub transmitter_uuid: Option<String>,
    pub transmitter_mode: Option<String>,
    pub transmitter_type: Option<String>,
    pub waterfall_status: Option<String>,
    pub vetted_status: Option<String>,
    pub vetted_user: Option<String>,
    pub observer: Option<String>,
    pub sat_id: Option<String>,
    pub start__lt: Option<DateTime<Utc>>,
    pub end__gt: Option<DateTime<Utc>>,
    pub observation_id: Option<u64>,
    pub norad_cat_id: Option<u32>,
}
impl QueryParameters for ObservationFilter {}

pub trait QueryParameters: Serialize + Sized {
    fn to_query(&self) -> Result<String, Error> {
        serde_urlencoded::to_string(self)
    }

    /// Generates a vector of fieldnames and values to be fed into ureqs query-function
    fn into_vec(self) -> Vec<(String, String)> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_job_filter_all_vec() {
        let f = JobFilter {
            status: None,
            ground_station: Some(1860),
            start: Some(
                DateTime::parse_from_rfc3339("2026-01-31T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            end: None,
            transmitter_uuid: None,
            transmitter_mode: None,
            transmitter_type: None,
            observer: None,
            sat_id: None,
            start__lt: None,
            end__gt: None,
            observation_id: None,
            norad_cat_id: None,
        };

        let query = f.into_vec();
        assert_eq!(
            query,
            vec![
                ("ground_station".to_string(), "1860".to_string()),
                ("start".to_string(), "2026-01-31T00:00:00Z".to_string()),
            ]
        );
    }

    #[test]
    fn test_station_filter_all_vec() {
        let f = StationFilter {
            status: Some("Online".to_string()),
            name: Some("BEEGND-4".to_string()),
            client_version: Some("2.1.1".to_string()),
        };

        let query = f.into_vec();
        assert_eq!(
            query,
            vec![
                ("client_version".to_string(), "2.1.1".to_string()),
                ("name".to_string(), "BEEGND-4".to_string()),
                ("status".to_string(), "Online".to_string()),
            ]
        );
    }

    #[test]
    fn test_station_filter_partial_vec() {
        let f = StationFilter {
            status: Some("Online".to_string()),
            name: None,
            client_version: Some("2.1.1".to_string()),
        };

        let query = f.into_vec();
        assert_eq!(
            query,
            vec![
                ("client_version".to_string(), "2.1.1".to_string()),
                ("status".to_string(), "Online".to_string()),
            ]
        );
    }

    #[test]
    fn test_station_filter_all() {
        let f = StationFilter {
            status: Some("Online".to_string()),
            name: Some("BEEGND-4".to_string()),
            client_version: Some("2.1.1".to_string()),
        };

        let query = f.to_query().unwrap();
        assert_eq!(query, "status=Online&name=BEEGND-4&client_version=2.1.1");
    }

    #[test]
    fn test_station_filter_partial() {
        let f = StationFilter {
            status: Some("Online".to_string()),
            name: None,
            client_version: None,
        };

        let query = f.to_query().unwrap();
        assert_eq!(query, "status=Online");
    }

    #[test]
    fn test_station_filter_none() {
        let f = StationFilter {
            status: None,
            name: None,
            client_version: None,
        };

        let query = f.to_query().unwrap();
        assert_eq!(query, "");
    }

    #[test]
    fn test_station_filter_order() {
        let f = StationFilter {
            client_version: Some("C".to_string()),
            name: Some("B".to_string()),
            status: Some("A".to_string()),
        };

        let query = f.to_query().unwrap();
        assert_eq!(query, "status=A&name=B&client_version=C");
    }
}
