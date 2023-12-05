// Standard Uses

// Crate Uses
use crate::{TEST_PACKAGE_CRATE_BUILD_PATH, TEST_PACKAGE_PATH};

// External Uses
use comline_codelib_gen::lib_gen::rust_c_ffi;

use glob::glob;


#[test]
fn generate_lib() {
    rust_c_ffi::generate_package_into_path(
        &TEST_PACKAGE_PATH, &TEST_PACKAGE_CRATE_BUILD_PATH
    ).unwrap();

    let pattern = &*format!("{}/**/*.rs", TEST_PACKAGE_CRATE_BUILD_PATH.display());
    let entries =  glob(pattern).unwrap();
    for entry in entries {

    }
}

