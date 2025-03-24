use rand::RngCore;

use crate::ports::object_id_generator::ObjectIdGeneratorPort;

pub struct ObjectIdGenerator;

impl ObjectIdGenerator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ObjectIdGeneratorPort for ObjectIdGenerator {
    async fn generate(&self) -> String {
        let mut bytes = vec![0u8; 16];
        rand::thread_rng().fill_bytes(&mut bytes);
        hex::encode(bytes)
    }
} 