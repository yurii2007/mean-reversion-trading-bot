use time::UtcDateTime;
use uuid::Uuid;

use crate::api::binance::response::BinanceResponse;

#[derive(Debug)]
pub struct Position {
    pub id: Uuid,
    pub symbol: String,
    pub entry_price: f64,
    pub quantity: f64,
    pub timestamp: UtcDateTime,
}

#[derive(Debug, Clone)]
pub struct ProcessedCandle {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Position {
    pub fn new(symbol: String, entry_price: f64, quantity: f64, timestamp: UtcDateTime) -> Self {
        Self { id: Uuid::new_v4(), symbol, entry_price, quantity, timestamp }
    }
}

impl ProcessedCandle {
    pub fn calculate_mean(&self) -> f64 {
        (self.open + self.close) / 2_f64
    }
}

impl From<BinanceResponse> for ProcessedCandle {
    fn from(value: BinanceResponse) -> Self {
        Self {
            timestamp: value.open_timestamp.unix_timestamp().try_into().unwrap(),
            close: value.close_price,
            low: value.low_price,
            high: value.high_price,
            open: value.open_price,
            volume: value.taker_buy_base_asset_vol,
        }
    }
}

impl From<&BinanceResponse> for ProcessedCandle {
    fn from(value: &BinanceResponse) -> Self {
        Self {
            timestamp: value.open_timestamp.unix_timestamp().try_into().unwrap(),
            close: value.close_price,
            low: value.low_price,
            high: value.high_price,
            open: value.open_price,
            volume: value.taker_buy_base_asset_vol,
        }
    }
}
