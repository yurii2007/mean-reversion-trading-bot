use serde::Deserialize;
use serde_json::value::RawValue;

use crate::api::binance::binance_request::BinanceStream;

#[derive(Debug, Deserialize)]
pub struct BinanceStreamResponse<'a> {
    pub stream: BinanceStream,
    #[serde(borrow)]
    pub data: &'a RawValue,
}
