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

        {
            let mut services_map = self.services.lock().unwrap();

            let key = (request.group, request.name);
            services_map.insert(key.clone(), (request.ip, request.port));

            if let Some((ip, port)) = services_map.get(&key).clone() {
                let res = RegisterServiceResponse {
                    ip: ip.to_owned(),
                    port: port.to_owned(),
                };

                return Ok(Response::new(res));
            } else {
                return Err(Status::internal("Failed to register service"));
            }
        }
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
        let res = make_example_get_service_response(request.group, request.name);

        Ok(Response::new(res))
    }

    async fn list_service(
        &self,
        request: Request<()>,
    ) -> Result<Response<ListServiceResponse>, Status> {
        println!("serdict : list_service : Got a request : {:?}", request);

        let res = make_example_list_service_response();
        Ok(Response::new(res))
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

fn make_example_register_service_response() -> RegisterServiceResponse {
    RegisterServiceResponse {
        ip: "127.0.0.1".into(),
        port: 50051,
    }
}

fn make_example_get_service_response(group: String, name: String) -> GetServiceResponse {
    GetServiceResponse {
        group,
        name,
        ip: "127.0.0.1".into(),
        port: 50050,
    }
}
fn make_example_list_service_response() -> ListServiceResponse {
    ListServiceResponse {
        services: vec![
            make_example_get_service_response("core".into(), "svc-dsc".into()),
            make_example_get_service_response("math".into(), "add".into()),
            make_example_get_service_response("math".into(), "sub".into()),
            make_example_get_service_response("math".into(), "mul".into()),
            make_example_get_service_response("math".into(), "div".into()),
        ],
    }
}
