// Standard Uses
use std::path::Path;

// Crate Uses
use crate::package_abi::interface::{load_root_module, PackageLibRef};

// External Uses


#[allow(unused)]
pub fn load_package_dylib(root_path: &Path) -> PackageLibRef {
    let library: PackageLibRef = load_root_module(
        "../../../target/debug".as_ref()
    ).unwrap_or_else(|e| panic!("{}", e));

    library
}

