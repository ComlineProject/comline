// Relative Modules
mod health;
mod ping;

// Standard Uses

// Crate Uses
use crate::context::{Consumer, ConsumerGroup};
use crate::generated::health::HealthCheckProtocol;

// External Uses
use eyre::{Result, eyre};


pub async fn consume() -> Result<()> {
    let consumers = BasicConsumer::new()
        .and_consumer(health::HealthCheck::new());
    let health_check_consumer = consumers.get::<health::HealthCheck>().unwrap();

    health_check_consumer.alive()
        .map_err(|_| eyre!("Provider isn't alive..."))?;

    println!("Alive and well");

    Ok(())
}

pub struct BasicConsumer {
    consumers: Vec<Box<dyn Consumer>>
}
impl BasicConsumer {
    pub fn new() -> Box<Self> { Box::new(Self { consumers: vec![] }) }
}
impl ConsumerGroup for BasicConsumer {
    fn consumers(&self) -> &Vec<Box<dyn Consumer>> { &self.consumers }
    fn consumers_mut(&mut self) -> &mut Vec<Box<dyn Consumer>> { &mut self.consumers }
}
