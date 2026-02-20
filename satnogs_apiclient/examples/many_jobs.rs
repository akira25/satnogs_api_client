use std::time::Duration;
use ureq::Agent;

use satnogs_apiclient::api_client::APIClient;
use satnogs_apiclient::filters::JobFilter;

static SATNOGS_NETWORK_URL: &str = "https://network.satnogs.org/api/";

fn main() {
    let config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(20)));

    let api = APIClient::new(config, SATNOGS_NETWORK_URL.to_string());
    let f = JobFilter {
        ground_station: Some(1860),
        ..Default::default()
    };
    let jobs = api.get_jobs(f).unwrap();

    println!("Found {} jobs.", jobs.len());
    for job in jobs.iter().take(15) {
        println!("{}\t{}\t{}", job.id, job.ground_station, job.tle0)
    }
}
