use std::collections::HashSet;

use tracing::{ error, info, debug, trace };
use uuid::Uuid;

use crate::{ api::{ client::ApiClient, error::ApiError }, strategy::strategy::Strategy };
use super::position_manager::PositionManager;

pub struct TradingStrategy {
    position_manager: PositionManager,
}

impl TradingStrategy {
    pub fn new(max_positions: usize) -> Self {
        Self {
            position_manager: PositionManager::new(max_positions),
        }
    }

    pub async fn check_entry_signals(
        &mut self,
        current_price: f64,
        deviation: f64,
        account_balance: f64,
        strategy: &'_ Strategy,
        api_client: &'_ dyn ApiClient
    ) -> Result<f64, ApiError> {
        if self.position_manager.len() >= strategy.risk_management.max_positions {
            info!("Max positions reached, not opening new positions");
            return Ok(0_f64);
        }

        if deviation <= -f64::from(strategy.measurement_deviation.enter_deviation) {
            info!("Entry signal detected! Deviation: {:.2}%", deviation);

            let capital_to_use =
                account_balance * f64::from(strategy.risk_management.capital_per_trade);
            let quantity = capital_to_use / current_price;

            self.position_manager
                .open_position(&strategy.symbol, quantity, current_price, api_client).await
                .map(|sum| -sum)
        } else {
            Ok(0_f64)
        }
    }

    pub async fn check_exit_signals(
        &mut self,
        current_price: f64,
        deviation: f64,
        strategy: &'_ Strategy,
        api_client: &'_ dyn ApiClient
    ) -> Result<f64, ApiError> {
        if self.position_manager.is_empty() {
            return Ok(0_f64);
        }

        let mut positions_to_close: HashSet<Uuid> = HashSet::new();

        self.position_manager.get_positions().for_each(|position| {
            let profit_percentage =
                ((current_price - position.entry_price) / position.entry_price) * 100_f64;
            trace!("Profit percentage for position {:?}: {:.2}%", position.id, profit_percentage);

            if profit_percentage <= -f64::from(strategy.risk_management.stop_loss) {
                info!(
                    "Stop loss triggered, closing position {} with loss: {:.2}%",
                    position.id,
                    profit_percentage
                );

                positions_to_close.insert(position.id);
            } else if deviation >= strategy.risk_management.profit_level.into() {
                info!(
                    "Profit triggered, closing position {} with gained profit: {:.2}%",
                    position.id,
                    profit_percentage
                );

                positions_to_close.insert(position.id);
            } else {
                debug!("Checking deviation for extreme value: {:.2}", deviation);

                if deviation >= strategy.risk_management.max_drawdown.into() {
                    info!(
                        "Mean Reversion exit triggered for unstable deviation, deviation: {:.2}%",
                        deviation
                    );

                    positions_to_close.insert(position.id);
                }
            }
        });

        let mut balance_difference = 0_f64;

        for position_id in positions_to_close {
            match
                self.position_manager.close_position(position_id, current_price, api_client).await
            {
                Ok(sum) => {
                    balance_difference += sum;
                }
                Err(e) =>
                    error!(
                        "Could not create an order to sell for position {:?}: {}",
                        position_id,
                        e
                    ),
            };
        }

        Ok(balance_difference)
    }

    pub fn open_positions_count(&self) -> usize {
        self.position_manager.len()
    }
}
