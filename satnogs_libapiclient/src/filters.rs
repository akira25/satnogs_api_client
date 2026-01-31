// use pyo3::prelude::*;
use serde::Serialize;
use serde_json;
use serde_urlencoded::ser::Error;

#[derive(Debug, Serialize, Default, Clone)]
// #[pyclass]
pub struct StationFilter {
    pub status: Option<String>,
    pub name: Option<String>,
    pub client_version: Option<String>,
}

impl StationFilter {
    pub fn to_query(&self) -> Result<String, Error> {
        serde_urlencoded::to_string(self)
    }

    /// Generates a vector of fieldnames and values to be fed into ureqs query-function
    pub fn into_vec(self) -> Vec<(String, String)> {
        let value = serde_json::to_value(self).unwrap();

        value
            .as_object()
            .unwrap()
            .iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
            .collect()
    }
}

//fn filters_as_pairs<T: Serialize>(filters: &T) -> Vec<(String, String)> {
//    let mut pairs = Vec::new();
//
//    let serializer = serde_urlencoded::Serializer::new(&mut pairs);
//    filters.serialize(serializer).unwrap();
//
//    pairs
//}

#[cfg(test)]
mod test {
    use super::*;

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
