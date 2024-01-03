// Standard Uses
use std::sync::{Arc, RwLock};

// Crate Uses
use crate::setup::call_system::{CallSystem, EventType};

// External Uses


#[derive(Default)]
pub struct JsonRPC {
    event_listeners: Vec<fn(&[u8])>
}

impl JsonRPC {
    pub fn into_threaded(self) -> Arc<RwLock<Self>> { Arc::new(RwLock::new(self)) }
}

#[allow(unused)]
impl CallSystem for JsonRPC {
    // type IncomingMessage = Request;
    // type OutgoingMessage = Response;

    fn add_event_listener(&mut self, event_type: EventType, callback: Box<dyn FnMut(&[u8])>) {
        self.event_listeners.push(callback);
    }

    fn on_receive_data(&mut self, data: Box<[u8]>) {
        //let inc: jsonrpc::Request = serde_json::from_slice(&data).unwrap();

        /*
        for listener in self.event_listeners {
            listener(dupe);
        }
        */
    }

    fn send_data(&mut self, data: &[u8]) {
        todo!()
    }

    /*
    fn on_sent_data(&self, data: &[u8]) {
        todo!()
    }
    */
    /*
    fn send_call_indexed<M: for<'a> Deserialize<'a>>(
        &mut self, meta: &dyn CallProtocolMeta, index: usize, message: M
    ) {
        // let call_meta = &consumer as &dyn CallProtocolMeta;
        let call_name = meta.calls_names()[index];
    }
    */
}

