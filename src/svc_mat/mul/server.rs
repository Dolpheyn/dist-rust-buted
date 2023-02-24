use dist_rust_buted::{
    dst_pfm::{serve_with_shutdown, ServiceConfig},
    svc_mat::{
        gen::{
            mul_server::{Mul, MulServer},
            BinaryOpRequest, MathResponse,
        },
        mul::{SERVICE_HOST, SERVICE_NAME, SERVICE_PORT},
        SERVICE_GROUP,
    },
};
use tonic::{Request, Response, Status};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mul = MulImpl::default();
    let service = MulServer::new(mul);
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
