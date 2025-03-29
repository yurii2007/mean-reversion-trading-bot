use std::collections::VecDeque;

use serde::{ Deserialize, Serialize };

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
