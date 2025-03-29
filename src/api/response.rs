use std::{ borrow::Cow, fmt };

use serde::{ de::Visitor, Deserialize, Deserializer };
use time::OffsetDateTime;

#[derive(Debug)]
pub struct BinanceResponse {
    pub open_timestamp: OffsetDateTime,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub volume: f64,
    pub close_timestamp: OffsetDateTime,
    pub quote_asset_vol: f64,
    pub num_of_trades: u32,
    pub taker_buy_base_asset_vol: f64,
    pub taker_buy_quote_asset_vol: f64,
}

#[derive(Debug, Deserialize)]
struct RawResponse(
    #[serde(deserialize_with = "deserialize_timestamp")] OffsetDateTime,
    #[serde(deserialize_with = "deserialize_float")] f64,
    #[serde(deserialize_with = "deserialize_float")] f64,
    #[serde(deserialize_with = "deserialize_float")] f64,
    #[serde(deserialize_with = "deserialize_float")] f64,
    #[serde(deserialize_with = "deserialize_float")] f64,
    #[serde(deserialize_with = "deserialize_timestamp")] OffsetDateTime,
    #[serde(deserialize_with = "deserialize_float")] f64,
    u32,
    #[serde(deserialize_with = "deserialize_float")] f64,
    #[serde(deserialize_with = "deserialize_float")] f64,
    String,
);

impl BinanceResponse {
    pub fn deserialize_response(json_data: Cow<'_, str>) -> Result<Vec<Self>, String> {
        let raw_response: Vec<RawResponse> = serde_json::from_str(&json_data).unwrap();

        Ok(raw_response.into_iter().map(Self::from).collect())
    }
}

impl From<RawResponse> for BinanceResponse {
    fn from(value: RawResponse) -> Self {
        let open_timestamp = value.0;
        let open_price = value.1;
        let high_price = value.2;
        let low_price = value.3;
        let close_price = value.4;
        let volume = value.5;
        let close_timestamp = value.6;
        let quote_asset_vol = value.7;
        let num_of_trades = value.8;
        let taker_buy_base_asset_vol = value.9;
        let taker_buy_quote_asset_vol = value.10;

        Self {
            open_timestamp,
            open_price,
            high_price,
            low_price,
            close_price,
            volume,
            close_timestamp,
            quote_asset_vol,
            num_of_trades,
            taker_buy_base_asset_vol,
            taker_buy_quote_asset_vol,
        }
    }
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where D: Deserializer<'de>
{
    struct TimestampVisitor;

    impl<'de> Visitor<'de> for TimestampVisitor {
        type Value = OffsetDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Expected a valid timestamp")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: serde::de::Error {
            OffsetDateTime::from_unix_timestamp_nanos((v as i128) * 1_000_000).map_err(E::custom)
        }
    }

    deserializer.deserialize_u64(TimestampVisitor)
}

fn deserialize_float<'de, D>(deserializer: D) -> Result<f64, D::Error> where D: Deserializer<'de> {
    struct FloatVisitor;

    impl<'de> Visitor<'de> for FloatVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Expected a valid f64")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
            v.parse().map_err(E::custom)
        }
    }

    deserializer.deserialize_str(FloatVisitor)
}
