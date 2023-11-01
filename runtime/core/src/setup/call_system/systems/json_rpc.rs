// Standard Uses

// Crate Uses
use crate::setup::call_system::{CallSystem, ListenerFn};

// External Uses
use json_rpc_v2::{Request, serde_json};
use crate::setup::call_system::meta::CallProtocolMeta;


pub struct JsonRPCv2 {
    event_listeners: Vec<fn(&[u8])>
}

impl JsonRPCv2 {
    pub fn new() -> Self { Self { event_listeners: vec![] } }
}

#[allow(unused)]
impl CallSystem for JsonRPCv2 {
    // type IncomingMessage = Request;
    // type OutgoingMessage = Response;

    fn add_event_listener(&mut self, callback: ListenerFn) {
        self.event_listeners.push(callback);
    }

    fn on_receive_data(&mut self, data: Box<[u8]>) {
        let incoming: Request = serde_json::from_slice(&*data).unwrap();

        /*
        for listener in self.event_listeners {
            listener(dupe);
        }
        */
    }

    fn on_send_data(&mut self, data: Box<[u8]>) {
        todo!()
    }

    fn on_received_data(&self, data: &[u8]) {
        todo!()
    }

    fn on_sent_data(&self, data: &[u8]) {
        todo!()
    }

    fn send_call(&self, call_meta: &dyn CallProtocolMeta, index: usize) {
        // let call_meta = &consumer as &dyn CallProtocolMeta;
        let call_name = call_meta.calls()[index];

    }
}

