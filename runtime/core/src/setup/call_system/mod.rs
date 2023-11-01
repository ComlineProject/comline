// Relative Modules
pub mod meta;
pub mod systems;

// Standard Uses

// Crate Uses
use crate::setup::call_system::meta::CallProtocolMeta;
use crate::setup::communication::consumer::ConsumerCapability;

// External Uses
use downcast_rs::{DowncastSync, impl_downcast};



pub type ListenerFn = fn(&[u8]);

pub trait CallSystem: Send + DowncastSync {
    // type IncomingMessage;
    // type OutgoingMessage;

    fn add_event_listener(&mut self, callback: ListenerFn);

    fn on_receive_data(&mut self, data: Box<[u8]>);
    fn on_send_data(&mut self, data: Box<[u8]>);

    fn on_received_data(&self, data: &[u8]);
    fn on_sent_data(&self, data: &[u8]);

    // message_format: dyn  MessageFormat
    #[allow(unused)]
    fn send_call(
        &self, call_meta: &dyn CallProtocolMeta, index: usize
    );
}
impl_downcast!(sync CallSystem);


pub enum Event<'a, Incoming, Outgoing> {
    ReceivedBytes(&'a [u8]),
    SentBytes(&'a [u8]),

    ReceivedMessage(&'a Incoming),
    SentMessage(&'a Outgoing),
}
