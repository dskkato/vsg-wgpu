use prost_build::Config;
use std::io::Result;

fn main() -> Result<()> {
    Config::new()
        .out_dir("src/proto")
        .include_file("mod.rs")
        .compile_protos(&["../proto/message.proto"], &["../proto"])?;
    Ok(())
}
