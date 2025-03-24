use chrono::{DateTime, Utc};

use crate::ports::clock::ClockPort;

pub struct Clock;

impl Clock {
    pub fn new() -> Self {
        Self
    }
}

impl ClockPort for Clock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
} 