// Standard Uses
use std::any::Any;
use std::sync::{Arc, RwLock};

// Crate Uses
use crate::setup::APIResult;
use crate::setup::communication::provider::CommunicationProvider;
use crate::setup::call_system::{CallSystem, EventType};

// External Uses
use jsonrpc::{Response, serde_json};
use crate::setup::call_system::provider::CallSystemProvider;


#[derive(Default)]
pub struct JsonRPC {
    transporter: Arc<RwLock<dyn CommunicationProvider>>,
    event_listeners: Vec<EventType>
}

impl JsonRPC {
    pub fn into_threaded(self) -> Arc<RwLock<Self>> { Arc::new(RwLock::new(self)) }
}

#[allow(unused)]
impl CallSystem for JsonRPC {
    fn add_event_listener(&mut self, callback: EventType) {
        self.event_listeners.push(callback);
    }

    fn on_receive_data(&mut self, data: Box<[u8]>) {
        let incoming: jsonrpc::Request = serde_json::from_slice(&data).unwrap();

        for listener in &self.event_listeners {
            match listener {
                EventType::ReceivedBytes(_) => {},
                _ => panic!("How are we here, this is a runtime developer mistake")
            }
        }
    }

    fn send_data(&mut self, data: &[u8]) {
        todo!()
    }

    /*
    fn send_call_indexed<M: for<'a> Deserialize<'a>>(
        &mut self, meta: &dyn CallProtocolMeta, index: usize, message: M
    ) {
        // let call_meta = &consumer as &dyn CallProtocolMeta;
        let call_name = meta.calls_names()[index];
    }
    */
}

impl CallSystemProvider for JsonRPC {
    fn send_blocking_call<P, R>(&mut self, name: &str, parameters: P) -> APIResult<R>
        where P: Any
    {
        let transporter = self.transporter.write()?;

        let response = Response {
            jsonrpc: None,
            id: name.into(),
            result: None,
            error: None,
        };
        let outgoing = serde_json::to_vec_pretty(&response)?;

        transporter.send_data(outgoing)
    }
}
