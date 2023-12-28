// Standard Uses

// Crate Uses
use crate::generated::health::HealthCheckConsumer;

// External Uses


pub struct Health;
impl HealthCheckConsumer for Health {
    fn alive(&self) {
        todo!()
    }

    fn capabilities(&self) {
        todo!()
    }
}
