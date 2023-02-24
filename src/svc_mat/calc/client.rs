use dotenv::dotenv;
use tonic::transport::Channel;

pub use crate::svc_mat::gen::calc_client::CalcClient;

use super::{SERVICE_HOST, SERVICE_PORT};

pub async fn client() -> Result<CalcClient<Channel>, Box<dyn std::error::Error>> {
    dotenv().expect("missing .env file. Create .env or run from the root of project");
    let host = SERVICE_HOST;
    let port = SERVICE_PORT;
    let addr = format!("http://{}:{}", host, port);

    let client = CalcClient::connect(addr).await?;
    Ok(client)
}
