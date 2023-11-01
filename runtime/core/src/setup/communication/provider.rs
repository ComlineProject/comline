// Standard Uses
use std::sync::Arc;

// Crate Uses
use crate::setup::communication::{MessageReceiver, MessageSender};
use crate::setup::call_system::CallSystem;
use crate::setup::message_format::MessageFormat;

// External Uses
use async_trait::async_trait;
use downcast_rs::{DowncastSync, impl_downcast};


#[async_trait]
pub trait CommunicationProvider: Send + DowncastSync {
    async fn listen_for_connections(
        &mut self, call_system: &mut dyn CallSystem, message_format: &dyn MessageFormat);
}
impl_downcast!(sync CommunicationProvider);


pub struct ProviderSetup {
    pub transport_method: Box<dyn CommunicationProvider>,
    pub message_format: Box<dyn MessageFormat>,
    pub call_system: Box<dyn CallSystem>,
    pub capabilities: Vec<Box<dyn MessageReceiver>>
}

impl ProviderSetup {
    pub fn into_arc(self) -> Arc<Self> { Arc::new(self) }

    pub fn add_capabilities(&mut self, mut capabilities: Vec<Box<dyn MessageReceiver>>) {
        self.capabilities.append(&mut capabilities);
    }
}

pub trait ProviderCapability: MessageSender + MessageReceiver {}

