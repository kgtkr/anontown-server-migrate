use chrono::{DateTime, Utc};

use crate::ports::{clock::ClockPort, safe_id_generator::SafeIdGeneratorPort};

#[derive(Debug, Clone)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub url: String,
    pub date: DateTime<Utc>,
    pub update: DateTime<Utc>,
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
            url: description,
            date: clock.now(),
            update: clock.now(),
        }
    }

    pub fn update(&mut self, name: String, description: String, clock: &impl ClockPort) {
        self.name = name;
        self.url = description;
        self.date = clock.now();
    }
} 