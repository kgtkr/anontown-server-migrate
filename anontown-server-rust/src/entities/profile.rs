use chrono::{DateTime, Utc};

use crate::ports::{clock::ClockPort, safe_id_generator::SafeIdGeneratorPort};

#[derive(Debug)]
pub struct Profile {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub date: DateTime<Utc>,
}

impl Profile {
    pub fn new(
        user_id: String,
        name: String,
        description: String,
        clock: &impl ClockPort,
        id_generator: &impl SafeIdGeneratorPort,
    ) -> Self {
        Self {
            id: id_generator.generate(),
            user_id,
            name,
            description,
            date: clock.now(),
        }
    }

    pub fn update(
        &mut self,
        name: String,
        description: String,
        clock: &impl ClockPort,
    ) {
        self.name = name;
        self.description = description;
        self.date = clock.now();
    }

    pub fn is_self(&self, user_id: &str) -> bool {
        self.user_id == user_id
    }
} 