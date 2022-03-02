fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/*");

    tonic_build::compile_protos("../proto/core.proto")?;
    tonic_build::compile_protos("../proto/admin.proto")?;
    Ok(())
}
