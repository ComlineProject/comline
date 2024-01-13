// Relative Modules
pub mod meta;
pub mod systems;
pub mod consumer;
pub mod provider;

// Standard Uses
use std::sync::{Arc, RwLock};

// Crate Uses
use crate::setup::call_system::provider::CallSystemProvider;
use crate::setup::transport::consumer::CommunicationConsumer;
use crate::setup::transport::provider::CommunicationProvider;
use crate::setup::call_system::consumer::CallSystemConsumer;

// External Uses
use downcast_rs::{DowncastSync, impl_downcast};
use tokio::sync::watch;


pub const DEFAULT_CALL_TIMEOUT: u128 = 800;

pub trait CallSystem: Send + Sync + DowncastSync {
    fn add_event_listener(&mut self, callback: EventType);

    fn receive_data(&mut self, data: &[u8]);
    fn send_data(&mut self, data: &[u8]);
}
impl_downcast!(sync CallSystem);

pub enum Origin {
    Consumer(Arc<RwLock<dyn CommunicationConsumer>>),
    Provider(Arc<RwLock<dyn CommunicationProvider>>)
}



pub enum EventType {
    ReceivedBytes(watch::Receiver<Option<Box<[u8]>>>),
    SentBytes(watch::Receiver<Option<Box<[u8]>>>),
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


/// Kind of call name
pub enum Kind {
    Id(u16),
    Named(String)
}

pub trait CallSystemBuilder {
    fn new_consumer() -> impl CallSystemConsumer;
    fn new_provider() -> impl CallSystemProvider;
}