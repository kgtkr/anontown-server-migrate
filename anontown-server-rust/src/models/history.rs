use chrono::{DateTime, Utc};
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
pub struct HistoryAPI {
    pub id: String,
    pub topic_id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub text: String,
    pub date: DateTime<Utc>,
    pub hash: String,
    pub self_: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct History {
    pub id: String,
    pub topic_id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub text: String,
    pub date: DateTime<Utc>,
    pub hash: String,
    pub user_id: String,
}

impl History {
    pub fn to_api(&self, auth_token: Option<&AuthToken>) -> HistoryAPI {
        HistoryAPI {
            id: self.id.clone(),
            topic_id: self.topic_id.clone(),
            title: self.title.clone(),
            tags: self.tags.clone(),
            text: self.text.clone(),
            date: self.date,
            hash: self.hash.clone(),
            self_: auth_token.map(|token| token.user_id == self.user_id),
        }
    }
} 