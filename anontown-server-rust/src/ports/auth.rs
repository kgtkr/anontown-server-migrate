use async_trait::async_trait;
use crate::models::AuthToken;

#[async_trait]
pub trait AuthPort {
    async fn get_token(&self) -> Option<AuthToken>;
    async fn set_token(&mut self, token: Option<AuthToken>) -> Result<(), Box<dyn std::error::Error>>;
} 