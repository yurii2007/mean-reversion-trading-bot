use async_trait::async_trait;
use tracing::{debug, info};

use crate::client::ApiClient;

#[derive(Debug, PartialEq)]
pub struct BinanceClient {}

impl BinanceClient {
    pub fn new() -> Self {
        debug!("Binance Client initialized");
        Self {}
    }
}

#[async_trait]
impl ApiClient for BinanceClient {
    async fn connect(&self) {
        info!("Successfully connected to the binance API");
    }
}
