// Standard Uses

// Crate Uses
use crate::setups::jrpc_tcp::generated::{
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

    let (address, port) = ("127.0.0.1", "2620");
    let full_address = &*(address.to_owned() + ":" + port);

    let call_system = JsonRPCv2::default().into_threaded();
    let mut setup = ProviderSetup {
        transport_method: TcpProvider::with_address(full_address).await.unwrap().into_threaded(),
            //.and_callback(|| &call_system.on_received_data),
        message_format: Box::new(MessagePack),
        call_system,
        capabilities: vec![]
    };
    setup.add_capabilities(vec![
        Box::new(GreetProvider)
    ]);

    respond_to_incoming_hellos(&mut setup).await;
}

async fn respond_to_incoming_hellos(setup: &mut ProviderSetup) {
    let provider = setup.transport_method.read().unwrap();
    provider.downcast_ref::<TcpProvider>().unwrap()
        .listen_incoming_connection(/*&mut *setup.call_system*/)
        .await;
}

