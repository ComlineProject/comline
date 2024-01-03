// Standard Uses
use std::any::Any;

// Crate Uses
use crate::setup::APIResult;
use crate::setup::call_system::CallSystem;

// External Uses


#[allow(unused)]
pub trait CallSystemConsumer: CallSystem {
    /*
    fn make_parameters<P>(&self, params: Vec<P>) -> P
        where P: for<'a> Deserialize<'a>, Self: Sized
    {
        todo!()
    }

    fn send_call_named<C>(&mut self, call_meta: &C, name: &str)
        where C: CallProtocolMeta, Self: Sized
    {
        todo!()
    }
    */

    fn send_blocking_call<P, R>(&mut self, name: &str, parameters: P) -> APIResult<R>
        where P: Any
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
