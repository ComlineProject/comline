// Standard Uses
use std::path::Path;

// Crate Uses
use crate::code_gen::rust_c_ffi::to_schemas_ffi;

// External Uses
use eyre::Result;
use comline::project;


pub fn generate_project_into(project: &Path, target: &Path) -> Result<()> {
    let (frozen_project, frozen_schemas) =
        project::ir::project_from_path(project)?;

    to_schemas_ffi(target, &frozen_schemas)
}
