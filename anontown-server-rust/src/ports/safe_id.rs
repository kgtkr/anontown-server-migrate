use async_trait::async_trait;

#[async_trait]
pub trait SafeIdPort {
    async fn generate(&self) -> Result<String, Box<dyn std::error::Error>>;
} 