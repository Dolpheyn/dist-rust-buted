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

### SVC-DSC - Service Discovery svc
- Create a service discovery system.
  - At the root of module.
  - Endpoints: RegisterService, DeregisterService
  - Basic DynamoDB table
    - PK: ServiceGroup, SK: ServiceName, IP:port (string)

#### SVC-DSC-0

- [x] Make gRPC gen from protobuf works using tonic

#### SVC-DSC-1

- [x] Generate endpoints and types RegisterService(RegisterServiceRequest) returns RegisterServiceResponse
    - RegisterServiceRequest: group string, name string
    - RegisterServiceResponse: ip string, port number
  - DeregisterService(DeregisterServiceRequest) returns DeregisterServiceResponse:
    - DeregisterServiceRequest: group string, name string
    - DeregisterServiceResponse: empty
  - GetService(GetServiceRequest) returns GetServiceResponse
    - GetServiceRequest: group string, name string
    - GetServiceResponse: group string, name string, ip string, port number
  - ListService(ListServiceRequest) returns ListServiceResponse
    - ListServiceRequest: empty
    - ListServiceResponse: repeats GetServiceResponse

#### SVC-DSC-2: In-memory service dictionary

- [x] Add a concurrent map to store services
  - Key: `type ServiceId = Tuple(group, svc-name)`
  - Val: `type ServiceAddr = Tuple(ip, port)`
  - Must have indirection bcs methods in server interface generated by tonic does not accept `&mut self`
    - Maybe wrap in `Rc<Mutex>>`
      - `services: Rc<Mutex<HashMap<ServiceId, ServiceAddr>>`

#### SVC-DSC-3 

- [x] Implement endpoints: RegisterService & DeregisterService

#### SVC-DSC-4

- [x] Implement endpoints: GetService & ListService

#### SVC-DSC-5

- [ ] Add endpoint: ListServiceByGroupName
  - ListServiceByGroupName(ListServiceByGroupNameRequest) returns ListServiceByGroupNameResponse
    - ListServiceByGroupNameRequest: group string
    - ListServiceByGroupNameResponse: repeats GetServiceResponse

### SVC-MAT - Math services

#### SVC-MAT-0

- [ ] Create proto for math services
  - Add, Sub, Div, Mul
