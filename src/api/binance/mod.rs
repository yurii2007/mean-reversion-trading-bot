use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{
    net::TcpStream,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info};

use crate::{
    api::{
        binance::{
            binance_request::{AveragePriceRequest, BinanceRequest},
            binance_response::BinanceStreamResponse,
        },
        error::ApiError,
        message::{ApiClientMessage, ClientMessage},
    },
    strategy::Strategy,
};

pub mod binance_request;
pub mod binance_response;

#[derive(Debug)]
pub struct BinanceClient {
    stream_write: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    stream_read: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

impl BinanceClient {
    pub fn new() -> Self {
        debug!("Binance Client initialized");

        Self {
            stream_write: None,
            stream_read: None,
        }
    }
}

impl BinanceClient {
    pub async fn subscribe(&mut self, streams: Vec<String>) -> Result<(), ApiError> {
        let ws_base_url =
            dotenv::var("BINANCE_WS_URL").expect("Failed to read BINANCE_WS_URL from .env");

        let ws_stream_url = BinanceClient::get_subscribe_url(ws_base_url, streams);
        debug!("Trying to connect to the url: {}", ws_stream_url);

        let (ws_stream, _) = match connect_async(&ws_stream_url).await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(ApiError::ConnectionError(format!(
                    "Failed to connect: {}",
                    e
                )))
            }
        };

        let (stream_write, stream_read) = ws_stream.split();

        self.stream_write = Some(stream_write);
        self.stream_read = Some(stream_read);

        Ok(())
    }

    pub async fn listen(
        &mut self,
        api_tx: UnboundedSender<ApiClientMessage>,
        client_tx: UnboundedSender<ClientMessage>,
        mut client_rx: UnboundedReceiver<ClientMessage>,
    ) -> Result<(JoinHandle<()>, JoinHandle<()>), ApiError> {
        let mut read_stream =
            self.stream_read
                .take()
                .ok_or(ApiError::ConnectionError(String::from(
                    "Failed to start listening, not connected to the ws",
                )))?;
        let mut write_stream =
            self.stream_write
                .take()
                .ok_or(ApiError::ConnectionError(String::from(
                    "Failed to start listening, not connected to the ws",
                )))?;

        let read_task = tokio::spawn(async move {
            debug!("Starting listening wss api");
            while let Some(msg) = read_stream.next().await {
                match msg {
                    Ok(message) => match message {
                        Message::Text(text) => {
                            info!("Received text message {}", text);
                            let raw_response =
                                serde_json::from_str::<BinanceStreamResponse>(text.as_str());
                            match raw_response {
                                Ok(response) => {
                                    info!("Deserialized value: {:?}", response);
                                }
                                Err(e) => error!("Failed to deserialized response: {}", e),
                            }
                        }
                        Message::Binary(bytes) => {
                            info!("received binary message, ");
                        }
                        Message::Ping(bytes) => {
                            debug!("Received ping");
                            match client_tx.send(ClientMessage::Pong(bytes)) {
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

        let write_task = tokio::spawn(async move {
            info!("Starting listening internal request signals");

            while let Some(msg) = client_rx.recv().await {
                match msg {
                    ClientMessage::AvgPrice(symbol) => {
                        info!("Sending Request Message for average price for {}", symbol);
                        let request: BinanceRequest<AveragePriceRequest> =
                            AveragePriceRequest::new(symbol).into();

                        if let Ok(json) = serde_json::to_string(&request) {
                            if let Err(e) = write_stream.send(Message::Text(json.into())).await {
                                error!("Failed to send pong back: {}", e);
                            };
                        }
                    }
                    ClientMessage::Pong(payload) => {
                        debug!("Sending pong frame");
                        if let Err(e) = write_stream.send(Message::Pong(payload)).await {
                            error!("Failed to send pong back: {}", e);
                        };
                    }
                }
            }
        });

        Ok((read_task, write_task))
    }

    fn get_subscribe_url(base_url: String, streams: Vec<String>) -> String {
        let mut url = format!("{base_url}/stream?streams=");

        url.push_str(streams.join("/").as_str());

        url
    }
}

pub fn get_binance_streams(strategy: &Strategy) -> Vec<String> {
    let kline_stream = format!(
        "{}@kline_{}",
        strategy.symbol.to_lowercase(),
        strategy.timeframe.interval
    );

    vec![kline_stream]
}
