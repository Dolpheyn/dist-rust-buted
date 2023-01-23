# DST-PFM - Distributed Platform

- Platform layer

## DST-PFM-0 - Graceful shutdown with svc-dsc deregistration

- [ ] Implement a graceful shutdown with a function to run during shutdown
  - Send a shutdown signal when ctrl-c signal is received
  - Wrap init server with shutdown step
- [ ] Deregister service during shutdown
