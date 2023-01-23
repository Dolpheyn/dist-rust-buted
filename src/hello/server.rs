pub mod gen {
    tonic::include_proto!("hello");
}

use std::{convert::Infallible, future::Future, net::SocketAddr};

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

use gen::{
    greeter_server::{Greeter, GreeterServer},
    SayRequest, SayResponse,
};

#[derive(Debug, Default)]
pub struct GreeterImpl {}

#[tonic::async_trait]
impl Greeter for GreeterImpl {
    async fn say_hello(
        &self,
        request: Request<SayRequest>,
    ) -> Result<Response<SayResponse>, Status> {
        println!("greeter : say_hello : Got a request : {:?}", request);

        let res = SayResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(res))
    }
}

struct ServiceConfig {
    addr: SocketAddr,
}

const SERVICE_GROUP: &str = "starter";
const SERVICE_NAME: &str = "greeter";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let greeter = GreeterImpl::default();
    let service = GreeterServer::new(greeter);
    let cfg = ServiceConfig {
        addr: "[::1]:50051".parse()?,
    };

    let register_service = async {
        // Get SerDict client,

        // Register service
        println!(
            "greeter: registering to group.name: {}.{}",
            SERVICE_GROUP, SERVICE_NAME
        );
    };
    init(register_service).await?;

    let do_shutdown = || {
        println!("greeter: shutting down...")
        // Get SerDict client,

        // Deregister service
    };
    serve_with_shutdown(service, cfg, do_shutdown).await?;

    Ok(())
}

async fn init<F>(do_before: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Future<Output = ()>,
{
    do_before.await;
    Ok(())
}

// TODO: extract to common platform lib
async fn serve_with_shutdown<S, F>(
    service: S,
    cfg: ServiceConfig,
    on_shutdown: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce() -> (),
    S: Service<HttpRequest<Body>, Response = HttpResponse<BoxBody>, Error = Infallible>
        + NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    let (shutdown_send, shutdown_recv) = oneshot::channel();

    // Serve server on another task(thread) with a shutdown message channel
    let server_task = tokio::spawn(async move {
        Server::builder()
            .add_service(service)
            .serve_with_shutdown(cfg.addr, shutdown_recv.map(drop))
            .await
            .expect("failed to serve service")
    });

    // Wait for ctrl_c
    let _ = tokio::signal::ctrl_c().await;

    println!("dst-pfm: gracefully shutting down server");

    // Send shutdown signal
    let _ = shutdown_send.send(());

    on_shutdown();

    // Wait for server task to finish exiting
    server_task.await?;

    Ok(())
}
