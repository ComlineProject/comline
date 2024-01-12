// Relative Modules
pub mod msgpack;

// Standard Uses
use std::any::Any;

// Crate Uses

// External Uses
use downcast_rs::{DowncastSync, impl_downcast};


#[allow(unused)]
pub trait MessageFormat: DowncastSync {
    fn serialize(&self, data: &dyn Any) -> eyre::Result<Box<[u8]>>;
    fn deserialize(&self, data: &[u8]) -> eyre::Result<Box<dyn Any>>;
}
impl_downcast!(sync MessageFormat);


pub struct Parameter<'a>(Inner<'a>); // for<'a> Deserialize<'a>
pub type Inner<'a> = &'a (dyn Any + Sync);

pub struct Message<'a> {
    pub parameters: Vec<Parameter<'a>>
}

impl<'a> Message<'a> {
    pub fn new() -> Self { Self { parameters: vec![] } }
    pub fn parameter<T: Any + Send + Sync>(mut self, parameter: &'a T) -> Self {
        self.parameters.push(Parameter(parameter)); self
    }
}

/*
impl<'a> Deserialize<'a> for Message<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        todo!()
    }
}


struct MessageVisitor;
impl Visitor for MessageVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}
*/

#[allow(unused)]
pub struct AbstractCall<P: Send> {
    pub(crate) settings: &'static [&'static (&'static str, &'static Setting)],
    pub(crate) parameters: P
}

pub enum Setting {
    None,
    Num(usize),
    Str(&'static str)
}
