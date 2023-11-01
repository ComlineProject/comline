/*
use tokio::tokio_main;

#[tokio_main]
fn main() {
    // This entry point is just an example of simulation, you would do differently
    // if not simulating both parts

    // Lets make a thread for the server, pretending its a different process
    let server_thread = std::thread::Builder::new().name("client_thread".into())
        .spawn(server::main).unwrap();

    // And for the client we just run in our existing thread
    let client_thread = std::thread::Builder::new().name("server_thread".into())
        .spawn(client::main).unwrap();

    // server_thread.join().unwrap();
    // client_thread.join().unwrap();
}


mod schemas {
    // This is pretend code replicating the same thing Comline would generate for schemas

    // External Uses
    use comline_runtime::setup::communication::MessageReceiver;

    pub trait GreetProtocol: MessageReceiver {
        fn greet(&self, name: &str) -> String;
    }
}

mod server {
    // Standard Uses

    // Crate Uses
    use crate::schemas::GreetProtocol;

    // External Uses
    use comline_runtime::setup::communication::MessageReceiver;
    use comline_runtime::setup::communication::provider::ProviderSetup;
    use comline_runtime::setup::communication::tcp::provider::TcpProvider;
    use comline_runtime::setup::call_system::json_rpc::JsonRPCv2;


    pub struct GreetProvider;
    impl MessageReceiver for GreetProvider {}
    impl GreetProtocol for GreetProvider {
        fn greet(&self, name: &str) -> String {
            "Hello".to_owned() + name
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
                TcpProvider::with_address_and_callback(full_address).unwrap()
            ),

            // For serialization, lets use the Msgpack format
            // message_format: MessageFormat::Msgpack,

            // For the call setup, lets use Json RPC
            call_system: Box::new(JsonRPCv2::new()),

            // And the capabilities of communication
            capabilities: vec![Box::new(GreetProvider)]
        };

        respond_to_incoming_hellos(&mut setup).await;
    }

    async fn respond_to_incoming_hellos(setup: &mut ProviderSetup) {
        setup.transport_method.listen_for_connections().await;
    }
}

mod client {
    // Standard Uses

    // Crate Uses
    use crate::schemas::GreetProtocol;
    use crate::server::GreetProvider;

    // External Uses
    use comline_runtime::setup::call_system::json_rpc::JsonRPCv2;
    use comline_runtime::setup::communication::consumer::ConsumerSetup;
    use comline_runtime::setup::communication::tcp::consumer::TcpConsumer;


    pub(crate) fn main() {
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

        greet_with_name(&mut setup);
    }


    fn greet_with_name(setup: &mut ConsumerSetup) {
        let greet_capability = setup.capability::<GreetProvider>();
        let name = "Client";

        let response = greet_capability.greet(name);
        assert_eq!("Hello Client", response);
    }
}
*/

fn main() {}
