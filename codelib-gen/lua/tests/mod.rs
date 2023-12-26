// Standard Uses
use std::path::{Path, PathBuf};

// Crate Uses

// External Uses
use once_cell::sync::Lazy;


static TEST_PACKAGE_PATH: Lazy<PathBuf> = Lazy::new(||
    PathBuf::from("../../__TEST_DATA__/test/")
);

static TEST_PACKAGE_BUILD_PATH: Lazy<PathBuf> = Lazy::new(||
    PathBuf::from("tests/__TEMP__/test_package/build/")
);

static TEST_PACKAGE_CRATE_BUILD_PATH: Lazy<PathBuf> = Lazy::new(||
    PathBuf::from("tests/__TEMP__/test_package/crate_build/")
);



#[test]
pub fn build_test_package_lib() {
    todo!()
}

