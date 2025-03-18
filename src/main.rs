use binance_spot_connector_rust::market::klines::Klines;
use logger::init_logger;
use tracing::info;

use strategy::strategy::Strategy;
use api::{ binance::BinanceApi, error::ApiError };

pub mod api;
pub mod logger;
pub mod strategy;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    init_logger();

    let strategy = Strategy::new();
    info!("Loaded strategy configuration: {:?}", strategy);

    let params = Klines::new(&strategy.symbol, strategy.timeframe.interval).limit(
        strategy.timeframe.period_measurement.measure_bars.try_into().unwrap()
    );

    let data = BinanceApi::get_kline_data(params).await?;

    info!("DATA: {:?}", data);

    Ok(())
}
