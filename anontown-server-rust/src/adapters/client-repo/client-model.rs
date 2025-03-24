use chrono::{DateTime, Utc};
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
pub struct ClientAPI {
    pub id: String,
    pub self_: Option<bool>,
    pub name: String,
    pub url: String,
    pub date: DateTime<Utc>,
    pub update: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub url: String,
    pub date: DateTime<Utc>,
    pub update: DateTime<Utc>,
}

impl Client {
    pub fn to_api(&self, auth_token: Option<&AuthTokenMaster>) -> ClientAPI {
        ClientAPI {
            id: self.id.clone(),
            self_: auth_token.map(|token| token.user_id == self.user_id),
            name: self.name.clone(),
            url: self.url.clone(),
            date: self.date,
            update: self.update,
        }
    }
} 