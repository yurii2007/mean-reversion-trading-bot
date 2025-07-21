use std::time::Duration;

use tokio::sync::mpsc::unbounded_channel;
use tracing::info;

use crate::{
    api::{
        get_client,
        message::{ApiClientMessage, ClientMessage},
    },
    client::{error::ClientError, ApiClient},
    strategy::Strategy,
};

mod kline_processor;

pub struct Bot {
    client: Box<dyn ApiClient>,
}

impl Bot {
    pub fn try_init(strategy: Strategy) -> Result<Self, ClientError> {
        let client = get_client(&strategy.exchange.api);
        Ok(Self {
            client: Box::new(client),
        })
    }

    pub async fn run(mut self) -> Result<(), ClientError> {
        info!("Starting a bot");
        let (api_tx, mut api_rx) = unbounded_channel::<ApiClientMessage>();
        let (client_tx, client_rx) = unbounded_channel::<ClientMessage>();

        let (api_write_task, api_read_task) = self
            .client
            .connect(api_tx, client_tx.clone(), client_rx)
            .await?;

        tokio::time::sleep(Duration::from_secs(2)).await;

        // let message = ClientMessage::AvgPrice(String::from("BTCUSDT"));
        // let _ = client_tx.send(message);

        let api_signal_handler_task = tokio::spawn(async move {
            while let Some(message) = api_rx.recv().await {
                info!("Received message from API: {:?}", message);
            }
        });

        let _ = tokio::join!(api_write_task, api_read_task, api_signal_handler_task);

        Ok(())
    }
}
