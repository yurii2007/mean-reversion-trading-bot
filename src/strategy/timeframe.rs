use std::fmt::Debug;

use binance_spot_connector_rust::market::klines::KlineInterval;
use serde::{de::Visitor, Deserialize, Deserializer};

use crate::{api::ApiError, strategy::ma_tracker::MeanCalculationMethod};

#[derive(Deserialize)]
pub struct StrategyTimeframe {
    #[serde(
        deserialize_with = "deserialize_kline_interval",
        serialize_with = "serialize_kline_interval"
    )]
    pub interval: KlineInterval,
    #[serde(with = "humantime_serde")]
    pub tick: std::time::Duration,

    pub period_measurement: PeriodMeasurement,
}

#[derive(Debug, Deserialize)]
pub struct PeriodMeasurement {
    pub measure_bars: usize,
    // enum
    pub mean_calculation_method: MeanCalculationMethod,
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

fn deserialize_kline_interval<'de, D>(deserializer: D) -> Result<KlineInterval, D::Error>
where
    D: Deserializer<'de>,
{
    struct StrVisitor;

    impl Visitor<'_> for StrVisitor {
        type Value = KlineInterval;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Expected a valid KlineInterval")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            map_string_to_kline(v).map_err(serde::de::Error::custom)
        }
    }

    deserializer.deserialize_str(StrVisitor)
}

fn map_string_to_kline(str: &str) -> Result<KlineInterval, ApiError> {
    match str {
        "1m" => Ok(KlineInterval::Minutes1),
        "3m" => Ok(KlineInterval::Minutes3),
        "5m" => Ok(KlineInterval::Minutes5),
        "15m" => Ok(KlineInterval::Minutes15),
        "30m" => Ok(KlineInterval::Minutes30),
        "1h" => Ok(KlineInterval::Hours1),
        "2h" => Ok(KlineInterval::Hours2),
        "4h" => Ok(KlineInterval::Hours4),
        "6h" => Ok(KlineInterval::Hours6),
        "8h" => Ok(KlineInterval::Hours8),
        "12h" => Ok(KlineInterval::Hours12),
        "1d" => Ok(KlineInterval::Days1),
        "3d" => Ok(KlineInterval::Days3),
        "1w" => Ok(KlineInterval::Weeks1),
        "1M" => Ok(KlineInterval::Months1),
        _ => Err(ApiError::ParseError("Invalid interval".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use toml;

    use super::*;

    #[test]
    fn test_timeframe_deserializing() {
        let toml_timeframe = r#"
            interval = "2h"
            tick = "30m"

            [period_measurement]
            measure_bars = 20
            mean_calculation_method = "SimpleMA"  
        "#;

        let timeframe: StrategyTimeframe = toml::from_str(toml_timeframe).unwrap();
        assert_eq!(timeframe.interval.to_string(), "2h");
        assert_eq!(timeframe.tick, Duration::from_secs(60 * 30));
        assert_eq!(timeframe.period_measurement.measure_bars, 20);
        assert_eq!(
            timeframe.period_measurement.mean_calculation_method,
            MeanCalculationMethod::SimpleMA
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_invalid_toml() {
        let toml_timeframe = r#"
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
            mean_calculation_method: MeanCalculationMethod::SimpleMA,
            measure_bars: 20,
        };
        let strategy_timeframe = StrategyTimeframe {
            interval: KlineInterval::Hours1,
            tick: Duration::from_secs(30 * 60),
            period_measurement,
        };

        let strategy_str = format!("{:?}", strategy_timeframe);
        let expected_str = String::from(
            "StrategyTimeframe {interval: \"1h\", tick: 1800s, period_measurement: PeriodMeasurement { measure_bars: 20, mean_calculation_method: SimpleMA }}"
        );

        assert_eq!(strategy_str, expected_str)
    }
}
