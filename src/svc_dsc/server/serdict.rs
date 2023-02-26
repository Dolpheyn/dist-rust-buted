pub mod gen {
    tonic::include_proto!("serdict");
}

use tonic::{Request, Response, Status};

use crate::svc_dsc::gen::{
    ser_dict_server::SerDict, DeregisterServiceRequest, GetServiceRequest, GetServiceResponse,
    ListServiceByGroupNameRequest, ListServiceResponse, RegisterServiceRequest,
    RegisterServiceResponse,
};

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

type ServiceId = (String, String);
type ServiceAddr = (String, u32);

#[derive(Debug)]
pub struct ServiceRecord {
    pub addr: ServiceAddr,
    pub last_updated: std::time::Instant,
}

impl ServiceRecord {
    fn new(addr: ServiceAddr) -> ServiceRecord {
        Self {
            addr,
            last_updated: std::time::Instant::now(),
        }
    }
}

type ServiceMap = HashMap<ServiceId, ServiceRecord>;

#[derive(Debug, Default)]
pub struct SerDictImpl {
    pub service_registry: Arc<RwLock<ServiceMap>>,
}

impl SerDictImpl {
    pub fn new(service_registry: Arc<RwLock<ServiceMap>>) -> SerDictImpl {
        Self { service_registry }
    }
}

#[tonic::async_trait]
impl SerDict for SerDictImpl {
    async fn register_service(
        &self,
        request: Request<RegisterServiceRequest>,
    ) -> Result<Response<RegisterServiceResponse>, Status> {
        println!("serdict::register_service: Got a request: {:?}", request);

        let request = request.into_inner();

        let mut services_map = self.service_registry.write().unwrap();

        let key = (request.group, request.name);
        services_map.insert(key.clone(), ServiceRecord::new((request.ip, request.port)));

        if let Some(record) = services_map.get(&key) {
            let (ip, port) = record.addr.to_owned();
            let res = RegisterServiceResponse { ip, port };

            return Ok(Response::new(res));
        }

        return Err(Status::internal("Failed to register service"));
    }

    async fn deregister_service(
        &self,
        request: Request<DeregisterServiceRequest>,
    ) -> Result<Response<()>, Status> {
        println!("serdict::deregister_service: Got a request: {:?}", request);

        let request = request.into_inner();

        {
            let mut services_map = self.service_registry.write().unwrap();

            let key = (request.group, request.name);
            services_map.remove(&key);
        };

        Ok(Response::new(()))
    }

    async fn get_service(
        &self,
        request: Request<GetServiceRequest>,
    ) -> Result<Response<GetServiceResponse>, Status> {
        println!("serdict::get_service: Got a request: {:?}", request);

        let request = request.into_inner();
        let GetServiceRequest { group, name } = request;

        let services_map = self.service_registry.read().unwrap();

        if group.is_empty() || name.is_empty() {
            return Err(Status::invalid_argument(
                "group and name parameter cannot be empty",
            ));
        }

        let key = (group.clone(), name.clone());
        if let Some(record) = services_map.get(&key) {
            let (group, name) = key.to_owned();
            let (ip, port) = record.addr.to_owned();

            let res = GetServiceResponse {
                group,
                name,
                ip,
                port,
            };

            return Ok(Response::new(res));
        }

        let msg = format!("Service {group}:{name} is not registered.");
        return Err(Status::not_found(msg));
    }

    async fn list_service(
        &self,
        request: Request<()>,
    ) -> Result<Response<ListServiceResponse>, Status> {
        println!("serdict::list_service: Got a request: {:?}", request);

        let services_map = self.service_registry.read().unwrap();

        let res = ListServiceResponse {
            services: services_map
                .iter()
                .map(|(key, record)| {
                    let (group, name) = key.to_owned();
                    let (ip, port) = record.addr.to_owned();

                    GetServiceResponse {
                        group,
                        name,
                        ip,
                        port,
                    }
                })
                .collect::<Vec<GetServiceResponse>>(),
        };

        return Ok(Response::new(res));
    }

    async fn list_service_by_group_name(
        &self,
        request: Request<ListServiceByGroupNameRequest>,
    ) -> Result<Response<ListServiceResponse>, Status> {
        println!(
            "serdict::list_service_by_group_name: Got a request: {:?}",
            request
        );

        let request = request.into_inner();
        if request.group.is_empty() {
            return Err(Status::invalid_argument("group parameter cannot be empty"));
        }

        let res = self.list_service(Request::new(())).await?;
        let mut res = res.into_inner();

        // Filter by group
        res.services = res
            .services
            .into_iter()
            .filter(|service| service.group.eq(&request.group))
            .collect::<Vec<_>>();

        return Ok(Response::new(res));
    }
}
