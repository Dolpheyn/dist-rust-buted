use dotenv::dotenv;
use tonic::transport::Channel;

use crate::svc_mat::gen::div_client::DivClient;

use super::{SERVICE_HOST, SERVICE_PORT};

pub async fn client() -> Result<DivClient<Channel>, Box<dyn std::error::Error>> {
    dotenv().expect("missing .env file. Create .env or run from the root of project");
    let host = SERVICE_HOST;
    let port = SERVICE_PORT;
    let addr = format!("http://{}:{}", host, port);

    let client = DivClient::connect(addr).await?;
    Ok(client)
}
