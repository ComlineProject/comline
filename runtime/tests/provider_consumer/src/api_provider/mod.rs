// Relative Modules
pub mod health;
pub mod ping;

// Standard Uses

// Crate Uses
use crate::Context;
use crate::context::{Provider, ProviderGroup};

// External Uses


pub async fn provide() {
    let context = Context::default();

    let server = BasicServer::new()
        .and_provider(health::HealthCheck::new());

    server.serve().await
}

pub struct BasicServer {
    providers: Vec<Box<dyn Provider>>
}
impl BasicServer {
    pub fn new() -> Self { Self { providers: vec![] } }
}

impl ProviderGroup for BasicServer {
    fn providers(&mut self) -> &mut Vec<Box<dyn Provider>> { &mut self.providers }

    async fn serve(&self) {
        todo!()
    }
}

