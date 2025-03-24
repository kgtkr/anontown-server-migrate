use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::ports::object_id::ObjectIdGenerator;
use crate::entities::user::User;
use crate::entities::topic::Topic;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ResType {
    Normal,
    History,
    Topic,
    Fork,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub user: String,
    pub value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reply {
    pub res: String,
    pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResBase {
    pub id: String,
    pub topic_id: String,
    pub date: DateTime<Utc>,
    pub user_id: String,
    pub votes: Vec<Vote>,
    pub lv: i32,
    pub hash: String,
    pub reply_count: i32,
    pub res_type: ResType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResSearchBase {
    #[serde(flatten)]
    base: ResBase,
}

impl ResSearchBase {
    pub fn id(&self) -> &str {
        &self.base.id
    }

    pub fn topic_id(&self) -> &str {
        &self.base.topic_id
    }

    pub fn date(&self) -> DateTime<Utc> {
        self.base.date
    }

    pub fn user_id(&self) -> &str {
        &self.base.user_id
    }

    pub fn votes(&self) -> &[Vote] {
        &self.base.votes
    }

    pub fn lv(&self) -> i32 {
        self.base.lv
    }

    pub fn hash(&self) -> &str {
        &self.base.hash
    }

    pub fn reply_count(&self) -> i32 {
        self.base.reply_count
    }

    pub fn res_type(&self) -> ResType {
        self.base.res_type
    }

    pub fn vote(&mut self, res_user: &mut User, user: &User, vote_type: &str) -> Result<(), String> {
        if self.base.user_id == user.id {
            return Err("自分に投票できません".to_string());
        }

        let vote_value = match vote_type {
            "uv" => 2,
            "dv" => -1,
            _ => return Err("無効な投票タイプです".to_string()),
        };

        if let Some(vote) = self.base.votes.iter_mut().find(|v| v.user == user.id) {
            if (vote.value > 0 && vote_type == "dv") || (vote.value < 0 && vote_type == "uv") {
                vote.value = vote_value;
            }
        } else {
            self.base.votes.push(Vote {
                user: user.id.clone(),
                value: vote_value,
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResNormal {
    #[serde(flatten)]
    base: ResSearchBase,
    pub name: Option<String>,
    pub text: String,
    pub reply: Option<Reply>,
    pub delete_flag: String,
    pub profile: Option<String>,
    pub age: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResHistory {
    #[serde(flatten)]
    base: ResSearchBase,
    pub history_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResTopic {
    #[serde(flatten)]
    base: ResSearchBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResFork {
    #[serde(flatten)]
    base: ResSearchBase,
    pub fork_id: String,
}

impl ResNormal {
    pub fn create(
        id_gen: &dyn ObjectIdGenerator,
        topic: &Topic,
        user: &User,
        name: Option<String>,
        text: String,
        reply: Option<Reply>,
        profile: Option<String>,
        age: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            base: ResSearchBase {
                base: ResBase {
                    id: id_gen.generate(),
                    topic_id: topic.id().to_string(),
                    date: now,
                    user_id: user.id.clone(),
                    votes: Vec::new(),
                    lv: user.lv * 5,
                    hash: topic.hash(now, user),
                    reply_count: 0,
                    res_type: ResType::Normal,
                },
            },
            name,
            text,
            reply,
            delete_flag: "active".to_string(),
            profile,
            age,
        }
    }

    pub fn base(&self) -> &ResSearchBase {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ResSearchBase {
        &mut self.base
    }
}

impl ResHistory {
    pub fn create(
        id_gen: &dyn ObjectIdGenerator,
        topic: &Topic,
        user: &User,
        history_id: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            base: ResSearchBase {
                base: ResBase {
                    id: id_gen.generate(),
                    topic_id: topic.id().to_string(),
                    date: now,
                    user_id: user.id.clone(),
                    votes: Vec::new(),
                    lv: user.lv * 5,
                    hash: topic.hash(now, user),
                    reply_count: 0,
                    res_type: ResType::History,
                },
            },
            history_id,
        }
    }

    pub fn base(&self) -> &ResSearchBase {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ResSearchBase {
        &mut self.base
    }
}

impl ResTopic {
    pub fn create(
        id_gen: &dyn ObjectIdGenerator,
        topic: &Topic,
        user: &User,
    ) -> Self {
        let now = Utc::now();
        Self {
            base: ResSearchBase {
                base: ResBase {
                    id: id_gen.generate(),
                    topic_id: topic.id().to_string(),
                    date: now,
                    user_id: user.id.clone(),
                    votes: Vec::new(),
                    lv: user.lv * 5,
                    hash: topic.hash(now, user),
                    reply_count: 0,
                    res_type: ResType::Topic,
                },
            },
        }
    }

    pub fn base(&self) -> &ResSearchBase {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ResSearchBase {
        &mut self.base
    }
}

impl ResFork {
    pub fn create(
        id_gen: &dyn ObjectIdGenerator,
        topic: &Topic,
        user: &User,
        fork_id: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            base: ResSearchBase {
                base: ResBase {
                    id: id_gen.generate(),
                    topic_id: topic.id().to_string(),
                    date: now,
                    user_id: user.id.clone(),
                    votes: Vec::new(),
                    lv: user.lv * 5,
                    hash: topic.hash(now, user),
                    reply_count: 0,
                    res_type: ResType::Fork,
                },
            },
            fork_id,
        }
    }

    pub fn base(&self) -> &ResSearchBase {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ResSearchBase {
        &mut self.base
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Res {
    Normal(ResNormal),
    History(ResHistory),
    Topic(ResTopic),
    Fork(ResFork),
}

impl Res {
    pub fn base(&self) -> &ResSearchBase {
        match self {
            Res::Normal(res) => res.base(),
            Res::History(res) => res.base(),
            Res::Topic(res) => res.base(),
            Res::Fork(res) => res.base(),
        }
    }

    pub fn base_mut(&mut self) -> &mut ResSearchBase {
        match self {
            Res::Normal(res) => res.base_mut(),
            Res::History(res) => res.base_mut(),
            Res::Topic(res) => res.base_mut(),
            Res::Fork(res) => res.base_mut(),
        }
    }
}