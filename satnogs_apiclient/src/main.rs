use std::time::Duration;
use ureq::Agent;

use satnogs_libapiclient::api_client::APIClient;
use satnogs_libapiclient::filters::*;

// static SATNOGS_NETWORK_URL: &str = "https://network-dev.satnogs.org";
static SATNOGS_NETWORK_URL: &str = "https://network.satnogs.org";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = SATNOGS_NETWORK_URL.to_owned() + "/api";
    // print!("{:?}", api);

    let config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .build();

    let agent: Agent = config.into();

    let api = APIClient::new(agent, api_url);
    let single_station = api.get_station(1860).unwrap();
    println!("{:?}", single_station);

    return Ok(());
}
