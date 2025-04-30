use std::fmt::Debug;

use config::{ Config, File };
use serde::{ Deserialize, Serialize };
use tracing::{ error, trace };

use crate::api::supported_api::Api;
use super::timeframe::StrategyTimeframe;

const CONFIG_FILE_PATH: &str = "strategy.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Strategy {
    pub symbol: String,

    pub trading_symbol: String,

    pub pair: String,

    pub exchange: Exchange,

    pub timeframe: StrategyTimeframe,

    pub risk_management: RiskManagement,

    pub measurement_deviation: MeasurementDeviation,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Exchange {
    pub api: Api,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RiskManagement {
    pub capital_per_trade: f32,
    pub max_positions: usize,
    pub max_drawdown: f32,
    pub stop_loss: f32,
    pub profit_level: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeasurementDeviation {
    pub enter_deviation: f32,
}

impl Strategy {
    pub fn new() -> Self {
        trace!("Trying to read configuration");
        let config = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .build()
            .inspect_err(|e| {
                error!("Failed to load configuration file: {}", e);
            })
            .unwrap();

        let strategy = config
            .try_deserialize::<Strategy>()
            .inspect_err(|e| {
                error!("Failed to deserialize strategy configuration: {}", e);
            })
            .unwrap();

        if strategy.symbol == strategy.trading_symbol {
            panic!("Invalid strategy: symbol and trading_symbol cannot be the same");
        }

        strategy
    }
}

impl Default for Strategy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::time::Duration;

    use tempfile::NamedTempFile;

    use crate::strategy::timeframe::duration_from_kline_interval;

    use super::*;

    fn create_tmp_test_config(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::with_suffix(".toml").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn parse_valid_toml_strategy() {
        let valid_toml_config =
            r#"
symbol = "BTCUSDT"
pair = "BTC/USDT"
trading_symbol = "USDT"

[timeframe]
interval = "1h"
tick = "15m"

[timeframe.period_measurement]
measure_bars = 20
mean_calculation_method="EMA"

[exchange]
api = "binance"

[risk_management]
capital_per_trade = 0.1
max_positions = 5
max_drawdown = 0.5
stop_loss = 0.3
profit_level = 0.15

[measurement_deviation]
enter_deviation = 0.1
            "#;
        let temp_config_file = create_tmp_test_config(valid_toml_config);
        let path = temp_config_file.path().to_str().unwrap();

        let config = Config::builder().add_source(File::with_name(path)).build().unwrap();

        let strategy = config.try_deserialize::<Strategy>().unwrap();

        assert_eq!(strategy.symbol, "BTCUSDT");
        assert_eq!(strategy.pair, "BTC/USDT");

        assert_eq!(
            duration_from_kline_interval(&strategy.timeframe.interval),
            std::time::Duration::from_secs(60 * 60)
        );
        assert_eq!(strategy.timeframe.tick, Duration::from_secs(60 * 15));

        assert_eq!(strategy.exchange.api, Api::Binance);

        assert_eq!(strategy.risk_management.capital_per_trade, 0.1);
        assert_eq!(strategy.risk_management.max_positions, 5);
        assert_eq!(strategy.risk_management.max_drawdown, 0.5);
        assert_eq!(strategy.risk_management.stop_loss, 0.3);
        assert_eq!(strategy.risk_management.profit_level, 0.15);

        assert_eq!(strategy.measurement_deviation.enter_deviation, 0.1);
    }

    #[test]
    fn parse_invalid_toml_strategy() {
        let invalid_toml_config =
            r#"
interval = "1"
tick = "1"

[exchange]
api = "foo"

capital_per_trade = 0.1
max_positions = 5
max_drawdown = 0.5
stop_loss = 0.3
profit_level = 0.15
    "#;
        let temp_config_file = create_tmp_test_config(invalid_toml_config);
        let path = temp_config_file.path().to_str().unwrap();

        let config = Config::builder().add_source(File::with_name(path)).build().unwrap();

        let parsing_error = config.try_deserialize::<Strategy>();

        assert!(parsing_error.is_err());
    }

    #[test]
    fn test_type_mismatch() {
        let invalid_types_toml_config =
            r#"
symbol = "BTC"
pair = "BTCUSDT"

[exchange]
api = "Binance"

[timeframe]
interval = "4h"

[risk_management]
capital_per_trade = "string"
max_positions = 3
max_drawdown = 0.05
stop_loss = 0.02
profit_level = 0.04

[measurement_deviation]
enter_deviation = 0.01
        "#;

        let temp_file = create_tmp_test_config(invalid_types_toml_config);
        let path = temp_file.path().to_str().unwrap();

        let config = Config::builder().add_source(File::with_name(path)).build().unwrap();

        let result = config.try_deserialize::<Strategy>();
        assert!(result.is_err());
    }

    #[test]
    fn test_nonexistent_file() {
        let config = Config::builder().add_source(File::with_name("nonexistent_file.toml")).build();

        assert!(config.is_err());
    }
}
