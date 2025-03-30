use logger::init_logger;
use tracing::info;
use dotenv::dotenv;

use strategy::strategy::Strategy;
use api::error::ApiError;
use core::bot::Bot;

pub mod api;
pub mod logger;
pub mod strategy;
pub mod core;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    dotenv().unwrap();
    init_logger();

    let strategy = Strategy::new();
    info!("Loaded strategy configuration: {:?}", strategy);

    let mut bot = Bot::new(strategy);

    bot.initialize().await?;

    bot.run().await
}
