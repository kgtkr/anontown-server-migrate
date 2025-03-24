use juniper::{GraphQLInputObject, GraphQLObject, ID, GraphQLEnum, GraphQLUnion};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::{client::Client, token::Token, user::User, topic::Topic, res::Res};
use crate::schema::scalar::DateTimeScalar;
use crate::ports::AuthContainer;

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
pub struct TopicBaseType {
    pub id: ID,
    pub title: String,
    pub update: DateTimeScalar,
    pub date: DateTimeScalar,
    pub res_count: i32,
    pub active: bool,
    pub subscribe: Option<bool>,
}

#[derive(GraphQLObject)]
pub struct TopicSearchBaseType {
    pub id: ID,
    pub title: String,
    pub update: DateTimeScalar,
    pub date: DateTimeScalar,
    pub res_count: i32,
    pub active: bool,
    pub subscribe: Option<bool>,
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(GraphQLObject)]
pub struct TopicNormalType {
    #[graphql(flatten)]
    pub base: TopicSearchBaseType,
}

#[derive(GraphQLObject)]
pub struct TopicOneType {
    #[graphql(flatten)]
    pub base: TopicSearchBaseType,
}

#[derive(GraphQLObject)]
pub struct TopicForkType {
    #[graphql(flatten)]
    pub base: TopicBaseType,
    pub parent: TopicNormalType,
}

#[derive(GraphQLObject)]
pub struct ResBaseType {
    pub id: ID,
    pub topic: TopicType,
    pub date: DateTimeScalar,
    pub self_: Option<bool>,
    pub uv: i32,
    pub dv: i32,
    pub hash: String,
    pub reply_count: i32,
    pub vote_flag: Option<VoteFlag>,
}

#[derive(GraphQLObject)]
pub struct ResNormalType {
    #[graphql(flatten)]
    pub base: ResBaseType,
    pub name: Option<String>,
    pub text: String,
    pub reply: Option<ResType>,
    pub profile: Option<ProfileType>,
    pub is_reply: Option<bool>,
}

#[derive(GraphQLObject)]
pub struct ResHistoryType {
    #[graphql(flatten)]
    pub base: ResBaseType,
    pub history: HistoryType,
}

#[derive(GraphQLObject)]
pub struct ResTopicType {
    #[graphql(flatten)]
    pub base: ResBaseType,
}

#[derive(GraphQLObject)]
pub struct ResForkType {
    #[graphql(flatten)]
    pub base: ResBaseType,
    pub fork: TopicForkType,
}

#[derive(GraphQLObject)]
pub struct ResDeleteType {
    #[graphql(flatten)]
    pub base: ResBaseType,
    pub flag: ResDeleteFlag,
}

#[derive(GraphQLUnion)]
pub enum TopicType {
    Normal(TopicNormalType),
    One(TopicOneType),
    Fork(TopicForkType),
}

#[derive(GraphQLUnion)]
pub enum ResType {
    Normal(ResNormalType),
    History(ResHistoryType),
    Topic(ResTopicType),
    Fork(ResForkType),
    Delete(ResDeleteType),
}

#[derive(GraphQLObject)]
pub struct TopicType {
    pub id: ID,
    pub title: String,
    pub description: String,
    pub topic_type: TopicTypeEnum,
    pub user_id: ID,
    pub created_at: DateTimeScalar,
    pub updated_at: DateTimeScalar,
    pub res_count: i32,
    pub last_res_at: DateTimeScalar,
    pub is_closed: bool,
    pub tags: Vec<String>,
}

