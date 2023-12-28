// Standard Uses

// Crate Uses
use crate::context::Provider;
use super::super::generated::ping::{PingProvider};

// External Uses
use comline_runtime::setup::APIResult;


pub struct Ping;
impl PingProvider for Ping {
    async fn ping(&self) -> APIResult<()> {
        todo!()
    }

    fn ping_limit(&self) -> APIResult<()> {
        todo!()
    }
}
impl Provider for Ping {}
