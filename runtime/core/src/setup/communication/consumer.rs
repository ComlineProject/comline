// Standard Uses
use std::sync::{Arc, RwLock};

// Crate Uses
use crate::setup::communication::{MessageReceiver, MessageSender};
use crate::setup::call_system::provider::CallSystemProvider;

// External Uses
use async_trait::async_trait;
use downcast_rs::{DowncastSync, impl_downcast};


#[async_trait]
pub trait CommunicationConsumer: Send + Sync {
    async fn connect_to_provider(&self);
    async fn send_data(&self, data: &[u8]) -> eyre::Result<()>;
}

pub struct ConsumerSetup {
    pub transport_method: Arc<RwLock<dyn CommunicationConsumer>>,
    pub call_system: Arc<RwLock<dyn CallSystemProvider>>,
    // pub message_format: MessageFormat,
    pub capabilities: Vec<Box<dyn ConsumerCapability>>
}

impl ConsumerSetup {
    /*
    pub fn new<C: CallSystem, T: CommunicationConsumer>(call_system: C, transport_method: T) -> Self {
        Self {
            transport_method: Box::new(transport_method),
            call_system: Arc::new(RwLock::new(call_system)),
            capabilities: vec![],
        }
    }
    */

    pub fn into_threaded(self) -> Arc<RwLock<Self>> { Arc::new(RwLock::new(self)) }

    /*
    pub fn add_default_capability<C: ConsumerCapability>(mut self, capability: C) -> Self {
        self.capabilities.push(Box::new(capability));
        self
    }
    */

    pub fn add_default_capability<C, Cfn>(mut self, capability: Cfn) -> Self
        where C: ConsumerCapability, Cfn: FnOnce(Arc<RwLock<dyn CallSystemProvider>>) -> C
    {
        self.capabilities.push(Box::new(capability(self.call_system.clone())));
        self
    }

    pub fn add_capability<
        C: ConsumerCapability,
        Cfn: Fn(&ConsumerSetup) -> C
    >(
        &mut self, capability_fn: Cfn
    ) {
        self.capabilities.push(Box::new(capability_fn(&*self)));
    }

    pub fn add_capabilities(&mut self, mut capabilities: Vec<Box<dyn ConsumerCapability>>) {
        self.capabilities.append(&mut capabilities);
    }

    pub fn capability<C: ConsumerCapability>(&self) -> Option<&C> {
        for capability in self.capabilities.iter() {
            if let Some(cap) = capability.downcast_ref::<C>() {
                return Some(cap);
            }
        }

        None
    }

    pub fn capability_mut<C: ConsumerCapability>(&mut self) -> Option<&mut C> {
        for capability in &mut self.capabilities {
            if let Some(cap) = capability.downcast_mut::<C>() {
                return Some(&mut *cap);
            }
        }

        None
    }
}


pub type SharedConsumerSetup = Arc<RwLock<ConsumerSetup>>;

pub trait ConsumerCapability: DowncastSync + MessageSender + MessageReceiver {
    //fn setup(&self) -> Arc<RwLock<ConsumerSetup>>;
}
impl_downcast!(sync ConsumerCapability);

