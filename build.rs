use cargo_toml::Manifest;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");

    let manifest = Manifest::from_slice(&fs::read("Cargo.toml").unwrap()).unwrap();
    if let Some(metadata) = manifest.package.and_then(|p| p.metadata) {
        if let Some(configuration_filename) = metadata.get("configuration_filename") {
            println!(
                "cargo:rustc-env=CONFIGURATION_FILENAME={}",
                configuration_filename
            );
        }

        if let Some(configuration_dir) = metadata.get("configuration_directory_name") {
            println!(
                "cargo:rustc-env=CONFIGURATION_DIRNAME={}",
                configuration_dir
            );
        }
    }
}
