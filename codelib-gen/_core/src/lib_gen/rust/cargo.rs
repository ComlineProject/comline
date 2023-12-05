// Standard Uses
use std::path::Path;

// Crate Uses
use crate::utils;

// External Uses
use comline_core::package::config::ir::frozen::basic_storage as basic_storage_package;
use comline_core::package::config::ir::frozen::FrozenUnit as FrozenPackageUnit;

use eyre::{Result, Context, bail};
use toml_edit::{Document, table, value};


// name: &str, version: &str,
pub fn generate_cargo_project(package_path: &Path, generation_path: &Path) -> Result<()> {
    let cargo_config = generate_latest_frozen_version_cargo_config(package_path)?;

    let cargo_config_path = generation_path.join("Cargo.toml");
    std::fs::write(&cargo_config_path, cargo_config).with_context(|| {
        format!("Could not create cargo configuration at '{}'", cargo_config_path.display())
    })?;

    Ok(())
}

fn generate_latest_frozen_version_cargo_config(package_path: &Path) -> Result<String> {
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

    let latest_config_path = latest_version_path.join("config");
    if !latest_config_path.exists() {
        bail!("No frozen config directory exists at {:?}", latest_config_path)
    }

    let latest_frozen_config = basic_storage_package::deserialize::from_latest_frozen(
        &frozen_path
    ).unwrap();

    generate_specific_frozen_version_cargo_config(
        &latest_frozen_config, "", ""
    )
}

fn generate_specific_frozen_version_cargo_config(
    package_config: &Vec<FrozenPackageUnit>,
    name: &str, version: &str
) -> Result<String> {
    let mut config = Document::new();

    config["package"] = table();
    config["package"]["name"] = value(name);
    config["package"]["version"] = value(version);
    config["package"]["edition"] = value("2021");


    /*
    config["lib"] = table();
    config["lib"]["name"] = value(name);

    let mut crate_type = Array::default();
    crate_type.push("dylib");
    config["lib"]["crate-type"] = value(crate_type);
    */


    config["dependencies"] = table();
    // TODO: Use versions instead of local paths when they are published on a register
    //config["dependencies"]["comline-core"] = value("0.1.0");
    //config["dependencies"]["comline-runtime"] = value("0.1.0");

    config["dependencies"]["comline-core"]["path"] = value("../../../../../../core");
    config["dependencies"]["comline-runtime"]["path"] = value("../../../../../../runtime/core");

    config["dependencies"]["abi_stable"]["version"] = value("^0.11.2");

    let output = format!("{}\n\n{}", utils::generation_note("#"), config.to_string());

    Ok(output)
}

/*
mod tests {
    #[test]
    pub fn output() {
        let config = super::generate_cargo_config("test", "0.1");

        println!("{}", config);
    }
}
*/

