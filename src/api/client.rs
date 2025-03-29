use async_trait::async_trait;

use crate::{ core::market::{ Position, ProcessedCandle }, strategy::strategy::Strategy, ApiError };

#[async_trait]
pub trait ApiClient {
    async fn get_candles(&self, strategy: &'_ Strategy) -> Result<Vec<ProcessedCandle>, ApiError>;

    async fn get_latest_candle(&self, strategy: &'_ Strategy) -> Result<ProcessedCandle, ApiError>;

    async fn place_order_to_buy(&self, pair: String, quantity: f64) -> Result<Position, ApiError>;

    async fn place_order_to_sell(&self, pair: String, quantity: f64) -> Result<(), ApiError>;
}
