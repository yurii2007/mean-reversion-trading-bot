use serde::{ Serialize, Deserialize };

use crate::api::{ binance::BinanceApi, client::ApiClient };

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Api {
    Binance,
}

impl Api {
    pub fn get_client(&self) -> impl ApiClient {
        match &self {
            Api::Binance => BinanceApi,
        }
    }
}
