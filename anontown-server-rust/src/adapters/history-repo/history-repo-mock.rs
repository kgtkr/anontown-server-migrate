use async_trait::async_trait;
use std::collections::HashMap;
use crate::entities::History;
use crate::ports::history::{HistoryPort, HistoryQuery, DateQuery};

pub struct HistoryRepoMock {
    histories: HashMap<String, History>,
}

impl HistoryRepoMock {
    pub fn new() -> Self {
        Self {
            histories: HashMap::new(),
        }
    }
}

#[async_trait]
impl HistoryPort for HistoryRepoMock {
    async fn insert(&mut self, history: &History) -> Result<(), Box<dyn std::error::Error>> {
        self.histories.insert(history.id.clone(), history.clone());
        Ok(())
    }

    async fn update(&mut self, history: &History) -> Result<(), Box<dyn std::error::Error>> {
        if self.histories.contains_key(&history.id) {
            self.histories.insert(history.id.clone(), history.clone());
            Ok(())
        } else {
            Err("History not found".into())
        }
    }

    async fn find_one(&mut self, id: &str) -> Result<History, Box<dyn std::error::Error>> {
        self.histories
            .get(id)
            .cloned()
            .ok_or_else(|| "History not found".into())
    }

    async fn find(&mut self, query: &HistoryQuery, limit: i32) -> Result<Vec<History>, Box<dyn std::error::Error>> {
        let mut histories = self.histories.values().cloned().collect::<Vec<_>>();

        // IDでフィルタリング
        if let Some(ids) = &query.id {
            histories.retain(|h| ids.contains(&h.id));
        }

        // トピックIDでフィルタリング
        if let Some(topic_ids) = &query.topic {
            histories.retain(|h| topic_ids.contains(&h.topic_id));
        }

        // 日付でフィルタリング
        if let Some(date_query) = &query.date {
            histories.retain(|h| {
                match date_query {
                    DateQuery::Gt(date) => h.created_at > *date,
                    DateQuery::Gte(date) => h.created_at >= *date,
                    DateQuery::Lt(date) => h.created_at < *date,
                    DateQuery::Lte(date) => h.created_at <= *date,
                }
            });
        }

        // ソート
        match query.date {
            Some(DateQuery::Gt(_) | DateQuery::Gte(_)) => {
                histories.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            }
            Some(DateQuery::Lt(_) | DateQuery::Lte(_)) => {
                histories.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            }
            None => {
                histories.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            }
        }

        // リミット適用
        histories.truncate(limit as usize);

        Ok(histories)
    }
} 