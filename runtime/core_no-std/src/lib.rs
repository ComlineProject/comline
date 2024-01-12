// Nightly Configuration (to be disabled whenever not necessary,
// and mandatory to be removed at Comline Runtime 1.0 release and above)
#![feature(trait_upcasting)]
#![feature(slice_ptr_get)]

// Crate configuration
#![cfg_attr(not(test), no_std)]
#![warn(clippy::std_instead_of_core, clippy::std_instead_of_alloc)]

// Extern Crates
extern crate core;
extern crate alloc;

// Relative Modules
pub mod call_system;
pub mod transport;
pub mod message;
