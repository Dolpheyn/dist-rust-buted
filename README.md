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
- Create a service discovery system.
  - At the root of module.
  - Endpoints: RegisterService, DeregisterService
  - Basic DynamoDB table
    - PK: ServiceGroup, SK: ServiceName, IP:port (string)

### SVC-DSC-0

- Make gRPC gen from protobuf works using tonic

### SVC-DSC-1

- Generate endpoints and types
  - RegisterService(RegisterServiceDto)
  - RegisterServiceDto: group string, name string, ip string, port number
  - DeregisterService(DeregisterServiceDto):
  - DeregisterServiceDto: group string, name string
