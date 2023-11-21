// Relative Modules
pub(crate) mod code_gen;
pub mod lib_gen;
pub(crate) mod builder;
mod utils;

// #[cfg(test)]
// mod tests;

// Standard Uses
use std::path::Path;

// Crate Uses
use crate::lib_gen::cargo;

// External Uses
use comline_core::{
    project::ir::frozen::{
        FrozenUnit as FrozenProjectUnit,
        FrozenWhole as FrozenProject
    },
    schema::ir::frozen::unit::{FrozenContextWhole as FrozenSchema},
};

use eyre::Result;


pub fn generate_for_frozen_project(
    target: &Path, project: &FrozenProject, schemas: &Vec<FrozenSchema>
) -> Result<()> {
    let mut name = None;
    let mut version = None;

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
