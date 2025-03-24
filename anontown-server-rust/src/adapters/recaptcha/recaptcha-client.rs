use async_trait::async_trait;
use reqwest::Client;
use crate::ports::recaptcha::RecaptchaPort;

pub struct RecaptchaClient {
    client: Client,
    secret_key: String,
}

impl RecaptchaClient {
    pub fn new(secret_key: String) -> Self {
        Self {
            client: Client::new(),
            secret_key,
        }
    }
}

#[async_trait]
impl RecaptchaPort for RecaptchaClient {
    async fn verify(&self, token: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let response = self.client
            .post("https://www.google.com/recaptcha/api/siteverify")
            .form(&[
                ("secret", &self.secret_key),
                ("response", token),
            ])
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        Ok(result["success"].as_bool().unwrap_or(false))
    }
} 