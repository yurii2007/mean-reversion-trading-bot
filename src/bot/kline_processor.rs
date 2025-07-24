use time::UtcDateTime;

#[derive(Debug)]
pub struct Candle {
    pub open_timestamp: UtcDateTime,
    pub close_timestamp: UtcDateTime,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
}

#[derive(Debug)]
pub struct KlineProcessor {
    candles_count: u64,
    current_ma: f64,
}
