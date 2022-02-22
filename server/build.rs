fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/core.proto")?;
    tonic_build::compile_protos("../proto/websocket.proto")?;
    Ok(())
}
