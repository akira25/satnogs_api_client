#!/usr/bin/python3

from satnogs_pyapi_client import APIClient, JobFilter, BasicStationInfo
import datetime as dt

token = "ADD_YOU_TOKEN_HERE"
c = APIClient("https://network-dev.satnogs.org/api", token)

# create Jobfilter with keyword-syntax
f = JobFilter(ground_station=263)

# create a datetime object for time filtering (tzinfo mandatory!)
date = dt.datetime.now(tz=dt.UTC) + dt.timedelta(hours=5)
# modification possible via getter/setter
f.start = date

station = BasicStationInfo(ground_station=263, lat=52.5, lon=13.5, alt=42)

jobs = c.get_jobs_heartbeat(f, station)

print(f"Found {len(jobs)} jobs.")
print("Station, JobID, TransmitterID, Start Time")
for j in jobs:
    print(f"{j.ground_station}\t{j.id}\t{j.transmitter}\t{j.start}")
