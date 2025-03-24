use async_trait::async_trait;

#[async_trait]
pub trait LoggerPort {
    async fn error(&self, msg: &str);
    async fn warn(&self, msg: &str);
    async fn info(&self, msg: &str);
    async fn verbose(&self, msg: &str);
    async fn debug(&self, msg: &str);
    async fn silly(&self, msg: &str);
} 