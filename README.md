# WIP: satnogs-api-client

A rust-based API client for the [SatNOGS network](https://network.satnogs.org)

> [!TIP]
> Main Repository: [codeberg.org/akira25/satnogs-api-client](https://codeberg.org/akira25/satnogs-api-client). Please contribute there.

This api client provides a rust and a python interface to communicate with the JSON-API of the SatNOGS network.

## General API Concept

Conceptually, we divide functions into two different groups:

- Single object calls, like e.g. `get_observation(id: uint64)` and
- Multi-Object calls e.g.: `get_observation*s*(f: Filter)`

Multi-Object functions take a filter object, that defines criteria that you search for. As your request might be that big, that it will be paginated by the server, you should pay extra attention with those.

Filter objects support both keyword-syntax and getter/setter syntax. Please check the [python examples](#python) below.

## Install: How to get this

For python users: You can get this package via pypi:

```sh
pip install satnogs-api-client
```

For Rustaceans: You can checkout this repo directly, or include it via cargo+git directly:

```toml
[dependencies]
satnogs_apiclient = { version = "0.2.1", git = "https://codeberg.org/akira25/satnogs-api-client"}
```
Have fun

## Rust

This section shows you an rust example for using the client. For more, please check the [examples-directory](https://codeberg.org/akira25/satnogs-api-client/src/branch/main/satnogs_apiclient/examples) in the repository.

### Fetch Observations

This requests the observations made by station 106 during the last 12 hours:

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


## Python

This section shows you python example for using the client. For more, please check the [examples-directory](https://codeberg.org/akira25/satnogs-api-client/src/branch/main/satnogs_pyapi_client/examples) in the repository.

### Fetch Observations

The same example from above looks like this in python:

```python
from satnogs_api_client import APIClient, ObservationFilter
import datetime as dt

c = APIClient()  # URL can be omitted

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
