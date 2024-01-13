// Standard Uses

// Crate Uses
use crate::setups::jrpc_tcp::generated::{
    schemas::consumer::GreetConsumerProtocol,
    consumer::GreetConsumer
};

// External Uses
use comline_runtime::setup::{
    transport::{
        consumer::ConsumerSetup,
        methods::tcp::consumer::TcpConsumer
    },
    call_system::{
        consumer::CallSystemConsumer,
        meta::CallProtocolMeta, Kind,
        systems::json_rpc::JsonRPCv2
    },
    CallResult
};


impl<CS: CallSystemConsumer> GreetConsumerProtocol for GreetConsumer<CS> {
    fn greet(&self, name: &str) -> CallResult<String> {
        let call_name = Kind::Named(self.call_name_from_id(0).unwrap().to_owned());
        let call = self.make_call(name.to_owned());

        let mut caller = self.caller.write().unwrap();

        let result = caller.send_blocking_call(call_name, call)?;
        Ok(result)
    }
}

pub(crate) async fn main() {
    println!("Running Client");

    let (address, port) = ("127.0.0.1", "2620");
    let full_address = &*(address.to_owned() + ":" + port);

    let transporter = TcpConsumer::with_address(full_address).unwrap();
    let mut setup = ConsumerSetup::with_transport(transporter)
        .with_call_system(JsonRPCv2::new)
        .with_capability(GreetConsumer::new);

    greet_with_name(&mut setup);
}


fn greet_with_name<CS: CallSystemConsumer>(setup: &mut ConsumerSetup<TcpConsumer, CS>) {
    //let mut setup_write = setup.write().unwrap();
    let greeter = setup.capability_mut::<GreetConsumer<CS>>().unwrap();
    let name = "Client";

    println!("[Client] Sending a greet request with name '{}'", name);
    let response = greeter.greet(name).unwrap();
    println!("[Client] Received a greet response saying: '{}'", response);

    assert_eq!("Hello Client", response);
}

