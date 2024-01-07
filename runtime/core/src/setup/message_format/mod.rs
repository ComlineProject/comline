// Relative Modules
pub mod msgpack;

// Standard Uses
use std::any::Any;

// Crate Uses

// External Uses
use eyre::Result;
use downcast_rs::{DowncastSync, impl_downcast};


#[allow(unused)]
pub trait MessageFormat: DowncastSync {
    fn serialize(&self, data: &dyn Any) -> Result<Box<[u8]>>;
    fn deserialize(&self, data: &[u8]) -> Result<Box<dyn Any>>;
}
impl_downcast!(sync MessageFormat);


pub struct Message<'a>(pub Vec<Parameter<'a>>);
impl<'a> Message<'a> {
    pub fn new() -> Self { Self { 0: vec![] } }
    pub fn parameter(mut self, param: &'a dyn Any) -> Self {
        self.0.push(Parameter(param)); self
    }
}
pub struct Parameter<'a>(&'a dyn Any); // for<'a> Deserialize<'a>
