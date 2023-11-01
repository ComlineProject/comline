// Standard Uses

// Crate Uses
use crate::package::TEST_PACKAGE_DIR;

// External Uses
use comline::project::build as project_build;


#[test]
fn build_package() {
    project_build::build(&TEST_PACKAGE_DIR).unwrap();
}
