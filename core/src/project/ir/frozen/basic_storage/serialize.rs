// Standard Uses

// Crate Uses
use crate::project::ir::frozen::FrozenUnit;

// External Uses
use rmp_serde::Serializer;


#[allow(unused)]
pub fn to_processed(units: &[FrozenUnit]) -> Vec<u8> {
    let mut processed = vec![];

    serde::Serialize::serialize(&units, &mut Serializer::new(&mut processed))
        .unwrap();

    processed
}
