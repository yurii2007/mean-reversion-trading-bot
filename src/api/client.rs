use std::time::Duration;

use async_trait::async_trait;
use binance_spot_connector_rust::market::klines::Klines;

use crate::{
    core::market::{ Position, ProcessedCandle },
    strategy::{
        strategy::Strategy,
        timeframe::{ duration_from_kline_interval, duration_into_kline_interval },
    },
    ApiError,
};

#[derive(Debug)]
pub struct KLineParams {
    candles_count: usize,
    symbol: String,
    interval: Duration,
    start_time: Option<u64>,
    end_time: Option<u64>,
}

#[async_trait]
pub trait ApiClient {
    async fn get_candles(&self, params: KLineParams) -> Result<Vec<ProcessedCandle>, ApiError>;

    async fn get_latest_candle(
        &self,
        symbol: &'_ str,
        interval: &'_ Duration
    ) -> Result<ProcessedCandle, ApiError>;

    async fn place_order_to_buy(
        &self,
        symbol: &'_ str,
        quantity: f64,
        price: f64,
    ) -> Result<Position, ApiError>;

    async fn place_order_to_sell(&self, symbol: &'_ str, quantity: f64) -> Result<(), ApiError>;

    async fn get_account_balance(&self) -> Result<f64, ApiError>;
}

impl KLineParams {
    pub fn new(strategy: &Strategy) -> Self {
        Self {
            candles_count: strategy.timeframe.period_measurement.measure_bars,
            symbol: strategy.symbol.clone(),
            interval: duration_from_kline_interval(&strategy.timeframe.interval),
            end_time: None,
            start_time: None,
        }
    }

    pub fn build(candles_count: usize, symbol: String, interval: Duration) -> Self {
        Self {
            candles_count,
            interval,
            symbol,
            end_time: None,
            start_time: None,
        }
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }
}

impl TryInto<Klines> for KLineParams {
    type Error = String;

    fn try_into(self) -> Result<Klines, Self::Error> {
        let symbol = &self.symbol;
        let interval = duration_into_kline_interval(&self.interval).ok_or(
            "Invalid interval provided".to_string()
        )?;

        let mut klines = Klines::new(symbol, interval).limit(self.candles_count as u32);

        if let Some(start_time) = self.start_time {
            klines = klines.start_time(start_time);
        }

        if let Some(end_time) = self.end_time {
            klines = klines.end_time(end_time);
        }

        Ok(klines)
    }
}
