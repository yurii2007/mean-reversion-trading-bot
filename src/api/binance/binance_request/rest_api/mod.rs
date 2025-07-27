use binance_spot_connector_rust::{
    http::request::Request, hyper::BinanceHttpClient, market::klines::Klines,
};
use tracing::debug;

use crate::{
    api::{binance::binance_response::rest_api::BinanceRawKlineResponse, ApiError},
    bot::kline_processor::Candle,
    strategy::Strategy,
};

pub struct BinanceRestApi;

impl BinanceRestApi {
    pub async fn get_kline_data(strategy: &Strategy) -> Result<Vec<Candle>, ApiError> {
        let klines = Klines::new(&strategy.symbol, strategy.timeframe.interval)
            .limit(strategy.timeframe.period_measurement.measure_bars as u32);

        let request = Request::from(klines);

        debug!(
            "Requesting Kline data from binance with params: {:?}",
            request.params()
        );
        let base_url = dotenv::var("BINANCE_API_URL").map_err(|_| {
            ApiError::NotFound("Failed to read BINANCE_API_URL from environment".to_string())
        })?;

        let client = BinanceHttpClient::with_url(&base_url);

        let response = client.send(request).await?.into_body_str().await?;

        let response: Vec<BinanceRawKlineResponse> = serde_json::from_str(&response)?;

        Ok(response.into_iter().map(Candle::from).collect())
    }
}
