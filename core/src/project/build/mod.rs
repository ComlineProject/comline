// Relative Modules
mod cas;
mod basic_storage;

// Standard Uses
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

// Crate Uses
use crate::project::idl::constants::CONGREGATION_EXTENSION;
use crate::project::ir::interpreter::ProjectInterpreter;
use crate::project::ir::{
    compiler, compiler::Compile,
    frozen as frozen_project,
    frozen::basic_storage as basic_storage_project
};
use crate::project::ir::context::ProjectContext;
use crate::schema::idl;
use crate::schema::ir::frozen::basic_storage as basic_storage_schema;
use crate::schema::ir::context::SchemaContext;
use crate::codelib_gen::{find_generator, GeneratorFn};

// External Uses
use eyre::{bail, Result};
use handlebars::{Handlebars, RenderError};
use serde_derive::{Serialize, Deserialize};


/// Builds the package, which step-by-step means:
/// - Compile configuration and schemas
/// - Freeze the results into frozen objects
/// - Generate code for targets (optional)
/// - Document changes (optional)
pub fn build(project_path: &Path) -> Result<ProjectContext> {
    let config_path = project_path.join(
        format!("config.{}", CONGREGATION_EXTENSION)
    );

    if !config_path.exists() {
        bail!(
            "Project directory has no configuration file {:?} at \"{}\"",
            config_path.file_name().unwrap(), project_path.display()
        )
    }

    let latest_project = ProjectInterpreter::from_origin(&config_path)?;

    unsafe {
        interpret_schemas(&latest_project, project_path)?;
    }


    // TODO: This basic storage setup is temporary, it helps in getting development going
    //       in the rest of things, but it should definitely be substituted with the CAS
    if basic_storage_project::has_any_frozen_content(project_path) {
        basic_storage::process_changes(&project_path, &latest_project)?;
    } else {
        basic_storage::process_initial_freezing(&project_path, &latest_project)?;
    }


    // generate_code_for_targets(&latest_project, project_path)?;

    Ok(latest_project)
}



/// Safety: This assumes caller handles mutability properly
unsafe fn interpret_schemas(
    compiled_project: &ProjectContext, package_path: &Path
) -> Result<()> {
    let schema_paths = frozen_project::schema_paths(
        compiled_project.config_frozen.as_ref().unwrap()
    );
    for relative in schema_paths {
        let concrete = format!("{:}/{}", package_path.to_str().unwrap(), relative);
        let concrete_path = Path::new(&concrete);

        let ast = idl::parser_new::from_path(concrete_path)?;

        let context = SchemaContext::with_ast(
            ast, concrete_path.file_stem().unwrap().to_str().unwrap().to_owned()
        );

        let ptr = compiled_project as *const ProjectContext;
        let ptr_mut = ptr as *mut ProjectContext;

        unsafe {
            (*ptr_mut).add_schema_context(
                Rc::new(RefCell::new(context))
            );
        }
    }

    compiler::interpret::interpret_context(compiled_project)
}

pub fn freeze_project_auto(
    latest_project: &ProjectContext, project_path: &Path
) -> Result<()> {
    basic_storage::package::freeze_project(
        &latest_project, &project_path
    )
}

#[allow(unused)]
fn generate_code_for_targets(
    compiled_project: &ProjectContext,
    base_path: &Path
) -> Result<()> {
    use crate::project::ir::frozen::FrozenUnit;

    for item in compiled_project.config_frozen.as_ref().unwrap().iter() {
        if let FrozenUnit::CodeGeneration(details) = item {
            let Some((name, version)) = details.name.split_once('#') else {
                panic!()
            };

            let args = Args {
                default_path: "generated/{{language}}/{{version}}".to_owned(),
                language: name.to_owned(),
                version: version.to_owned(),
            };

            let path = resolve_path_query(&details.generation_path, args).unwrap();
            let path = base_path.join(path);

            let Some((gen_fn, extension)) = find_generator(name, version)
                else
            {
                panic!(
                    "No generator found for language named '{}' with version '{}'",
                    name, version
                )
            };

            generate_code_for_context(
                compiled_project, gen_fn, extension, &path
            )?;
        }
    }

    Ok(())
}


#[derive(Serialize, Deserialize)]
pub struct Args {
    default_path: String,
    language: String,
    version: String
}

pub fn resolve_path_query(query: &Option<String>, args: Args) -> Result<String, RenderError> {
    let mut reg = Handlebars::new();
    reg.set_strict_mode(true);

    if query.is_some() {
        reg.render_template(&query.clone().unwrap(), &args)
    } else {
        reg.render_template(&args.default_path, &args)
    }
}

#[allow(unused)]
pub fn generate_code_for_context(
    context: &ProjectContext,
    generator: &GeneratorFn, extension: &str,
    target_path: &Path
) -> Result<()> {
    std::fs::create_dir_all(target_path)?;

    for schema_context in context.schema_contexts.iter() {
        let schema_ctx = schema_context.borrow();
        let frozen_schema = schema_ctx.frozen_schema.as_ref().unwrap();
        let file_path = target_path.join(
            format!("{}.{}", &schema_ctx.name, extension)
        );

        let code = &*generator(frozen_schema);

        std::fs::write(file_path, code).unwrap();
    }

    Ok(())
}

pub struct BuildOptions {}
