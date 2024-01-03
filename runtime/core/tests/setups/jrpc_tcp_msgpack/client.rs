// Standard Uses
use std::sync::{Arc, RwLock};

// Crate Uses
use crate::setups::jrpc_tcp_msgpack::generated::{
    schemas::GreetConsumerProtocol,
    consumer::GreetConsumer
};

// External Uses
use comline_runtime::setup::{APIResult, communication::{
    consumer::{ConsumerSetup, SharedConsumerSetup},
    methods::tcp::consumer::TcpConsumer
}, call_system::{
    CallSystemProvider,
    systems::json_rpc::JsonRPCv2
}, call_system};


impl GreetConsumerProtocol for GreetConsumer {
    fn greet(&self, name: &str) -> APIResult<String> {
        let mut caller = self.caller.write().unwrap();

        let message: String = call_system::make_parameters(&mut *caller, &[name.to_owned()]);
        caller.send_blocking_call("greet", &*message)
        //call_system::send_blocking_call(&mut *caller, "greet", message)
    }

    /*
    fn greet(&self, name: &str) -> APIResult<String> {
        let mut caller= self.caller.write().unwrap() as _;
        let message = caller.into_message(name);

        let mut caller= self.caller.write().unwrap()
            .downcast_ref::<dyn CallSystemSender>().unwrap();

        caller.send_call_while_blocking(self, 0, message)

        todo!()
    }
     */
}

pub(crate) async fn main() {
    println!("Running Client");

    // Okay so first we need an address to connect to
    let (address, port) = ("127.0.0.1", "2620");
    let full_address = &*(address.to_owned() + ":" + port);

    // So lets wrap all up all the setup in a structure for convenience (might be optional later)
    let setup = ConsumerSetup {
        // To transport our messages, we are going to use TCP
        transport_method: TcpConsumer::with_address(full_address).unwrap().into_threaded(),

        // To talk with the other side, we are going to speak the Msgpack format
        // message_format: MessageFormat::Msgpack,

        // The call system we are going to speak will be json-rpc
        call_system: Arc::new(RwLock::new(JsonRPCv2::default())),

        capabilities: vec![],
    }.add_default_capability(|caller| GreetConsumer::new(caller));

    let setup_threaded = setup.into_threaded();

    greet_with_name(setup_threaded);
}


fn greet_with_name(setup: SharedConsumerSetup) {
    let mut setup_write = setup.write().unwrap();
    let greeter = setup_write.capability_mut::<GreetConsumer>().unwrap();
    let name = "Client";

    println!("[Client] Sending a greet request with name '{}'", name);
    let response = greeter.greet(name).unwrap();
    println!("[Client] Received a greet response saying: '{}'", response);

    assert_eq!("Hello Client", response);
}

