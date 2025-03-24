use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct ProfileModel {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub date: DateTime<Utc>,
} 