// Standard Uses
use std::any::Any;
use std::sync::{Arc, RwLock};

// Crate Uses
use crate::setup::APIResult;
use crate::setup::call_system::{CallSystem, CallSystemProvider, EventType};

// External Uses
use json_rpc_v2::{
    Request, Response,
    serde_json, serde_json::Value,
};


#[derive(Default)]
pub struct JsonRPCv2 {
    event_listeners: Vec<EventType>
}

impl JsonRPCv2 {
    pub fn into_threaded(self) -> Arc<RwLock<Self>> { Arc::new(RwLock::new(self)) }
}

#[allow(unused)]
impl CallSystem for JsonRPCv2 {
    fn add_event_listener(&mut self, callback: EventType) {
        self.event_listeners.push(callback);
    }

    fn on_receive_data(&mut self, data: Box<[u8]>) {
        let incoming: Request = serde_json::from_slice(&data).unwrap();

        for listener in self.event_listeners {
            match listener {
                EventType::ReceivedBytes(cb) => cb(&*data),
                _ => panic!("How are we here, this is a runtime developer mistake")
            }
        }
    }

    fn send_data(&mut self, data: &[u8]) {
        todo!()
    }
}

impl CallSystemProvider for JsonRPCv2 {
    fn send_blocking_call<P, R>(&mut self, name: &str, parameters: P) -> APIResult<R>
        where P: Any
    {
        let call = Response::ok(Id, Value::new());
        let outgoing: Response: serde_json::to_vec();
    }
}
