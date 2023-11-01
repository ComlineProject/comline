// Standard Uses
use std::any::Any;

// Crate Uses
use crate::setup::message_format::MessageFormat;

// External Uses
use eyre::Result;
use serde::{Deserialize, Serialize};


pub struct MessagePack;
#[allow(unused)]
impl MessageFormat for MessagePack {
    fn serialize(&self, data: &dyn Any) -> Result<Box<[u8]>> {
        todo!()
    }

    fn deserialize(&self, data: &[u8]) -> Result<Box<dyn Any>> {
        todo!()
    }
}

#[allow(unused)]
fn serialize<T: Serialize>(
    message_format: &dyn MessageFormat, data: T
) -> Result<Box<[u8]>> {
    let mut buf = vec![];
    data.serialize(&mut rmp_serde::Serializer::new(&mut buf))?;

    Ok(buf.into_boxed_slice())
}

#[allow(unused)]
fn deserialize<T: for<'a> Deserialize<'a>>(
    message_format: &dyn MessageFormat, data: &[u8]
) -> Result<T> {
    Ok(T::deserialize(&mut rmp_serde::Deserializer::new(data))?)
}
