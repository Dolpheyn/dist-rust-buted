use dist_rust_buted::dst_pfm::ServiceConfig;
use dist_rust_buted::{
    dst_pfm::serve_with_shutdown,
    svc_mat::{
        div::{SERVICE_HOST, SERVICE_NAME, SERVICE_PORT},
        gen::{
            div_server::{Div, DivServer},
            BinaryOpRequest, MathResponse,
        },
        SERVICE_GROUP,
    },
};
use tonic::{Request, Response, Status};

#[derive(Default)]
struct DivImpl {}

#[tonic::async_trait]
impl Div for DivImpl {
    async fn div(
        &self,
        request: Request<BinaryOpRequest>,
    ) -> Result<Response<MathResponse>, Status> {
        println!("math.div: Got a request: {:?}", request);

        let request = request.into_inner();
        let BinaryOpRequest { num1, num2 } = request;

        if num2 == 0 {
            return Err(Status::invalid_argument("denominator cannot be 0"));
        }

        let result = num1 / num2;

        Ok(Response::new(MathResponse { result }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let div = DivImpl::default();
    let service = DivServer::new(div);
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
