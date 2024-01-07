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
    use std::sync::{Arc, RwLock};
    // Crate Uses
    use super::schemas::GreetProtocol;

    // Internal Uses
    use comline_runtime::setup::call_system::meta::CallProtocolMeta;
    use comline_runtime::setup::call_system::provider::CallSystemProvider;
    use comline_runtime::setup::communication::{
        provider::ProviderCapability
    };

    #[allow(dead_code)]
    pub struct GrootProvider;


    pub struct GreetProvider {
        #[allow(dead_code)]
        pub(crate) caller: Arc<RwLock<dyn CallSystemProvider>>,
    }
    impl GreetProvider {
        pub fn new(caller: Arc<RwLock<dyn CallSystemProvider>>) -> Self { Self { caller } }
    }

    impl GreetProtocol for GreetProvider {}
    impl ProviderCapability for GreetProvider {}
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
            consumer::ConsumerCapability
        },
        call_system::{
            consumer::CallSystemConsumer,
            meta::CallProtocolMeta
        },
    };

    pub struct GreetConsumer {
        #[allow(dead_code)]
        pub(crate) caller: Arc<RwLock<dyn CallSystemConsumer>>,
    }
    impl GreetConsumer {
        pub fn new(caller: Arc<RwLock<dyn CallSystemConsumer>>) -> Self { Self { caller } }
    }

    impl GreetProtocol for GreetConsumer {}
    impl ConsumerCapability for GreetConsumer {}
    impl CallProtocolMeta for GreetConsumer {
        fn calls_names(&self) -> &'static [&'static str] {
            &["greet"]
        }
    }
}

