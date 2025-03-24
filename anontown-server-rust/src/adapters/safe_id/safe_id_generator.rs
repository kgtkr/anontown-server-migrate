use async_trait::async_trait;
use nanoid::nanoid;
use crate::ports::safe_id::SafeIdPort;

pub struct SafeIdGenerator;

impl SafeIdGenerator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SafeIdPort for SafeIdGenerator {
    async fn generate(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(nanoid!())
    }
} 