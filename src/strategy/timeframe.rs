use std::fmt::Debug;

use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct StrategyTimeframe {
    pub interval: String,
    #[serde(with = "humantime_serde")]
    pub tick: std::time::Duration,

    pub period_measurement: PeriodMeasurement,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PeriodMeasurement {
    pub measure_bars: usize,
    // enum
    pub mean_calculation_method: String,
}

impl Debug for StrategyTimeframe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StrategyTimeframe {{interval: {:?}, tick: {:?}, period_measurement: {:?}}}",
            self.interval.to_string(),
            self.tick,
            self.period_measurement
        )
    }
}
