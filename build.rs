fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/hello.proto")?;
    tonic_build::compile_protos("proto/serdict.proto")?;
    tonic_build::compile_protos("proto/math.proto")?;

    Ok(())
}
