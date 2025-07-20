use async_trait::async_trait;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::{
    api::message::{ApiClientMessage, ClientMessage},
    client::error::ClientError,
};

pub mod error;

#[async_trait]
pub trait ApiClient {
    async fn connect(
        &mut self,
        api_tx: UnboundedSender<ApiClientMessage>,
        request_tx: UnboundedSender<ClientMessage>,
        request_rx: UnboundedReceiver<ClientMessage>,
    ) -> Result<(JoinHandle<()>, JoinHandle<()>), ClientError>;
    async fn send(&self, msg: ClientMessage) -> Result<(), ClientError>;
}
