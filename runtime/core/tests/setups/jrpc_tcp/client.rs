// Standard Uses

// Crate Uses
use crate::setups::jrpc_tcp::generated::{
    schemas::GreetConsumerProtocol,
    consumer::GreetConsumer
};

// External Uses
use comline_runtime::setup::{
    APIResult,
    communication::{
        consumer::{ConsumerSetup, SharedConsumerSetup},
        methods::tcp::consumer::TcpConsumer
    },
    call_system::systems::json_rpc::JsonRPCv2,
    message_format::Message
};
use downcast_rs::Downcast;


impl GreetConsumerProtocol for GreetConsumer {
    fn greet(&self, name: &str) -> APIResult<String> {
        let name = name.to_owned();
        let message = Message::new().parameter(name.as_any());

        let mut caller = self.caller.write().unwrap();
        let result = caller.send_blocking_call("greet", message)?;
        let result = result.downcast::<String>().unwrap();

        Ok(*result)
    }
}

pub(crate) async fn main() {
    println!("Running Client");

    let (address, port) = ("127.0.0.1", "2620");
    let full_address = &*(address.to_owned() + ":" + port);

    let transporter = TcpConsumer::with_address(full_address).unwrap();
    let setup = ConsumerSetup::with_transport(transporter)
        .with_call_system(JsonRPCv2::new)
        .with_capability(GreetConsumer::new);

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

