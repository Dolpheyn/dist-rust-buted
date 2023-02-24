use dist_rust_buted::{
    dst_pfm::{serve_with_shutdown, ServiceConfig},
    svc_mat::{
        calc::{self, SERVICE_HOST, SERVICE_NAME, SERVICE_PORT},
        gen::{
            calc_server::{Calc, CalcServer},
            MathExpressionRequest, MathResponse,
        },
        SERVICE_GROUP,
    },
};
use tonic::{Code, Request, Response, Status};

#[derive(Default)]
struct CalcImpl {}

#[tonic::async_trait]
impl Calc for CalcImpl {
    async fn evaluate(
        &self,
        request: Request<MathExpressionRequest>,
    ) -> Result<Response<MathResponse>, Status> {
        println!("math.calc: Got a request: {:?}", request);

        let request = request.into_inner();
        let MathExpressionRequest { expression } = request;

        let expression = calc::parse(expression);
        println!("math.calc: parsed expression: {:?}", expression);
        if expression.is_none() {
            return Err(Status::new(Code::InvalidArgument, "the heyl mayn"));
        }

        let result = calc::eval(&expression.unwrap()).await;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calc = CalcImpl::default();
    let service = CalcServer::new(calc);
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
