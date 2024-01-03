use std::cell::OnceCell;
// Standard Uses
use std::sync::{Arc, RwLock};

// Crate Uses
use crate::setup::call_system::{CallSystem, CallSystemSender, EventType};

// External Uses
use json_rpc_v2::{Request, serde_json};


#[derive(Default)]
pub struct JsonRPCv2 {
    event_listeners: Vec<OnceCell<dyn FnMut(&[u8])>>
}

impl JsonRPCv2 {
    pub fn into_threaded(self) -> Arc<RwLock<Self>> { Arc::new(RwLock::new(self)) }
}

#[allow(unused)]
impl CallSystem for JsonRPCv2 {
    // type IncomingMessage = Request;
    // type OutgoingMessage = Response;

    fn add_event_listener(&mut self, event_type: EventType, callback: Box<dyn FnMut(&[u8])>) {
        self.event_listeners.push(*callback);
    }

    fn on_receive_data(&mut self, data: Box<[u8]>) {
        let incoming: Request = serde_json::from_slice(&data).unwrap();

        // for listener in self.event_listeners { listener(dupe); }
    }

    fn send_data(&mut self, data: &[u8]) {
        todo!()
    }
}

impl CallSystemSender for JsonRPCv2 {
    /*
    fn send_call_indexed<M: for<'a> Deserialize<'a>>(
        &mut self, meta: &dyn CallProtocolMeta, index: usize, message: M
    ) {
        // let call_meta = &consumer as &dyn CallProtocolMeta;
        let call_name = meta.calls_names()[index];
    }
    */
}
