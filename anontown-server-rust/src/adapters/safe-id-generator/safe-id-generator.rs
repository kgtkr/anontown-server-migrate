use nanoid::nanoid;

use crate::ports::safe_id_generator::SafeIdGeneratorPort;

pub struct SafeIdGenerator;

impl SafeIdGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl SafeIdGeneratorPort for SafeIdGenerator {
    fn generate(&self) -> String {
        nanoid!()
    }
} 