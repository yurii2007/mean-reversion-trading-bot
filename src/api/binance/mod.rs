use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, info};

use crate::{
    api::{
        binance::binance_request::{AveragePriceRequest, BinanceRequest, StreamSubscribeRequest},
        message::{ApiClientMessage, ClientMessage},
    },
    client::{error::ClientError, ApiClient},
};

mod binance_request;
mod binance_response;

#[derive(Debug)]
pub struct BinanceClient {}

impl BinanceClient {
    pub fn new() -> Self {
        debug!("Binance Client initialized");

        Self {}
    }
}

#[async_trait]
impl ApiClient for BinanceClient {
    async fn connect(
        &mut self,
        api_tx: UnboundedSender<ApiClientMessage>,
        request_tx: UnboundedSender<ClientMessage>,
        mut request_rx: UnboundedReceiver<ClientMessage>,
    ) -> Result<(JoinHandle<()>, JoinHandle<()>), ClientError> {
        let ws_url =
            // dotenv::var("BINANCE_WS_URL").expect("Failed to read BINANCE_WS_URL env variable");
            dotenv::var("BINANCE_WS_STREAM_URL").expect("Failed to read BINANCE_WS_STREAM_URL env variable");

        debug!("Trying to connect to the url: {}", ws_url);

        let (ws_stream, _) = match connect_async(&ws_url).await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(ClientError::ConnectionError(format!(
                    "Failed to connect: {}",
                    e
                )))
            }
        };
        info!("Successfuly connected to the WSS server");

        let (mut write, mut read) = ws_stream.split();

        let write_task = tokio::spawn(async move {
            info!("Subscribing to the btcusdt kline stream");
            let subscribe_request =
                StreamSubscribeRequest::new(vec![String::from("btcusdt@kline_4h")]);

            if let Ok(json) = serde_json::to_string(&subscribe_request) {
                if let Err(e) = write.send(Message::Text(json.into())).await {
                    error!("Failed to send subscribe request: {}", e);
                }
            }

            info!("Starting listening internal signal handler");

            while let Some(msg) = request_rx.recv().await {
                match msg {
                    ClientMessage::AvgPrice(symbol) => {
                        info!("Sending Request Message for average price for {}", symbol);
                        let request: BinanceRequest<AveragePriceRequest> =
                            AveragePriceRequest::new(symbol).into();

                        if let Ok(json) = serde_json::to_string(&request) {
                            if let Err(e) = write.send(Message::Text(json.into())).await {
                                error!("Failed to send pong back: {}", e);
                            };
                        }
                    }
                    ClientMessage::Pong(payload) => {
                        debug!("Sending pong frame");
                        if let Err(e) = write.send(Message::Pong(payload)).await {
                            error!("Failed to send pong back: {}", e);
                        };
                    }
                }
            }
        });

        let read_task = tokio::spawn(async move {
            info!("Starting listening WSS Api");
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(message) => match message {
                        Message::Text(text) => {
                            info!("Received text message {}", text);
                            if let Err(e) =
                                api_tx.send(ApiClientMessage::AvgPrice(text.to_string()))
                            {
                                error!("Failed to send back response from an API: {}", e);
                            }
                        }
                        Message::Binary(bytes) => {
                            info!("received binary message, ");
                        }
                        Message::Ping(bytes) => {
                            debug!("Received ping");
                            match request_tx.send(ClientMessage::Pong(bytes)) {
                                Ok(_) => {
                                    debug!("Sent a pong frame");
                                }
                                Err(e) => {
                                    error!("Failed to send pong frame: {}", e);
                                }
                            }
                        }
                        Message::Pong(bytes) => {
                            info!("REceived pong");
                        }
                        Message::Close(frame) => {
                            info!("recevied close ");
                        }
                        Message::Frame(frame) => {
                            info!("recevied frame ");
                        }
                    },
                    Err(e) => {
                        error!("ws error: {}", e);
                    }
                }
            }
        });

        info!("Successfully connected to the binance API");

        Ok((write_task, read_task))
    }

    async fn send(&self, msg: ClientMessage) -> Result<(), ClientError> {
        Ok(())
    }
}
