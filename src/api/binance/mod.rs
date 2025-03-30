use std::borrow::Cow;

use async_trait::async_trait;
use binance_spot_connector_rust::{
    http::{ request::Request, Credentials },
    hyper::BinanceHttpClient,
    market::klines::Klines,
};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use response::BinanceResponse;
use time::OffsetDateTime;
use tracing::{ debug, warn };

use crate::{
    Strategy,
    core::market::{ Position, ProcessedCandle },
    api::client::ApiClient,
    ApiError,
};

pub mod response;

const ENV_BINANCE_API_KEY: &str = "BINANCE_API_KEY";
const ENV_BINANCE_API_SECRET: &str = "BINANCE_API_SECRET";

pub struct BinanceApi {
    client: BinanceHttpClient<HttpsConnector<HttpConnector>>,
}

impl BinanceApi {
    pub fn new() -> Self {
        let credentials = Credentials::from_hmac(
            dotenv::var(ENV_BINANCE_API_KEY).expect("BINANCE_API_KEY is missing"),
            dotenv::var(ENV_BINANCE_API_SECRET).expect("BINANCE_API_SECRET is missing")
        );

        Self {
            client: BinanceHttpClient::default().credentials(credentials),
        }
    }

    pub async fn get_kline_data(&self, params: Klines) -> Result<Vec<BinanceResponse>, ApiError> {
        let request = Request::from(params);

        debug!("Requesting Kline data from binance with params: {:?}", request.params());

        let response = self.client
            .send(request).await
            .map_err(ApiError::from)?
            .into_body_str().await
            .map_err(ApiError::from)?;

        let raw_kline_data = BinanceResponse::deserialize_response(Cow::from(response)).unwrap();

        Ok(raw_kline_data)
    }
}

#[async_trait]
impl ApiClient for BinanceApi {
    async fn get_candles(&self, strategy: &Strategy) -> Result<Vec<ProcessedCandle>, ApiError> {
        let params = Klines::new(&strategy.symbol, strategy.timeframe.interval).limit(
            strategy.timeframe.period_measurement.measure_bars as u32
        );

        let response = self.get_kline_data(params).await?;

        if response.is_empty() {
            return Err(ApiError::MarketError("No data received from binance".to_string()));
        }

        Ok(response.iter().map(ProcessedCandle::from).collect())
    }

    async fn get_latest_candle(&self, strategy: &'_ Strategy) -> Result<ProcessedCandle, ApiError> {
        let params = Klines::new(&strategy.symbol, strategy.timeframe.interval).limit(1);

        let response = self.get_kline_data(params).await?;

        if response.is_empty() {
            return Err(ApiError::MarketError("No data received from binance".to_string()));
        }

        Ok(ProcessedCandle::from(&response[0]))
    }

    async fn place_order_to_buy(&self, pair: String, quantity: f64) -> Result<Position, ApiError> {
        warn!("UNIMPLEMENTED: should place order to buy for {} {}", quantity, pair);

        Ok(Position {
            entry_price: 83600_f64,
            quantity,
            symbol: pair,
            timestamp: OffsetDateTime::now_utc(),
        })
    }

    async fn place_order_to_sell(&self, pair: String, quantity: f64) -> Result<(), ApiError> {
        warn!("UNIMPLEMENTED: should place order to sell for {} {}", quantity, pair);

        Ok(())
    }
}
