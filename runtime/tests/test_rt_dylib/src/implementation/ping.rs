// Standard Uses

// Crate Uses
use crate::generated::ping::{PingProvider};

// External Uses


pub struct Ping;
impl PingProvider for Ping {
    async fn ping(&self) {
        todo!()
    }

    fn ping_limit(&self) {
        todo!()
    }
}

