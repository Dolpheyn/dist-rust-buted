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

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     dotenv().expect("missing .env file. Create .env or run from the root of project");
//     let host = env::var("SERVICE_DISCOVERY_HOST").expect("SERVICE_DISCOVERY_HOST must be set");
//     let port = env::var("SERVICE_DISCOVERY_PORT").expect("SERVICE_DISCOVERY_PORT must be set");
//     let addr = format!("{}:{}", host, port);
//
//     let mut client = SerDictClient::connect(addr)
//         .await
//         .expect("failed to connect");
//
//     client
//         .register_service(RegisterServiceRequest {
//             group: "other".into(),
//             name: "hehe".into(),
//             ip: "127.0.0.1".into(),
//             port: 1336,
//         })
//         .await?;
//
//     client
//         .register_service(RegisterServiceRequest {
//             group: "math".into(),
//             name: "add".into(),
//             ip: "127.0.0.1".into(),
//             port: 1337,
//         })
//         .await?;
//
//     client
//         .register_service(RegisterServiceRequest {
//             group: "math".into(),
//             name: "sub".into(),
//             ip: "127.0.0.1".into(),
//             port: 1338,
//         })
//         .await?;
//
//     client
//         .register_service(RegisterServiceRequest {
//             group: "math".into(),
//             name: "div".into(),
//             ip: "127.0.0.1".into(),
//             port: 1339,
//         })
//         .await?;
//
//     client
//         .register_service(RegisterServiceRequest {
//             group: "math".into(),
//             name: "mul".into(),
//             ip: "127.0.0.1".into(),
//             port: 1340,
//         })
//         .await?;
//
//     client
//         .register_service(RegisterServiceRequest {
//             group: "junk".into(),
//             name: "deregisterme".into(),
//             ip: "127.0.0.1".into(),
//             port: 1341,
//         })
//         .await?;
//
//     client
//         .deregister_service(DeregisterServiceRequest {
//             group: "junk".into(),
//             name: "deregisterme".into(),
//         })
//         .await?;
//
//     let res = client.list_service(()).await?;
//
//     println!("ListService response = {:#?}", res);
//
//     assert_eq!(res.into_inner().services.len(), 5);
//
//     let res = client
//         .list_service_by_group_name(ListServiceByGroupNameRequest {
//             group: "math".into(),
//         })
//         .await?;
//
//     println!("ListServiceByGroupName({}) Response = {:#?}", "math", res);
//
//     assert_eq!(res.into_inner().services.len(), 4);
//
//     let res = client
//         .list_service_by_group_name(ListServiceByGroupNameRequest {
//             group: "other".into(),
//         })
//         .await?;
//
//     println!("ListServiceByGroupName({}) Response = {:#?}", "other", res);
//
//     assert_eq!(res.into_inner().services.len(), 1);
//
//     Ok(())
// }
