#!/usr/bin/python3

from satnogs_pyapi_client import APIClient, JobFilter
import datetime as dt

c = APIClient("https://network.satnogs.org/api")

# create Jobfilter with keyword-syntax
f = JobFilter(ground_station=106)

# create a datetime object for time filtering (tzinfo mandatory!)
date = dt.datetime(2026, 2, 7, 0, 25, 0, tzinfo=dt.timezone.utc) + dt.timedelta(hours=5)
# modification possible via getter/setter
f.start = date

jobs = c.get_jobs(f)

print(f"Found {len(jobs)} jobs.")
print("Station, JobID, TransmitterID, Start Time")
for j in jobs:
    print(f"{j.ground_station}\t{j.id}\t{j.transmitter}\t{j.start}")
