use dotenv::dotenv;
use tonic::transport::Channel;

pub use crate::svc_mat::gen::add_client::AddClient;

use super::{SERVICE_HOST, SERVICE_PORT};

pub async fn client() -> Result<AddClient<Channel>, Box<dyn std::error::Error>> {
    dotenv().expect("missing .env file. Create .env or run from the root of project");
    let host = SERVICE_HOST;
    let port = SERVICE_PORT;
    let addr = format!("http://{}:{}", host, port);

    let client = AddClient::connect(addr).await?;
    Ok(client)
}
