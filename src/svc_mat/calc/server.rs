use std::{convert::Infallible, sync::Arc};

use dist_rust_buted::{
    svc_dsc::{
        self,
        gen::{DeregisterServiceRequest, RegisterServiceRequest},
    },
    svc_mat::{
        add,
        calc::{parse::parse, Evaluator, MathOpClients, SERVICE_HOST, SERVICE_NAME, SERVICE_PORT},
        gen::{
            calc_server::{Calc, CalcServer},
            MathExpressionRequest, MathResponse,
        },
        SERVICE_GROUP,
    },
};
use futures::FutureExt;
use futures::{lock::Mutex, Future};
use http::{Request as HttpRequest, Response as HttpResponse};
use hyper::Body;
use tokio::sync::oneshot;
use tonic::{
    body::BoxBody,
    codegen::Service,
    transport::{NamedService, Server},
    Code, Request, Response, Status,
};

#[derive(Default)]
struct CalcImpl {
    evaluator: Evaluator,
}

impl CalcImpl {
    fn new(clients: MathOpClients) -> Self {
        CalcImpl {
            evaluator: Evaluator::new(clients),
        }
    }
}

#[tonic::async_trait]
impl Calc for CalcImpl {
    async fn evaluate(
        &self,
        request: Request<MathExpressionRequest>,
    ) -> Result<Response<MathResponse>, Status> {
        println!("math.calc: Got a request: {:?}", request);

        let request = request.into_inner();
        let MathExpressionRequest { expression } = request;

        let expression = parse(expression);
        if expression.is_none() {
            return Ok(Response::new(MathResponse { result: 0 }));
        }
        let result = self.evaluator.eval(&expression.unwrap()).await;

        match result {
            Ok(response) => {
                return Ok(Response::new(response));
            }
            Err(err) => {
                return Err(Status::new(
                    Code::Internal,
                    format!("calc failed with reason {}", err),
                ));
            }
        }
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
    let add_client = add::client::client().await?;
    let calc = CalcImpl::new(MathOpClients {
        add: Some(Arc::new(Mutex::new(add_client))),
        ..Default::default()
    });
    let service = CalcServer::new(calc);
    let cfg = ServiceConfig {
        service_name: SERVICE_NAME.to_string(),
        host: SERVICE_HOST.to_string(),
        port: SERVICE_PORT,
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