#[derive(GraphQLEnum)]
pub enum TopicTypeEnum {
    Normal,
    One,
    Fork,
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
        let base = topic.base();
        match topic {
            Topic::Normal(normal) => TopicType::Normal(TopicNormalType {
                base: TopicSearchBaseType {
                    id: ID::new(&base.id),
                    title: base.title.clone(),
                    update: DateTimeScalar::new(base.updated_at),
                    date: DateTimeScalar::new(base.created_at),
                    res_count: base.res_count,
                    active: !base.is_closed,
                    subscribe: None,
                    tags: base.tags.clone(),
                    text: base.description.clone(),
                },
            }),
            Topic::One(one) => TopicType::One(TopicOneType {
                base: TopicSearchBaseType {
                    id: ID::new(&base.id),
                    title: base.title.clone(),
                    update: DateTimeScalar::new(base.updated_at),
                    date: DateTimeScalar::new(base.created_at),
                    res_count: base.res_count,
                    active: !base.is_closed,
                    subscribe: None,
                    tags: base.tags.clone(),
                    text: base.description.clone(),
                },
            }),
            Topic::Fork(fork) => TopicType::Fork(TopicForkType {
                base: TopicBaseType {
                    id: ID::new(&base.id),
                    title: base.title.clone(),
                    update: DateTimeScalar::new(base.updated_at),
                    date: DateTimeScalar::new(base.created_at),
                    res_count: base.res_count,
                    active: !base.is_closed,
                    subscribe: None,
                },
                parent: TopicNormalType {
                    base: TopicSearchBaseType {
                        id: ID::new(&base.id),
                        title: base.title.clone(),
                        update: DateTimeScalar::new(base.updated_at),
                        date: DateTimeScalar::new(base.created_at),
                        res_count: base.res_count,
                        active: !base.is_closed,
                        subscribe: None,
                        tags: base.tags.clone(),
                        text: base.description.clone(),
                    },
                },
            }),
        }
    }
}

impl From<Res> for ResType {
    fn from(res: Res) -> Self {
        let base = res.base();
        match res {
            Res::Normal(normal) => ResType::Normal(ResNormalType {
                base: ResBaseType {
                    id: ID::new(&base.id),
                    topic: TopicType::from(normal.topic),
                    date: DateTimeScalar::new(base.created_at),
                    self_: None,
                    uv: base.uv,
                    dv: base.dv,
                    hash: base.hash,
                    reply_count: base.reply_count,
                    vote_flag: base.vote_flag,
                },
                name: normal.name,
                text: normal.text,
                reply: normal.reply.map(ResType::from),
                profile: None,
                is_reply: None,
            }),
            Res::History(history) => ResType::History(ResHistoryType {
                base: ResBaseType {
                    id: ID::new(&base.id),
                    topic: TopicType::from(history.topic),
                    date: DateTimeScalar::new(base.created_at),
                    self_: None,
                    uv: base.uv,
                    dv: base.dv,
                    hash: base.hash,
                    reply_count: base.reply_count,
                    vote_flag: base.vote_flag,
                },
                history: HistoryType {
                    id: history.history.id,
                    topic: TopicType::from(history.topic),
                    title: history.history.title,
                    text: history.history.text,
                    tags: history.history.tags,
                    created_at: history.history.created_at,
                },
            }),
            Res::Topic(topic) => ResType::Topic(ResTopicType {
                base: ResBaseType {
                    id: ID::new(&base.id),
                    topic: TopicType::from(topic.topic),
                    date: DateTimeScalar::new(base.created_at),
                    self_: None,
                    uv: base.uv,
                    dv: base.dv,
                    hash: base.hash,
                    reply_count: base.reply_count,
                    vote_flag: base.vote_flag,
                },
            }),
            Res::Fork(fork) => ResType::Fork(ResForkType {
                base: ResBaseType {
                    id: ID::new(&base.id),
                    topic: TopicType::from(fork.topic),
                    date: DateTimeScalar::new(base.created_at),
                    self_: None,
                    uv: base.uv,
                    dv: base.dv,
                    hash: base.hash,
                    reply_count: base.reply_count,
                    vote_flag: base.vote_flag,
                },
                fork: TopicForkType {
                    base: TopicBaseType {
                        id: ID::new(&fork.fork.base.id),
                        title: fork.fork.base.title.clone(),
                        update: DateTimeScalar::new(fork.fork.base.updated_at),
                        date: DateTimeScalar::new(fork.fork.base.created_at),
                        res_count: fork.fork.base.res_count,
                        active: !fork.fork.base.is_closed,
                        subscribe: None,
                    },
                    parent: TopicNormalType {
                        base: TopicSearchBaseType {
                            id: ID::new(&fork.fork.base.id),
                            title: fork.fork.base.title.clone(),
                            update: DateTimeScalar::new(fork.fork.base.updated_at),
                            date: DateTimeScalar::new(fork.fork.base.created_at),
                            res_count: fork.fork.base.res_count,
                            active: !fork.fork.base.is_closed,
                            subscribe: None,
                            tags: fork.fork.base.tags.clone(),
                            text: fork.fork.base.description.clone(),
                        },
                    },
                },
            }),
            Res::Delete(delete) => ResType::Delete(ResDeleteType {
                base: ResBaseType {
                    id: ID::new(&base.id),
                    topic: TopicType::from(delete.topic),
                    date: DateTimeScalar::new(base.created_at),
                    self_: None,
                    uv: base.uv,
                    dv: base.dv,
                    hash: base.hash,
                    reply_count: base.reply_count,
                    vote_flag: base.vote_flag,
                },
                flag: delete.flag,
            }),
        }
    }
}

