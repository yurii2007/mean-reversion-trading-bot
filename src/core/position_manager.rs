use std::collections::HashMap;

use uuid::Uuid;
use tracing::{ info, debug };

use crate::api::{ client::ApiClient, error::ApiError };
use super::market::Position;

pub struct PositionManager {
    open_positions: HashMap<Uuid, Position>,
    max_positions: usize,
}

impl PositionManager {
    pub fn new(max_positions: usize) -> Self {
        Self {
            open_positions: HashMap::with_capacity(max_positions),
            max_positions,
        }
    }

    pub async fn open_position(
        &mut self,
        symbol: &str,
        quantity: f64,
        price: f64,
        client: &dyn ApiClient
    ) -> Result<f64, ApiError> {
        if self.open_positions.len() >= self.max_positions {
            debug!("Failed to open new position, reached maximum value");

            return Err(
                ApiError::ValidationError("Maximum value of open positions reached".to_string())
            );
        }

        let position = client.place_order_to_buy(symbol, quantity, price).await?;
        let position_price = position.entry_price * position.quantity;

        info!("Opened position: {:?}", position);

        self.open_positions.insert(position.id, position);

        Ok(position_price)
    }

    pub async fn close_position(
        &mut self,
        position_id: Uuid,
        current_price: f64,
        client: &dyn ApiClient
    ) -> Result<f64, ApiError> {
        let position = self.open_positions
            .get(&position_id)
            .ok_or(ApiError::NotFound(format!("Position with id {position_id} not found")))?;
        let sell_price = current_price * position.quantity;

        client.place_order_to_sell(&position.symbol, position.quantity).await?;

        info!("Closing position {}", position.id);

        self.open_positions.remove(&position_id);

        Ok(sell_price)
    }

    pub fn get_positions(&self) -> impl Iterator<Item = &Position> {
        self.open_positions.values()
    }

    pub fn is_empty(&self) -> bool {
        self.open_positions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.open_positions.len()
    }
}
