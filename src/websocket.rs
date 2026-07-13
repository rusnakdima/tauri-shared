use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{stream::StreamExt, SinkExt};
use crate::error::AppError;

pub struct WsClient {
    pub url: String,
}

impl WsClient {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn send(&self, message: String) -> Result<(), AppError> {
        let (mut socket, _) = connect_async(&self.url).await?;
        socket.send(Message::Text(message)).await?;
        Ok(())
    }

    pub async fn receive(&self) -> Result<String, AppError> {
        let (mut socket, _) = connect_async(&self.url).await?;
        if let Some(message) = socket.next().await {
            let msg = message?;
            Ok(msg.to_string())
        } else {
            Err(AppError::RequestFailed("No message received".to_string()))
        }
    }

    pub async fn subscribe(&self, channel: String) -> Result<String, AppError> {
        self.send(format!("subscribe:{}", channel)).await?;
        self.receive().await
    }
}