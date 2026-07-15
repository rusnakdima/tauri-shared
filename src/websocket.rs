use crate::error::AppError;
use futures_util::{stream::StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

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
    let msg = socket
      .next()
      .await
      .ok_or_else(|| AppError::RequestFailed("No message received".to_string()))??;
    Ok(msg.to_string())
  }

  pub async fn subscribe(&self, channel: String) -> Result<String, AppError> {
    self.send(format!("subscribe:{}", channel)).await?;
    self.receive().await
  }
}
