use std::borrow::Cow;

use async_trait::async_trait;
use binance_spot_connector_rust::{
    http::{ request::Request, Credentials },
    hyper::BinanceHttpClient,
    market::klines::Klines,
    wallet::balance,
};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use response::BinanceResponse;
use time::{ OffsetDateTime, UtcDateTime };
use tracing::{ debug, warn };

use crate::{
    Strategy,
    core::market::{ Position, ProcessedCandle },
    api::client::{ ApiClient, KLineParams },
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
    async fn get_candles(&self, params: KLineParams) -> Result<Vec<ProcessedCandle>, ApiError> {
        let params: Klines = params.try_into().map_err(ApiError::ParseError)?;

        let response = self.get_kline_data(params).await?;

        if response.is_empty() {
            return Err(ApiError::MarketError("No data received from binance".to_string()));
        }

        Ok(response.iter().map(ProcessedCandle::from).collect())
    }

    async fn get_latest_candle(&self, strategy: &'_ Strategy) -> Result<ProcessedCandle, ApiError> {
        let current_date_timestamp = UtcDateTime::now().unix_timestamp() * 1000;
        let latest_kline_start_date =
            (current_date_timestamp as u128) - strategy.timeframe.execution.as_millis();

        let params = Klines::new(&strategy.symbol, strategy.timeframe.interval)
            .start_time(
                latest_kline_start_date
                    .try_into()
                    .map_err(|e|
                        ApiError::ParseError(format!("Could not convert timestamp: {}", e))
                    )?
            )
            .end_time(current_date_timestamp as u64)
            .limit(1);

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

    async fn get_account_balance(&self) -> Result<f64, ApiError> {
        let account_data_request = balance();
        let account_data = self.client.send(account_data_request).await?.into_body_str().await?;

        warn!("account data response: {:?}", account_data);

        Ok(100_f64)
    }
}
