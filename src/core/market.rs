use time::OffsetDateTime;

use crate::api::binance::response::BinanceResponse;

#[derive(Debug)]
pub struct Position {
    pub symbol: String,
    pub entry_price: f64,
    pub quantity: f64,
    pub timestamp: OffsetDateTime,
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

impl ProcessedCandle {
    pub fn calculate_mean(&self) -> f64 {
        (self.open + self.close) / 2_f64
    }
}

// todo move this implementation into binance module
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
