use serde::{Deserialize, Serialize};
use crate::ports::object_id::ObjectIdGenerator;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub sn: String,
    pub pass: String,
}

impl User {
    pub fn create(id_gen: &dyn ObjectIdGenerator, sn: String, pass: String) -> Self {
        Self {
            id: id_gen.generate(),
            sn,
            pass,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    pub token: String,
}

impl Token {
    pub fn create(id_gen: &dyn ObjectIdGenerator, user_id: String, token: String) -> Self {
        Self {
            id: id_gen.generate(),
            user_id,
            token,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub url: String,
}

impl Client {
    pub fn create(id_gen: &dyn ObjectIdGenerator, name: String, url: String) -> Self {
        Self {
            id: id_gen.generate(),
            name,
            url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub title: String,
    pub text: String,
    pub user_id: String,
}

impl Topic {
    pub fn create(id_gen: &dyn ObjectIdGenerator, title: String, text: String, user_id: String) -> Self {
        Self {
            id: id_gen.generate(),
            title,
            text,
            user_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Res {
    pub id: String,
    pub text: String,
    pub topic_id: String,
    pub user_id: String,
}

impl Res {
    pub fn create(id_gen: &dyn ObjectIdGenerator, text: String, topic_id: String, user_id: String) -> Self {
        Self {
            id: id_gen.generate(),
            text,
            topic_id,
            user_id,
        }
    }
} 