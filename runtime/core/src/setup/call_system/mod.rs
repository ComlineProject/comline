// Relative Modules
pub mod meta;
pub mod systems;

// Standard Uses
use std::any::Any;
use std::time::Instant;

// Crate Uses
use crate::setup::APIResult;
use crate::setup::call_system::meta::CallProtocolMeta;

// External Uses
use downcast_rs::{DowncastSync, impl_downcast};
use serde::de::DeserializeOwned;


pub trait CallSystem: DowncastSync {
    // type IncomingMessage;
    // type OutgoingMessage;

    fn add_event_listener(&mut self, event_type: EventType, callback: Box<dyn FnMut(&[u8])>);

    fn on_receive_data(&mut self, data: Box<[u8]>);
    fn send_data(&mut self, data: &[u8]);

    // fn on_received_data(&self, data: &[u8]);
    // fn on_sent_data(&self, data: &[u8]);

    // message_format: dyn  MessageFormat
}
impl_downcast!(sync CallSystem);


#[allow(unused)]
pub trait CallSystemSender: CallSystem {
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

    fn send_blocking_call<C, P>(&mut self, call_meta: &C, name: &str, parameters: P) -> P
        where C: CallProtocolMeta, P: Any, Self: Sized
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
    */
}

#[allow(unused)]
pub fn make_parameters<P, R>(call_system: &mut dyn CallSystemSender, params: &[P]) -> R
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

pub fn send_blocking_call<M, R>(
    call_system: &mut dyn CallSystemSender, name: &str, message: M
) -> APIResult<R>
    where
        // C: CallSystem + CallProtocolMeta,
        M: Any + for <'message> Into<&'message [u8]>
{
    let mut response: Option<&[u8]> = None;
    let start_time = Instant::now();

    call_system.add_event_listener(EventType::ReceivedBytes, |data| {
        response = Some(data);
    });

    call_system.send_data(message.into());

    while response.is_none() {
        if start_time.elapsed().as_millis() > CALL_TIMEOUT { return Err(()) }
    }

    let response_message: R = todo!();

    Ok(response_message)
}

pub const CALL_TIMEOUT: u128 = 800;

pub enum EventType {
    ReceivedBytes,
    ReceivedMessage,
    SentBytes,
    SentMessage
}

pub enum Event<'data, Incoming, Outgoing> {
    ReceivedBytes(&'data [u8]),
    SentBytes(&'data [u8]),

    ReceivedMessage(&'data Incoming),
    SentMessage(&'data Outgoing),
}
