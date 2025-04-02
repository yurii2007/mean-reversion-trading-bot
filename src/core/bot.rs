use std::{ collections::HashSet, time::Duration };

use tokio::time::sleep;
use tracing::{ debug, error, info, trace };
use uuid::Uuid;

use crate::{
    api::{ client::{ ApiClient, KLineParams }, error::ApiError },
    strategy::{
        mean_calculation::{ MaTracker, MeanCalculation },
        strategy::Strategy,
        timeframe::duration_from_kline_interval,
    },
};

use super::{ market::ProcessedCandle, position_manager::PositionManager };

const MA_PERIOD_DIFFERENCE: usize = 3;
const TRADINC_CYCLE_RECOVERY_PERIOD: u64 = 30;

pub struct Bot {
    strategy: Strategy,
    api_client: Box<dyn ApiClient>,
    position_manager: PositionManager,
    long_ma: MaTracker,
    short_ma: MaTracker,
    candles: Vec<ProcessedCandle>,
    account_balance: f64,
}

impl Bot {
    pub fn new(strategy: Strategy) -> Self {
        let long_period = strategy.timeframe.period_measurement.measure_bars;
        let short_period = long_period / MA_PERIOD_DIFFERENCE;
        let api_client = strategy.exchange.api.get_client();

        Self {
            api_client: Box::new(api_client),
            position_manager: PositionManager::new(strategy.risk_management.max_positions),
            account_balance: 0_f64,
            candles: Vec::new(),
            long_ma: MaTracker::new(
                long_period,
                strategy.timeframe.period_measurement.mean_calculation_method
            ),
            short_ma: MaTracker::new(
                short_period,
                strategy.timeframe.period_measurement.mean_calculation_method
            ),
            strategy,
        }
    }

    pub async fn initialize(&mut self) -> Result<(), ApiError> {
        let candles = self.api_client.get_candles(
            KLineParams::build(
                self.strategy.timeframe.period_measurement.measure_bars * MA_PERIOD_DIFFERENCE,
                self.strategy.symbol.clone(),
                duration_from_kline_interval(&self.strategy.timeframe.interval)
            )
        ).await?;

        self.candles = candles
            .into_iter()
            .map(|candle| {
                self.long_ma.update(candle.close);

                candle
            })
            .collect();

        let account_balance = self.api_client.get_account_balance().await?;

        if let Some(latest_candles) = self.candles.chunks(MA_PERIOD_DIFFERENCE).last() {
            latest_candles.iter().for_each(|candle| {
                self.short_ma.update(candle.close);
            });
        }

        self.candles.iter().for_each(|candle| {
            self.long_ma.update(candle.close);
        });

        self.account_balance = account_balance;

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

        let mut interval = tokio::time::interval(self.strategy.timeframe.tick);

        loop {
            info!("Waiting for the next execution cycle");
            interval.tick().await;

            if let Err(e) = self.execute_trading_cycle().await {
                error!("Error executing trading cycle: {}", e);
                sleep(Duration::from_secs(TRADINC_CYCLE_RECOVERY_PERIOD)).await;
            }
        }
    }

    async fn execute_trading_cycle(&mut self) -> Result<(), ApiError> {
        info!("Starting trading cycle with balance: {}", self.account_balance);

        let latest_candle = self.api_client.get_latest_candle(
            &self.strategy.symbol,
            &self.strategy.timeframe.tick
        ).await?;

        info!("Received the latest candle: {:?}", latest_candle);

        let short_ma = self.short_ma.update(latest_candle.close);
        let long_ma = self.long_ma.update(latest_candle.close);

        info!("Updated MA: {{short: {}, long: {}}}", short_ma, long_ma);

        let deviation = ((short_ma - long_ma) / long_ma) * 100_f64;
        info!("Current mean deviation: {}", deviation);

        self.check_exit_signals(latest_candle.close).await?;
        self.check_entry_signal(latest_candle.close, deviation).await?;

        self.candles.push(ProcessedCandle::from(latest_candle));

        if
            self.candles.len() >
            self.strategy.timeframe.period_measurement.measure_bars * MA_PERIOD_DIFFERENCE
        {
            self.candles.remove(0);
        }

        info!(
            "Trading cycle executed, current balance: {}\nopen positions: {}",
            self.account_balance,
            self.position_manager.len()
        );

        Ok(())
    }

    async fn check_exit_signals(&mut self, current_price: f64) -> Result<(), ApiError> {
        if self.position_manager.is_empty() {
            return Ok(());
        }

        let mut positions_to_close: HashSet<Uuid> = HashSet::new();

        for position in self.position_manager.get_positions() {
            let profit_percentage =
                ((current_price - position.entry_price) / position.entry_price) * 100_f64;
            trace!("Profit percentage for position {:?}: {:.2}%", position.id, profit_percentage);

            if profit_percentage <= -f64::from(self.strategy.risk_management.stop_loss) {
                info!(
                    "Stop loss triggered, closing position {} with loss: {:.2}%",
                    position.id,
                    profit_percentage
                );

                positions_to_close.insert(position.id);
            } else if profit_percentage >= self.strategy.risk_management.profit_level.into() {
                info!(
                    "Profit triggered, closing position {} with gained profit: {:.2}%",
                    position.id,
                    profit_percentage
                );

                positions_to_close.insert(position.id);
            } else {
                let deviation =
                    (current_price - self.short_ma.calculate()) / self.short_ma.calculate();

                trace!("Checking deviation for extreme value: {:.2}", deviation);

                if deviation >= self.strategy.risk_management.max_drawdown.into() {
                    info!(
                        "Mean Reversion exit triggered for unstable deviation, deviation: {:.2}%",
                        deviation
                    );

                    positions_to_close.insert(position.id);
                }
            }
        }

        for position_id in positions_to_close {
            match
                self.position_manager.close_position(
                    position_id,
                    current_price,
                    self.api_client.as_ref()
                ).await
            {
                Ok(sum) => self.update_balance(sum),
                Err(e) =>
                    error!(
                        "Could not create an order to sell for position {:?}: {}",
                        position_id,
                        e
                    ),
            };
        }

        Ok(())
    }

    async fn check_entry_signal(
        &mut self,
        current_price: f64,
        deviation: f64
    ) -> Result<(), ApiError> {
        if self.position_manager.len() >= self.strategy.risk_management.max_positions {
            info!("Max positions reached, not opening new positions");
            return Ok(());
        }

        if deviation <= -(self.strategy.measurement_deviation.enter_deviation as f64) {
            info!("Entry signal detected! Deviation: {:.2}%", deviation);

            let capital_to_use =
                self.account_balance * f64::from(self.strategy.risk_management.capital_per_trade);
            let quantity = capital_to_use / current_price;

            match
                self.position_manager.open_position(
                    &self.strategy.symbol,
                    quantity,
                    current_price,
                    self.api_client.as_ref()
                ).await
            {
                Ok(sum) => {
                    self.update_balance(-sum);
                }
                Err(e) => {
                    error!("Could not open new position: {}", e);
                }
            }
        }

        Ok(())
    }

    fn update_balance(&mut self, sum: f64) {
        debug!("Updating balance with sum: {}", sum);
        self.account_balance += sum;
    }
}
