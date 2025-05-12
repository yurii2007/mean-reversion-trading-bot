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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use toml;

    use super::*;

    #[test]
    fn test_timeframe_deserializing() {
        let toml_timeframe =
            r#"
            interval = "2h"
            tick = "30m"

            [period_measurement]
            measure_bars = 20
            mean_calculation_method = "SimpleMA"  
        "#;

        let timeframe: StrategyTimeframe = toml::from_str(toml_timeframe).unwrap();
        assert_eq!(timeframe.interval, "2h");
        assert_eq!(timeframe.tick, Duration::from_secs(60 * 30));
        assert_eq!(timeframe.period_measurement.measure_bars, 20);
        assert_eq!(timeframe.period_measurement.mean_calculation_method, "SimpleMA");
    }

    #[test]
    #[should_panic]
    fn test_panic_invalid_toml() {
        let toml_timeframe =
            r#"
        interval = "1"
        tick = "30m"

        [period_measurement]
        measure_bars = ""
        mean_calculation_method = "Foo"
        "#;

        let _: StrategyTimeframe = toml::from_str(toml_timeframe).unwrap();
    }

    #[test]
    fn test_valid_debug() {
        let period_measurement = PeriodMeasurement {
            mean_calculation_method: String::from("SimpleMA"),
            measure_bars: 20,
        };
        let strategy_timeframe = StrategyTimeframe {
            interval: String::from("1h"),
            tick: Duration::from_secs(30 * 60),
            period_measurement,
        };

        let strategy_str = format!("{:?}", strategy_timeframe);
        let expected_str = String::from(
            "StrategyTimeframe {interval: \"1h\", tick: 1800s, period_measurement: PeriodMeasurement { measure_bars: 20, mean_calculation_method: \"SimpleMA\" }}"
        );

        assert_eq!(strategy_str, expected_str)
    }
}
