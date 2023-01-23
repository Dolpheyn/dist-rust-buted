use std::convert::Infallible;

use dist_rust_buted::svc_dsc::{
    self,
    client::gen::{DeregisterServiceRequest, RegisterServiceRequest},
};
use dist_rust_buted::svc_mat::{
    gen::{
        mul_server::{Mul, MulServer},
        BinaryOpRequest, MathResponse,
    },
    mul::SERVICE_NAME,
    SERVICE_GROUP,
};
use futures::{Future, FutureExt};
use http::{Request as HttpRequest, Response as HttpResponse};
use hyper::Body;
use tokio::sync::oneshot;
use tonic::{
    body::BoxBody,
    codegen::Service,
    transport::{NamedService, Server},
    Request, Response, Status,
};

#[derive(Default)]
struct MulImpl {}

#[tonic::async_trait]
impl Mul for MulImpl {
    async fn mul(
        &self,
        request: Request<BinaryOpRequest>,
    ) -> Result<Response<MathResponse>, Status> {
        println!("math.mul: Got a request: {:?}", request);

        let request = request.into_inner();
        let BinaryOpRequest { num1, num2 } = request;

        let result = num1 * num2;

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
    let mul = MulImpl::default();
    let service = MulServer::new(mul);
    let cfg = ServiceConfig {
        service_name: SERVICE_NAME.to_string(),
        host: "[::1]".to_string(),
        port: 50054,
    };

    // register to svc_dsc
    init(&cfg).await?;

    let do_shutdown = async {
        println!("math.mul: shutting down...");

        // Get SerDict client,
        let mut svc_dsc_client = svc_dsc::client::client()
            .await
            .expect("cannot get svc_dsc client");

        // Deregister service
        println!("math.mul: deregistering self...");
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

    println!("math.mul: registering self at {}:{}", ip, port);
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

// TODO: extract to common platform lib
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

    let mulr = format!("{}:{}", cfg.host, cfg.port).parse()?;
    let service_name = cfg.service_name.clone();

    // Serve server on another task(thread) with a shutdown message channel
    let server_task = tokio::spawn(async move {
        println!("dst-pfm: serving {} at {}", service_name, mulr);
        Server::builder()
            .add_service(service)
            .serve_with_shutdown(mulr, shutdown_recv.map(drop))
            .await
            .expect("failed to serve service")
    });

    println!("dst-pfm: gracefully shutting down server");

    // Wait for either server_task finish or ctrl_c is pressed
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            // Send shutdown signal
            let _ = shutdown_send.send(());
        },
        _ = server_task => {
        }
    }

    on_shutdown.await;

    Ok(())
}
