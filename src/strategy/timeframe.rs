use std::fmt::Debug;

use binance_spot_connector_rust::market::klines::KlineInterval;
use serde::{ de::Visitor, Deserialize, Deserializer, Serialize, Serializer };

use crate::api::error::ApiError;

#[derive(Deserialize, Serialize)]
pub struct StrategyTimeframe {
    #[serde(
        deserialize_with = "deserialize_kline_interval",
        serialize_with = "serialize_kline_interval"
    )]
    pub interval: KlineInterval,
    #[serde(with = "humantime_serde")]
    pub tick: std::time::Duration,
    #[serde(with = "humantime_serde")]
    pub execution: std::time::Duration,

    pub period_measurement: PeriodMeasurement,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PeriodMeasurement {
    pub measure_bars: usize,
    // todo enum
    pub mean_calculation_method: String,
}

impl Debug for StrategyTimeframe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StrategyTimeframe {{interval: {:?}, tick: {:?}, execution: {:?}, period_measurement: {:?}}}",
            self.interval.to_string(),
            self.tick,
            self.execution,
            self.period_measurement
        )
    }
}

fn deserialize_kline_interval<'de, D>(deserializer: D) -> Result<KlineInterval, D::Error>
    where D: Deserializer<'de>
{
    struct StrVisitor;

    impl<'de> Visitor<'de> for StrVisitor {
        type Value = KlineInterval;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Expected a valid KlineInterval")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
            map_string_to_kline(v).map_err(serde::de::Error::custom)
        }
    }

    deserializer.deserialize_str(StrVisitor)
}

fn serialize_kline_interval<S>(val: &KlineInterval, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    s.serialize_str(&val.to_string())
}

fn map_string_to_kline<'a>(str: &'a str) -> Result<KlineInterval, ApiError> {
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
