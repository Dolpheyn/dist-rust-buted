# Rust Distributed Systems

## Project moved

Due to this:
```shell
❯ cargo b
error: failed to determine package fingerprint for build script for dist-rust-buted v0.1.0 (/path/to/dolpheyn/dist-rust-buted)

Caused by:
  failed to determine the most recently modified file in /path/to/dolpheyn/dist-rust-buted

Caused by:
  failed to determine list of files in /path/to/dolpheyn/dist-rust-buted

Caused by:
  failed to open git index at /path/to/dolpheyn/dist-rust-buted/.git/

Caused by:
  invalid data in index - calculated checksum does not match expected; class=Index (10)
```

## Objectives

- Build a service discovery system
- Endpoints described & generated w protobuf
- Build client SDK for each service
- Build a dumb math service (1 service for each math operation like add, subtract etc.).
  - Make each operation service register itself.
  - Each service needs to know the service discovery system's IP. Maybe take the value from config (shared file).
- Have a service as entrypoint.

## Tasks

- [SVC-DSC - Service discovery](docs/tasks/svc-dsc.md)
- [SVC-MAT - Math services](docs/tasks/svc-mat.md)
- [DST-PFM - Distributed platform](docs/tasks/dst-pfm.md)
