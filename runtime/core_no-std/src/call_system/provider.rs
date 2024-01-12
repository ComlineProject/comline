// No Std Uses

// Crate Uses
use crate::call_system::{CallResult, CallSystem};
use crate::call_system::consumer::Kind;
use crate::message::Message;

// External Uses


pub trait CallSystemProvider: CallSystem {
    async fn send_async_call(&mut self, kind: Kind, message: Message<'_>) -> CallResult<Message>;

    fn send_blocking_call(&mut self, name: &str, message: Message) -> CallResult<Message>;
}
