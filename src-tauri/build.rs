use std::process::Command;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let data_model_manifest = std::path::Path::new(&manifest_dir)
        .join("../../kakebo-data-model/Cargo.toml");
    let export_dir = std::path::Path::new(&manifest_dir)
        .join("../src/bindings");

    // Re-run this build script if the data model source files change
    println!("cargo:rerun-if-changed={}", data_model_manifest.parent().unwrap().join("src").display());

    let status = Command::new("cargo")
        .arg("test")
        .arg("--manifest-path")
        .arg(&data_model_manifest)
        .arg("--no-default-features")
        .arg("--features")
        .arg("sqlite,to-typescript")
        .env("TS_RS_EXPORT_DIR", &export_dir)
        .status();

    if let Ok(s) = status {
        if !s.success() {
            println!("cargo:warning=Failed to generate TypeScript bindings from kakebo-data-model");
        }
    } else {
        println!("cargo:warning=Could not execute cargo to generate TypeScript bindings");
    }

    tauri_build::build()
}