pub trait ToSchemaType {
    type SchemaType;
    fn to_schema_type(&self, auth_container: &AuthContainer) -> Self::SchemaType;
}

impl ToSchemaType for Client {
    type SchemaType = ClientType;

    fn to_schema_type(&self, auth_container: &AuthContainer) -> Self::SchemaType {
        let self_ = auth_container.get_token_master_or_null().map(|token| token.user == self.id);
        ClientType {
            id: self.id.clone(),
            name: self.name.clone(),
            url: self.url.clone(),
            self_,
            created_at: self.date,
            updated_at: self.update,
        }
    }
}

impl ToSchemaType for User {
    type SchemaType = UserType;

    fn to_schema_type(&self, _auth_container: &AuthContainer) -> Self::SchemaType {
        UserType {
            id: self.id.clone(),
            sn: self.sn.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl ToSchemaType for Token {
    type SchemaType = TokenType;

    fn to_schema_type(&self, _auth_container: &AuthContainer) -> Self::SchemaType {
        TokenType {
            id: self.id.clone(),
            key: self.key.clone(),
            date: self.date,
        }
    }
}

impl ToSchemaType for Topic {
    type SchemaType = TopicType;

    fn to_schema_type(&self, auth_container: &AuthContainer) -> Self::SchemaType {
        let base = self.base();
        let subscribe = auth_container.get_token_or_null().map(|token| {
            // TODO: Implement subscription check
            false
        });

        match self {
            Topic::Normal(normal) => TopicType::Normal(TopicNormalType {
                base: TopicSearchBaseType {
                    id: ID::new(&base.id),
                    title: base.title.clone(),
                    update: DateTimeScalar::new(base.updated_at),
                    date: DateTimeScalar::new(base.created_at),
                    res_count: base.res_count,
                    active: !base.is_closed,
                    subscribe,
                    tags: base.tags.clone(),
                    text: base.description.clone(),
                },
            }),
            Topic::One(one) => TopicType::One(TopicOneType {
                base: TopicSearchBaseType {
                    id: ID::new(&base.id),
                    title: base.title.clone(),
                    update: DateTimeScalar::new(base.updated_at),
                    date: DateTimeScalar::new(base.created_at),
                    res_count: base.res_count,
                    active: !base.is_closed,
                    subscribe,
                    tags: base.tags.clone(),
                    text: base.description.clone(),
                },
            }),
            Topic::Fork(fork) => TopicType::Fork(TopicForkType {
                base: TopicBaseType {
                    id: ID::new(&base.id),
                    title: base.title.clone(),
                    update: DateTimeScalar::new(base.updated_at),
                    date: DateTimeScalar::new(base.created_at),
                    res_count: base.res_count,
                    active: !base.is_closed,
                    subscribe,
                },
                parent: TopicNormalType {
                    base: TopicSearchBaseType {
                        id: ID::new(&fork.parent.base.id),
                        title: fork.parent.base.title.clone(),
                        update: DateTimeScalar::new(fork.parent.base.updated_at),
                        date: DateTimeScalar::new(fork.parent.base.created_at),
                        res_count: fork.parent.base.res_count,
                        active: !fork.parent.base.is_closed,
                        subscribe: None,
                        tags: fork.parent.base.tags.clone(),
                        text: fork.parent.base.description.clone(),
                    },
                },
            }),
        }
    }
}

impl ToSchemaType for Res {
    type SchemaType = ResType;

    fn to_schema_type(&self, auth_container: &AuthContainer) -> Self::SchemaType {
        let base = self.base();
        let self_ = auth_container.get_token_or_null().map(|token| token.user == base.user_id);

        match self {
            Res::Normal(normal) => ResType::Normal(ResNormalType {
                base: ResBaseType {
                    id: ID::new(&base.id),
                    topic: TopicType::from(normal.topic.clone()),
                    date: DateTimeScalar::new(base.created_at),
                    self_,
                    uv: base.uv,
                    dv: base.dv,
                    hash: base.hash.clone(),
                    reply_count: base.reply_count,
                    vote_flag: base.vote_flag,
                },
                name: normal.name.clone(),
                text: normal.text.clone(),
                reply: normal.reply.clone().map(ResType::from),
                profile: None,
                is_reply: None,
            }),
            Res::History(history) => ResType::History(ResHistoryType {
                base: ResBaseType {
                    id: ID::new(&base.id),
                    topic: TopicType::from(history.topic.clone()),
                    date: DateTimeScalar::new(base.created_at),
                    self_,
                    uv: base.uv,
                    dv: base.dv,
                    hash: base.hash.clone(),
                    reply_count: base.reply_count,
                    vote_flag: base.vote_flag,
                },
                history: HistoryType {
                    id: history.history.id.clone(),
                    topic: TopicType::from(history.topic.clone()),
                    title: history.history.title.clone(),
                    text: history.history.text.clone(),
                    tags: history.history.tags.clone(),
                    created_at: history.history.created_at,
                },
            }),
            Res::Topic(topic) => ResType::Topic(ResTopicType {
                base: ResBaseType {
                    id: ID::new(&base.id),
                    topic: TopicType::from(topic.topic.clone()),
                    date: DateTimeScalar::new(base.created_at),
                    self_,
                    uv: base.uv,
                    dv: base.dv,
                    hash: base.hash.clone(),
                    reply_count: base.reply_count,
                    vote_flag: base.vote_flag,
                },
            }),
            Res::Fork(fork) => ResType::Fork(ResForkType {
                base: ResBaseType {
                    id: ID::new(&base.id),
                    topic: TopicType::from(fork.topic.clone()),
                    date: DateTimeScalar::new(base.created_at),
                    self_,
                    uv: base.uv,
                    dv: base.dv,
                    hash: base.hash.clone(),
                    reply_count: base.reply_count,
                    vote_flag: base.vote_flag,
                },
                fork: TopicForkType {
                    base: TopicBaseType {
                        id: ID::new(&fork.fork.base.id),
                        title: fork.fork.base.title.clone(),
                        update: DateTimeScalar::new(fork.fork.base.updated_at),
                        date: DateTimeScalar::new(fork.fork.base.created_at),
                        res_count: fork.fork.base.res_count,
                        active: !fork.fork.base.is_closed,
                        subscribe: None,
                    },
                    parent: TopicNormalType {
                        base: TopicSearchBaseType {
                            id: ID::new(&fork.fork.base.id),
                            title: fork.fork.base.title.clone(),
                            update: DateTimeScalar::new(fork.fork.base.updated_at),
                            date: DateTimeScalar::new(fork.fork.base.created_at),
                            res_count: fork.fork.base.res_count,
                            active: !fork.fork.base.is_closed,
                            subscribe: None,
                            tags: fork.fork.base.tags.clone(),
                            text: fork.fork.base.description.clone(),
                        },
                    },
                },
            }),
            Res::Delete(delete) => ResType::Delete(ResDeleteType {
                base: ResBaseType {
                    id: ID::new(&base.id),
                    topic: TopicType::from(delete.topic.clone()),
                    date: DateTimeScalar::new(base.created_at),
                    self_,
                    uv: base.uv,
                    dv: base.dv,
                    hash: base.hash.clone(),
                    reply_count: base.reply_count,
                    vote_flag: base.vote_flag,
                },
                flag: delete.flag,
            }),
        }
    }
}

impl ToSchemaType for History {
    type SchemaType = HistoryType;

    fn to_schema_type(&self, _auth_container: &AuthContainer) -> Self::SchemaType {
        HistoryType {
            id: self.id.clone(),
            topic: TopicType::from(self.topic.clone()),
            title: self.title.clone(),
            text: self.text.clone(),
            tags: self.tags.clone(),
            created_at: self.created_at,
        }
    }
}

impl ToSchemaType for Profile {
    type SchemaType = ProfileType;

    fn to_schema_type(&self, _auth_container: &AuthContainer) -> Self::SchemaType {
        ProfileType {
            id: self.id.clone(),
            name: self.name.clone(),
            text: self.text.clone(),
            sn: self.sn.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl ToSchemaType for Storage {
    type SchemaType = StorageType;

    fn to_schema_type(&self, _auth_container: &AuthContainer) -> Self::SchemaType {
        StorageType {
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
} 