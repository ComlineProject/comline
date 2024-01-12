// Relative Modules
pub mod consumer;
pub mod provider;
pub mod meta;
pub mod default;
pub mod systems;

// No Standard Uses

// Crate Uses

// External Uses
use tokio::sync::watch;


pub trait CallSystem: Send + Sync {
    fn add_event_listener(&mut self, callback: EventType);

    fn receive_data(&mut self, data: &[u8]);
    fn send_data(&mut self, data: &[u8]);
}

pub enum Origin {
    None
    //Consumer(Arc<RwLock<dyn CommunicationConsumer>>),
    //Provider(Arc<RwLock<dyn CommunicationProvider>>)
}

pub enum EventType {
    ReceivedBytes(watch::Receiver<&'static [u8]>),
    SentBytes(watch::Receiver<&'static [u8]>),
}

pub trait Callback: Send + Sync {
    fn on_received_data(&mut self, data: &[u8]);
    fn on_sent_data(&mut self, data: &[u8]);
}

pub type CallResult<T> = Result<T, ()>;
