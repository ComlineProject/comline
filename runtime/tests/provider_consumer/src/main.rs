// Relative Modules
pub mod context;
pub mod generated;

mod api_consumer;
mod api_provider;

// Standard Uses

// Crate Uses

// External Uses


#[derive(Default)]
pub struct Context(String);

#[tokio::main]
async fn main() {
    api_provider::provide().await;

    println!("Served")
}
