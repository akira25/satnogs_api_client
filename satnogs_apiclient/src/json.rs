use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Antenna {
    pub frequency: u64,
    pub frequency_max: u64,
    pub band: String,              // "UHF",
    pub antenna_type: String,      // "cross-yagi",
    pub antenna_type_name: String, //"Cross Yagi"
}

//pub enum StationStatus {
//    "Online",
//    "Testing",
//    "Offline"
//}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]

pub enum SuccessRate {
    Bool(bool),
    Int(u64),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Station {
    pub id: u32,
    pub name: String,
    pub altitude: i32,
    pub min_horizon: u32,
    pub lat: f32,
    pub lng: f32,
    pub qthlocator: String,
    pub antenna: Vec<Antenna>,
    pub created: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
    pub status: String,            // "Online",
    pub observations: u64,
    pub future_observations: u32,
    pub description: String,
    pub client_version: String,
    pub target_utilization: Option<u32>,
    pub image: String,
    pub success_rate: SuccessRate,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: u64,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub ground_station: u64,
    pub tle0: String, //"0 OBJECT XW",
    pub tle1: String, //"1 66910U 98067XW  26030.36276135  .00109916  00000-0  13335-2 0  9999",
    pub tle2: String, //"2 66910  51.6269 259.4751 0001545 149.9128 210.1954 15.59763179  9188",
    pub frequency: u64,
    pub mode: Option<String>, //"GFSK",
    pub transmitter: String,  //"Ymz7CW3EYAYxCV9JNvbD98",
    pub baud: Option<f32>,
    pub max_altitude: f32,
    pub norad_cat_id: u32,
}

type URL = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct DemodData {
    pub payload_demod: URL,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Observation {
    pub id: u64,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub ground_station: u32,
    pub transmitter: String, // "Ymz7CW3EYAYxCV9JNvbD98",
    pub norad_cat_id: u32,
    pub payload: Option<URL>,
    pub waterfall: Option<URL>,
    pub demoddata: Vec<Option<DemodData>>,
    pub station_name: String, // "EagleSat",
    pub station_lat: f32,
    pub station_lng: f32,
    pub station_alt: u32,
    pub vetted_status: String,           // "unknown",
    pub vetted_user: Option<u64>,     // null,
    pub vetted_datetime: Option<String>, // null,
    pub archived: bool,
    pub archive_url: Option<URL>,
    pub client_version: String,
    pub client_metadata: String,
    pub status: String,
    pub waterfall_status: String,                  // "unknown",
    pub waterfall_status_user: Option<u64>,     // null,
    pub waterfall_status_datetime: Option<String>, // null,
    pub rise_azimuth: f32,
    pub set_azimuth: f32,
    pub max_altitude: f32,
    pub transmitter_uuid: String,            // "Ymz7CW3EYAYxCV9JNvbD98",
    pub transmitter_description: String,     // "Mode U - 19K2 GFSK",
    pub transmitter_type: String,            // "Transmitter",
    pub transmitter_uplink_low: Option<u64>, // null,
    pub transmitter_uplink_high: Option<u64>, // null,
    pub transmitter_uplink_drift: Option<i64>, // null,
    pub transmitter_downlink_low: u64,       // 437165000,
    pub transmitter_downlink_high: Option<u64>, // null,
    pub transmitter_downlink_drift: Option<i64>, // null,
    pub transmitter_mode: Option<String>,    // "GFSK",
    pub transmitter_invert: bool,
    pub transmitter_baud: Option<f32>,
    pub transmitter_updated: String, // "2024-03-20T07:13:34.988090Z",
    pub transmitter_status: String,  // "active",
    pub tle0: String,                // "0 OBJECT XW",
    pub tle1: String, // "1 66910U 98067XW  26030.36276135  .00109916  00000-0  13335-2 0  9999",
    pub tle2: String, // "2 66910  51.6269 259.4751 0001545 149.9128 210.1954 15.59763179  9188",
    pub tle_source: String, // "Space-Track.org",
    pub center_frequency: Option<u64>,
    pub observer: String,
    pub observation_frequency: u64, // 437165000,
    pub transmitter_unconfirmed: bool,
    pub sat_id: String, // "NTJV-2293-0787-8038-8343"
}
