use async_trait::async_trait;

#[async_trait]
pub trait ObjectIdPort {
    async fn generate(&self) -> Result<String, Box<dyn std::error::Error>>;
} 