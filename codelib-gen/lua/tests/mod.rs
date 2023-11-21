// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use comline_codelib_gen;
use comline_codelib_gen_lua;


const TEST_PACKAGE_DATA: &str = "__TEST_DATA__/test/";
const TEST_PACKAGE_BUILD_PATH: &str = "tests/__DATA__/lua/test/";


#[test]
pub fn build_test_package_lib() {
    let package_path = Path::new(TEST_PACKAGE_DATA);
    let build_path = Path::new(TEST_PACKAGE_BUILD_PATH);


    todo!()
}

