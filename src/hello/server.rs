pub mod gen {
    tonic::include_proto!("hello");
}

use dist_rust_buted::dst_pfm::{serve_with_shutdown, ServiceConfig};
use tonic::{Request, Response, Status};

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
        println!("greeter: say_hello: Got a request: {:?}", request);

        let res = SayResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(res))
    }
}

const SERVICE_GROUP: &str = "starter";
const SERVICE_NAME: &str = "greeter";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let greeter = GreeterImpl::default();
    let service = GreeterServer::new(greeter);
    let cfg = ServiceConfig {
        service_group: SERVICE_GROUP.into(),
        service_name: SERVICE_NAME.into(),
        host: "[::1]".into(),
        port: 50051,
        should_register: true,
    };

    serve_with_shutdown(service, &cfg).await?;

    Ok(())
}
