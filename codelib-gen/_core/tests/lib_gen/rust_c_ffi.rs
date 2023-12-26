// Standard Uses

// Crate Uses
use crate::{TEST_PACKAGE_BUILD_PATH, TEST_PACKAGE_CRATE_RUST_C_FFI_BUILD_PATH, TEST_PACKAGE_PATH};

// External Uses
use comline_codelib_gen::lib_gen::rust_c_ffi;



#[test]
fn generate_lib() {
    std::fs::remove_dir_all(&*TEST_PACKAGE_CRATE_RUST_C_FFI_BUILD_PATH).ok();

    rust_c_ffi::generate_package_into_path(
        &TEST_PACKAGE_PATH, &TEST_PACKAGE_CRATE_RUST_C_FFI_BUILD_PATH
    ).unwrap();

    let pattern = &*format!("{}/**/*.rs", TEST_PACKAGE_CRATE_RUST_C_FFI_BUILD_PATH.display());
    let entries = glob::glob(pattern).unwrap();
    for entry in entries {
        let path = entry.unwrap();
        let code = std::fs::read_to_string(&path).unwrap();

        println!("Code for file {}", path.display());
        println!("{}\n\n", code);
    }
}

