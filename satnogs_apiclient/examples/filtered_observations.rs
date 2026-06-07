use std::time::Duration;
use chrono::Utc;
use ureq::Agent;

use satnogs_apiclient::api_client::APIClient;
use satnogs_apiclient::filters::*;

static SATNOGS_NETWORK_URL: &str = "https://network.satnogs.org";

fn main() {
    let api_url = SATNOGS_NETWORK_URL.to_owned() + "/api";

    let config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)));

    let api = APIClient::new(config, api_url, None);

    let f = ObservationFilter {
        status: Some("good".to_string()),
        ground_station: Some(1860),
        start: Some(Utc::now() - Duration::from_hours(14*24)),
        ..Default::default()
    };
    let obs = api.get_observations(f);
    let obs = match obs {
        Ok(obs) => obs,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        },
    };

    println!("{} Observations found.", obs.len());
    for o in obs.iter().take(25) {
        println!("{}:\t{}\t{}", o.id, o.ground_station, o.transmitter_uuid)
    }
}
