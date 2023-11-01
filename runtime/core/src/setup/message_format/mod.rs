// Relative Modules
pub mod msgpack;

// Standard Uses
use std::any::Any;

// Crate Uses

// External Uses
use eyre::Result;
use downcast_rs::{DowncastSync, impl_downcast};
use serde::{ser::Serialize, de::DeserializeOwned};


#[allow(unused)]
pub trait MessageFormat: DowncastSync {
    fn serialize(&self, data: &dyn Any) -> Result<Box<[u8]>>;
    fn deserialize(&self, data: &[u8]) -> Result<Box<dyn Any>>;
}
impl_downcast!(sync MessageFormat);

pub trait Message: Serialize + DeserializeOwned {}
