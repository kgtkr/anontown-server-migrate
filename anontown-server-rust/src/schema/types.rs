use juniper::{GraphQLInputObject, GraphQLObject, ID};
use chrono::{DateTime, Utc};

use crate::entities::{client::Client, token::Token, user::User, topic::Topic, res::Res};

#[derive(GraphQLObject)]
pub struct UserType {
    pub id: String,
    pub sn: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(GraphQLInputObject)]
pub struct CreateUserInput {
    pub sn: String,
    pub pass: String,
}

#[derive(GraphQLInputObject)]
pub struct UpdateUserInput {
    pub id: String,
    pub sn: Option<String>,
    pub pass: Option<String>,
}

#[derive(GraphQLObject)]
pub struct ClientType {
    pub id: String,
    pub name: String,
    pub url: String,
    pub self_: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(GraphQLInputObject)]
pub struct CreateClientInput {
    pub name: String,
    pub url: String,
}

#[derive(GraphQLInputObject)]
pub struct UpdateClientInput {
    pub id: String,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(GraphQLObject)]
pub struct TokenType {
    pub id: String,
    pub key: String,
    pub date: DateTime<Utc>,
}

#[derive(GraphQLObject)]
pub struct TokenReq {
    pub token: String,
    pub key: String,
}

#[derive(GraphQLObject)]
pub struct CreateTokenGeneralResponse {
    pub token: TokenType,
    pub req: TokenReq,
}

#[derive(GraphQLObject)]
pub struct TopicType {
    pub id: String,
    pub title: String,
    pub update: DateTime<Utc>,
    pub date: DateTime<Utc>,
    pub res_count: i32,
    pub active: bool,
    pub subscribe: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub text: Option<String>,
    pub parent: Option<Box<TopicType>>,
}

#[derive(GraphQLObject)]
pub struct ResType {
    pub id: String,
    pub topic: TopicType,
    pub date: DateTime<Utc>,
    pub self_: Option<bool>,
    pub uv: i32,
    pub dv: i32,
    pub hash: String,
    pub reply_count: i32,
    pub vote_flag: Option<VoteFlag>,
    pub name: Option<String>,
    pub text: String,
    pub profile: Option<ProfileType>,
}

#[derive(GraphQLObject)]
pub struct ResSubscript {
    pub res: ResType,
    pub count: i32,
}

#[derive(GraphQLObject)]
pub struct ProfileType {
    pub id: String,
    pub name: String,
    pub text: String,
    pub sn: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(GraphQLObject)]
pub struct HistoryType {
    pub id: String,
    pub topic: TopicType,
    pub title: String,
    pub text: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(GraphQLObject)]
pub struct StorageType {
    pub key: String,
    pub value: String,
}

#[derive(GraphQLInputObject)]
pub struct SetStoragesInput {
    pub storages: Vec<StorageInput>,
}

#[derive(GraphQLInputObject)]
pub struct StorageInput {
    pub key: String,
    pub value: String,
}

#[derive(GraphQLObject)]
pub struct SetStoragesPayload {
    pub storages: Vec<StorageType>,
}

#[derive(GraphQLObject)]
pub struct TagType {
    pub name: String,
    pub count: i32,
}

#[derive(GraphQLEnum)]
pub enum VoteType {
    Uv,
    Dv,
    Cv,
}

#[derive(GraphQLEnum)]
pub enum VoteFlag {
    Uv,
    Dv,
    Cv,
}

#[derive(GraphQLEnum)]
pub enum ResDeleteFlag {
    User,
    Admin,
}

#[derive(GraphQLInputObject)]
pub struct AuthUser {
    pub id: String,
    pub pass: String,
}

impl From<User> for UserType {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            sn: user.sn,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<Client> for ClientType {
    fn from(client: Client) -> Self {
        Self {
            id: client.id,
            name: client.name,
            url: client.url,
            self_: client.self_,
            created_at: client.created_at,
            updated_at: client.updated_at,
        }
    }
}

impl From<Token> for TokenType {
    fn from(token: Token) -> Self {
        Self {
            id: token.id,
            key: token.key,
            date: token.date,
        }
    }
}

impl From<Topic> for TopicType {
    fn from(topic: Topic) -> Self {
        Self {
            id: topic.id,
            title: topic.title,
            update: topic.update,
            date: topic.date,
            res_count: topic.res_count,
            active: topic.active,
            subscribe: topic.subscribe,
            tags: topic.tags,
            text: topic.text,
            parent: topic.parent.map(|p| Box::new(p.into())),
        }
    }
}

impl From<Res> for ResType {
    fn from(res: Res) -> Self {
        Self {
            id: res.id,
            topic: res.topic.into(),
            date: res.date,
            self_: res.self_,
            uv: res.uv,
            dv: res.dv,
            hash: res.hash,
            reply_count: res.reply_count,
            vote_flag: res.vote_flag,
            name: res.name,
            text: res.text,
            profile: res.profile.map(|p| p.into()),
        }
    }
} 