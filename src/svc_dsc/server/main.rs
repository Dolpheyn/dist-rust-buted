#![feature(hash_drain_filter)]
#![feature(exact_size_is_empty)]

use dotenv::dotenv;
use tokio::sync::oneshot::{self, error::TryRecvError};

use dist_rust_buted::{
    dst_pfm::{serve_with_shutdown, ServiceConfig},
    svc_dsc::{
        gen::ser_dict_server::SerDictServer, server::serdict::SerDictImpl,
        server::serdict::ServiceRecord, HEARTBEAT_INTERVAL, SERVICE_GROUP, SERVICE_NAME,
    },
};

use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("missing .env file. Create .env or run from the root of project");

    let host = env::var("SERVICE_DISCOVERY_HOST").expect("SERVICE_DISCOVERY_HOST must be set");
    let port = env::var("SERVICE_DISCOVERY_PORT").expect("SERVICE_DISCOVERY_PORT must be set");

    let service_map = Arc::new(RwLock::new(HashMap::new()));
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
