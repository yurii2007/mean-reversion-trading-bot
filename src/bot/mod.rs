use std::time::Duration;

use tokio::sync::mpsc::unbounded_channel;
use tracing::{debug, info};

use crate::{
    api::{
        binance::{
            binance_request::{rest_api::BinanceRestApi, stream::BinanceStream},
            BinanceClient,
        },
        message::{ApiClientMessage, ClientMessage},
        ApiError,
    },
    strategy::{
        ma_tracker::{get_ma_tracker, MaTracker},
        Strategy,
    },
};

pub mod kline_processor;

pub struct Bot {
    client: BinanceClient,
    strategy: Strategy,
    ma_tracker: Box<dyn MaTracker + Send>,
}

impl Bot {
    pub fn try_init(strategy: Strategy) -> Result<Self, ApiError> {
        let client = BinanceClient::new();
        let ma_tracker = get_ma_tracker(
            &strategy
                .timeframe
                .period_measurement
                .mean_calculation_method,
        );

        Ok(Self {
            client,
            strategy,
            ma_tracker,
        })
    }

    pub async fn run(mut self) -> Result<(), ApiError> {
        info!("Starting a bot");
        let (api_tx, mut api_rx) = unbounded_channel::<ApiClientMessage>();
        let (client_tx, client_rx) = unbounded_channel::<ClientMessage>();
        let streams = BinanceStream::get_stream_signatures(&self.strategy);

        let initial_candles_data = BinanceRestApi::get_kline_data(&self.strategy).await?;
        debug!("received initial candle data: {:?}", initial_candles_data);

        self.client.subscribe(streams).await?;
        let (api_read_task, api_write_task) = self
            .client
            .listen(api_tx, client_tx.clone(), client_rx)
            .await?;

        tokio::time::sleep(Duration::from_secs(2)).await;

        let api_signal_handler_task = tokio::spawn(async move {
            while let Some(message) = api_rx.recv().await {
                debug!("Received message from API: {:?}", message);
                match message {
                    ApiClientMessage::Candle(candle) => {
                        self.ma_tracker.update(&candle);

                        info!("Received new candle data: {:?}", candle);
                    }
                }
            }
        });

        let _ = tokio::join!(api_read_task, api_write_task, api_signal_handler_task);

        Ok(())
    }
}
