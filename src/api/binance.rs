use std::borrow::Cow;

use binance_spot_connector_rust::{
    http::request::Request,
    hyper::BinanceHttpClient,
    market::klines::Klines,
};
use tracing::debug;

use super::{ response::BinanceResponse, error::ApiError };

pub struct BinanceApi;

impl BinanceApi {
    pub async fn get_kline_data(params: Klines) -> Result<Vec<BinanceResponse>, ApiError> {
        let client = BinanceHttpClient::default();

        let request = Request::from(params);

        debug!("Requesting Kline data from binance with params: {:?}", request.params());

        let data = client
            .send(request).await
            .map_err(ApiError::from)?
            .into_body_str().await
            .map_err(ApiError::from)?;

        let bin_res = BinanceResponse::deserialize_response(Cow::from(data)).unwrap();

        Ok(bin_res)
    }
}
