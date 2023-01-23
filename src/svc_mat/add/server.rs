pub mod gen {
    tonic::include_proto!("math");
}

use std::convert::Infallible;

use dist_rust_buted::{
    svc_dsc::{
        self,
        client::gen::{DeregisterServiceRequest, RegisterServiceRequest},
    },
    svc_mat::add::{SERVICE_GROUP, SERVICE_NAME},
};
use futures::Future;
use futures::FutureExt;
use http::{Request as HttpRequest, Response as HttpResponse};
use hyper::Body;
use tokio::sync::oneshot;
use tonic::{
    body::BoxBody,
    codegen::Service,
    transport::{NamedService, Server},
    Request, Response, Status,
};

use gen::add_server::Add;

use self::gen::{add_server::AddServer, BinaryOpRequest, MathResponse};

#[derive(Default)]
struct AddImpl {}

#[tonic::async_trait]
impl Add for AddImpl {
    async fn add(
        &self,
        request: Request<BinaryOpRequest>,
    ) -> Result<Response<MathResponse>, Status> {
        println!("math.add: Got a request: {:?}", request);

        let request = request.into_inner();
        let BinaryOpRequest { num1, num2 } = request;

        let result = num1 + num2;

        Ok(Response::new(MathResponse { result }))
    }
}

#[derive(Clone)]
struct ServiceConfig {
    service_name: String,
    host: String,
    port: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let add = AddImpl::default();
    let service = AddServer::new(add);
    let cfg = ServiceConfig {
        service_name: SERVICE_NAME.to_string(),
        host: "[::1]".to_string(),
        port: 50052,
    };

    init(&cfg).await?;

    let do_shutdown = async {
        println!("math.add: shutting down...");

        // Get SerDict client,
        let mut svc_dsc_client = svc_dsc::client::client()
            .await
            .expect("cannot get svc_dsc client");

        // Deregister service
        println!("math.add: deregistering self...");
        svc_dsc_client
            .deregister_service(DeregisterServiceRequest {
                group: SERVICE_GROUP.into(),
                name: SERVICE_NAME.into(),
            })
            .await
            .expect("cannot deregister service");
    };

    serve_with_shutdown(service, &cfg, do_shutdown).await?;

    Ok(())
}

async fn init(cfg: &ServiceConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut svc_dsc_client = svc_dsc::client::client()
        .await
        .expect("unable to connect to svc_dsc");

    let (ip, port) = (cfg.host.clone(), cfg.port);

    println!("math.add: registering self at {}:{}", ip, port);
    svc_dsc_client
        .register_service(RegisterServiceRequest {
            group: SERVICE_GROUP.into(),
            name: SERVICE_NAME.into(),
            ip: ip.into(),
            port,
        })
        .await
        .expect("unable to register self");

    Ok(())
}

async fn serve_with_shutdown<S, F>(
    service: S,
    cfg: &ServiceConfig,
    on_shutdown: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Future<Output = ()>,
    S: Service<HttpRequest<Body>, Response = HttpResponse<BoxBody>, Error = Infallible>
        + NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    let (shutdown_send, shutdown_recv) = oneshot::channel();

    let addr = format!("{}:{}", cfg.host, cfg.port).parse()?;
    let service_name = cfg.service_name.clone();

    // Serve server on another task(thread) with a shutdown message channel
    let server_task = tokio::spawn(async move {
        println!("dst-pfm: serving {} at {}", service_name, addr);
        Server::builder()
            .add_service(service)
            .serve_with_shutdown(addr, shutdown_recv.map(drop))
            .await
            .expect("failed to serve service")
    });

    // Wait for ctrl_c
    let _ = tokio::signal::ctrl_c().await;

    println!("dst-pfm: gracefully shutting down server");

    // Send shutdown signal
    let _ = shutdown_send.send(());

    on_shutdown.await;

    // Wait for server task to finish exiting
    server_task.await?;

    Ok(())
}
