use std::collections::VecDeque;

use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MeanCalculationMethod {
    #[serde(rename = "PascalCase")]
    SimpleMA,
    EMA,
    VMAP,
}

pub struct MaTracker {
    method: MeanCalculationMethod,
    period: usize,
    values: VecDeque<f64>,
    sum: f64,
}

pub trait MeanCalculation {
    fn update(&mut self, value: f64) -> f64;

    fn calculate(&self) -> f64;
}

impl MaTracker {
    pub fn new(period: usize, ma_calculation_method: MeanCalculationMethod) -> Self {
        Self {
            method: ma_calculation_method,
            period,
            sum: 0_f64,
            values: VecDeque::new(),
        }
    }
}

impl MeanCalculation for MaTracker {
    fn update(&mut self, value: f64) -> f64 {
        self.sum += value;
        self.values.push_back(value);

        if self.values.len() > self.period {
            if let Some(outdated) = self.values.pop_front() {
                self.sum -= outdated;
            }
        }

        self.calculate()
    }

    fn calculate(&self) -> f64 {
        if self.values.is_empty() {
            return 0_f64;
        }

        match &self.method {
            MeanCalculationMethod::SimpleMA => { self.sum / (self.values.len() as f64) }
            MeanCalculationMethod::EMA => todo!(),
            MeanCalculationMethod::VMAP => todo!(),
        }
    }
}
