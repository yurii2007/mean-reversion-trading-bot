use std::{ error::Error, fmt::Display };

#[derive(Debug)]
pub enum ApiError {
    ParseError(String),
    NetworkError(String),
}

impl Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::ParseError(reason) => write!(f, "Failed to parse: {}", reason),
            ApiError::NetworkError(reason) => write!(f, "Failed to fetch: {}", reason),
        }
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(value: serde_json::error::Error) -> Self {
        Self::ParseError(format!("Failed to parse: {}", value))
    }
}

impl From<binance_spot_connector_rust::hyper::Error> for ApiError {
    fn from(value: binance_spot_connector_rust::hyper::Error) -> Self {
        Self::NetworkError(format!("Failed to fetch: {:?}", value))
    }
}
