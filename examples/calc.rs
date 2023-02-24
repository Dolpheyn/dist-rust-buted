use dist_rust_buted::svc_mat::{
    calc,
    gen::{self, MathResponse},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to calc...");
    let mut calc_client = calc::client::client().await?;
    println!("Connected!");

    let cases = vec![
        (" - + 5 5 20", -10),
        ("* 5 10", 50),
        ("/ 10 2", 5),
        ("/ * 10 - 20 + 5 10 5", 10),
    ];
    for (input, expected) in cases {
        println!("Sending input {}", input);
        let res = calc_client
            .evaluate(gen::MathExpressionRequest {
                expression: input.to_string(),
            })
            .await?
            .into_inner();

        println!("Response: {:?}", res);
        let MathResponse { result } = res;
        assert_eq!(result, expected);
    }
    // ~/dev/dist-rust-buted on svc-mat-3-parse-and-eval *3 !7                                                               at 01:23:23
    // ❯ cargo run --example calc_add
    //    Compiling dist-rust-buted v0.1.0 (~/dev/dist-rust-buted)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.61s
    //      Running `target/debug/examples/calc_add`
    // Connecting to calc...
    // Connected!
    // Sending input  - + 5 5 20
    // Response: MathResponse { result: -10 }
    // Sending input * 5 10
    // Response: MathResponse { result: 50 }
    // Sending input / 10 2
    // Response: MathResponse { result: 5 }
    // Sending input / * 10 - 20 + 5 10 5
    // Response: MathResponse { result: 10 }

    Ok(())
}
