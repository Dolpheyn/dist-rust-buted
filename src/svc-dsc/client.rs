pub mod gen {
    tonic::include_proto!("serdict");
}

use gen::ser_dict_client::SerDictClient;

use crate::gen::{DeregisterServiceRequest, RegisterServiceRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SerDictClient::connect("http://[::1]:50050").await?;

    client
        .register_service(RegisterServiceRequest {
            group: "math".into(),
            name: "add".into(),
            ip: "127.0.0.1".into(),
            port: 1337,
        })
        .await?;

    client
        .register_service(RegisterServiceRequest {
            group: "math".into(),
            name: "sub".into(),
            ip: "127.0.0.1".into(),
            port: 1338,
        })
        .await?;

    client
        .register_service(RegisterServiceRequest {
            group: "math".into(),
            name: "div".into(),
            ip: "127.0.0.1".into(),
            port: 1339,
        })
        .await?;

    client
        .register_service(RegisterServiceRequest {
            group: "math".into(),
            name: "mul".into(),
            ip: "127.0.0.1".into(),
            port: 1340,
        })
        .await?;

    client
        .register_service(RegisterServiceRequest {
            group: "junk".into(),
            name: "deregisterme".into(),
            ip: "127.0.0.1".into(),
            port: 1341,
        })
        .await?;

    client
        .deregister_service(DeregisterServiceRequest {
            group: "junk".into(),
            name: "deregisterme".into(),
        })
        .await?;

    let res = client.list_service(()).await?;

    println!("Response = {:#?}", res);

    assert_eq!(res.into_inner().services.len(), 4);

    Ok(())
}
