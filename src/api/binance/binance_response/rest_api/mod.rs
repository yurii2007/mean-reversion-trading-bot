use serde::Deserialize;
use time::UtcDateTime;

use crate::{
    api::binance::binance_response::{deserialize_float, deserialize_timestamp},
    bot::kline_processor::Candle,
};

#[derive(Debug, Deserialize)]
pub struct BinanceRawKlineResponse(
    #[serde(deserialize_with = "deserialize_timestamp")] UtcDateTime, // open timestamp
    #[serde(deserialize_with = "deserialize_float")] f64,             // open price
    #[serde(deserialize_with = "deserialize_float")] f64,             // high price
    #[serde(deserialize_with = "deserialize_float")] f64,             // low price
    #[serde(deserialize_with = "deserialize_float")] f64,             // close price
    #[serde(deserialize_with = "deserialize_float")] f64,             // volume
    #[serde(deserialize_with = "deserialize_timestamp")] UtcDateTime, // close timestamp
    #[serde(deserialize_with = "deserialize_float")] f64,             // quote asset volume
    u32,                                                              // number of trades
    #[serde(deserialize_with = "deserialize_float")] f64,             // taker buy base asset volume
    #[serde(deserialize_with = "deserialize_float")] f64, // taker buy qoute asset volume
    String,                                               // unused
);

impl From<BinanceRawKlineResponse> for Candle {
    fn from(value: BinanceRawKlineResponse) -> Self {
        let open_timestamp = value.0;
        let open_price = value.1;
        let high_price = value.2;
        let low_price = value.3;
        let close_price = value.4;
        let close_timestamp = value.6;

        Candle {
            open_timestamp,
            close_timestamp,
            open_price,
            high_price,
            low_price,
            close_price,
        }
    }
}
