use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use crate::entities::{Topic, TopicType};
use crate::ports::topic::{TopicPort, TopicQuery};

pub struct TopicRepoMock {
    topics: HashMap<String, Topic>,
    subscriptions: HashMap<String, HashSet<String>>,
}

impl TopicRepoMock {
    pub fn new() -> Self {
        Self {
            topics: HashMap::new(),
            subscriptions: HashMap::new(),
        }
    }
}

#[async_trait]
impl TopicPort for TopicRepoMock {
    async fn find_by_id(&self, id: &str) -> Result<Option<Topic>, Box<dyn std::error::Error>> {
        Ok(self.topics.get(id).cloned())
    }

    async fn find_by_user_id(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Topic>, Box<dyn std::error::Error>> {
        let mut topics: Vec<Topic> = self
            .topics
            .values()
            .filter(|topic| topic.user_id == user_id)
            .cloned()
            .collect();

        topics.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        topics = topics.into_iter().skip(offset as usize).take(limit as usize).collect();

        Ok(topics)
    }

    async fn find_by_hash(&self, hash: &str) -> Result<Option<Topic>, Box<dyn std::error::Error>> {
        Ok(self.topics.values().find(|topic| topic.hash == hash).cloned())
    }

    async fn create(&self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>> {
        self.topics.insert(topic.id.clone(), topic.clone());
        Ok(())
    }

    async fn update(&self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>> {
        if self.topics.contains_key(&topic.id) {
            self.topics.insert(topic.id.clone(), topic.clone());
            Ok(())
        } else {
            Err("Topic not found".into())
        }
    }

    async fn update_res_count(&self, id: &str, count: i64) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(topic) = self.topics.get_mut(id) {
            topic.res_count = count;
            topic.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Topic not found".into())
        }
    }

    async fn update_age(&self, id: &str, age: bool) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(topic) = self.topics.get_mut(id) {
            topic.age = age;
            topic.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Topic not found".into())
        }
    }

    async fn count_by_type(&self, topic_type: TopicType) -> Result<i64, Box<dyn std::error::Error>> {
        let count = self
            .topics
            .values()
            .filter(|topic| topic.topic_type == topic_type)
            .count() as i64;
        Ok(count)
    }

    async fn count_by_user_id(&self, user_id: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let count = self
            .topics
            .values()
            .filter(|topic| topic.user_id == user_id)
            .count() as i64;
        Ok(count)
    }

    async fn find_tags(&mut self, _limit: i32) -> Result<Vec<(String, i32)>, Box<dyn std::error::Error>> {
        let mut tag_counts: HashMap<String, i32> = HashMap::new();
        for topic in self.topics.values() {
            for tag in &topic.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        Ok(tag_counts.into_iter().collect())
    }

    async fn insert(&mut self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>> {
        self.topics.insert(topic.id.clone(), topic.clone());
        Ok(())
    }

    async fn update_topic(&mut self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>> {
        self.topics.insert(topic.id.clone(), topic.clone());
        Ok(())
    }

    async fn cron_topic_check(&mut self, _now: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn find(&mut self, query: &TopicQuery, _skip: i32, _limit: i32) -> Result<Vec<Topic>, Box<dyn std::error::Error>> {
        let mut topics = self.topics.values().cloned().collect::<Vec<_>>();

        if let Some(active_only) = query.active_only {
            topics.retain(|t| t.active == active_only);
        }

        if let Some(ids) = &query.id {
            topics.retain(|t| ids.contains(&t.id));
        }

        if let Some(parent) = &query.parent {
            topics.retain(|t| t.parent_id.as_ref().map_or(false, |p| p == parent));
        }

        if let Some(tags) = &query.tags {
            topics.retain(|t| tags.iter().all(|tag| t.tags.contains(tag)));
        }

        if let Some(title) = &query.title {
            topics.retain(|t| t.title.contains(title));
        }

        Ok(topics)
    }

    async fn subscription_user_ids(&mut self, topic_id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(self.subscriptions
            .get(topic_id)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default())
    }

    async fn enable_subscription(&mut self, topic_id: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.subscriptions
            .entry(topic_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(user_id.to_string());
        Ok(())
    }

    async fn disable_subscription(&mut self, topic_id: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(subscribers) = self.subscriptions.get_mut(topic_id) {
            subscribers.remove(user_id);
        }
        Ok(())
    }

    async fn get_subscription(&mut self, topic_id: &str, user_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.subscriptions
            .get(topic_id)
            .map(|s| s.contains(user_id))
            .unwrap_or(false))
    }
} 