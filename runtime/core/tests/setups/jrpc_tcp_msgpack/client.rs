// Standard Uses

// Crate Uses
use crate::setups::jrpc_tcp_msgpack::generated::{
    schemas::GreetConsumerProtocol,
    consumer::GreetConsumer
};

// External Uses
use comline_runtime::setup::APIResult;
use comline_runtime::setup::communication::{
    consumer::ConsumerSetup,
    methods::tcp::consumer::TcpConsumer
};
use comline_runtime::setup::call_system::systems::json_rpc::JsonRPCv2;


impl GreetConsumerProtocol for GreetConsumer {
    fn greet(&self, name: &str) -> APIResult<String> {
        // let setup: &mut ConsumerSetup = todo!();
        // setup.call_system.send_call(self, 0);

        todo!()
    }
}

pub(crate) async fn main() {
    println!("Running Client");

    // Okay so first we need an address to connect to
    let (address, port) = ("127.0.0.1", "2620");
    let full_address = &*(address.to_owned() + ":" + port);

    // So lets wrap all up all the setup in a structure for convenience (might be optional later)
    let mut setup = ConsumerSetup {
        // To transport our messages, we are going to use TCP
        transport_method: Box::new(
            TcpConsumer::with_address_and_callback(full_address).unwrap()
        ),
        // To talk with the other side, we are going to speak the Msgpack format
        // message_format: MessageFormat::Msgpack,
        // The call system we are going to speak will be json-rpc
        call_system: Box::new(JsonRPCv2::new()),
        capabilities: vec![],
    };

    setup.add_capabilities(vec![
        Box::new(GreetConsumer)
    ]);

    greet_with_name(&mut setup);
}


fn greet_with_name(setup: &mut ConsumerSetup) {
    let greeter = setup.capability_mut::<GreetConsumer>().unwrap();
    let name = "Client";

    println!("[Client] Sending a greet request with name '{}'", name);

    let response = greeter.greet(name).unwrap();
    println!("[Client] Received a greet response saying: '{}'", response);

    assert_eq!("Hello Client", response);
}

