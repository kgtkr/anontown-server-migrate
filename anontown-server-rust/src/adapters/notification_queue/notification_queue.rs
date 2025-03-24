use async_trait::async_trait;
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::notification::Notification;
use crate::ports::notification_queue::NotificationQueuePort;

pub struct NotificationQueue {
    sender: mpsc::Sender<Notification>,
    receiver: Arc<Mutex<mpsc::Receiver<Notification>>>,
}

impl NotificationQueue {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(100);
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
}

#[async_trait]
impl NotificationQueuePort for NotificationQueue {
    async fn push(&self, notification: Notification) -> Result<(), Box<dyn std::error::Error>> {
        self.sender.send(notification).await?;
        Ok(())
    }

    async fn pop(&self) -> Result<Option<Notification>, Box<dyn std::error::Error>> {
        let mut receiver = self.receiver.lock().await;
        Ok(receiver.recv().await)
    }
} 