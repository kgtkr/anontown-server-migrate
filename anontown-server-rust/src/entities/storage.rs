use chrono::{DateTime, Utc};

use crate::ports::{clock::ClockPort, safe_id_generator::SafeIdGeneratorPort};

#[derive(Debug)]
pub struct Storage {
    pub id: String,
    pub client_id: Option<String>,
    pub user_id: String,
    pub key: String,
    pub value: String,
    pub date: DateTime<Utc>,
}

impl Storage {
    pub fn new(
        user_id: String,
        key: String,
        value: String,
        client_id: Option<String>,
        clock: &impl ClockPort,
        id_generator: &impl SafeIdGeneratorPort,
    ) -> Self {
        Self {
            id: id_generator.generate(),
            client_id,
            user_id,
            key,
            value,
            date: clock.now(),
        }
    }

    pub fn update(&mut self, value: String, clock: &impl ClockPort) {
        self.value = value;
        self.date = clock.now();
    }

    pub fn is_self(&self, user_id: &str) -> bool {
        self.user_id == user_id
    }

    pub fn is_client(&self, client_id: &str) -> bool {
        self.client_id.as_deref() == Some(client_id)
    }
} 