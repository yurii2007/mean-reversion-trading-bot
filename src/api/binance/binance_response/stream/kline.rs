use serde::Deserialize;
use time::UtcDateTime;

use crate::{
    api::binance::binance_response::{deserialize_float, deserialize_timestamp},
    bot::kline_processor::Candle,
};

#[derive(Debug, Deserialize)]
pub struct BinanceKlinePayload {
    e: String, // Event type
    #[serde(deserialize_with = "deserialize_timestamp")]
    E: UtcDateTime, // Event time
    s: String, // Symbol
    k: BinanceKlineRawResponse,
}

#[derive(Debug, Deserialize)]
pub struct BinanceKlineRawResponse {
    #[serde(deserialize_with = "deserialize_timestamp")]
    t: UtcDateTime, // Kline start time
    #[serde(deserialize_with = "deserialize_timestamp")]
    T: UtcDateTime, // Kline close time
    s: String, // Symbol
    i: String, // Interval
    f: i64,    // First trade ID
    L: i64,    // Last trade ID
    #[serde(deserialize_with = "deserialize_float")]
    o: f64, // Open price
    #[serde(deserialize_with = "deserialize_float")]
    c: f64, // Close price
    #[serde(deserialize_with = "deserialize_float")]
    h: f64, // High price
    #[serde(deserialize_with = "deserialize_float")]
    l: f64, // Low price
    #[serde(deserialize_with = "deserialize_float")]
    v: f64, // Base asset volume
    n: u32,    // Number of trades
    x: bool,   // Is this kline closed?
    #[serde(deserialize_with = "deserialize_float")]
    q: f64, // Quote asset volume
    #[serde(deserialize_with = "deserialize_float")]
    V: f64, // Taker buy base asset volume
    #[serde(deserialize_with = "deserialize_float")]
    Q: f64, // Taker buy quote asset volume
    #[serde(skip_deserializing)]
    B: bool,
}

impl From<BinanceKlinePayload> for Candle {
    fn from(value: BinanceKlinePayload) -> Self {
        value.k.into()
    }
}

impl From<BinanceKlineRawResponse> for Candle {
    fn from(value: BinanceKlineRawResponse) -> Self {
        let open_timestamp = value.t;
        let close_timestamp = value.T;
        let open_price = value.o;
        let high_price = value.h;
        let low_price = value.l;
        let close_price = value.c;

        Self {
            open_timestamp,
            close_timestamp,
            open_price,
            high_price,
            low_price,
            close_price,
        }
    }
}

// {
//   "e": "kline",         // Event type
//   "E": 1672515782136,   // Event time
//   "s": "BNBBTC",        // Symbol
//   "k": {
//     "t": 1672515780000, // Kline start time
//     "T": 1672515839999, // Kline close time
//     "s": "BNBBTC",      // Symbol
//     "i": "1m",          // Interval
//     "f": 100,           // First trade ID
//     "L": 200,           // Last trade ID
//     "o": "0.0010",      // Open price
//     "c": "0.0020",      // Close price
//     "h": "0.0025",      // High price
//     "l": "0.0015",      // Low price
//     "v": "1000",        // Base asset volume
//     "n": 100,           // Number of trades
//     "x": false,         // Is this kline closed?
//     "q": "1.0000",      // Quote asset volume
//     "V": "500",         // Taker buy base asset volume
//     "Q": "0.500",       // Taker buy quote asset volume
//     "B": "123456"       // Ignore
//   }
// }
