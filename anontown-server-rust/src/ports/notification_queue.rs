use async_trait::async_trait;
use crate::models::notification::Notification;

#[async_trait]
pub trait NotificationQueuePort {
    async fn push(&self, notification: Notification) -> Result<(), Box<dyn std::error::Error>>;
    async fn pop(&self) -> Result<Option<Notification>, Box<dyn std::error::Error>>;
} 