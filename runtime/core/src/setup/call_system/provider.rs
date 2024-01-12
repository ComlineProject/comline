// Standard Uses

use serde::{Deserialize, Serialize};

// Crate Uses
use crate::setup::CallResult;
use crate::setup::call_system::CallSystem;
use crate::setup::call_system::Kind;
use crate::setup::abstract_call::AbstractCall;

// External Uses


pub trait CallSystemProvider: CallSystem {
    fn receive_async_call<M>(&mut self, kind: Kind, message: AbstractCall<M>)
        -> CallResult<M>
        where M: Send + Serialize + for<'de> Deserialize<'de>
    ;
}

pub fn send_call_with_conditions<M>() -> CallResult<M> {
    todo!()
}
