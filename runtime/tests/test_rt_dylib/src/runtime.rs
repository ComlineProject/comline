// Standard Uses

// Crate Uses

/*
// External Uses
use abi_stable::{
    export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn,
    erased_types::TD_Opaque,
    std_types::RVec
};


use comline_runtime::package_abi::interface::{
    PackageLib, PackageLibRef,
    Message_TO, MessageBox,
};


#[export_root_module]
pub fn get_library() -> PackageLibRef {
    PackageLib {
        to_message
    }.leak_into_prefix()
}


#[sabi_extern_fn]
pub fn to_message(data: RVec<u8>) -> MessageBox {
    Message_TO::from_value(RVec::new(), TD_Opaque)
}
*/
