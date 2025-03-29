use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub enum Api {
    #[serde(rename = "lowercase")]
    Binance,
}
