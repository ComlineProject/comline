// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use abi_stable::{
    library::{LibraryError, RootModule},
    sabi_types::VersionStrings, StableAbi,
    std_types::{RBox, RVec},
    package_version_strings, sabi_trait,
};


#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = PackageLibRef)))]
#[sabi(missing_field(panic))]
pub struct PackageLib {
    pub to_message: extern "C" fn(data: RVec<u8>) -> MessageBox
}

impl RootModule for PackageLibRef {
    abi_stable::declare_root_module_statics!{PackageLibRef}

    const BASE_NAME: &'static str = "package_lib";
    const NAME: &'static str = "package_lib";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

#[sabi_trait]
pub trait Message {}

pub type MessageBox = Message_TO<'static, RBox<()>>;


pub fn load_root_module(directory: &Path) -> Result<PackageLibRef, LibraryError> {
    PackageLibRef::load_from_directory(directory)
}
