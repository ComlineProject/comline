// Standard Uses

// Crate Uses

// External Uses


pub trait CallProtocolMeta {
    fn calls_names(&self) -> &'static [&'static str];
    fn call_name_from_id(&self, id: u16) -> Option<&'static str> {
        self.calls_names().get(id as usize).copied()
    }

    fn arguments() -> &'static [(usize, )];
}

