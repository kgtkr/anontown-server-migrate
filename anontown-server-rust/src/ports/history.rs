use async_trait::async_trait;
use crate::models::history::History;
use crate::ports::types::DateQuery;

#[async_trait]
pub trait HistoryPort {
    async fn find_one(&mut self, id: &str) -> Result<History, Box<dyn std::error::Error>>;
    async fn find(&mut self, query: &HistoryQuery, limit: i32) -> Result<Vec<History>, Box<dyn std::error::Error>>;
    async fn insert(&mut self, history: &History) -> Result<(), Box<dyn std::error::Error>>;
    async fn update(&mut self, history: &History) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&mut self, id: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct HistoryQuery {
    pub date: Option<DateQuery>,
    pub id: Option<Vec<String>>,
    pub topic: Option<Vec<String>>,
} 