pub mod auth;
pub mod client;
pub mod clock;
pub mod history;
pub mod ip;
pub mod logger;
pub mod notification_queue;
pub mod notification_sender;
pub mod object_id;
pub mod profile;
pub mod push_subscriptions;
pub mod recaptcha;
pub mod safe_id;
pub mod storage;
pub mod token;
pub mod types;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait RecaptchaPort {
    async fn verify(&self, token: &str) -> Result<()>;
}

#[async_trait]
pub trait DatabasePort {
    async fn connect(&self) -> Result<sqlx::PgPool>;
}

#[async_trait]
pub trait RedisPort {
    async fn connect(&self) -> Result<redis::Client>;
}

pub struct Ports {
    pub recaptcha: Box<dyn RecaptchaPort>,
    pub database: Box<dyn DatabasePort>,
    pub redis: Box<dyn RedisPort>,
} 