// Standard Uses
use std::sync::{Arc, RwLock};

// Crate Uses
use crate::setup::call_system::{Callback, Origin};
use crate::setup::call_system::provider::CallSystemProvider;

// External Uses
use async_trait::async_trait;
use downcast_rs::{DowncastSync, impl_downcast};


#[async_trait]
pub trait CommunicationProvider: DowncastSync {
    fn add_received_data_callback(&mut self, callback: Arc<RwLock<dyn Callback>>);
    async fn listen_for_connections(&mut self);
}
impl_downcast!(sync CommunicationProvider);


pub struct ProviderSetup<T, CS> {
    pub transporter: Arc<RwLock<T>>,
    pub call_system: Option<Arc<RwLock<CS>>>,
    pub capabilities: Vec<Box<dyn ProviderCapability>>
}

impl<T, CS> ProviderSetup<T, CS> where T: CommunicationProvider, CS: CallSystemProvider {
    pub fn with_transporter(transporter: T) -> Self {
        Self {
            transporter: Arc::new(RwLock::new(transporter)),
            call_system: None,
            capabilities: vec![],
        }
    }

    pub fn with_call_system<C, Cfn>(mut self, call_system: Cfn) -> Self
        where
            CS: CallSystemProvider + Callback,
            Cfn: FnOnce(Origin) -> CS
    {
        let call_system = Arc::new(RwLock::new(
            call_system(Origin::Provider(self.transporter.clone()))
        ));
        self.transporter.write().unwrap().add_received_data_callback(call_system.clone());

        self.call_system = Some(call_system);
        self
    }

    pub fn with_capability<C, Cfn>(mut self, capability: Cfn) -> Self
        where
            C: ProviderCapability + 'static,
            Cfn: FnOnce(Arc<RwLock<CS>>) -> C
    {
        self.capabilities.push(Box::new(capability(
            self.call_system.as_ref().unwrap().clone()
        )));
        self
    }

    pub fn add_capabilities(&mut self, mut capabilities: Vec<Box<dyn ProviderCapability>>) {
        self.capabilities.append(&mut capabilities);
    }

    pub fn into_threaded(self) -> Arc<Self> { Arc::new(self) }
}

pub trait ProviderCapability {}

