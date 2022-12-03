pub mod gen {
    tonic::include_proto!("serdict");
}

use std::collections::HashMap;
use std::sync::Mutex;

use tonic::transport::Server;
use tonic::{Request, Response, Status};

use gen::ser_dict_server::{SerDict, SerDictServer};
use gen::{
    DeregisterServiceRequest, GetServiceRequest, GetServiceResponse, ListServiceResponse,
    RegisterServiceRequest, RegisterServiceResponse,
};

type ServiceId = (String, String);
type ServiceAddr = (String, u32);

type ServiceMap = HashMap<ServiceId, ServiceAddr>;

#[derive(Debug, Default)]
pub struct SerDictImpl {
    services: Mutex<ServiceMap>,
}

#[tonic::async_trait]
impl SerDict for SerDictImpl {
    async fn register_service(
        &self,
        request: Request<RegisterServiceRequest>,
    ) -> Result<Response<RegisterServiceResponse>, Status> {
        println!("serdict : register_service : Got a request : {:?}", request);

        let request = request.into_inner();

        let mut services_map = self.services.lock().unwrap();

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
        println!(
            "serdict : deregister_service : Got a request : {:?}",
            request
        );

        let request = request.into_inner();

        {
            let mut services_map = self.services.lock().unwrap();

            let key = (request.group, request.name);
            services_map.remove(&key);
        };

        Ok(Response::new(()))
    }

    async fn get_service(
        &self,
        request: Request<GetServiceRequest>,
    ) -> Result<Response<GetServiceResponse>, Status> {
        println!("serdict : get_service : Got a request : {:?}", request);

        let request = request.into_inner();
        let GetServiceRequest { group, name } = request;

        let services_map = self.services.lock().unwrap();

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
        println!("serdict : list_service : Got a request : {:?}", request);

        let services_map = self.services.lock().unwrap();

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50050".parse()?;
    let serdict = SerDictImpl::default();

    Server::builder()
        .add_service(SerDictServer::new(serdict))
        .serve(addr)
        .await?;

    Ok(())
}
