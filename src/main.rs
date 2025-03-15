use logger::init_logger;
use tracing::info;

use strategy::strategy::Strategy;
use api::binance::BinanceApi;

pub mod api;
pub mod logger;
pub mod strategy;

#[tokio::main]
async fn main() -> Result<(), binance_spot_connector_rust::hyper::Error> {
    init_logger();

    let strategy = Strategy::new();
    info!("Loaded strategy configuration: {:?}", strategy);

    let data = BinanceApi::get_data().await?;

    info!("DATA: {:?}", data);

    Ok(())
}
