pub mod gen {
    tonic::include_proto!("serdict");
}

use gen::ser_dict_client::SerDictClient;

use crate::gen::{DeregisterServiceRequest, ListServiceByGroupNameRequest, RegisterServiceRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SerDictClient::connect("http://[::1]:50050")
        .await
        .expect("failed to connect");

    client
        .register_service(RegisterServiceRequest {
            group: "other".into(),
            name: "hehe".into(),
            ip: "127.0.0.1".into(),
            port: 1336,
        })
        .await?;

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

    println!("ListService response = {:#?}", res);

    assert_eq!(res.into_inner().services.len(), 5);

    let res = client
        .list_service_by_group_name(ListServiceByGroupNameRequest {
            group: "math".into(),
        })
        .await?;

    println!("ListServiceByGroupName({}) Response = {:#?}", "math", res);

    assert_eq!(res.into_inner().services.len(), 4);

    let res = client
        .list_service_by_group_name(ListServiceByGroupNameRequest {
            group: "other".into(),
        })
        .await?;

    println!("ListServiceByGroupName({}) Response = {:#?}", "other", res);

    assert_eq!(res.into_inner().services.len(), 1);

    Ok(())
}
