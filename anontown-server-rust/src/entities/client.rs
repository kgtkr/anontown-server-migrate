use chrono::{DateTime, Utc};

use crate::ports::{clock::ClockPort, safe_id_generator::SafeIdGeneratorPort};

#[derive(Debug)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub description: String,
    pub date: DateTime<Utc>,
}

impl Client {
    pub fn new(
        name: String,
        description: String,
        clock: &impl ClockPort,
        id_generator: &impl SafeIdGeneratorPort,
    ) -> Self {
        Self {
            id: id_generator.generate(),
            name,
            description,
            date: clock.now(),
        }
    }

    pub fn update(&mut self, name: String, description: String, clock: &impl ClockPort) {
        self.name = name;
        self.description = description;
        self.date = clock.now();
    }
} 