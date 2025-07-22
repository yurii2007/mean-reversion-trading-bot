use strategy::Strategy;

use crate::bot::Bot;

mod api;
mod bot;
mod logger;
mod strategy;

#[tokio::main]
async fn main() -> Result<(), String> {
    logger::init_logger();

    let strategy = Strategy::new();

    let bot = Bot::try_init(strategy).expect("Failed to initialize bot");
    bot.run().await.unwrap();

    Ok(())
}
