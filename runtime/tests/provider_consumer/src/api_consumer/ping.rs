// Standard Uses

// Crate Uses
use crate::context::Provider;
use super::super::generated::ping::{PingProvider};

// External Uses
use comline_runtime::setup::CallResult;


pub struct Ping;
impl PingProvider for Ping {
    async fn ping(&self) -> CallResult<()> {
        todo!()
    }

    fn ping_limit(&self) -> CallResult<()> {
        todo!()
    }
}
impl Provider for Ping {}
