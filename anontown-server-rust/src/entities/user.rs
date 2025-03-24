use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::ports::object_id::ObjectIdGenerator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub screen_name: String,
    pub lv: i32,
    pub res_last_created_at: DateTime<Utc>,
    pub count_created_res_m10: i32,
    pub count_created_res_m30: i32,
    pub count_created_res_h1: i32,
    pub count_created_res_h6: i32,
    pub count_created_res_h12: i32,
    pub count_created_res_d1: i32,
    pub topic_last_created_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub point: i32,
    pub one_topic_last_created_at: DateTime<Utc>,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn create(
        id_gen: &dyn ObjectIdGenerator,
        screen_name: String,
        name: String,
        email: String,
        password_hash: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: id_gen.generate(),
            screen_name,
            lv: 1,
            res_last_created_at: now,
            count_created_res_m10: 0,
            count_created_res_m30: 0,
            count_created_res_h1: 0,
            count_created_res_h6: 0,
            count_created_res_h12: 0,
            count_created_res_d1: 0,
            topic_last_created_at: now,
            created_at: now,
            point: 0,
            one_topic_last_created_at: now,
            name,
            email,
            password_hash,
            updated_at: now,
        }
    }

    pub fn increment_res_count(&mut self, time_range: TimeRange) {
        match time_range {
            TimeRange::M10 => self.count_created_res_m10 += 1,
            TimeRange::M30 => self.count_created_res_m30 += 1,
            TimeRange::H1 => self.count_created_res_h1 += 1,
            TimeRange::H6 => self.count_created_res_h6 += 1,
            TimeRange::H12 => self.count_created_res_h12 += 1,
            TimeRange::D1 => self.count_created_res_d1 += 1,
        }
        self.res_last_created_at = Utc::now();
        self.updated_at = Utc::now();
    }

    pub fn update_topic_last_created_at(&mut self) {
        self.topic_last_created_at = Utc::now();
        self.updated_at = Utc::now();
    }

    pub fn update_one_topic_last_created_at(&mut self) {
        self.one_topic_last_created_at = Utc::now();
        self.updated_at = Utc::now();
    }

    pub fn add_point(&mut self, point: i32) {
        self.point += point;
        self.updated_at = Utc::now();
    }

    pub fn can_create_res(&self) -> bool {
        let now = Utc::now();
        let one_hour_ago = now - chrono::Duration::hours(1);
        self.res_last_created_at < one_hour_ago
    }

    pub fn can_create_topic(&self) -> bool {
        let now = Utc::now();
        let one_hour_ago = now - chrono::Duration::hours(1);
        self.topic_last_created_at < one_hour_ago
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeRange {
    M10,
    M30,
    H1,
    H6,
    H12,
    D1,
} 