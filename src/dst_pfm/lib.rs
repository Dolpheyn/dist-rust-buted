use std::convert::Infallible;

use futures::FutureExt;
use http::{Request as HttpRequest, Response as HttpResponse};
use hyper::service::Service;
use hyper::Body;
use tokio::sync::oneshot;
use tonic::{
    body::BoxBody,
    transport::{NamedService, Server},
};

use crate::svc_dsc;

#[derive(Clone)]
pub struct ServiceConfig {
    pub service_group: String,
    pub service_name: String,
    pub host: String,
    pub port: u32,
    pub should_register: bool,
}

async fn init_service(cfg: &ServiceConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut svc_dsc_client = svc_dsc::client::client()
        .await
        .expect("dst-pfm::init_service: unable to connect to svc_dsc");

    let ServiceConfig {
        service_group,
        service_name,
        host,
        port,
        should_register,
    } = cfg;

    if *should_register {
        println!(
            "dst-pfm::init_service: registering {}/{} at {}:{}",
            service_group, service_name, host, port
        );
        svc_dsc_client
            .register_service(svc_dsc::RegisterServiceRequest {
                group: service_group.clone(),
                name: service_name.clone(),
                ip: host.into(),
                port: *port,
            })
            .await
            .expect("dst-pfm::init_service: unable to register service");
    }

    Ok(())
}

pub async fn serve_with_shutdown<S>(
    service: S,
    cfg: &ServiceConfig,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: Service<HttpRequest<Body>, Response = HttpResponse<BoxBody>, Error = Infallible>
        + NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    init_service(&cfg).await?;

    let ServiceConfig {
        service_group,
        service_name,
        host,
        port,
        should_register,
    } = cfg;

    let (shutdown_send, shutdown_recv) = oneshot::channel();

    let addr = format!("{}:{}", host, port).parse()?;

    // Serve server on another task(thread) with a shutdown message channel
    let name = service_name.clone();
    let group = service_group.clone();
    let server_task = tokio::spawn(async move {
        println!(
            "dst-pfm::serve_with_shutdown: serving {}/{} at {}",
            group, name, addr
        );
        Server::builder()
            .add_service(service)
            .serve_with_shutdown(addr, shutdown_recv.map(drop))
            .await
            .expect("dst_pfm::serve_with_shutdown: failed to serve service")
    });

    // Wait for either server_task finish or ctrl_c is pressed
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            // Send shutdown signal
            let _ = shutdown_send.send(());
        },
        _ = server_task => {
        }
    }

    println!(
        "dst-pfm::serve_with_shutdown: gracefully shutting down service {}/{}",
        service_group, service_name
    );

    let do_shutdown = async {
        if *should_register {
            // Get SerDict client,
            let mut svc_dsc_client = svc_dsc::client::client()
                .await
                .expect("dst_pfm::serve_with_shutdown: cannot get svc_dsc client");

            // Deregister service
            println!(
                "dst_pfm::serve_with_shutdown: deregistering {}/{}...",
                service_group, service_name
            );
            svc_dsc_client
                .deregister_service(svc_dsc::DeregisterServiceRequest {
                    group: service_group.clone(),
                    name: service_name.clone(),
                })
                .await
                .expect("dst_pfm::serve_with_shutdown: cannot deregister service");
        }
    };

    do_shutdown.await;

    Ok(())
}
