use async_trait::async_trait;
use std::sync::Mutex;
use crate::models::AuthToken;
use crate::ports::auth::AuthPort;

pub struct AuthContainer {
    token: Mutex<Option<AuthToken>>,
}

impl AuthContainer {
    pub fn new(token: Option<AuthToken>) -> Self {
        Self {
            token: Mutex::new(token),
        }
    }
}

#[async_trait]
impl AuthPort for AuthContainer {
    async fn get_token(&self) -> Option<AuthToken> {
        self.token.lock().unwrap().clone()
    }

    async fn set_token(&mut self, token: Option<AuthToken>) -> Result<(), Box<dyn std::error::Error>> {
        *self.token.lock().unwrap() = token;
        Ok(())
    }
} 