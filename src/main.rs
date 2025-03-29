use logger::init_logger;
use tracing::info;

use strategy::strategy::Strategy;
use api::error::ApiError;
use core::bot::Bot;

pub mod api;
pub mod logger;
pub mod strategy;
pub mod core;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    init_logger();

    let strategy = Strategy::new();
    info!("Loaded strategy configuration: {:?}", strategy);

    let mut bot = Bot::new(strategy, 100_f64);

    bot.initialize().await?;

    bot.run().await
}
