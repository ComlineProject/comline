// Standard Uses

// Crate Uses
use crate::setups::jrpc_tcp_msgpack::generated::{
    schemas::GreetProviderProtocol, provider::GreetProvider
};

// External Uses
use comline_runtime::setup::CallResult;
use comline_runtime::setup::{
    communication::{methods::tcp::provider::TcpProvider, provider::ProviderSetup},
    call_system::systems::json_rpc::JsonRPCv2,
};


impl<CS> GreetProviderProtocol for GreetProvider<CS> {
    fn greet(&self, name: &str) -> CallResult<String> {
        println!("[Server] Received a greet request with name '{}'", name);

        Ok("Hello ".to_owned() + name)
    }
}



pub(crate) async fn main() {
    println!("Running Server");

    // So first we need an address to bind and listen on
    let (address, port) = ("127.0.0.1", "2620");
    let full_address = &*(address.to_owned() + ":" + port);

    // So lets wrap all up the setup parts in a structure for convenience (might be optional later)
    let mut setup = ProviderSetup::with_transporter(
        // To transport our messages, lets use TCP
        TcpProvider::with_address(full_address).await.unwrap()
    )
        // For the call system, lets use Json RPC
        .with_call_system::<JsonRPCv2, _>(JsonRPCv2::new)

        // For serialization, lets use the Msgpack format
        // message_format: Box::new(MessagePack),

        // And for capabilities of calls, lets just add a greet API
        .with_capability(GreetProvider::new)
    ;

    respond_to_incoming_hellos(&mut setup).await;
}

async fn respond_to_incoming_hellos(setup: &mut ProviderSetup<TcpProvider, JsonRPCv2>) {
    setup.transporter.read().unwrap()
        .listen_incoming_connection(/*&mut *setup.call_system*/)
        .await;
}

