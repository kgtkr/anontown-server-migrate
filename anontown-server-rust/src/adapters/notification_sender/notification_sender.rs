use async_trait::async_trait;
use reqwest::Client;
use crate::entities::notification::Notification;
use crate::ports::notification_sender::NotificationSenderPort;

pub struct NotificationSender {
    client: Client,
}

impl NotificationSender {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl NotificationSenderPort for NotificationSender {
    async fn send(&self, notification: Notification) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .post(&notification.endpoint)
            .json(&notification.payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to send notification: {}", response.status()).into());
        }

        Ok(())
    }
} 