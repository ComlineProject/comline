// Relative Modules
pub mod projects;
pub mod users;

// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use once_cell::sync::Lazy;


pub const FROZEN_PROJECT_RELATIVE_DIR: &str = "storage/projects/";

#[allow(unused)]
static FROZEN_PROJECTS_DIR: Lazy<String> = Lazy::new(||
    format!("{}/{}", std::env::current_dir().unwrap().to_str().unwrap(), FROZEN_PROJECT_RELATIVE_DIR)
);

#[allow(unused)]
static FROZEN_PROJECTS_PATH: Lazy<&Path> = Lazy::new(|| {
    let path = Path::new(&*FROZEN_PROJECTS_DIR);

    if !path.exists() { std::fs::create_dir_all(path).unwrap(); }

    path
});


pub fn init() {
    projects::init();
}

