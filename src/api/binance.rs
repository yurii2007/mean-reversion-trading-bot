use std::borrow::Cow;

use binance_spot_connector_rust::{
    http::{ request::RequestBuilder, Method },
    hyper::{ BinanceHttpClient, Error },
};

use super::response::BinanceResponse;

pub struct BinanceApi;

impl BinanceApi {
    pub async fn get_data() -> Result<Vec<BinanceResponse>, Error> {
        let client = BinanceHttpClient::default();

        let builder = RequestBuilder::new(Method::Get, "/api/v3/klines").params(
            vec![("symbol", "BTCUSDT"), ("interval", "1m"), ("limit", "2")]
        );

        let data = client.send(builder).await.expect("Request failed").into_body_str().await?;

        let bin_res = BinanceResponse::deserialize_response(Cow::from(data)).unwrap();

        Ok(bin_res)
    }
}
