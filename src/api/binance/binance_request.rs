use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use crate::strategy::Strategy;

#[derive(Debug)]
pub enum BinanceStream {
    KlineStream(String),
}

impl BinanceStream {
    pub fn get_stream_signatures(strategy: &Strategy) -> Vec<BinanceStream> {
        let kline_stream_signature = format!(
            "{}@kline_{}",
            strategy.symbol.to_lowercase(),
            strategy.timeframe.interval
        );

        vec![BinanceStream::KlineStream(kline_stream_signature)]
    }
}

impl<'de> Deserialize<'de> for BinanceStream {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BinanceStreamVisitor;

        impl Visitor<'_> for BinanceStreamVisitor {
            type Value = BinanceStream;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Expected valid stream signature")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    val if val.contains("kline") => Ok(BinanceStream::KlineStream(String::from(v))),
                    _ => Err(serde::de::Error::custom("Unknown stream variant")),
                }
            }
        }

        deserializer.deserialize_str(BinanceStreamVisitor)
    }
}

impl From<&BinanceStream> for String {
    fn from(value: &BinanceStream) -> Self {
        match value {
            BinanceStream::KlineStream(signature) => signature.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BinanceRequest<T: Serialize> {
    id: Uuid,
    method: String,
    params: Option<T>,
}

impl<T: Serialize> BinanceRequest<T> {
    pub fn new(method: String, params: Option<T>) -> Self {
        Self {
            id: Uuid::new_v4(),
            method,
            params,
        }
    }
}
