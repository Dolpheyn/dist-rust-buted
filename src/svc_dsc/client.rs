use std::env;

use dotenv::dotenv;
use tonic::transport::Channel;

use crate::svc_dsc::gen::ser_dict_client::SerDictClient;

pub async fn client() -> Result<SerDictClient<Channel>, Box<dyn std::error::Error>> {
    dotenv().expect("missing .env file. Create .env or run from the root of project");
    let host = env::var("SERVICE_DISCOVERY_HOST").expect("SERVICE_DISCOVERY_HOST must be set");
    let port = env::var("SERVICE_DISCOVERY_PORT").expect("SERVICE_DISCOVERY_PORT must be set");
    let addr = format!("http://{}:{}", host, port);

    let client = SerDictClient::connect(addr).await?;
    Ok(client)
}
