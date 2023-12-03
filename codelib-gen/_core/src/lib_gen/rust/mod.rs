// Relative Modules
pub mod cargo;

// Standard Uses
use std::path::Path;

// Crate Uses
use crate::code_gen;

// External Uses
use comline_core::package::config::ir::frozen::basic_storage as basic_storage_package;

use eyre::{Result, bail};


pub fn generate_frozen_package_into(package_path: &Path, generation_path: &Path) -> Result<()> {
    let frozen_path = package_path.join(".frozen/");
    if !frozen_path.exists() {
        panic!("Package {:?} has no '.frozen' directory", package_path)
    }

    let latest_version = basic_storage_package::get_latest_version(&frozen_path)
        .unwrap();

    let latest_version_path = frozen_path.join(
        format!("package/versions/{latest_version}/")
    );
    if !latest_version_path.exists() {
        bail!("No version directory exists at {:?}", latest_version_path)
    }

    code_gen::rust::generate_frozen_schemas_into_path(&latest_version_path, generation_path)?;

    Ok(())
}
