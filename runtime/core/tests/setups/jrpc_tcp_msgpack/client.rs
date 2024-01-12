// Standard Uses

// Crate Uses
use crate::setups::jrpc_tcp_msgpack::generated::{
    schemas::GreetConsumerProtocol,
    consumer::GreetConsumer
};

// External Uses
use comline_runtime::setup::{
    CallResult, communication::{
        consumer::{ConsumerSetup, SharedConsumerSetup},
    },
    call_system::systems::json_rpc::JsonRPCv2,
};
use comline_runtime::setup::call_system::consumer::CallSystemConsumer;
use comline_runtime::setup::communication::methods::tcp::consumer::TcpConsumer;


impl<CS: CallSystemConsumer> GreetConsumerProtocol for GreetConsumer<CS> {
    #[allow(unused_variables)]
    fn greet(&self, name: &str) -> CallResult<String> {
        todo!()
    }
}

pub(crate) async fn main() {
    println!("Running Client");

    // Okay so first we need an address to connect to
    let (address, port) = ("127.0.0.1", "2620");
    let full_address = &*(address.to_owned() + ":" + port);

    // So lets wrap all up all the setup in a structure for convenience (might be optional later)
    let setup = ConsumerSetup::with_transport(
        // To transport our messages, we are going to use TCP
        TcpConsumer::with_address(full_address).unwrap(),
    )
        // To talk with the other side, we are going to speak the Msgpack format
        // message_format: MessageFormat::Msgpack,

        // The call system we are going to speak will be json-rpc
        .with_call_system(JsonRPCv2::new)

        // And for capabilities of calls, lets just add a greet API
        .with_capability(GreetConsumer::new)
    ;

    let setup_threaded = setup.into_threaded();

    greet_with_name(setup_threaded);
}


fn greet_with_name(setup: SharedConsumerSetup<TcpConsumer, JsonRPCv2>) {
    let mut setup_write = setup.write().unwrap();
    let greeter = setup_write.capability_mut::<GreetConsumer<JsonRPCv2>>().unwrap();
    let name = "Client";

    println!("[Client] Sending a greet request with name '{}'", name);
    let response = greeter.greet(name).unwrap();
    println!("[Client] Received a greet response saying: '{}'", response);

    assert_eq!("Hello Client", response);
}

