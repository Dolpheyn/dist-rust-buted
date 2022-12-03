pub mod hello {
    tonic::include_proto!("hello");
}

use hello::SayRequest;

fn main() {
    let req = SayRequest::default();
    dbg!(req);
    println!("Hello server")
}
