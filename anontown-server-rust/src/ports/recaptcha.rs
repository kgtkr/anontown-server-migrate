use async_trait::async_trait;

#[async_trait]
pub trait RecaptchaPort {
    async fn verify(&self, token: &str) -> Result<bool, Box<dyn std::error::Error>>;
} 