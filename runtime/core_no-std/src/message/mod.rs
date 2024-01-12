// No Std Uses
use core::any::Any;
use alloc::vec::Vec;
use alloc::vec;

// Crate Uses

// External Uses


pub struct Message<'a>(pub Vec<Parameter<'a>>);

impl<'a> Message<'a> {
    pub fn new() -> Self { Self { 0: vec![] } }
    pub fn parameter(mut self, param: Inner<'a>) -> Self {
        self.0.push(Parameter(param)); self
    }
}

pub struct Parameter<'a>(Inner<'a>); // for<'a> Deserialize<'a>
pub type Inner<'a> = &'a (dyn Any + Sync);
