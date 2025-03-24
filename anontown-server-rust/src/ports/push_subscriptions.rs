use async_trait::async_trait;
use crate::models::push_subscription::PushSubscription;

#[async_trait]
pub trait PushSubscriptionsPort {
    async fn find_all(&self) -> Result<Vec<PushSubscription>, Box<dyn std::error::Error>>;
    async fn insert(&self, subscription: PushSubscription) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>>;
} 