use rand::{thread_rng, Rng};

use crate::ports::id_generator::IdGeneratorPort;

pub struct IdGenerator;

impl IdGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl IdGeneratorPort for IdGenerator {
    fn generate(&self) -> String {
        let mut rng = thread_rng();
        let mut id = [0u8; 16];
        rng.fill_bytes(&mut id);
        hex::encode(id)
    }
} 