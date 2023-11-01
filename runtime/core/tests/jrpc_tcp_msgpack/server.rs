// Standard Uses

// Crate Uses
use crate::jrpc_tcp_msgpack::schemas::{GreetConsumerProtocol, GreetProtocol, GreetProviderProtocol};

// External Uses
use eyre::Result;
use comline_runtime::setup::call_system::meta::CallProtocolMeta;
use comline_runtime::setup::communication::{MessageReceiver, MessageSender};
use comline_runtime::setup::communication::provider::{ProviderCapability, ProviderSetup};
use comline_runtime::setup::communication::methods::tcp::provider::TcpProvider;
use comline_runtime::setup::communication::consumer::{ConsumerCapability, ConsumerSetup};
use comline_runtime::setup::call_system::systems::json_rpc::JsonRPCv2;
use comline_runtime::setup::message_format::msgpack::MessagePack;


// These provider structures are just mimics of what Comline would generate
pub struct GrootProvider;
impl MessageReceiver for GrootProvider {}
impl MessageSender for GrootProvider {}


pub struct GreetProvider;

impl GreetProtocol for GreetProvider {}
impl ProviderCapability for GreetProvider {}
impl MessageSender for GreetProvider {}
impl MessageReceiver for GreetProvider {}
impl CallProtocolMeta for GreetProvider {
    fn calls(&self) -> &'static [&'static str] {
        &["greet"]
    }
}

#[allow(unused)]
impl<'setup> GreetProviderProtocol for GreetProvider {
    fn greet(&self, setup: &mut ProviderSetup, name: &str) -> Result<String> {
        println!("[Server] Received a greet request with name '{}'", name);

        Ok("Hello ".to_owned() + name)
    }
}


pub struct GreetConsumer;

impl GreetProtocol for GreetConsumer {}
impl ConsumerCapability for GreetConsumer {}
impl MessageSender for GreetConsumer {}
impl MessageReceiver for GreetConsumer {}
impl CallProtocolMeta for GreetConsumer {
    fn calls(&self) -> &'static [&'static str] {
        &["greet"]
    }
}

#[allow(unused)]
impl GreetConsumerProtocol for GreetConsumer {
    fn greet(&self, setup: &mut ConsumerSetup, name: &str) -> Result<String> {
        setup.call_system.send_call(self, 0);

        todo!()
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

