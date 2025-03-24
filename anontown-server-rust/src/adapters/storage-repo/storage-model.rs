use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct StorageModel {
    pub id: String,
    pub client_id: Option<String>,
    pub user_id: String,
    pub key: String,
    pub value: String,
    pub date: DateTime<Utc>,
} 