use satnogs_apiclient::*;
use satnogs_apiclient::api_client::APIClient;
use ureq::Agent;
use chrono::Utc;
use std::time::Duration;

fn main() {
    let api_url = "https://network.satnogs.org/api/".to_string();
    let conf = Agent::config_builder();
    let client = APIClient::new(conf, api_url, None);

    let f = filters::ObservationFilter {
        ground_station: Some(106),
        start: Some(Utc::now() - Duration::from_hours(24)),
        end: Some(Utc::now() - Duration::from_hours(12)),
        ..Default::default()
    };

    let obs = client.get_observations(f).unwrap();
    println!("{} Observations...", obs.len());

    for ob in obs {
        println!("{}\t{}\t{}", ob.id, ob.status, ob.ground_station)
    }

}
