use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub screen_name: String,
    pub encrypted_password: String,
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub screen_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub url: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "general")]
    General,
    #[serde(rename = "master")]
    Master,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub key: String,
    pub token_type: TokenType,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub client_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenReq {
    pub key: String,
    pub expires: DateTime<Utc>,
    pub active: bool,
    pub token_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    pub client_id: String,
    pub user_id: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "history")]
    History,
    #[serde(rename = "topic")]
    Topic,
    #[serde(rename = "fork")]
    Fork,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResDeleteFlag {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "self")]
    Self_,
    #[serde(rename = "freeze")]
    Freeze,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Res {
    pub id: String,
    pub res_type: ResType,
    pub topic_id: String,
    pub created_at: DateTime<Utc>,
    pub user_id: String,
    pub lv: i32,
    pub hash: String,
    pub name: Option<String>,
    pub content: Option<String>,
    pub reply_id: Option<String>,
    pub delete_flag: Option<ResDeleteFlag>,
    pub profile_id: Option<String>,
    pub age: Option<bool>,
    pub history_id: Option<String>,
    pub fork_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResVote {
    pub res_id: String,
    pub order: i32,
    pub user_id: String,
    pub vote: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub id: String,
    pub topic_id: String,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub hash: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryTag {
    pub history_id: String,
    pub order: i32,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TopicType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "one")]
    One,
    #[serde(rename = "fork")]
    Fork,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub topic_type: TopicType,
    pub title: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub age_updated_at: DateTime<Utc>,
    pub active: bool,
    pub description: Option<String>,
    pub parent_id: Option<String>,
} 