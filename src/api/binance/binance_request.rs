use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct BinanceRequest<T: Serialize> {
    id: Uuid,
    method: String,
    params: Option<T>,
}

impl<T: Serialize> BinanceRequest<T> {
    pub fn new(method: String, params: Option<T>) -> Self {
        Self {
            id: Uuid::new_v4(),
            method,
            params,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AveragePriceRequest {
    symbol: String,
}

impl AveragePriceRequest {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }
}

impl From<AveragePriceRequest> for BinanceRequest<AveragePriceRequest> {
    fn from(value: AveragePriceRequest) -> Self {
        BinanceRequest::new(String::from("avgPrice"), Some(value))
    }
}
