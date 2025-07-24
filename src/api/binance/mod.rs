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

use crate::api::{
    binance::{
        binance_request::BinanceStream,
        binance_response::{BinanceKlinePayload, BinanceStreamResponse},
    },
    error::ApiError,
    message::{ApiClientMessage, ClientMessage},
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
    pub async fn subscribe(&mut self, streams: Vec<BinanceStream>) -> Result<(), ApiError> {
        let ws_base_url =
            dotenv::var("BINANCE_WS_URL").expect("Failed to read BINANCE_WS_URL from .env");

        let ws_stream_url = BinanceClient::get_subscribe_url(ws_base_url, &streams);

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
                                    debug!("Deserialized raw response: {:?}", response);

                                    if let Err(e) = BinanceClient::send_response(response, &api_tx)
                                    {
                                        error!("Failed to send message: {}", e);
                                    }
                                }
                                Err(e) => error!("Failed to deserialized response: {}", e),
                            }
                        }
                        Message::Binary(_bytes) => {
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
                        Message::Pong(_bytes) => {
                            info!("REceived pong");
                        }
                        Message::Close(_frame) => {
                            info!("recevied close ");
                        }
                        Message::Frame(_frame) => {
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

    fn get_subscribe_url(base_url: String, streams: &[BinanceStream]) -> String {
        let mut url = format!("{base_url}/stream?streams=");

        let stream_signatures: Vec<String> = streams.iter().map(|s| s.into()).collect();

        url.push_str(stream_signatures.join("/").as_str());

        url
    }

    fn send_response(
        raw_response: BinanceStreamResponse<'_>,
        api_tx: &UnboundedSender<ApiClientMessage>,
    ) -> Result<(), ApiError> {
        match raw_response.stream {
            BinanceStream::KlineStream(_) => {
                let binance_raw_kline =
                    serde_json::from_str::<'_, BinanceKlinePayload>(raw_response.data.get())?;

                if let Err(e) = api_tx.send(ApiClientMessage::Candle(binance_raw_kline.into())) {
                    error!("Failed to send candle data: {}", e);
                    return Err(ApiError::SendError(e.to_string()));
                }
            }
        }

        Ok(())
    }
}
