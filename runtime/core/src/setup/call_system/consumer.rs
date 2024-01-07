// Standard Uses
use std::any::Any;

// Crate Uses
use crate::setup::APIResult;
use crate::setup::call_system::CallSystem;

// External Uses
use crate::setup::message_format::Message;


#[allow(unused)]
pub trait CallSystemConsumer: CallSystem {
    /*
    fn send_call_named<C>(&mut self, call_meta: &C, name: &str)
        where C: CallProtocolMeta, Self: Sized
    {
        todo!()
    }
    */

    fn send_blocking_call(&mut self, name: &str, message: Message) -> APIResult<Box<dyn Any>>
    {
        todo!()
    }

    /*
    fn send_call_indexed<M: for<'a> Deserialize<'a>>(
        &mut self, meta: &dyn CallProtocolMeta, index: usize, message: M
    );
    */

    /*
    fn send_call_indexed_block<Result>(
        &mut self, call_meta: &dyn CallProtocolMeta
    ) -> APIResult<Result> {
        todo!()
    }
    */
}
