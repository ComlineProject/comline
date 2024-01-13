// These structures are just mimics of what Comline would generate

pub mod schemas {

    // Internal Uses
    use comline_runtime::setup::CallResult;
    use comline_runtime::setup::{
        call_system::meta::CallProtocolMeta,
        transport::{
            provider::ProviderCapability,
            consumer::ConsumerCapability
        }
    };

    // External Uses


    pub trait GreetProtocol: CallProtocolMeta {}

    pub trait GreetProviderProtocol: GreetProtocol + ProviderCapability {
        fn greet(&self, name: &str) -> CallResult<String>;
    }

    pub trait GreetConsumerProtocol: GreetProtocol + ConsumerCapability {
        fn greet(&self, name: &str) -> CallResult<String>;
    }
}

pub mod provider {
    // Standard Uses
    use std::sync::{Arc, RwLock};

    // Crate Uses
    use super::schemas::GreetProtocol;

    // Internal Uses
    use comline_runtime::setup::{
        call_system::meta::CallProtocolMeta,
        transport::provider::ProviderCapability
    };

    #[allow(dead_code)]
    pub struct GrootProvider;


    pub struct GreetProvider<CS> {
        #[allow(dead_code)]
        pub(crate) caller: Arc<RwLock<CS>>,
    }
    impl<CS> GreetProvider<CS> {
        pub fn new(caller: Arc<RwLock<CS>>) -> Self { Self { caller } }
    }

    impl<CS> GreetProtocol for GreetProvider<CS> {}
    impl<CS> ProviderCapability for GreetProvider<CS> {}

    impl<CS> CallProtocolMeta for GreetProvider<CS> {
        #[inline]
        fn calls_names(&self) -> &'static [&'static str] {
            &["greet"]
        }
    }

    pub trait GreetProtocoAl<const CALL_COUNT: usize> {
        const CALL_NAMES: [&'static str; CALL_COUNT];
    }
}

pub mod consumer {
    // Standard Uses
    use std::sync::{Arc, RwLock};

    // Crate Uses
    use super::schemas::GreetProtocol;

    // External Uses
    use comline_runtime::setup::{
        transport::{
            consumer::ConsumerCapability
        },
        call_system::{
            consumer::CallSystemConsumer,
            meta::CallProtocolMeta
        },
    };

    pub struct GreetConsumer<CS: CallSystemConsumer> {
        #[allow(dead_code)]
        pub(crate) caller: Arc<RwLock<CS>>,
    }
    impl<CS: CallSystemConsumer> GreetConsumer<CS> {
        pub fn new(caller: Arc<RwLock<CS>>) -> Self { Self { caller } }
    }

    impl<CS: CallSystemConsumer> GreetProtocol for GreetConsumer<CS> {}
    impl<CS: CallSystemConsumer> ConsumerCapability for GreetConsumer<CS> {}
    impl<CS: CallSystemConsumer> CallProtocolMeta for GreetConsumer<CS> {
        //const CALL_NAMES: &'static [&'static str] = &["greet];

        fn calls_names(&self) -> &'static [&'static str] {
            todo!()
        }
    }
}

