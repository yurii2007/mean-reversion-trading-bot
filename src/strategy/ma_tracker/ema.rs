use crate::strategy::ma_tracker::MaTracker;

pub struct EmaTracker {}

impl EmaTracker {
    pub fn new() -> Self {
        Self {}
    }
}

unsafe impl Send for EmaTracker {}

impl MaTracker for EmaTracker {
    fn current_average(&self) -> f64 {
        1.0
    }

    fn update(&mut self, latest_candle: &crate::bot::kline_processor::Candle) -> f64 {
        latest_candle.close_price
    }
}
