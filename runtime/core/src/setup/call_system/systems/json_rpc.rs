// Standard Uses
use std::sync::{Arc, RwLock};

// Crate Uses
use crate::setup::CallResult;
use crate::setup::call_system;
use crate::setup::call_system::{Callback, CallSystem, CallSystemProvider, EventType, Origin};
use crate::setup::call_system::consumer::CallSystemConsumer;
use crate::setup::call_system::Kind;
use crate::setup::abstract_call::AbstractCall;

// External Uses
use eyre::Result;
use json_rpc_types::Id;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::sync::watch;


// type Request = json_rpc_types::Request<serde_json::Value>;
type Request<P> = json_rpc_types::Request<P>;
type Response<R> = json_rpc_types::Response<R, serde_json::Value>;


#[allow(dead_code)]
pub struct JsonRPCv2 {
    transporter: call_system::Origin,
    events_sender: watch::Sender<Option<Box<[u8]>>>,
    events_receiver: watch::Receiver<Option<Box<[u8]>>>,
    event_listeners: Vec<EventType>,
}

impl JsonRPCv2 {
    pub fn new(origin: call_system::Origin) -> Self {
        let (sx, rx) = watch::channel(None);

        Self {
            transporter: origin,
            events_sender: sx,
            events_receiver: rx,
            event_listeners: vec![],
        }
    }
    pub fn into_threaded(self) -> Arc<RwLock<Self>> { Arc::new(RwLock::new(self)) }
}

#[allow(unused_variables)]
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


impl CallSystemProvider for JsonRPCv2 {
    fn receive_async_call<M>(&mut self, kind: Kind, message: AbstractCall<M>)
        -> CallResult<M>
        where M: Send + Serialize + for<'de> Deserialize<'de>
    {
        // let incoming: Request<serde_json::Value> = serde_json::from_slice(data).unwrap();

        todo!()
    }
}


impl CallSystemConsumer for JsonRPCv2 {
    async fn send_async_call<M>(&mut self, kind: Kind, call: AbstractCall<M>)
        -> CallResult<M>
        where M: Send + Serialize + for<'de> Deserialize<'de>
    {
        // TODO: Request Id might be just a call instance number
        let id = Some(Id::Num(0));

        let method = match kind {
            Kind::Id(id) => id.to_string().as_str().try_into().unwrap(),
            Kind::Named(name) => name.as_str().try_into().unwrap(),
        };

        let request = Request {
            jsonrpc: json_rpc_types::Version::V2,
            id: id.clone(), method,
            //params: map_call_request_to_json(call).map_err(|_| ())?,
            params: Some(call.parameters),
        };
        let request = serde_json::to_vec(&request).unwrap();

        match &self.transporter {
            Origin::Consumer(transporter) => {
                let mut transporter = transporter.write().unwrap();
                transporter.send_data(&request).unwrap(); // TODO: This needs to be properly raised up
            },
            _ => panic!(
                "A Call System Provider implementation can only hold a\
                 Consumer transporter and not Provider transporter"
            )
        }


        let mut response: Option<Response<M>> = None;

        for listener in self.event_listeners.iter_mut() {
            match listener {
                EventType::ReceivedBytes(cb) => {
                    response = Some(receive_event_by_id(cb, id.clone().unwrap()).await.unwrap());
                },
                _ => ()
            }
        }
        let inner = response.unwrap().payload.map_err(|_| ())?;

        Ok(inner)
    }

    fn send_blocking_call<M>(&mut self, kind: Kind, call: AbstractCall<M>)
        -> CallResult<M>
        where M: Send + Serialize + for<'de> Deserialize<'de>
    {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async move {
                self.send_async_call(kind, call).await
            })
        })
    }
}


impl Callback for JsonRPCv2 {
    fn on_received_data(&mut self, data: &[u8]) {
        //self.events_sender.send(data).unwrap();

        /*
        for listener in &self.events_sender {
            match listener {
                EventType::ReceivedBytes(cb) => {
                    // TODO: Data reference
                    // self.events_sender.send(data);
                },
                _ => panic!("How are we here, this is a runtime developer mistake")
            }
        }
        */
    }

    #[allow(unused)]
    fn on_sent_data(&mut self, data: &[u8]) {
        todo!()
    }
}

async fn receive_event_by_id<M: DeserializeOwned>(
    receiver: &mut watch::Receiver<Option<Box<[u8]>>>, id: Id
) -> Result<Response<M>> {
    let data = receiver.wait_for(|v| {
        let v = v.as_ref().unwrap();

        match id {
            Id::Num(n) => {
                let incoming_id = u64::from_le_bytes(v[0..8].try_into().unwrap());
                incoming_id == n
            },
            Id::Str(s) => {
                let incoming_id = std::str::from_utf8(&v[0..36]).map_err(|_| ()).unwrap();
                incoming_id == s
            }
        }
    }).await?;

    let request: Response<M> = serde_json::from_slice(&data.to_owned().unwrap()).unwrap();

    Ok(request)
}

/*
fn map_call_request_to_json<M: Send>(message: M) -> Result<Option<serde_json::Value>> {
    // TODO: If there is not parameters, assigning as None is much better
    let parameters = Some(serde_json::Value::Array(vec![]));

    /*
    for parameter in message.parameters {

    }
    */

    Ok(parameters)
}
*/

