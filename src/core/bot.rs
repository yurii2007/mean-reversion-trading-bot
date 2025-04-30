use std::time::Duration;

use tokio::time::sleep;
use tracing::{ debug, error, info };

use crate::{
    api::{ client::{ ApiClient, KLineParams }, error::ApiError },
    strategy::{
        mean_calculation::{ MaTracker, MeanCalculation },
        strategy::Strategy,
        timeframe::duration_from_kline_interval,
    },
};
use super::{ market::ProcessedCandle, trading_strategy::TradingStrategy };

const MA_PERIOD_DIFFERENCE: usize = 3;
const TRADINC_CYCLE_RECOVERY_PERIOD: u64 = 30;

pub struct Bot {
    strategy: Strategy,
    api_client: Box<dyn ApiClient>,
    long_ma: MaTracker,
    short_ma: MaTracker,
    candles: Vec<ProcessedCandle>,
    account_balance: f64,
    trading_strategy: TradingStrategy,
}

impl Bot {
    pub fn new(strategy: Strategy) -> Self {
        let long_period = strategy.timeframe.period_measurement.measure_bars;
        let short_period = long_period / MA_PERIOD_DIFFERENCE;
        let api_client = strategy.exchange.api.get_client();

        Self {
            api_client: Box::new(api_client),
            trading_strategy: TradingStrategy::new(strategy.risk_management.max_positions),
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

        let account_balance = self.api_client.get_account_balance(
            &self.strategy.trading_symbol
        ).await?;

        if let Some(latest_candles) = self.candles.chunks(MA_PERIOD_DIFFERENCE).last() {
            for candle in latest_candles.iter() {
                self.short_ma.update(candle.close);
            }
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

            if let Err(e) = self.execute_trading_cycle().await {
                error!("Error executing trading cycle: {}", e);
                sleep(Duration::from_secs(TRADINC_CYCLE_RECOVERY_PERIOD)).await;
            }

            interval.tick().await;
        }
    }

    async fn execute_trading_cycle(&mut self) -> Result<(), ApiError> {
        info!("Starting trading cycle with balance: {}", self.account_balance);

        let latest_candle = self.api_client.get_latest_candle(
            &self.strategy.symbol,
            &self.strategy.timeframe.tick
        ).await?;
        let current_price = latest_candle.close;

        info!("Received the latest candle: {:?}", latest_candle);

        self.insert_candle(latest_candle);

        let short_ma = self.short_ma.update(current_price);
        let long_ma = self.long_ma.update(current_price);

        info!("Updated MA: {{short: {}, long: {}}}", short_ma, long_ma);

        let deviation =
            ((self.short_ma.calculate() - self.long_ma.calculate()) / long_ma) * 100_f64;
        info!("Current mean deviation: {}", deviation);

        let balance_difference = self.trading_strategy.check_exit_signals(
            current_price,
            deviation,
            &self.strategy,
            self.api_client.as_ref()
        ).await?;

        self.update_balance(balance_difference);

        let balance_difference = self.trading_strategy.check_entry_signals(
            current_price,
            deviation,
            self.account_balance,
            &self.strategy,
            self.api_client.as_ref()
        ).await?;

        self.update_balance(balance_difference);

        info!(
            "Trading cycle executed, current balance: {}\nopen positions: {}",
            self.account_balance,
            self.trading_strategy.open_positions_count()
        );

        Ok(())
    }

    fn update_balance(&mut self, sum: f64) {
        debug!("Updating balance with sum: {}", sum);
        self.account_balance += sum;
    }

    fn insert_candle(&mut self, candle: ProcessedCandle) {
        self.candles.push(candle);

        if
            self.candles.len() >
            self.strategy.timeframe.period_measurement.measure_bars * MA_PERIOD_DIFFERENCE
        {
            self.candles.remove(0);
        }
    }
}
