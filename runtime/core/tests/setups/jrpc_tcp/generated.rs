// These structures are just mimics of what Comline would generate

pub mod schemas {

    // Internal Uses
    use comline_runtime::setup::APIResult;
    use comline_runtime::setup::{
        call_system::meta::CallProtocolMeta,
        communication::{
            provider::ProviderCapability,
            consumer::ConsumerCapability
        }
    };

    // External Uses


    pub trait GreetProtocol: CallProtocolMeta {}

    pub trait GreetProviderProtocol: GreetProtocol + ProviderCapability {
        fn greet(&self, name: &str) -> APIResult<String>;
    }

    pub trait GreetConsumerProtocol: GreetProtocol + ConsumerCapability {
        fn greet(&self, name: &str) -> APIResult<String>;
    }
}

pub mod provider {
    // Crate Uses
    use super::schemas::GreetProtocol;

    // Internal Uses
    use comline_runtime::setup::{
        call_system::meta::CallProtocolMeta
    };
    use comline_runtime::setup::{
        communication::{
            MessageReceiver, MessageSender,
            provider::ProviderCapability
        }
    };

    pub struct GreetProvider;

    impl GreetProtocol for GreetProvider {}
    impl ProviderCapability for GreetProvider {}
    impl MessageSender for GreetProvider {}
    impl MessageReceiver for GreetProvider {}
    impl CallProtocolMeta for GreetProvider {
        fn calls_names(&self) -> &'static [&'static str] {
            &["greet"]
        }
    }
}

pub mod consumer {
    // Standard Uses
    use std::sync::{Arc, RwLock};

    // Crate Uses
    use super::schemas::GreetProtocol;

    // External Uses
    use comline_runtime::setup::{
        communication::{
            MessageReceiver, MessageSender,
            consumer::ConsumerCapability
        },
        call_system::{CallSystem, meta::CallProtocolMeta},
    };

    pub struct GreetConsumer {
        #[allow(unused_variables)]
        pub(crate) caller: Arc<RwLock<dyn CallSystem>>,
    }

    impl GreetProtocol for GreetConsumer {}
    impl ConsumerCapability for GreetConsumer {}
    impl MessageSender for GreetConsumer {}
    impl MessageReceiver for GreetConsumer {}
    impl CallProtocolMeta for GreetConsumer {
        fn calls_names(&self) -> &'static [&'static str] {
            &["greet"]
        }
    }
}

