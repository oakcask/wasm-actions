use std::{
    env,
    fs::{self, File},
    path::PathBuf,
    str::FromStr,
};

use cargo_toml::{Manifest, Product};

mod action;

/// Generates metadata YAML and entrypoint script in
/// recommended configuration based on Cargo.toml.
pub fn generate_recommended() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = env::var("CARGO_MANIFEST_DIR")?;
    let manifest_path = PathBuf::from(manifest_path).join("Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path)?;
    let manifest = Manifest::from_str(&manifest)?;
    {
        let metadata_read_path = if let Some(Product {
            path: Some(path), ..
        }) = &manifest.lib
        {
            PathBuf::from_str(path)?
        } else {
            PathBuf::from_str("src/lib.rs")?
        };
        let mut metadata = File::create("action.yaml")?;
        action::generate_metadata_yaml(&mut metadata, &metadata_read_path)?;
        let metadata_read_path = metadata_read_path.to_str().unwrap();
        println!("cargo::rerun-if-changed={metadata_read_path}");
    }
    {
        let package_name = manifest.package().name();
        let crate_name = package_name.replace("-", "_");
        let index_cjs = PathBuf::from_str("index.cjs")?;
        let mut index_cjs = File::create(&index_cjs)?;
        action::generate_index_cjs(&mut index_cjs, &crate_name)?;
    }
    Ok(())
}
