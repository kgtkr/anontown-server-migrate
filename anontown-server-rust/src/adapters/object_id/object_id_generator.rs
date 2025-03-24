use async_trait::async_trait;
use bson::oid::ObjectId;
use crate::ports::object_id::ObjectIdPort;

pub struct ObjectIdGenerator;

impl ObjectIdGenerator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ObjectIdPort for ObjectIdGenerator {
    async fn generate(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(ObjectId::new().to_hex())
    }
} 