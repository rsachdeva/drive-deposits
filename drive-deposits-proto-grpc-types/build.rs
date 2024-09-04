use std::env;
use std::path::PathBuf;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("OUT_DIR: {:?}", out_dir);
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("drivedeposits_descriptor.bin"))
        .compile(&["v1/drivedeposits.proto"], &["proto"])
        .inspect_err(|err| println!("build.rs error  : {:?}", err))?;

    Ok(())
}
