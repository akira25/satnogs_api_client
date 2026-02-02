use std::time::Duration;
use ureq::Agent;

use satnogs_libapiclient::api_client::APIClient;
use satnogs_libapiclient::filters::JobFilter;

static SATNOGS_NETWORK_URL: &str = "https://network.satnogs.org/api/";

fn main() {
    let config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(20)))
        .build();

    let agent: Agent = config.into();

    let api = APIClient::new(agent, SATNOGS_NETWORK_URL.to_string());
    let f = JobFilter {
        status: None,
        ground_station: Some(1860),
        start: None,
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
    let jobs = api.get_jobs(f).unwrap();

    for job in jobs.iter().take(15) {
        println!("{}\t{}\t{}", job.id, job.ground_station, job.tle0)
    }
}
