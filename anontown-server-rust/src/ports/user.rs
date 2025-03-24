use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::models::User;

#[async_trait]
pub trait UserPort {
    async fn find_by_id(&mut self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn find_by_screen_name(&mut self, screen_name: &str) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn create(&mut self, user: &User) -> Result<User, Box<dyn std::error::Error>>;
    async fn update(&mut self, user: &User) -> Result<User, Box<dyn std::error::Error>>;
    async fn update_res_count(&mut self, id: &str, count: i32) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_topic_count(&mut self, id: &str, count: i32) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_point(&mut self, id: &str, point: i32) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_res_last_created_at(&mut self, id: &str, created_at: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_topic_last_created_at(&mut self, id: &str, created_at: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_one_topic_last_created_at(&mut self, id: &str, created_at: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>>;
} 