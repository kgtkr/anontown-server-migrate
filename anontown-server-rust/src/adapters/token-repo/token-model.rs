use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct TokenModel {
    pub id: String,
    pub user_id: String,
    pub client_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub date: DateTime<Utc>,
} 