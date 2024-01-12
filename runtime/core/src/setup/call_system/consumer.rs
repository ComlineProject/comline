// Standard Uses

// Crate Uses
use crate::setup::CallResult;
use crate::setup::call_system::{CallSystem, Kind, DEFAULT_CALL_TIMEOUT};
use crate::setup::abstract_call::AbstractCall;

// External Uses
use serde::{Deserialize, Serialize};


pub trait CallSystemConsumer: CallSystem {
    fn send_async_call<M>(&mut self, kind: Kind, message: AbstractCall<M>)
        -> impl std::future::Future<Output = CallResult<M>> + Send
        where M: Send + Serialize + for<'de> Deserialize<'de>
    ;

    fn send_blocking_call<M: Send + Serialize>(&mut self, kind: Kind, call: AbstractCall<M>)
        -> CallResult<M>
        where M: Send + Serialize + for<'de> Deserialize<'de>
    ;
}

pub fn send_call_with_conditions<M, R>(kind: Kind, call: AbstractCall<M>) -> CallResult<M>
    where M: Send
{
    let call_timeout = DEFAULT_CALL_TIMEOUT;
    let start_time = std::time::Instant::now();

    let response: Option<R> = None;
    while response.is_none() {
        if start_time.elapsed().as_millis() > call_timeout {
            return Err(())
        }
    }

    todo!()
}
