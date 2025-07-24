use tokio_tungstenite::tungstenite::Bytes;

use crate::bot::kline_processor::Candle;

#[derive(Debug)]
pub enum ApiClientMessage {
    Candle(Candle),
}

#[derive(Debug)]
pub enum ClientMessage {
    Pong(Bytes),
}
