// Relative Modules
mod lib_gen;
mod code_gen;

// Standard Uses
use std::path::{Path, PathBuf};

// Crate Uses

// External Uses
use once_cell::sync::Lazy;


static TEST_PACKAGE_PATH: Lazy<PathBuf> = Lazy::new(||
    Path::new("../../__TEST_DATA__/test/").to_path_buf()
);

static TEST_PACKAGE_BUILD_PATH: Lazy<PathBuf> = Lazy::new(||
    Path::new("tests/__TEMP__/test_package/build/").to_path_buf()
);

/*
#[test]
pub fn build_test_lib() {
    // let test_path = Path::new("tests/__data__/lua/test/");

    todo!()
}
*/

pub mod utils {
    use std::path::Path;
    use eyre::WrapErr;

    pub fn load_str(p: &Path) -> String {
        std::fs::read_to_string(p)
            .with_context(|| format!("Could not load file at path '{}'", p.display())).unwrap()
    }
}
