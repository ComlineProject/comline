// Standard Uses

// Crate Uses

// External Uses


pub trait CallProtocolMeta {
    fn calls(&self) -> &'static [&'static str];
    // const ARGUMENTS: &'static [(usize, )];
}

