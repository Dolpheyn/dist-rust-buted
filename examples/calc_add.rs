use dist_rust_buted::svc_mat::{calc, gen};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to calc...");
    let mut calc_client = calc::client::client().await?;
    println!("Connected!");
    let input = "+ 1 5";
    println!("Sending input {}", input);
    let res = calc_client
        .evaluate(gen::MathExpressionRequest {
            expression: input.to_string(),
        })
        .await;

    println!("result = {:?}", res);
    // IT WORKS!!!!!
    // ~/dev/dist-rust-buted on svc-mat-1-add-sub-div-mul *3 !8 ?3                                                           at 21:40:58
    // ❯ cargo run --example calc_add
    //    Compiling dist-rust-buted v0.1.0 (/Users/[redacted]/dev/dist-rust-buted)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.57s
    //      Running `target/debug/examples/calc_add`
    // Connecting to calc...
    // Connected!
    // Sending input + 1 5
    // result = Ok(Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Fri, 24 Feb 2023 13:41:12 GMT", "grpc-status": "0"} }, message: MathResponse { result: 6 }, extensions: Extensions })

    Ok(())
}
