// Standard Uses

// Crate Uses
use crate::context::Consumer;
use crate::generated::health::{Capability, HealthCheckProtocol};

// External Uses
use comline_runtime::setup::CallResult;


pub struct HealthCheck;
impl HealthCheck {
    pub fn new() -> Box<Self> { Box::new(Self) }
}

impl HealthCheckProtocol for HealthCheck {
    fn alive(&self) -> CallResult<()> { Ok(()) }

    fn capabilities(&self) -> CallResult<Vec<Capability>> {
        Ok(vec![Capability("Talk".to_owned())])
    }
}
impl Consumer for HealthCheck {}
