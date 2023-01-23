# Rust Distributed Systems

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

