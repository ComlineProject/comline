// Relative Modules
pub(crate) mod _1_7_0;

// Standard Uses
use std::collections::HashMap;

// Crate Uses
use crate::code_gen::VersionGenerators;

// External Uses
use once_cell::sync::Lazy;


#[allow(unused)]
pub(crate) static GENERATORS: VersionGenerators = Lazy::new(|| {
    HashMap::from([
        ("1.70.0", _1_7_0::frozen_to_code as _)
    ])
});

