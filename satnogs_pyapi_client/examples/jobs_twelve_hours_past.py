from satnogs_pyapi_client import APIClient, ObservationFilter
import datetime as dt

c = APIClient("https://network.satnogs.org/api")

now = dt.datetime.now(tz=dt.UTC)

# create Jobfilter with keyword-syntax
f = ObservationFilter(
    ground_station = 106,
    start = now - dt.timedelta(hours=12),
    end = now,
)

obs = c.get_observations(f)

for ob in obs:
    print(ob)
