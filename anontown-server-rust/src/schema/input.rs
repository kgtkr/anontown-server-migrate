use juniper::GraphQLInputObject;
use chrono::{DateTime, Utc};

#[derive(GraphQLInputObject)]
pub struct DateQuery {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(GraphQLInputObject)]
pub struct ResQuery {
    pub id: Option<Vec<String>>,
    pub topic: Option<String>,
    pub notice: Option<bool>,
    pub hash: Option<String>,
    pub reply: Option<String>,
    pub profile: Option<String>,
    pub self_: Option<bool>,
    pub text: Option<String>,
    pub date: Option<DateQuery>,
}

#[derive(GraphQLInputObject)]
pub struct TopicQuery {
    pub id: Option<Vec<String>>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub active_only: Option<bool>,
    pub parent: Option<String>,
}

#[derive(GraphQLInputObject)]
pub struct CreateResInput {
    pub topic: String,
    pub name: Option<String>,
    pub text: String,
    pub reply: Option<String>,
    pub profile: Option<String>,
    pub age: bool,
}

#[derive(GraphQLInputObject)]
pub struct CreateTopicNormalInput {
    pub title: String,
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(GraphQLInputObject)]
pub struct CreateTopicOneInput {
    pub title: String,
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(GraphQLInputObject)]
pub struct CreateTopicForkInput {
    pub title: String,
    pub parent: String,
}

#[derive(GraphQLInputObject)]
pub struct UpdateTopicInput {
    pub id: String,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub text: Option<String>,
} 