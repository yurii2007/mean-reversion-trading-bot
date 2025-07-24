use std::time::Duration;

use tokio::sync::mpsc::unbounded_channel;
use tracing::info;

use crate::{
    api::{
        binance::{binance_request::BinanceStream, BinanceClient},
        message::{ApiClientMessage, ClientMessage},
        ApiError,
    },
    strategy::Strategy,
};

pub mod kline_processor;

pub struct Bot {
    client: BinanceClient,
    strategy: Strategy,
}

impl Bot {
    pub fn try_init(strategy: Strategy) -> Result<Self, ApiError> {
        let client = BinanceClient::new();

        Ok(Self { client, strategy })
    }

    pub async fn run(mut self) -> Result<(), ApiError> {
        info!("Starting a bot");
        let (api_tx, mut api_rx) = unbounded_channel::<ApiClientMessage>();
        let (client_tx, client_rx) = unbounded_channel::<ClientMessage>();
        let streams = BinanceStream::get_stream_signatures(&self.strategy);

        self.client.subscribe(streams).await?;
        let (api_read_task, api_write_task) = self
            .client
            .listen(api_tx, client_tx.clone(), client_rx)
            .await?;

        tokio::time::sleep(Duration::from_secs(2)).await;

        let api_signal_handler_task = tokio::spawn(async move {
            while let Some(message) = api_rx.recv().await {
                info!("Received message from API: {:?}", message);
                match message {
                    ApiClientMessage::Candle(candle) => {
                        info!("Received new candle data: {:?}", candle);
                    }
                }
            }
        });

        let _ = tokio::join!(api_read_task, api_write_task, api_signal_handler_task);

        Ok(())
    }
}
