use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::entities::{Res, ResType, ResDeleteFlag};

#[async_trait]
pub trait ResPort {
    async fn find_by_id(&self, id: &str) -> Result<Option<Res>, Box<dyn std::error::Error>>;
    async fn find_by_topic_id(&self, topic_id: &str, limit: i32, offset: i32) -> Result<Vec<Res>, Box<dyn std::error::Error>>;
    async fn find_by_reply_id(&self, reply_id: &str) -> Result<Vec<Res>, Box<dyn std::error::Error>>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Res>, Box<dyn std::error::Error>>;
    async fn find_by_hash(&self, hash: &str) -> Result<Option<Res>, Box<dyn std::error::Error>>;
    async fn create(&self, res: &Res) -> Result<Res, Box<dyn std::error::Error>>;
    async fn update(&self, res: &Res) -> Result<Res, Box<dyn std::error::Error>>;
    async fn update_delete_flag(&self, id: &str, delete_flag: ResDeleteFlag) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_age(&self, id: &str, age: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn count_by_type(&self, res_type: ResType) -> Result<i64, Box<dyn std::error::Error>>;
    async fn count_by_topic_id(&self, topic_id: &str) -> Result<i64, Box<dyn std::error::Error>>;
    async fn count_by_user_id(&self, user_id: &str) -> Result<i64, Box<dyn std::error::Error>>;
} 