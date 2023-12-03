// Relative Modules
pub(crate) mod publish;
pub(crate) mod registry;
pub(crate) mod dependency;

// Standard Uses
use std::path::Path;

// External Uses
use comline_core::package::config::idl::constants::CONGREGATION_EXTENSION;

use eyre::{bail, Context, Result};


pub fn create_empty_package_at(new_package_path: &Path, name: &str, force: &bool) -> Result<()> {
    if !new_package_path.exists() {
        std::fs::create_dir_all(new_package_path)
            .context(new_package_path.to_str().unwrap().to_string())?;
    } else if !force {
        bail!(
            "Directory already exists, if you want to package files to be created anyway\n\
            pass the `--force' flag.\n\
            Mind that if any expected file exists, the creation will be aborted",
        )
    }

    let config = make_default_config(name);
    let config_path = new_package_path.join(
        format!("config.{}", CONGREGATION_EXTENSION)
    );

    if config_path.exists() {
        bail!(
            "Cannot create {} since it already exists",
            config_path.file_stem().unwrap().to_str().unwrap()
        )
    }

    std::fs::write(config_path, config)?;

    Ok(())
}

// TODO: This var is something to be moved into the core library
const SPECIFICATION_VERSION: u8 = 1;
fn make_default_config(name: &str) -> String {
    format!(
        "congregation {}\n\
        specification_version = {}\n\
        ", name, SPECIFICATION_VERSION
    )
}

