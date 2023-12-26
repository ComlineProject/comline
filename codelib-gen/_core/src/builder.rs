// Standard Uses
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

// Crate Uses

// External Uses
use eyre::{Result, bail};


#[allow(unused)]
pub fn build_lib(path: &Path, name: &str) -> Result<PathBuf> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let status = Command::new(cargo)
        .current_dir(path)
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        bail!("Failed to build code lib")
    }

    Ok(path.join(format!("target/release/{}", name)))
}

