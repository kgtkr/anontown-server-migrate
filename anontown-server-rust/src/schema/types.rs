use juniper::{GraphQLInputObject, GraphQLObject, ID, GraphQLEnum};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::{client::Client, token::Token, user::User, topic::Topic, res::Res};
use crate::schema::scalar::DateTimeScalar;

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
    pub id: ID,
    pub title: String,
    pub text: String,
    pub update: DateTimeScalar,
    pub date: DateTimeScalar,
    pub user_id: ID,
    pub vote_flag: Option<VoteFlag>,
}

#[derive(GraphQLObject)]
pub struct ResType {
    pub id: ID,
    pub text: String,
    pub topic_id: ID,
    pub user_id: ID,
    pub created_at: DateTimeScalar,
    pub updated_at: DateTimeScalar,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, GraphQLEnum)]
pub enum VoteFlag {
    Up,
    Down,
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
            text: topic.text,
            update: topic.update,
            date: topic.date,
            user_id: topic.user_id,
            vote_flag: topic.vote_flag,
        }
    }
}

impl From<Res> for ResType {
    fn from(res: Res) -> Self {
        Self {
            id: res.id,
            text: res.text,
            topic_id: res.topic_id,
            user_id: res.user_id,
            created_at: res.created_at,
            updated_at: res.updated_at,
        }
    }
} 