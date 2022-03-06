use prost_build::Config;
use std::io::Result;

fn main() -> Result<()> {
    let proto_path = "../proto";
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed={}", proto_path);

    Config::new()
        .out_dir("src/proto")
        .include_file("mod.rs")
        .compile_protos(&[format!("{}/message.proto", proto_path)], &[proto_path])?;
    Ok(())
}
