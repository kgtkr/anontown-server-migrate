use chrono::{DateTime, Utc};
use crate::ports::clock::ClockPort;

pub struct FixClock {
    now: DateTime<Utc>,
}

impl FixClock {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self { now }
    }
}

impl ClockPort for FixClock {
    fn now(&self) -> DateTime<Utc> {
        self.now
    }
} 