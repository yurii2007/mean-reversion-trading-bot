use async_trait::async_trait;

use crate::api::{init_client, ApiClientEnum};

pub struct Client {
    _client: Box<dyn ApiClient>,
}

#[async_trait]
pub trait ApiClient {
    async fn connect(&self) -> ();
}

impl Client {
    pub fn new(client: &ApiClientEnum) -> Self {
        let client = init_client(client);

        Self {
            _client: Box::new(client),
        }
    }

    pub async fn connect(&self) {
        self._client.connect().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_initializing() {
        let client_type = ApiClientEnum::BINANCE;

        let client = Client::new(&client_type);

        // todo Add proper tests for api type
        client._client.connect().await;
    }

    #[tokio::test]
    async fn test_client_connect() {
        let client_type = ApiClientEnum::BINANCE;

        let client = Client::new(&client_type);

        // todo Add proper tests for connecting
        client.connect().await;
    }
}
