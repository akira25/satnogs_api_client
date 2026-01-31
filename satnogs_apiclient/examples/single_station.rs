use std::time::Duration;
use ureq::Agent;

use satnogs_libapiclient::api_client::APIClient;

static SATNOGS_NETWORK_URL: &str = "https://network.satnogs.org/api/";

fn main() {
    let config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .build();
    let agent: Agent = config.into();

    let api = APIClient::new(agent, SATNOGS_NETWORK_URL.to_string());
    let single_station = api.get_station(1860).unwrap();
    println!("{:?}", single_station);

}
