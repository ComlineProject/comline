// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use comline_core::package;

const TEST_PACKAGE_PATH: &str = "../__TEST_DATA__/test/";

#[test]
fn build_test_package_and_expect_version() {
    let package_path = Path::new(TEST_PACKAGE_PATH);
    let build = package::build::build(&package_path).unwrap();
    let version = package::config::ir::frozen::version(build.config_frozen.as_ref().unwrap())
        .unwrap();

    assert_eq!(version, "0.0.2");
}

