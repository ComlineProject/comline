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
    call_system::systems::json_rpc::JsonRPCv2
};

impl GreetConsumerProtocol for GreetConsumer {
    fn greet(&self, name: &str) -> APIResult<String> {
        // let mut call = self.caller.write().unwrap();
        //let message = todo!();

        // caller.send_call_indexed(self, 0);
        todo!()
    }
}

pub(crate) async fn main() {
    println!("Running Client");

    let (address, port) = ("127.0.0.1", "2620");
    let full_address = &*(address.to_owned() + ":" + port);

    let setup = ConsumerSetup {
        transport_method: TcpConsumer::with_address(full_address).unwrap().into_threaded(),
        call_system: JsonRPCv2::default().into_threaded(),
        capabilities: vec![],
    };
    //setup.add_capability(GreetConsumer::new());

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

