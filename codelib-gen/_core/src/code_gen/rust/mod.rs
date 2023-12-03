// Relative Modules
pub mod _1_7_0;

// Standard Uses
use std::collections::HashMap;
use std::path::Path;

// Crate Uses
use crate::code_gen;
use crate::code_gen::VersionGenerators;

// External Uses
use comline_core::schema::ir::frozen::basic_storage as basic_storage_schema;
use comline_core::schema::ir::frozen::unit as schema_unit;

use once_cell::sync::Lazy;
use eyre::{Result, eyre};


#[allow(unused)]
pub(crate) static GENERATORS: VersionGenerators = Lazy::new(|| {
    HashMap::from([
        ("1.70.0", _1_7_0::frozen_schema_to_code as _)
    ])
});


pub fn generate_frozen_schemas_into_path(version_path: &Path, generation_path: &Path) -> Result<()> {
    let src_path = generation_path.join("src/");
    let schemas = basic_storage_schema::deserialize::all_from_version_frozen(
        &version_path
    )?;

    for schema in &schemas {
        let namespace = schema_unit::schema_namespace(schema)
            .ok_or_else(|| eyre!("No namespace is present on schema"))?;

        // TODO: Make schema to code generation generic here for any rust version
        let result = code_gen::rust::_1_7_0::frozen_schema_to_code(schema);

        std::fs::write(src_path.join(format!("{}.rs", namespace)), result)?;
    }

    Ok(())
}

