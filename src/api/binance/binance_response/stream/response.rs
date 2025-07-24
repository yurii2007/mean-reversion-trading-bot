use serde::Deserialize;
use serde_json::value::RawValue;

use crate::api::binance::binance_request::stream::BinanceStream;

#[derive(Debug, Deserialize)]
pub struct BinanceStreamResponse<'a> {
    pub stream: BinanceStream,
    #[serde(borrow)]
    pub data: &'a RawValue,
}
