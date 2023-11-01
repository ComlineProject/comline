// Standard Uses
use std::rc::Rc;

// Crate Uses
use crate::project::ir::context::ProjectContext;
use crate::schema::ir::compiler::interpreter::{meta_stage, object_stage};

// External Uses
use eyre::{anyhow, Result};


pub fn interpret_context(project_context: &ProjectContext) -> Result<()> {
    for schema_context in project_context.schema_contexts.iter() {
        meta_stage::compile_schema_metadata(
            Rc::clone(schema_context), project_context
        ).map_err(|e| anyhow!("{}", e))?;
    }

    for schema_context in project_context.schema_contexts.iter() {
        object_stage::compile_schema(
            Rc::clone(schema_context),
            project_context
        ).map_err(|e| anyhow!("{}", e))?;
    }

    Ok(())
}

