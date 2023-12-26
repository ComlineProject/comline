// Relative Modules
pub mod cargo;

// Standard Uses
use std::path::Path;

// Crate Uses
use crate::{code_gen, lib_gen};

// External Uses
use comline_core::package::config::ir::frozen::basic_storage as basic_storage_package;

use eyre::{Result, bail, Context};


pub fn generate_frozen_package_into(package_path: &Path, generation_path: &Path) -> Result<()> {
    let frozen_path = package_path.join(".frozen/");
    if !frozen_path.exists() {
        bail!(
            "Package {:?} has no '.frozen' directory, meaning it is not frozen,\n\
            If it was not frozen before, please freeze it first,\
            If it was frozen before, the frozen data might be lost and it will be treated as a fresh package.\n\
            ",
            package_path
        )
    }

    let latest_version = basic_storage_package::get_latest_version(&frozen_path)
        .unwrap();

    let latest_version_path = frozen_path.join(
        format!("package/versions/{latest_version}/")
    );
    if !latest_version_path.exists() {
        bail!("No version directory exists at {:?}", latest_version_path)
    }

    std::fs::create_dir_all(generation_path).with_context(|| {
        format!("Could not create generation directory at '{}'", generation_path.display())
    })?;

    cargo::generate_cargo_project(package_path, generation_path)?;

    code_gen::rust::generate_frozen_schemas_into_path(&latest_version_path, generation_path)?;

    Ok(())
}
