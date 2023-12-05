// Relative Modules
pub mod _1_7_0;

// Standard Uses
use std::collections::HashMap;
use std::path::Path;

// Crate Uses
use crate::{code_gen, utils};
use crate::code_gen::VersionGenerators;

// External Uses
use comline_core::schema::ir::frozen::basic_storage as basic_storage_schema;
use comline_core::schema::ir::frozen::unit as schema_unit;

use once_cell::sync::Lazy;
use eyre::{Result, Context, eyre};


#[allow(unused)]
pub(crate) static GENERATORS: VersionGenerators = Lazy::new(|| {
    HashMap::from([
        ("1.70.0", _1_7_0::frozen_schema_to_code as _)
    ])
});


pub fn generate_frozen_schemas_into_path(
    version_path: &Path, generation_path: &Path
) -> Result<()> {
    let src_path = generation_path.join("src/");
    std::fs::create_dir(&src_path).with_context(|| {
        format!("Could not create crate src directory at '{}'", src_path.display())
    })?;

    let mut lib_mod = utils::generation_note("//");

    let schemas = basic_storage_schema::deserialize::all_from_version_frozen(
        &version_path
    )?;

    for schema in &schemas {
        let namespace = schema_unit::schema_namespace_as_path(schema)
            .ok_or_else(|| eyre!("No namespace is present on schema"))?;

        // TODO: Make schema to code generation generic here for any rust version if possible
        let result = code_gen::rust::_1_7_0::frozen_schema_to_code(schema);
        let schema_code_path = src_path.join(format!("{}.rs", namespace));

        std::fs::create_dir_all(&schema_code_path.parent().unwrap()).with_context(|| {
            format!(
                "Could not create schema code directory at '{}'",
                schema_code_path.parent().unwrap().display()
            )
        })?;

        std::fs::write(&schema_code_path, result).with_context(|| {
            format!(
                "Could not write schema language code to path '{}'",
                schema_code_path.display()
            )
        })?;

        lib_mod.push_str(&*format!("pub mod {};\n", namespace))
    }

    let lib_mod_path = src_path.join("lib.rs");
    std::fs::write(&lib_mod_path, lib_mod).with_context(|| {
        format!("Could not create crate lib module at '{}'", lib_mod_path.display())
    })?;

    Ok(())
}

