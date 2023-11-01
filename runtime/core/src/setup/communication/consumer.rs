// Standard Uses
use std::sync::Arc;

// Crate Uses
use crate::setup::call_system::CallSystem;
use crate::setup::communication::{MessageReceiver, MessageSender};

// External Uses
use async_trait::async_trait;
use downcast_rs::impl_downcast;


#[async_trait]
pub trait CommunicationConsumer: Send {
    async fn connect_to_provider(&self);
}

#[allow(unused)]
pub struct ConsumerSetup {
    pub transport_method: Box<dyn CommunicationConsumer>,
    // pub message_format: MessageFormat,
    pub call_system: Box<dyn CallSystem>,
    pub capabilities: Vec<Box<dyn ConsumerCapability>>
}

impl ConsumerSetup {
    pub fn into_arc(self) -> Arc<Self> { Arc::new(self) }
}

impl ConsumerSetup {
    pub fn add_capabilities(&mut self, mut capabilities: Vec<Box<dyn ConsumerCapability>>) {
        self.capabilities.append(&mut capabilities);
    }

    /*
    pub fn capability<C: ConsumerCapability>(&self) -> Option<&C> {
        for capability in self.capabilities.iter() {
            if let Some(cap) = capability.downcast_ref::<C>() {
                return Some(&*cap);
            }
        }

        None
    }
    */

    pub fn capability_mut<C: ConsumerCapability>(&mut self) -> Option<&mut C> {
        for capability in &mut self.capabilities {
            if let Some(cap) = capability.downcast_mut::<C>() {
                return Some(&mut *cap);
            }
        }

        None
    }

    #[allow(unused)]
    pub fn send_message(&self, message: &[u8]) {
        todo!()
    }
}

pub trait ConsumerCapability: MessageSender + MessageReceiver {}
impl_downcast!(sync ConsumerCapability);

