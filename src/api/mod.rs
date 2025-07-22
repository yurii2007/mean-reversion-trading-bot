use serde::Deserialize;

pub mod binance;
pub mod error;
pub mod message;

pub use error::*;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiClientEnum {
    Binance,
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
