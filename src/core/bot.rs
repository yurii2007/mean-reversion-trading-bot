use tracing::{ debug, error, info, trace, warn };

use crate::{
    api::{ client::ApiClient, error::ApiError },
    strategy::{ mean_calculation::SimpleMa, strategy::Strategy },
};

use super::market::{ Position, ProcessedCandle };

pub struct Bot {
    strategy: Strategy,
    api_client: Box<dyn ApiClient>,
    long_ma: SimpleMa,
    short_ma: SimpleMa,
    candles: Vec<ProcessedCandle>,
    open_positions: Vec<Position>,
    account_balance: f64,
}

impl Bot {
    pub fn new(strategy: Strategy, balance: f64) -> Self {
        let long_period = strategy.timeframe.period_measurement.measure_bars;
        let short_period = long_period / 3;
        let api_client = strategy.exchange.api.get_client();

        Self {
            strategy,
            api_client: Box::new(api_client),
            account_balance: balance,
            candles: Vec::new(),
            long_ma: SimpleMa::new(long_period),
            short_ma: SimpleMa::new(short_period),
            open_positions: Vec::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<(), ApiError> {
        let candles = self.api_client.get_candles(&self.strategy).await?;

        self.candles = candles
            .into_iter()
            .map(|candle| {
                self.long_ma.update(candle.close);
                self.short_ma.update(candle.close);

                candle
            })
            .collect();

        info!("Bot initialized with {} candles", self.candles.len());

        debug!("Opening candles data: {:?}", self.candles);

        info!(
            "Strating with {{short_ma: {}, long_ma: {} }}",
            self.short_ma.calculate(),
            self.long_ma.calculate()
        );

        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), ApiError> {
        info!("Starting bot for symbol: {}", self.strategy.symbol);

        self.initialize().await?;

        let mut interval = tokio::time::interval(self.strategy.timeframe.execution);

        loop {
            info!("Waiting for the next execution cycle");
            interval.tick().await;

            if let Err(e) = self.execute_trading_cycle().await {
                error!("Error executing trading cycle: {}", e);
            }
        }
    }

    async fn execute_trading_cycle(&mut self) -> Result<(), ApiError> {
        info!("Starting trading cycle");

        let latest_candle = self.api_client.get_latest_candle(&self.strategy).await?;

        info!("Received the latest candle: {:?}", latest_candle);

        let short_ma = self.short_ma.update(latest_candle.close);
        let long_ma = self.long_ma.update(latest_candle.close);

        info!("Updated MA: {{short: {}, long: {}}}", short_ma, long_ma);

        let deviation = (latest_candle.close - long_ma) / long_ma;
        info!("Current mean deviation: {}", deviation * 100_f64);

        self.check_exit_signals(latest_candle.close).await?;
        self.check_entry_signal(latest_candle.close, deviation).await?;

        self.candles.push(ProcessedCandle::from(latest_candle));

        if self.candles.len() > self.strategy.timeframe.period_measurement.measure_bars * 3 {
            self.candles.remove(0);
        }

        Ok(())
    }

    async fn check_exit_signals(&mut self, current_price: f64) -> Result<(), ApiError> {
        if self.open_positions.is_empty() {
            return Ok(());
        }

        let mut position_to_close = Vec::new();

        for (idx, position) in self.open_positions.iter().enumerate() {
            let profit_percentage = (current_price - position.entry_price) / position.entry_price;

            if profit_percentage <= -f64::from(self.strategy.risk_management.stop_loss) {
                info!(
                    "Stop loss triggered, closing position with loss: {:.2}%",
                    profit_percentage * 100.0
                );

                position_to_close.push(idx);

                warn!("UNIMPLEMENTED: should create an order here to loss position");
            } else if profit_percentage >= self.strategy.risk_management.profit_level.into() {
                info!(
                    "Profit triggered, closing position with gained profit: {:.2}%",
                    profit_percentage * 100.0
                );

                position_to_close.push(idx);

                warn!("UNIMPLENTED: shuold create an order here to close profit position");
            } else {
                let deviation =
                    (current_price - self.short_ma.calculate()) / self.short_ma.calculate();

                if deviation >= self.strategy.risk_management.max_drawdown.into() {
                    info!(
                        "Mean Reversion exit triggered for unstable deviation, deviation: {:.2}%",
                        deviation
                    );

                    position_to_close.push(idx);

                    warn!(
                        "UNIMPLEMENTED: should create an order for exit because of unstable deviation"
                    );
                }
            }
        }

        Ok(())
    }

    async fn check_entry_signal(
        &mut self,
        current_price: f64,
        deviation: f64
    ) -> Result<(), ApiError> {
        if self.open_positions.len() >= self.strategy.risk_management.max_positions {
            info!("Max positions reached, not opening new positions");
            return Ok(());
        }

        if deviation <= -(self.strategy.measurement_deviation.enter_deviation as f64) / 100.0 {
            info!("Entry signal detected! Deviation: {:.2}%", deviation * 100.0);

            let capital_to_use =
                self.account_balance * f64::from(self.strategy.risk_management.capital_per_trade);
            let quantity = capital_to_use / current_price;

            let position = self.api_client.place_order_to_buy(
                self.strategy.pair.clone(),
                quantity
            ).await?;

            trace!("Updating open positions with value: {:?}", position);
            self.open_positions.push(position);
        }

        Ok(())
    }
}
