// Standard Uses
use std::path::{Path, PathBuf};

// Crate Uses

// External Uses
use once_cell::sync::Lazy;
use comline_codelib_gen::lib_gen::rust_c_ffi;


static TEST_PACKAGE_PATH: Lazy<PathBuf> = Lazy::new(||
    Path::new("../__TEST_DATA__/test/").to_path_buf()
);

#[test]
fn generate_lib() {
    rust_c_ffi::generate_project_into(
        &TEST_PACKAGE_PATH, &Path::new("src/tests/__temp__/test_package/")
    ).unwrap();
}

