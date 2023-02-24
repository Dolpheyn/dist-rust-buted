pub mod gen {
    tonic::include_proto!("serdict");
}

use std::{collections::HashMap, env, sync::Mutex};

use dist_rust_buted::{
    dst_pfm::{serve_with_shutdown, ServiceConfig},
    svc_dsc::{SERVICE_GROUP, SERVICE_NAME},
};
use dotenv::dotenv;
use tonic::{Request, Response, Status};

use gen::{
    ser_dict_server::{SerDict, SerDictServer},
    DeregisterServiceRequest, GetServiceRequest, GetServiceResponse, ListServiceByGroupNameRequest,
    ListServiceResponse, RegisterServiceRequest, RegisterServiceResponse,
};

type ServiceId = (String, String);
type ServiceAddr = (String, u32);

type ServiceMap = HashMap<ServiceId, ServiceAddr>;

#[derive(Debug, Default)]
pub struct SerDictImpl {
    service_registry: Mutex<ServiceMap>,
}

#[tonic::async_trait]
impl SerDict for SerDictImpl {
    async fn register_service(
        &self,
        request: Request<RegisterServiceRequest>,
    ) -> Result<Response<RegisterServiceResponse>, Status> {
        println!("serdict: register_service: Got a request: {:?}", request);

        let request = request.into_inner();

        let mut services_map = self.service_registry.lock().unwrap();

        let key = (request.group, request.name);
        services_map.insert(key.clone(), (request.ip, request.port));

        if let Some((ip, port)) = services_map.get(&key).clone() {
            let res = RegisterServiceResponse {
                ip: ip.to_owned(),
                port: port.to_owned(),
            };

            return Ok(Response::new(res));
        }

        return Err(Status::internal("Failed to register service"));
    }

    async fn deregister_service(
        &self,
        request: Request<DeregisterServiceRequest>,
    ) -> Result<Response<()>, Status> {
        println!("serdict: deregister_service: Got a request: {:?}", request);

        let request = request.into_inner();

        {
            let mut services_map = self.service_registry.lock().unwrap();

            let key = (request.group, request.name);
            services_map.remove(&key);
        };

        Ok(Response::new(()))
    }

    async fn get_service(
        &self,
        request: Request<GetServiceRequest>,
    ) -> Result<Response<GetServiceResponse>, Status> {
        println!("serdict: get_service: Got a request: {:?}", request);

        let request = request.into_inner();
        let GetServiceRequest { group, name } = request;

        let services_map = self.service_registry.lock().unwrap();

        if group.is_empty() || name.is_empty() {
            return Err(Status::invalid_argument(
                "group and name parameter cannot be empty",
            ));
        }

        let key = (group.clone(), name.clone());
        if let Some(val) = services_map.get(&key) {
            let (group, name) = key.to_owned();
            let (ip, port) = val.to_owned();

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
        println!("serdict: list_service: Got a request: {:?}", request);

        let services_map = self.service_registry.lock().unwrap();

        let res = ListServiceResponse {
            services: services_map
                .iter()
                .map(|(key, val)| {
                    let (group, name) = key.to_owned();
                    let (ip, port) = val.to_owned();

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
            "serdict: list_service_by_group_name: Got a request: {:?}",
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("missing .env file. Create .env or run from the root of project");

    let host = env::var("SERVICE_DISCOVERY_HOST").expect("SERVICE_DISCOVERY_HOST must be set");
    let port = env::var("SERVICE_DISCOVERY_PORT").expect("SERVICE_DISCOVERY_PORT must be set");

    let serdict = SerDictImpl::default();
    let service = SerDictServer::new(serdict);

    let cfg = ServiceConfig {
        service_group: SERVICE_GROUP.to_string(),
        service_name: SERVICE_NAME.to_string(),
        host,
        port: port.parse()?,
    };

    serve_with_shutdown(service, &cfg).await?;

    Ok(())
}
