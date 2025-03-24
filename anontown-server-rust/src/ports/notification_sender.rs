use async_trait::async_trait;
use crate::models::notification::Notification;

#[async_trait]
pub trait NotificationSenderPort {
    async fn send(&self, notification: Notification) -> Result<(), Box<dyn std::error::Error>>;
} 