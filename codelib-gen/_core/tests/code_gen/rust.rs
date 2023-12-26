// Standard Uses

// Crate Uses
use crate::{TEST_PACKAGE_BUILD_PATH, TEST_PACKAGE_PATH};
use crate::utils;

// External Uses
//use comline_core::package::config::ir::frozen::basic_storage as basic_storage_package;
use comline_codelib_gen::{code_gen, lib_gen};


#[test]
fn generate_code_from_test_package() {
    std::fs::remove_dir_all(&*TEST_PACKAGE_BUILD_PATH).ok();

    lib_gen::rust::generate_frozen_package_into(
        &TEST_PACKAGE_PATH, &TEST_PACKAGE_BUILD_PATH
    ).unwrap();

    let gen_src_path = TEST_PACKAGE_BUILD_PATH.join("src/");
    assert_eq!(
        utils::load_str(&gen_src_path.join("health.rs")),
        "// Generated with Comline compiler and code generator\n\
        //\n\
        // Advisory: This file is hashed and should not be manually edited, the runtime will complain\n\
        // and might not even run if so (this is to be decided yet)\n\
        pub trait HealthCheck {\n\
	        \tfn alive(&self);\n\
	        \tfn capabilities(&self);\n\
        \n\
        }\n\
        \n"
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

