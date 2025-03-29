use std::collections::VecDeque;

use serde::{ Deserialize, Serialize };

use crate::core::market_analysis::ProcessedCandle;

// Simple MA, EMA, VWAP
#[derive(Debug, Serialize, Deserialize)]
pub enum MeanCalculationMethod {
    #[serde(rename = "PascalCase")]
    SimpleMA,
    EMA,
    VMAP,
}

pub struct SimpleMa {
    period: usize,
    values: VecDeque<f64>,
    sum: f64,
}

impl SimpleMa {
    pub fn new(period: usize) -> Self {
        Self { period, values: VecDeque::new(), sum: 0_f64 }
    }

    pub fn update(&mut self, value: f64) -> f64 {
        self.sum += value;
        self.values.push_back(value);

        if self.values.len() > self.period {
            if let Some(outdated) = self.values.pop_front() {
                self.sum -= outdated;
            }
        }

        self.calculate()
    }

    pub fn calculate(&self) -> f64 {
        if self.values.is_empty() {
            return 0_f64;
        }

        self.sum / (self.values.len() as f64)
    }
}

pub struct MATracker {
    long_ma: SimpleMa,
    short_ma: SimpleMa,
}

impl MATracker {
    pub fn new(long_period: usize, short_period: usize) -> Self {
        Self { long_ma: SimpleMa::new(long_period), short_ma: SimpleMa::new(short_period) }
    }

    pub fn update(&mut self, price: f64) -> (f64, f64) {
        (self.long_ma.update(price), self.short_ma.update(price))
    }

    pub fn get_values(&self) -> (f64, f64) {
        (self.long_ma.calculate(), self.short_ma.calculate())
    }
}

pub fn process_candles(
    candles: &Vec<&ProcessedCandle>,
    long_period: usize,
    short_period: usize
) -> Vec<(f64, f64)> {
    let mut ma_tracker = MATracker::new(long_period, short_period);
    let mut res = Vec::with_capacity(candles.len());

    candles.iter().for_each(|candle| {
        let moving_averages = ma_tracker.update(candle.close);

        res.push(moving_averages)
    });

    res
}
