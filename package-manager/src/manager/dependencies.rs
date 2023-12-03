// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use comline_core::package::config::ir::frozen::Dependency;
use comline_core::package::config::ir::context::ProjectContext;
use comline_core::package::config::ir::frozen;

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

