fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("gRPC_server_sample/gRPC_server_sample-api.proto")?;
    Ok(())
}