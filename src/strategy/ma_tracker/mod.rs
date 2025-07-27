use serde::{Deserialize, Serialize};

use crate::{
    bot::kline_processor::Candle,
    strategy::ma_tracker::{ema::EmaTracker, simple_ma::SimpleMATracker, vmap::VMapTracker},
};

mod ema;
mod simple_ma;
mod vmap;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum MeanCalculationMethod {
    SimpleMA,
    EMA,
    VMAP,
}

pub fn get_ma_tracker(method: &MeanCalculationMethod) -> Box<dyn MaTracker + Send> {
    match method {
        MeanCalculationMethod::SimpleMA => Box::new(SimpleMATracker::new()),
        MeanCalculationMethod::EMA => Box::new(EmaTracker::new()),
        MeanCalculationMethod::VMAP => Box::new(VMapTracker::new()),
    }
}

pub trait MaTracker {
    fn current_average(&self) -> f64;
    fn update(&mut self, latest_candle: &Candle) -> f64;
}
