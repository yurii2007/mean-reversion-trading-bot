use client::Client;
use strategy::Strategy;

mod api;
mod client;
mod logger;
mod strategy;

#[tokio::main]
async fn main() -> Result<(), String> {
    logger::init_logger();

    let strategy = Strategy::new();
    let client = Client::new(&strategy.exchange.api);

    client.connect().await;
    Ok(())
}
