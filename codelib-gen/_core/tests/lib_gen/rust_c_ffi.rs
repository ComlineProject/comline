// Standard Uses
use std::path::{Path, PathBuf};

// Crate Uses

// External Uses
use once_cell::sync::Lazy;
use comline_codelib_gen::lib_gen::rust_c_ffi;


static TEST_PACKAGE_PATH: Lazy<PathBuf> = Lazy::new(||
    Path::new("../../__TEST_DATA__/test/").to_path_buf()
);

static TEST_PACKAGE_BUILD_PATH: Lazy<PathBuf> = Lazy::new(||
    Path::new("tests/__TEMP__/test_package/").to_path_buf()
);


#[test]
fn generate_lib() {
    rust_c_ffi::generate_project_into(
        &TEST_PACKAGE_PATH, &TEST_PACKAGE_BUILD_PATH
    ).unwrap();
}

