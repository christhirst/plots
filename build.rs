use std::error::Error;
use std::fs;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::create_dir_all("client")?;
    tonic_build::configure()
        .build_client(true)
        .out_dir("client")
        .file_descriptor_set_path(out_dir.join("plot_descriptor.bin"))
        .compile(&["proto/plot.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/plot.proto")?;

    Ok(())
}
