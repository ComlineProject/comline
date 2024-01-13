// Standard Uses

// Crate Uses
use std::sync::{Arc, RwLock};

// External Uses
use comline_runtime::setup::transport::provider::ProviderSetup;


pub trait Context {
    fn context(&self) -> Self;
}

pub trait Provider {}

pub trait Consumer {}

#[allow(async_fn_in_trait)]
pub trait ProviderGroup {
    fn providers(&mut self) -> &mut Vec<Box<dyn Provider>>;
    fn and_provider(mut self, provider: Box<dyn Provider>) -> impl ProviderGroup where Self: Sized {
        self.providers().push(provider);
        self
    }
    async fn serve(&self) {
        todo!()
    }
}

pub trait ConsumerGroup {
    fn consumers(&self) -> &Vec<Box<dyn Consumer>>;
    fn consumers_mut(&mut self) -> &mut Vec<Box<dyn Consumer>>;
    fn and_consumer(mut self, consumer: Box<dyn Consumer>) -> impl ConsumerGroup where Self: Sized {
        self.consumers_mut().push(consumer);
        self
    }
    fn get<C>(&self) -> Option<&C> {
        for consumer in self.consumers() {
            if let Some(con) = consumer.downcast_mut::<C>() { return Some(&*con); }
        }

        None
    }
}

