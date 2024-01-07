// Nightly Configuration (to be disabled whenever not necessary,
// and mandatory to be removed at Comline Runtime 1.0 release and above)
#![feature(trait_upcasting)]
#![feature(slice_ptr_get)]

// Crate configuration
//#![no_std]


// Relative Modules
/*
pub mod prelude {
    pub use crate::setup::APIResult;
}
*/
pub mod package_abi;
pub mod setup;
