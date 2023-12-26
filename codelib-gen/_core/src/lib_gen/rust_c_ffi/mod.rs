// Relative Modules
pub(crate) mod cargo;

// Standard Uses
use std::path::Path;

// Crate Uses
use crate::code_gen;

// External Uses
use comline_core::{
    package, package::config::ir::frozen::{
        FrozenUnit as FrozenProjectUnit,
        FrozenWhole as FrozenProject
    },
    schema::ir::frozen::unit::{FrozenContextWhole as FrozenSchema},
};

use eyre::{Context, Result};


/// Generates a package's frozen content into Rust code with C FFI support as a crate
pub fn generate_package_into_path(package_path: &Path, generation_path: &Path) -> Result<()> {
    std::fs::create_dir_all(&generation_path.parent().unwrap()).with_context(|| {
        format!(
            "Could not create package generated crate build directory at '{}'",
            generation_path.display()
        )
    })?;

    let (_, frozen_schemas) =
        package::config::ir::package_from_path_without_context(package_path)?;

    code_gen::rust_c_ffi::to_schemas_ffi(generation_path, &frozen_schemas)?;
    code_gen::rust_c_ffi::tests::to_schemas_tests(generation_path, &frozen_schemas)?;

    Ok(())
}


pub fn generate_for_frozen_project(
    target: &Path, project: &FrozenProject, schemas: &Vec<FrozenSchema>
) -> Result<()> {
    let mut name = None;
    let version = None;

    for unit in project.1.iter() {
        match unit {
            FrozenProjectUnit::Namespace(n) => name = Some(n),
            //FrozenProjectUnit::Version(v) => version = Some(v),
            _ => {}
        }
    }

    cargo::generate_cargo_project(
        name.unwrap(), version.unwrap(), target, schemas
    )?;

    Ok(())
}
