use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use crate::models::Topic;
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
    async fn find_one(&mut self, id: &str) -> Result<Topic, Box<dyn std::error::Error>> {
        self.topics.get(id)
            .cloned()
            .ok_or_else(|| "Topic not found".into())
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

    async fn update(&mut self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>> {
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