use binance::BinanceClient;
use serde::Deserialize;

use crate::client::ApiClient;

mod binance;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiClientEnum {
    BINANCE,
}

pub fn init_client(client: &ApiClientEnum) -> impl ApiClient {
    match client {
        ApiClientEnum::BINANCE => BinanceClient::new(),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Error;

    use super::*;

    #[tokio::test]
    async fn test_init_client() {
        let client_type = ApiClientEnum::BINANCE;

        let client = init_client(&client_type);

        client.connect().await
    }

    #[test]
    fn test_deserialize_client() {
        let client_string = r#""binance""#;

        let client_type: ApiClientEnum = serde_json::from_str(client_string).unwrap();

        assert_eq!(client_type, ApiClientEnum::BINANCE);

        let client_string = "bin";

        let client_type: Result<ApiClientEnum, Error> = serde_json::from_str(client_string);

        assert!(client_type.is_err());
    }
}
