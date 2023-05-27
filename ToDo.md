# ASTM communication with equipments in Lab

- [ ] Continuous scanning for machines
- [ ] maintain connection status, last communication timestamp for monitoring
      connectivity status
- [ ] rerun analytical data transmission
- [ ] host application code, calibration and inventory use ACN however
- [ ] checksum validator
- [ ] handle incoming traffic from machine with respect to ETB OR ETX, update
      listener
- [ ] implement channels for communicating with machines

## Objectives

- if machine goes offline, send alert
- add machine and initialize comms as soon as machine is connected to system
- if server is removed from in-between i.e. taken offline, send alert

**Notes**

- every message must begin in a new frame
