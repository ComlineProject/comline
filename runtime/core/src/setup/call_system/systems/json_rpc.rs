// Standard Uses
use std::any::Any;
use std::sync::{Arc, RwLock};

// Crate Uses
use crate::setup::APIResult;
use crate::setup::call_system;
use crate::setup::call_system::{Callback, CallSystem, CallSystemProvider, EventType, Origin};
use crate::setup::call_system::consumer::CallSystemConsumer;
use crate::setup::message_format::Message;

// External Uses
use json_rpc_types::Id;


type Request = json_rpc_types::Request<Vec<serde_json::Value>>;
#[allow(unused)]
type Response = json_rpc_types::Response<serde_json::Value, serde_json::Value>;


pub struct JsonRPCv2 {
    transporter: call_system::Origin,
    event_listeners: Vec<EventType>
}

impl JsonRPCv2 {
    pub fn new(origin: call_system::Origin) -> Self {
        Self { transporter: origin, event_listeners: vec![] }
    }
    pub fn into_threaded(self) -> Arc<RwLock<Self>> { Arc::new(RwLock::new(self)) }
}

#[allow(unused)]
impl CallSystem for JsonRPCv2 {
    fn add_event_listener(&mut self, callback: EventType) {
        self.event_listeners.push(callback);
    }

    fn receive_data(&mut self, data: &[u8]) {
        todo!()
    }
    fn send_data(&mut self, data: &[u8]) {
        todo!()
    }
}

#[allow(unused)]
impl CallSystemProvider for JsonRPCv2 {
    fn send_blocking_call(&mut self, name: &str, parameters: &dyn Any) -> APIResult<Box<dyn Any>> {
        /*
            let call = Response::ok(Id, Value::new());
            let outgoing: Response: serde_json::to_vec();
        */
        todo!()
    }
}

impl CallSystemConsumer for JsonRPCv2 {
    fn send_blocking_call(&mut self, name: &str, message: Message) -> APIResult<Box<dyn Any>> {
        let request: Request = Request {
            jsonrpc: json_rpc_types::Version::V2,
            method: name.try_into().unwrap(),
            params: Default::default(),
            id: Some(Id::Num(1))
        };
        let request = serde_json::to_vec(&request).unwrap();
        let req = serde_json::to_string(&request).unwrap();

        //let response = None;
        //let cb = move |data: &[u8]| { response = Some(data); };
        // self.add_event_listener(EventType::ReceivedBytes(cb as _));

        match &self.transporter {
            Origin::Consumer(transporter) => {
                let mut transporter = transporter.write().unwrap();
                transporter.send_data(&request);
            },
            _ => panic!(
                "A Call System Provider implementation can only hold a\
                 Consumer transporter and not Provider transporter"
            )
        }

        for listener in &self.event_listeners {
            match listener {
                EventType::SentBytes(cb) => cb(&*request),
                _ => ()
            }
        }


        Ok(Box::new("Haha".to_owned()))
    }
}

impl Callback for JsonRPCv2 {
    fn on_received_data(&mut self, data: &[u8]) {
        let incoming: Request = serde_json::from_slice(&data).unwrap();

        for listener in &self.event_listeners {
            match listener {
                EventType::ReceivedBytes(cb) => {
                    // cb(&*data)
                },
                _ => panic!("How are we here, this is a runtime developer mistake")
            }
        }

        todo!()
    }

    #[allow(unused)]
    fn on_sent_data(&mut self, data: &[u8]) {
        todo!()
    }
}