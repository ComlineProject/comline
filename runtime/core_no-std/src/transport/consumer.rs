// No Std Uses

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use tokio::sync::RwLock;
// Crate Uses
use crate::call_system::consumer::CallSystemConsumer;
use crate::call_system::{CallSystem, Origin};

// External Uses


pub trait CommunicationConsumer: Send + Sync {
    async fn connect_to_provider(&self);

    fn send_data(&mut self, data: &[u8]) -> eyre::Result<()>;
    async fn send_data_async(&mut self, data: &[u8]) -> eyre::Result<()>;
}

pub struct ConsumerSetup {
    pub transport_method: Arc<RwLock<dyn CommunicationConsumer>>,
    pub call_system: Option<Arc<RwLock<dyn CallSystemConsumer>>>,
    pub capabilities: Vec<Box<dyn ConsumerCapability>>
}

impl ConsumerSetup {
    pub fn with_transport<T: CommunicationConsumer + 'static>(transporter: T) -> Self {
        Self {
            transport_method: Arc::new(RwLock::new(transporter)),
            call_system: None,
            capabilities: vec![],
        }
    }

    pub fn with_call_system<C, CFn>(mut self, call_system: CFn) -> Self
        where
            C: CallSystem + CallSystemConsumer,
            CFn: FnOnce(Origin) -> C
    {
        /*
        self.call_system = Some(Arc::new(RwLock::new(call_system(
            Origin::Consumer(self.transport_method.clone()),
        ))));
        */
        self
    }

    pub fn with_capability<C, Cfn>(mut self, capability: Cfn) -> Self
        where
            C: ConsumerCapability,
            Cfn: FnOnce(Arc<RwLock<dyn CallSystemConsumer>>) -> C
    {
        self.capabilities.push(Box::new(capability(self.call_system.as_ref().unwrap().clone())));
        self
    }

    pub fn into_threaded(self) -> Arc<RwLock<Self>> { Arc::new(RwLock::new(self)) }

    pub fn add_default_capability<C, Cfn>(mut self, capability: Cfn) -> Self
        where
            C: ConsumerCapability,
            Cfn: FnOnce(Arc<RwLock<dyn CallSystemConsumer>>) -> C
    {
        self.capabilities.push(Box::new(capability(self.call_system.as_ref().unwrap().clone())));
        self
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

pub trait ConsumerCapability {
    //fn setup(&self) -> Arc<RwLock<ConsumerSetup>>;
}

