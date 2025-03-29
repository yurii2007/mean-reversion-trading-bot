use crate::{ api::response::BinanceResponse, strategy::mean_calculation::MeanCalculationMethod };

#[derive(Debug, Clone)]
pub struct ProcessedCandle {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub deviation_from_mean: Option<f64>,
}

#[derive(Debug)]
struct MarketAnalysis {
    pub candles: Vec<ProcessedCandle>,
    pub current_mean: f64,
    pub mean_type: MeanCalculationMethod,
    pub standard_deviation: f64,
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
            deviation_from_mean: None,
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
            deviation_from_mean: None,
        }
    }
}
