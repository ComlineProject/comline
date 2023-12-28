// Standard Uses

// Crate Uses
use crate::setups::jrpc_tcp_msgpack::generated::{
    schemas::GreetProviderProtocol, provider::GreetProvider
};

// External Uses
use comline_runtime::setup::APIResult;
use comline_runtime::setup::{
    communication::{methods::tcp::provider::TcpProvider, provider::ProviderSetup},
    call_system::systems::json_rpc::JsonRPCv2,
    message_format::msgpack::MessagePack
};


impl GreetProviderProtocol for GreetProvider {
    fn greet(&self, name: &str) -> APIResult<String> {
        println!("[Server] Received a greet request with name '{}'", name);

        Ok("Hello ".to_owned() + name)
    }
}



pub(crate) async fn main() {
    println!("Running Server");

    // Okay so first we need an address to bind and listen on
    let (address, port) = ("127.0.0.1", "2620");
    let full_address = &*(address.to_owned() + ":" + port);

    let mut setup = ProviderSetup {
        // To transport our messages, lets use TCP
        transport_method: Box::new(
            TcpProvider::with_address_and_callback(full_address).await.unwrap()
        ),

        // For serialization, lets use the Msgpack format
        message_format: Box::new(MessagePack),

        // For the call setup, lets use Json RPC
        call_system: Box::new(JsonRPCv2::new()),
        capabilities: vec![]
    };

    // And the capabilities of communication
    setup.add_capabilities(vec![
        Box::new(GreetProvider)
    ]);

    respond_to_incoming_hellos(&mut setup).await;
}

async fn respond_to_incoming_hellos(setup: &mut ProviderSetup) {
    // We re only downcasting here because we need to just listen to one connection and leave
    // but in many cases it might not be necessary to downcast
    let tcp_listener = setup.transport_method.downcast_mut::<TcpProvider>().unwrap();

    tcp_listener.listen_incoming_connection(&mut *setup.call_system, &*setup.message_format).await;
}

