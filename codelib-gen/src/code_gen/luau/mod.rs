// https://luau-lang.org/

// Relative Modules
pub mod luau;

// Standard Uses
use std::collections::HashMap;

// Crate Uses
use crate::code_gen::VersionGenerators;

// External Uses
use once_cell::sync::Lazy;


#[allow(unused)]
pub(crate) static GENERATORS: VersionGenerators = Lazy::new(|| {
    HashMap::from([
        ("Luau", luau::frozen_to_code as _)
    ])
});

