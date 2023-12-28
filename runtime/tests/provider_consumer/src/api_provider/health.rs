// Standard Uses

// Crate Uses
use crate::context::Provider;
use crate::generated::health::{Capability, HealthCheckProtocol};

// External Uses
use comline_runtime::setup::APIResult;


pub struct HealthCheck;
impl HealthCheck {
    pub fn new() -> Box<Self> { Box::new(Self) }
}
impl HealthCheckProtocol for HealthCheck {
    fn alive(&self) -> APIResult<()> {
        todo!()
    }

    fn capabilities(&self) -> APIResult<Vec<Capability>> {
        todo!()
    }
}
impl Provider for HealthCheck {}
