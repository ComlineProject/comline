// Standard Uses
use std::any::Any;

// Crate Uses
use crate::setup::APIResult;
use crate::setup::call_system::CallSystem;

// External Uses


#[allow(unused)]
pub trait CallSystemProvider: CallSystem {
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

/*
pub mod consumer {
    use std::time::Instant;
    /*
    #[allow(unused)]
    pub fn make_parameters<P, R>(call_system: &mut dyn CallSystemProvider, params: &[P]) -> R
        where
            //C: CallSystem + CallSystemSender,
            //P: for<'a> Deserialize<'a>,
            P: DeserializeOwned
    {
        todo!()
    }

    #[allow(unused)]
    fn send_call_named<C>(call_system: &mut dyn CallSystem, call_meta: &C, name: &str)
        where C: CallProtocolMeta
    {
        todo!()
    }
    */

    use std::any::Any;
    use std::time::Instant;
    use crate::setup::APIResult;
    use crate::setup::call_system::{CALL_TIMEOUT, CallSystemProvider};

    pub fn send_blocking_call<M, R>(
        call_system: &mut dyn CallSystemProvider, name: &str, message: M
    ) -> APIResult<R>
        where
        // C: CallSystem + CallProtocolMeta,
            M: Any + for <'message> Into<&'message [u8]>
    {
        let mut response: Option<&[u8]> = None;
        let start_time = Instant::now();

        /*
        call_system.add_event_listener(EventType::ReceivedBytes(|data| {
            response = Some(data);
        }));
        */

        call_system.send_data(message.into());

        while response.is_none() {
            if start_time.elapsed().as_millis() > CALL_TIMEOUT { return Err(()) }
        }

        let response_message: R = todo!();

        Ok(response_message)
    }
}
*/
