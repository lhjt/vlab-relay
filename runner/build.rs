use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/*");

    let files = Path::new("../proto/")
        .read_dir()
        .expect("failed to read proto files")
        .map(|f| f.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();

    prost_build::compile_protos(&files, &["../proto/"])?;
    Ok(())
}
