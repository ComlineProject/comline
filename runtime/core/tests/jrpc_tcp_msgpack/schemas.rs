// This is pretend code replicating the same thing Comline would generate for schemas

// External Uses
use eyre::Result;
use comline_runtime::setup::call_system::meta::CallProtocolMeta;
use comline_runtime::setup::communication::consumer::{ConsumerCapability, ConsumerSetup};
use comline_runtime::setup::communication::provider::{ProviderCapability, ProviderSetup};


pub trait GreetProtocol: CallProtocolMeta {}

pub trait GreetProviderProtocol: GreetProtocol + ProviderCapability {
    fn greet(&self, setup: &mut ProviderSetup, name: &str) -> Result<String>;
}

pub trait GreetConsumerProtocol: GreetProtocol + ConsumerCapability {
    fn greet(&self, setup: &mut ConsumerSetup, name: &str) -> Result<String>;
}

