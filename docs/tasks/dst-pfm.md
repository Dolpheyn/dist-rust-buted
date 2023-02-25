# DST-PFM - Distributed Platform

- Platform layer

## DST-PFM-1 - Common lib [PR](https://github.com/Dolpheyn/dist-rust-buted/pull/12)

- [x] Extract platform-layer routines and utils to a common library

## DST-PFM-0 - Graceful shutdown [PR](https://github.com/Dolpheyn/dist-rust-buted/pull/7)

- [x] Implement a graceful shutdown with a function to run during shutdown
  - Send a shutdown signal when ctrl-c signal is received
  - Wrap init server with shutdown step
