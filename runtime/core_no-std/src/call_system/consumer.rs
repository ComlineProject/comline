// No Std Uses
use alloc::string::String;

// Crate Uses
use crate::call_system::{CallResult, CallSystem};
use crate::message::Message;

// External Uses


pub trait CallSystemConsumer: CallSystem {
    async fn send_async_call(&mut self, kind: Kind, message: Message<'_>) -> CallResult<Message>;

    fn send_blocking_call(&mut self, name: &str, message: Message) -> CallResult<Message>;
}

pub enum Kind {
    Id(u16),
    Named(String)
}
