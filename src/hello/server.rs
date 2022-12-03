pub mod hello {
    tonic::include_proto!("hello");
}

use hello::greeter_server::{Greeter, GreeterServer};
use hello::{SayRequest, SayResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = GreeterImpl::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
