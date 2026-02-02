use satnogs_libapiclient::*;
use satnogs_libapiclient::api_client::APIClient;
use ureq;
use chrono::Utc;
use std::time::Duration;

fn main() {
    let api_url = "https://network.satnogs.org/api/".to_string();
    let agent = ureq::Agent::new_with_defaults();
    let client = APIClient {agent, api_url };

    let f = filters::ObservationFilter {
        status: None,
        ground_station: Some(1860),
        start: Some(Utc::now() - Duration::from_hours(24)),
        end: Some(Utc::now() - Duration::from_hours(12)),
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
    println!("{} Observations...", obs.len());

    for ob in obs {
        println!("{}\t{}\t{}", ob.id, ob.status, ob.ground_station)
    }

}
