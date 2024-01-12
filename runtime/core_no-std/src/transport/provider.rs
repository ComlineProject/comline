use alloc::boxed::Box;
// No Standard Uses
use alloc::sync::{Arc};
use alloc::vec;
use alloc::vec::Vec;

// Crate Uses
use crate::call_system::{Callback, CallSystem, Origin};
use crate::call_system::provider::CallSystemProvider;

// External Uses
use tokio::sync::RwLock;


pub trait CommunicationProvider {
    fn add_received_data_callback(&mut self, callback: Arc<RwLock<dyn Callback>>);
    async fn listen_for_connections(&mut self);
}


pub struct ProviderSetup {
    pub transport_method: Arc<RwLock<dyn CommunicationProvider>>,
    pub call_system: Option<Arc<RwLock<dyn CallSystemProvider>>>,
    pub capabilities: Vec<Box<dyn ProviderCapability>>
}

impl ProviderSetup {
    pub fn with_transporter(transporter: impl CommunicationProvider) -> Self {
        Self {
            transport_method: Arc::new(RwLock::new(transporter)),
            call_system: None,
            capabilities: vec![],
        }
    }

    pub fn with_call_system<C, Cfn>(mut self, call_system: Cfn) -> Self
        where
            C: CallSystem + CallSystemProvider + Callback,
            Cfn: FnOnce(Origin) -> C
    {
        /*
        let call_system = Arc::new(RwLock::new(
            call_system(Origin::Provider(self.transport_method.clone()))
        ));
        */
        self.transport_method.write().unwrap().add_received_data_callback(call_system.clone());

        self.call_system = Some(call_system);
        self
    }

    pub fn with_capability<C, Cfn>(mut self, capability: Cfn) -> Self
        where
            C: ProviderCapability + 'static,
            Cfn: FnOnce(Arc<RwLock<dyn CallSystemProvider>>) -> C
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

