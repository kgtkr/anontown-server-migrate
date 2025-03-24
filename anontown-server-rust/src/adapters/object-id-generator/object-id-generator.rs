use crate::ports::object_id_generator::ObjectIdGeneratorPort;
use bson::oid::ObjectId;
pub struct ObjectIdGenerator;

impl ObjectIdGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl ObjectIdGeneratorPort for ObjectIdGenerator {
    fn generate(&self) -> String {
        ObjectId::new().to_hex()
    }
} 