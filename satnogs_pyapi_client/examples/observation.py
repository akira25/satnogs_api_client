#!/usr/bin/python3

from satnogs_pyapi_client import *

c = APIClient("https://network.satnogs.org/api")

ob = c.get_observation(13341257)
print(ob)
