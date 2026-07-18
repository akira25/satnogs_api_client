from satnogs_api_client import APIClient, StationFilter

client = APIClient()

# get information of a single station
beegnd = client.get_station(1860)
print(beegnd, "\n")

# get multiple stations with filter
f = StationFilter(status="2", name="BEEGND-1")
stations = client.get_stations(f)

print(f"Found {len(stations)} stations.")
for s in stations:
    print(s.id, s.status, s.name)
