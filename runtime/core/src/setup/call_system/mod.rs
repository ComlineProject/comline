// Relative Modules
pub mod meta;
pub mod systems;
pub mod consumer;
pub mod provider;

// Standard Uses

// Crate Uses

// External Uses
use downcast_rs::{DowncastSync, impl_downcast};


pub trait CallSystem: DowncastSync {
    fn add_event_listener(&mut self, callback: EventType);

    fn on_receive_data(&mut self, data: Box<[u8]>);
    fn send_data(&mut self, data: &[u8]);

    // fn on_received_data(&self, data: &[u8]);
    // fn on_sent_data(&self, data: &[u8]);

    // message_format: dyn  MessageFormat
}
impl_downcast!(sync CallSystem);



pub const CALL_TIMEOUT: u128 = 800;

pub enum EventType {
    //ReceivedBytes(Box<dyn FnMut(OnceLock<&[u8]>)>),
    ReceivedBytes(fn(&[u8])),
    //ReceivedMessage,
    //SentBytes,
    //SentMessage
}

pub enum Event<'data, Incoming, Outgoing> {
    ReceivedBytes(&'data [u8]),
    SentBytes(&'data [u8]),

    ReceivedMessage(&'data Incoming),
    SentMessage(&'data Outgoing),
}
