use std::time::Duration;
use ureq::Agent;

use satnogs_libapiclient::api_client::APIClient;
use satnogs_libapiclient::filters::*;

static SATNOGS_NETWORK_URL: &str = "https://network.satnogs.org";

fn main() {
    let api_url = SATNOGS_NETWORK_URL.to_owned() + "/api";

    let config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .build();
    let agent: Agent = config.into();

    let api = APIClient::new(agent, api_url);

    let f: StationFilter = StationFilter {
        // 0 - Offline, 1 - Testing, 2 - Online
        status: Some("2".to_string()),
        name: None,
        client_version: None,
    };
    let stations = api.get_stations(f).unwrap();

    for station in stations.iter().take(25) {
        println!("{}:\t{}\t{}", station.id, station.status, station.name)
    }

}
