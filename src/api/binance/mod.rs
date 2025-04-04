use std::{ borrow::Cow, time::Duration };

use async_trait::async_trait;
use binance_spot_connector_rust::{
    http::{ request::Request, Credentials },
    hyper::BinanceHttpClient,
    market::klines::Klines,
    trade,
    wallet::user_asset::UserAsset,
};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use rust_decimal::{ Decimal, prelude::FromPrimitive };
use time::UtcDateTime;
use tracing::{ debug, info, warn };

use crate::{
    api::client::{ ApiClient, KLineParams },
    core::market::{ Position, ProcessedCandle },
    strategy::timeframe::duration_into_kline_interval,
    ApiError,
};
use response::{ BinanceResponse, BalanceResponse };

// todo Should not be public
pub mod response;

const ENV_BINANCE_API_KEY: &str = "BINANCE_API_KEY";
const ENV_BINANCE_API_SECRET: &str = "BINANCE_API_SECRET";

pub struct BinanceApi {
    client: BinanceHttpClient<HttpsConnector<HttpConnector>>,
}

impl BinanceApi {
    const QUANTITY_FLOAT_PRECISION: u32 = 5;

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

        let response = self.client.send(request).await?.into_body_str().await?;

        let raw_kline_data = BinanceResponse::deserialize_response(Cow::from(response)).unwrap();

        Ok(raw_kline_data)
    }

    fn get_decimal_quantity(quantity: f64) -> Result<Decimal, ApiError> {
        let mut decimal = Decimal::from_f64(quantity).ok_or(
            ApiError::ParseError("Failed to parse quantity when creating an order".to_string())
        )?;

        decimal.rescale(BinanceApi::QUANTITY_FLOAT_PRECISION);

        Ok(decimal)
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

    async fn get_latest_candle(
        &self,
        symbol: &'_ str,
        interval: &'_ Duration
    ) -> Result<ProcessedCandle, ApiError> {
        let kline_interval = duration_into_kline_interval(interval).ok_or(
            ApiError::ParseError("Invalid interval provided".to_string())
        )?;

        let params = Klines::new(symbol, kline_interval).limit(1);

        let response = self.get_kline_data(params).await?;

        if response.is_empty() {
            return Err(ApiError::MarketError("No data received from binance".to_string()));
        }

        Ok(ProcessedCandle::from(&response[0]))
    }

    async fn place_order_to_buy(
        &self,
        symbol: &str,
        quantity: f64,
        price: f64
    ) -> Result<Position, ApiError> {
        let decimal_quantity = BinanceApi::get_decimal_quantity(quantity)?;

        let order = trade
            ::new_order(symbol, trade::order::Side::Buy, "MARKET")
            .quantity(decimal_quantity);

        info!("Created order to buy for {} {}", symbol, quantity);

        self.client.send(order).await?;

        Ok(Position::new(symbol.to_string(), price, quantity, UtcDateTime::now()))
    }

    async fn place_order_to_sell(&self, symbol: &str, quantity: f64) -> Result<(), ApiError> {
        let decimal_quantity = BinanceApi::get_decimal_quantity(quantity)?;

        let order = trade
            ::new_order(symbol, trade::order::Side::Sell, "MARKET")
            .quantity(decimal_quantity);

        info!("Created order to Sell for {} {}", symbol, quantity);
        self.client.send(order).await?;

        Ok(())
    }

    async fn get_account_balance(&self, symbol: &'_ str) -> Result<f64, ApiError> {
        let user_asset_request = UserAsset::new().asset(symbol);
        let user_asset_response = self.client
            .send(user_asset_request).await?
            .into_body_str().await?;

        let assets = BalanceResponse::deserialize_response(Cow::from(user_asset_response)).map_err(
            ApiError::ParseError
        )?;

        warn!("account data response: {:?}", assets);

        let account_balance = assets
            .get(0)
            .ok_or(ApiError::ValidationError("Empty balance data received".to_string()))?;

        Ok(account_balance.free)
    }
}
