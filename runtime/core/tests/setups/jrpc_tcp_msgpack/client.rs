// Standard Uses

// Crate Uses
use crate::setups::jrpc_tcp_msgpack::server::GreetConsumer;
use crate::setups::jrpc_tcp_msgpack::schemas::GreetConsumerProtocol;

// External Uses
use comline_runtime::setup::call_system::systems::json_rpc::JsonRPCv2;
use comline_runtime::setup::communication::consumer::ConsumerSetup;
use comline_runtime::setup::communication::methods::tcp::consumer::TcpConsumer;


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


fn greet_with_name(mut setup: &mut ConsumerSetup) {
    let greet_capability = setup.capability_mut::<GreetConsumer>();
    let name = "Client";

    println!("[Client] Sending a greet request with name '{}'", name);

    // TODO: Check how to flow mutability shared here
    /*
    let response = greet_capability.unwrap().greet(&mut setup, name).unwrap();
    println!("[Client] Received a greet response saying: '{}'", response);

    assert_eq!("Hello Client", response);
    */
}

