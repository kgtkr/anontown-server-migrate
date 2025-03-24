use async_trait::async_trait;

#[async_trait]
pub trait ObjectIdGeneratorPort: Send + Sync {
    async fn generate(&self) -> String;
} 