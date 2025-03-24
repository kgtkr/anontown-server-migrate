use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::models::Topic;

#[derive(Debug, Clone)]
pub struct TopicQuery {
    pub active_only: Option<bool>,
    pub id: Option<Vec<String>>,
    pub parent: Option<String>,
    pub tags: Option<Vec<String>>,
    pub title: Option<String>,
}

#[async_trait]
pub trait TopicPort {
    async fn find_one(&mut self, id: &str) -> Result<Topic, Box<dyn std::error::Error>>;
    async fn find_tags(&mut self, limit: i32) -> Result<Vec<(String, i32)>, Box<dyn std::error::Error>>;
    async fn insert(&mut self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>>;
    async fn update(&mut self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>>;
    async fn cron_topic_check(&mut self, now: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>>;
    async fn find(&mut self, query: &TopicQuery, skip: i32, limit: i32) -> Result<Vec<Topic>, Box<dyn std::error::Error>>;
    async fn subscription_user_ids(&mut self, topic_id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn enable_subscription(&mut self, topic_id: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn disable_subscription(&mut self, topic_id: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_subscription(&mut self, topic_id: &str, user_id: &str) -> Result<bool, Box<dyn std::error::Error>>;
} 