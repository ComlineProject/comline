// Relative Modules
mod storage;
mod rest;
mod web_ui;

// Standard Uses

// Crate Uses

// External Uses


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    storage::init();

    rest::init().await;
}
