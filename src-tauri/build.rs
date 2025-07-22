// File: src-tauri/build.rs
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    tauri_build::build();

    let out_dir = env::var("OUT_DIR").unwrap();

    #[cfg(target_os = "macos")]
    {
        let resource_dir = PathBuf::from("src-tauri/moltenvk");

        // Only attempt to copy MoltenVK files if the directory exists
        if resource_dir.exists() {
            let target_dir = PathBuf::from(&out_dir).join("../../../Resources");
            fs::create_dir_all(&target_dir).unwrap();

            if resource_dir.join("libMoltenVK.dylib").exists() {
                fs::copy(
                    resource_dir.join("libMoltenVK.dylib"),
                    target_dir.join("libMoltenVK.dylib"),
                )
                .expect("Failed to copy libMoltenVK.dylib");
            }

            if resource_dir.join("MoltenVK_icd.json").exists() {
                fs::copy(
                    resource_dir.join("MoltenVK_icd.json"),
                    target_dir.join("MoltenVK_icd.json"),
                )
                .expect("Failed to copy MoltenVK_icd.json");
            }
        }
    }
}
