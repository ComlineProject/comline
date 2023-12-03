// Standard Uses

// Crate Uses
use crate::{TEST_PACKAGE_BUILD_PATH, TEST_PACKAGE_PATH};
use crate::utils::load_str;

// External Uses
//use comline_core::package::config::ir::frozen::basic_storage as basic_storage_package;
use comline_codelib_gen::{code_gen, lib_gen};


#[test]
fn generate_code_from_test_package() {
    lib_gen::rust::generate_frozen_package_into(
        &TEST_PACKAGE_PATH, &TEST_PACKAGE_BUILD_PATH
    ).unwrap();

    let gen_src_path = TEST_PACKAGE_BUILD_PATH.join("src/");
    assert_eq!(
        load_str(&gen_src_path.join("health.rs")),
        "// Test"
    );
}


// TODO: The function above goes trough lib_gen, but we want to test only code_gen, the function
//       below should be uncommented and fixed, then move the above to lib_gen tests folder in super::lib_gen
/*
#[test]
fn generate_code_from_test_package() {
    let frozen_path = TEST_PACKAGE_PATH.join(".frozen/");

    let latest_version = basic_storage_package::get_latest_version(&frozen_path)
        .unwrap();

    let latest_version_path = frozen_path.join(
        format!("package/versions/{latest_version}/")
    );

    code_gen::rust::_1_7_0::frozen_schema_to_code();
}
*/

