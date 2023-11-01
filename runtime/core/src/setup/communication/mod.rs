// Relative Modules
pub mod provider;
pub mod consumer;
pub mod methods;

// External Uses
use downcast_rs::{DowncastSync, impl_downcast};


pub trait MessageReceiver: Send + DowncastSync {
    #[allow(unused)]
    fn receive_data(&self, data: &[u8]) {
        todo!()
    }
}
impl_downcast!(sync MessageReceiver);

#[allow(unused)]
pub trait MessageSender: Send + DowncastSync {
    fn send_data(&self, data: &[u8]) {
        todo!()
    }
}
impl_downcast!(sync MessageSender);

pub trait MessageSenderBuilder: MessageSender {
    fn new() -> Self;
}

