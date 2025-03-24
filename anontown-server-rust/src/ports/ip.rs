use async_trait::async_trait;

#[async_trait]
pub trait IpPort {
    async fn get_ip(&self) -> Option<String>;
} 