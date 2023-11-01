// Relative Modules
pub mod compiler;
pub mod interpreter;
pub mod context;
pub mod frozen;
pub mod diff;

// Standard Uses
use std::path::Path;

// Local Uses
use crate::project::ir::frozen::{
    FrozenUnit as ProjectUnit,
    basic_storage as basic_storage_project
};
use crate::schema::ir::frozen::{
    unit::FrozenUnit as SchemaUnit,
    basic_storage as basic_storage_schema
};

// External Uses
use eyre::{bail, Result};


pub fn project_from_path_without_context(path: &Path)
    -> Result<(Vec<ProjectUnit>, Vec<Vec<SchemaUnit>>)>
{
    /*
    let config_path = path.join(format!("config.{}", CONGREGATION_EXTENSION));

    if !config_path.exists() {
        panic!(
            "Project directory {:?} has no configuration file {:?}",
            path, config_path.file_name().unwrap()
        )
    }
    */

    let frozen_path = path.join(".frozen/");

    if !frozen_path.exists() {
        panic!("Project directory {:?} has no '.frozen' directory", path)
    }

    let latest_version = basic_storage_project::get_latest_version(&frozen_path)
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

    let latest_schemas_path = latest_version_path.join("schemas/");
    if !latest_schemas_path.exists() {
        bail!("No frozen schemas directory exists at {:?}", latest_schemas_path)
    }

    let frozen_project = basic_storage_project::deserialize::from_latest_frozen(
        &frozen_path
    ).unwrap();

    // TODO: This should be the CAS loader instead of Basic Storage loader
    let frozen_schemas =
        basic_storage_schema::deserialize::all_from_version_frozen(&frozen_path)?;

    Ok((frozen_project, frozen_schemas))
}
