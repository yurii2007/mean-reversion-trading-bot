use binance::BinanceClient;
use serde::Deserialize;

use crate::client::ApiClient;

mod binance;
pub mod message;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiClientEnum {
    Binance,
}

pub fn get_client(
    client: &ApiClientEnum,
    // internal_rx: UnboundedReceiver<ClientMessage>,
) -> impl ApiClient {
    let api_client = match client {
        ApiClientEnum::Binance => BinanceClient::new(),
    };

    api_client
}

#[cfg(test)]
mod tests {
    use serde_json::Error;

    use super::*;

    #[test]
    fn test_deserialize_client() {
        let client_string = r#""binance""#;

        let client_type: ApiClientEnum = serde_json::from_str(client_string).unwrap();

        assert_eq!(client_type, ApiClientEnum::Binance);

        let client_string = "bin";

        let client_type: Result<ApiClientEnum, Error> = serde_json::from_str(client_string);

        assert!(client_type.is_err());
    }
}
