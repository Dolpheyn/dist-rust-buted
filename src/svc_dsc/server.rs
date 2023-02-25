#![feature(hash_drain_filter)]
#![feature(exact_size_is_empty)]

pub mod gen {
    tonic::include_proto!("serdict");
}

use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};

use dist_rust_buted::{
    dst_pfm::{serve_with_shutdown, ServiceConfig},
    svc_dsc::{HEARTBEAT_INTERVAL, SERVICE_GROUP, SERVICE_NAME},
};
use dotenv::dotenv;
use tokio::sync::oneshot::{self, error::TryRecvError};
use tonic::{Request, Response, Status};

use gen::{
    ser_dict_server::{SerDict, SerDictServer},
    DeregisterServiceRequest, GetServiceRequest, GetServiceResponse, ListServiceByGroupNameRequest,
    ListServiceResponse, RegisterServiceRequest, RegisterServiceResponse,
};

type ServiceId = (String, String);
type ServiceAddr = (String, u32);

#[derive(Debug)]
pub struct ServiceRecord {
    addr: ServiceAddr,
    last_updated: std::time::Instant,
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
    service_registry: Arc<RwLock<ServiceMap>>,
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

        if let Some(record) = services_map.get(&key).clone() {
            let (ip, port) = record.addr.to_owned();
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("missing .env file. Create .env or run from the root of project");

    let host = env::var("SERVICE_DISCOVERY_HOST").expect("SERVICE_DISCOVERY_HOST must be set");
    let port = env::var("SERVICE_DISCOVERY_PORT").expect("SERVICE_DISCOVERY_PORT must be set");

    let service_map = Arc::new(RwLock::new(ServiceMap::new()));
    let serdict = SerDictImpl::new(Arc::clone(&service_map));
    let service = SerDictServer::new(serdict);

    let cfg = ServiceConfig {
        service_group: SERVICE_GROUP.to_string(),
        service_name: SERVICE_NAME.to_string(),
        host,
        port: port.parse()?,
        should_register: false,
    };

    let (shutdown_send, mut shutdown_recv) = oneshot::channel::<()>();

    tokio::spawn(async move {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(HEARTBEAT_INTERVAL));

            match shutdown_recv.try_recv() {
                Ok(_) | Err(TryRecvError::Closed) => {
                    println!("svc_dsc::heartbeat_task: terminating...");
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
            println!("svc_dsc::heartbeat_task: beating...");

            let mut map_lock = service_map
                .write()
                .expect("svc_dsc::heartbeat_task: service_map lock is poisoned");
            let registered_services = map_lock.keys();
            let registered_services_count = registered_services.clone().count();
            if registered_services.is_empty() {
                println!("svc_dsc::heartbeat_task: no service registered");
                continue;
            }

            let drained = map_lock
                .drain_filter(|_, ServiceRecord { last_updated, .. }| {
                    last_updated.elapsed().as_millis() >= (HEARTBEAT_INTERVAL as u128)
                })
                .collect::<HashMap<_, _>>();
            if drained.keys().is_empty() {
                println!(
                    "svc_dsc::heartbeat_task: all {} registered service(s) are still alive",
                    registered_services_count
                );
                continue;
            }
            let drained_services = drained
                .keys()
                .map(|(group, name)| format!("{}/{}", group, name))
                .collect::<Vec<_>>();
            println!(
                "svc_dsc::heartbeat_task: bye bye dead services: {:?}",
                drained_services
            );
        }
    });

    if let Err(e) = serve_with_shutdown(service, &cfg).await {
        println!("svc-dsc: error {}", e);
    };
    shutdown_send
        .send(())
        .expect("svc-dsc: failed at sending shutdown signal");

    Ok(())
}
