use config::{ Config, File };
use serde::{ Deserialize, Serialize };
use tracing::{ error, trace };

const CONFIG_FILE_PATH: &'static str = "strategy.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Strategy {
    // todo enum
    pub symbol: String,
    // todo enum
    pub pair: String,

    pub timeframe: StrategyTimeframe,

    pub risk_management: RiskManagement,

    pub period_measurement: PeriodMeasurement,

    pub measurement_deviation: MeasurementDeviation,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StrategyTimeframe {
    #[serde(with = "humantime_serde")]
    pub interval: std::time::Duration,
    #[serde(with = "humantime_serde")]
    pub tick: std::time::Duration,
    #[serde(with = "humantime_serde")]
    pub execution: std::time::Duration,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Exchange {
    // todo enum
    pub api: String,
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
pub struct PeriodMeasurement {
    pub measure_bars: usize,
    // todo enum
    pub mean_calculation_method: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeasurementDeviation {
    pub enter_deviation: f32,
    pub exit_deviation: f32,
}

impl Strategy {
    pub fn new() -> Self {
        trace!("Trying to read configuration");
        let config = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .build()
            .inspect_err(|e| {
                error!("Could not load configuration file: {}", e);
            })
            .unwrap();

        config
            .try_deserialize::<Strategy>()
            .inspect_err(|e| {
                error!("Failed to deserialize strategy configuration: {}", e);
            })
            .unwrap()
    }
}
