use dist_rust_buted::{
    dst_pfm::{serve_with_shutdown, ServiceConfig},
    svc_mat::{
        add::{SERVICE_HOST, SERVICE_NAME, SERVICE_PORT},
        gen::{
            add_server::{Add, AddServer},
            BinaryOpRequest, MathResponse,
        },
        SERVICE_GROUP,
    },
};

use tonic::{Request, Response, Status};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let add = AddImpl::default();
    let service = AddServer::new(add);
    let cfg = ServiceConfig {
        service_group: SERVICE_GROUP.to_string(),
        service_name: SERVICE_NAME.to_string(),
        host: SERVICE_HOST.to_string(),
        port: SERVICE_PORT,
        should_register: true,
    };

    serve_with_shutdown(service, &cfg).await?;

    Ok(())
}
