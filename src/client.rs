pub mod hello {
    tonic::include_proto!("hello");
}

use hello::greeter_client::GreeterClient;
use hello::SayRequest;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let req = Request::new(SayRequest {
        name: "Dolpheyn".into(),
    });

    let res = client.say_hello(req).await?;

    println!("Response = {:?}", res);

    Ok(())
}
