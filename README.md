# WIP: satnogs-api-client

A rust-based API client for the [SatNOGS network](https://network.satnogs.org)

> [!TIP]
> Main Repository: [codeberg.org/akira25/satnogs-api-client](https://codeberg.org/akira25/satnogs-api-client). Please contribute there.

This api client proved a rust and a python interface to communicate with the JSON-API of the SatNOGS network.

Conceptually, there we divide functions into different groups:

- Single object calls, like e.g. `get_observation(id: uint64)` and
- Multi-Object calls e.g.: `get_observation*s*(f: Filter)`

Multi-Object functions take a filter object, that defines criteria that you search for. As your request might be that big, that it will be paginated by the server, you should pay extra attention with those.

## Example: Fetch observations

This sample requests the observations made by station 106 during the last 12 hours:

```rust
use satnogs_apiclient::*;
use satnogs_apiclient::api_client::APIClient;
use ureq::Agent;
use chrono::Utc;
use std::time::Duration;

fn main() {
    let api_url = "https://network.satnogs.org/api/".to_string();
    let conf = Agent::config_builder();
    let client = APIClient::new(conf, api_url);

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
```

The same example looks like this in python:

```python
from satnogs_pyapi_client import APIClient, ObservationFilter
import datetime as dt

c = APIClient("https://network.satnogs.org/api")

now = dt.datetime.utcnow()

# create Jobfilter with keyword-syntax
f = ObservationFilter(
    ground_station = 106,
    start = now - dt.time(hour=12),
    end = now,
)

obs = c.get_observations(f)

for ob in obs:
    print(ob)

```
