use std::{ fmt::Debug, time::Duration };

use binance_spot_connector_rust::market::klines::KlineInterval;
use serde::{ de::Visitor, Deserialize, Deserializer, Serialize, Serializer };

use crate::api::error::ApiError;
use super::mean_calculation::MeanCalculationMethod;

#[derive(Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct PeriodMeasurement {
    pub measure_bars: usize,
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
    where D: Deserializer<'de>
{
    struct StrVisitor;

    impl Visitor<'_> for StrVisitor {
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

pub fn duration_from_kline_interval(interval: &KlineInterval) -> std::time::Duration {
    match interval {
        KlineInterval::Minutes1 => std::time::Duration::from_secs(60),
        KlineInterval::Minutes3 => std::time::Duration::from_secs(180),
        KlineInterval::Minutes5 => std::time::Duration::from_secs(300),
        KlineInterval::Minutes15 => std::time::Duration::from_secs(900),
        KlineInterval::Minutes30 => std::time::Duration::from_secs(1_800),
        KlineInterval::Hours1 => std::time::Duration::from_secs(3_600),
        KlineInterval::Hours2 => std::time::Duration::from_secs(7_200),
        KlineInterval::Hours4 => std::time::Duration::from_secs(14_400),
        KlineInterval::Hours6 => std::time::Duration::from_secs(21_600),
        KlineInterval::Hours8 => std::time::Duration::from_secs(28_800),
        KlineInterval::Hours12 => std::time::Duration::from_secs(43_200),
        KlineInterval::Days1 => std::time::Duration::from_secs(86_400),
        KlineInterval::Days3 => std::time::Duration::from_secs(259_200),
        KlineInterval::Weeks1 => std::time::Duration::from_secs(604_800),
        KlineInterval::Months1 => std::time::Duration::from_secs(2_419_200),
    }
}

pub fn duration_into_kline_interval(duration: &Duration) -> Option<KlineInterval> {
    match duration.as_secs() {
        60 => Some(KlineInterval::Minutes1),
        180 => Some(KlineInterval::Minutes3),
        300 => Some(KlineInterval::Minutes5),
        900 => Some(KlineInterval::Minutes15),
        1_800 => Some(KlineInterval::Minutes30),
        3_600 => Some(KlineInterval::Hours1),
        7_200 => Some(KlineInterval::Hours2),
        14_400 => Some(KlineInterval::Hours4),
        21_600 => Some(KlineInterval::Hours6),
        28_800 => Some(KlineInterval::Hours8),
        43_200 => Some(KlineInterval::Hours12),
        86_400 => Some(KlineInterval::Days1),
        259_200 => Some(KlineInterval::Days3),
        604_800 => Some(KlineInterval::Weeks1),
        2_419_200 => Some(KlineInterval::Months1),
        _ => None,
    }
}
