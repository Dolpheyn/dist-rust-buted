pub mod gen {
    tonic::include_proto!("serdict");
}

use gen::ser_dict_client::SerDictClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SerDictClient::connect("http://[::1]:50050").await?;

    let res = client.list_service(()).await?;

    println!("Response = {:?}", res);

    Ok(())
}
