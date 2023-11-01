// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use comline::project::ir::frozen::Dependency;
use comline::project::ir::context::ProjectContext;
use comline::project::ir::frozen;
use eyre::Result;


#[allow(unused)]
pub fn fetch_dependencies(project: &ProjectContext, cache_path: &Path) -> Result<()> {
    let dependencies = frozen::dependencies(
        project.config_frozen.as_ref().unwrap()
    );

    for dependency in dependencies {
        let dep = fetch_dependency(dependency)?;
    }

    Ok(())
}

#[allow(unused)]
pub fn fetch_dependency(dependency: &Dependency) -> Result<()> {

    todo!()
}

