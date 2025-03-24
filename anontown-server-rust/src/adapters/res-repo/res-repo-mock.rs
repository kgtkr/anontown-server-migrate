use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::{Stream, StreamExt};
use std::collections::HashMap;
use crate::entities::{Res, ResType, ResDeleteFlag, ResNormal, ResHistory, ResTopic, ResFork};
use crate::ports::res::ResPort;

pub struct ResRepoMock {
    reses: HashMap<String, Res>,
}

impl ResRepoMock {
    pub fn new() -> Self {
        Self {
            reses: HashMap::new(),
        }
    }
}

#[async_trait]
impl ResPort for ResRepoMock {
    async fn find_by_id(&self, id: &str) -> Result<Option<Res>, Box<dyn std::error::Error>> {
        Ok(self.reses.get(id).cloned())
    }

    async fn find_by_topic_id(
        &self,
        topic_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Res>, Box<dyn std::error::Error>> {
        let mut reses: Vec<Res> = self
            .reses
            .values()
            .filter(|res| res.topic_id == topic_id)
            .cloned()
            .collect();

        reses.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        reses = reses.into_iter().skip(offset as usize).take(limit as usize).collect();

        Ok(reses)
    }

    async fn find_by_reply_id(&self, reply_id: &str) -> Result<Vec<Res>, Box<dyn std::error::Error>> {
        Ok(self.reses
            .values()
            .filter(|res| {
                if let Res::Normal(normal) = res {
                    normal.reply.as_ref().map_or(false, |reply| reply.get("res") == Some(reply_id))
                } else {
                    false
                }
            })
            .cloned()
            .collect())
    }

    async fn find_by_user_id(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Res>, Box<dyn std::error::Error>> {
        let mut reses: Vec<Res> = self
            .reses
            .values()
            .filter(|res| res.user_id == user_id)
            .cloned()
            .collect();

        reses.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        reses = reses.into_iter().skip(offset as usize).take(limit as usize).collect();

        Ok(reses)
    }

    async fn find_by_hash(&self, hash: &str) -> Result<Option<Res>, Box<dyn std::error::Error>> {
        Ok(self.reses
            .values()
            .find(|res| res.base().hash == hash)
            .cloned())
    }

    async fn create(&self, res: &Res) -> Result<(), Box<dyn std::error::Error>> {
        self.reses.insert(res.id.clone(), res.clone());
        Ok(())
    }

    async fn update(&self, res: &Res) -> Result<(), Box<dyn std::error::Error>> {
        if self.reses.contains_key(&res.id) {
            self.reses.insert(res.id.clone(), res.clone());
            Ok(())
        } else {
            Err("Res not found".into())
        }
    }

    async fn update_delete_flag(&self, id: &str, delete_flag: ResDeleteFlag) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(res) = self.reses.get_mut(id) {
            if let Res::Normal(normal) = res {
                normal.delete_flag = Some(delete_flag);
            }
        }
        Ok(())
    }

    async fn update_age(&self, id: &str, age: bool) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(res) = self.reses.get_mut(id) {
            if let Res::Normal(normal) = res {
                normal.age = age;
            }
        }
        Ok(())
    }

    async fn count_by_type(&self, res_type: ResType) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(self.reses
            .values()
            .filter(|res| res.base().res_type == res_type)
            .count() as i64)
    }

    async fn count_by_topic_id(&self, topic_id: &str) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(self.reses
            .values()
            .filter(|res| res.base().topic_id == topic_id)
            .count() as i64)
    }

    async fn count_by_user_id(&self, user_id: &str) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(self.reses
            .values()
            .filter(|res| res.base().user_id == user_id)
            .count() as i64)
    }

    async fn subscribe_insert_event(
        &self,
        topic_id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<(Res, i64), Box<dyn std::error::Error>>> + Send + Unpin>, Box<dyn std::error::Error>> {
        let topic_id = topic_id.to_string();
        let reses = self.reses.clone();

        let stream = async_stream::stream! {
            while false {
                // Mock implementation that never yields any events
                yield Err("Not implemented".into());
            }
        };

        Ok(Box::new(stream))
    }
} 