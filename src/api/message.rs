use tokio_tungstenite::tungstenite::Bytes;

#[derive(Debug)]
pub enum ApiClientMessage {
    AvgPrice(String),
}

#[derive(Debug)]
pub enum ClientMessage {
    AvgPrice(String),
    Pong(Bytes),
}
