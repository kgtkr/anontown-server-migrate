use chrono::{DateTime, Utc};
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
pub struct ProfileAPI {
    pub id: String,
    pub self_: Option<bool>,
    pub name: String,
    pub text: String,
    pub date: DateTime<Utc>,
    pub update: DateTime<Utc>,
    pub sn: String,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub text: String,
    pub date: DateTime<Utc>,
    pub update: DateTime<Utc>,
    pub sn: String,
}

impl Profile {
    pub fn to_api(&self, auth_token: Option<&AuthToken>) -> ProfileAPI {
        ProfileAPI {
            id: self.id.clone(),
            self_: auth_token.map(|token| token.user_id == self.user_id),
            name: self.name.clone(),
            text: self.text.clone(),
            date: self.date,
            update: self.update,
            sn: self.sn.clone(),
        }
    }
} 