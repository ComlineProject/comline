// Standard Uses
use std::path::Path;

// Crate Uses
use crate::schema::ir::frozen::unit::FrozenUnit;

// External Uses
use eyre::{anyhow, Result};


/// Load all schema frozen units from a frozen origin schemas directory
pub(crate) fn all_from_version_frozen(version_path: &Path) -> Result<Vec<Vec<FrozenUnit>>> {
    let schemas_path = version_path.join("schemas/");

    let mut schemas = vec![];

    for file in std::fs::read_dir(&schemas_path)? {
        let schema_path = file?.path();

        if !schema_path.is_file() {
            panic!(
                "Expected a frozen schema file but got something else instead at '{:?}'",
                &schema_path
            )
        }

        schemas.push(from_schema_frozen(&schema_path)?);
    }

    Ok(schemas)
}

pub(crate) fn from_schema_frozen(schema_path: &Path) -> Result<Vec<FrozenUnit>> {
    let schema = std::fs::read(schema_path)?;
    let schema_frozen  = rmp_serde::from_slice(&schema)
        .map_err(|e| anyhow!(e));

    schema_frozen
}

