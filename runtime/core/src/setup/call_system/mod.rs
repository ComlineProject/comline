// Relative Modules
pub mod meta;
pub mod systems;
pub mod consumer;
pub mod provider;

// Standard Uses
use std::sync::{Arc, RwLock};

// Crate Uses
use crate::setup::call_system::provider::CallSystemProvider;
use crate::setup::communication::consumer::CommunicationConsumer;
use crate::setup::communication::provider::CommunicationProvider;

// External Uses
use downcast_rs::{DowncastSync, impl_downcast};



pub trait CallSystem: Send + DowncastSync {
    fn add_event_listener(&mut self, callback: EventType);

    fn receive_data(&mut self, data: &[u8]);
    fn send_data(&mut self, data: &[u8]);

    // message_format: dyn  MessageFormat
}
impl_downcast!(sync CallSystem);

pub enum Origin {
    Consumer(Arc<RwLock<dyn CommunicationConsumer>>),
    Provider(Arc<RwLock<dyn CommunicationProvider>>)
}

pub const CALL_TIMEOUT: u128 = 800;


pub enum EventType {
    //ReceivedBytes(Box<dyn FnMut(OnceLock<&[u8]>)>),
    ReceivedBytes(fn(&[u8])),
    //ReceivedMessage,
    SentBytes(fn(&[u8])),
    //SentMessage
}

pub enum Event<'data, Incoming, Outgoing> {
    ReceivedBytes(&'data [u8]),
    SentBytes(&'data [u8]),

    ReceivedMessage(&'data Incoming),
    SentMessage(&'data Outgoing),
}

pub trait Callback: Send + Sync {
    fn on_received_data(&mut self, data: &[u8]);
    fn on_sent_data(&mut self, data: &[u8]);
}
