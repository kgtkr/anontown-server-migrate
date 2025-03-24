use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use std::collections::HashMap;

use crate::entities::{Topic, TopicType};
use crate::ports::topic::TopicPort;

pub struct TopicRepo {
    pool: PgPool,
}

impl TopicRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TopicPort for TopicRepo {
    async fn find_by_id(&self, id: &str) -> Result<Option<Topic>, Box<dyn std::error::Error>> {
        let topic = sqlx::query_as!(
            Topic,
            r#"
            SELECT id, title, text, created_at, updated_at, user_id, topic_type as "topic_type: TopicType",
                   res_count, hash, one, profile_id, age, history_id, fork_id
            FROM topics
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(topic)
    }

    async fn find_by_user_id(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Topic>, Box<dyn std::error::Error>> {
        let topics = sqlx::query_as!(
            Topic,
            r#"
            SELECT id, title, text, created_at, updated_at, user_id, topic_type as "topic_type: TopicType",
                   res_count, hash, one, profile_id, age, history_id, fork_id
            FROM topics
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(topics)
    }

    async fn find_by_hash(&self, hash: &str) -> Result<Option<Topic>, Box<dyn std::error::Error>> {
        let topic = sqlx::query_as!(
            Topic,
            r#"
            SELECT id, title, text, created_at, updated_at, user_id, topic_type as "topic_type: TopicType",
                   res_count, hash, one, profile_id, age, history_id, fork_id
            FROM topics
            WHERE hash = $1
            "#,
            hash
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(topic)
    }

    async fn create(&self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO topics (id, title, text, created_at, updated_at, user_id, topic_type,
                              res_count, hash, one, profile_id, age, history_id, fork_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
            topic.id,
            topic.title,
            topic.text,
            topic.created_at,
            topic.updated_at,
            topic.user_id,
            topic.topic_type as TopicType,
            topic.res_count,
            topic.hash,
            topic.one,
            topic.profile_id,
            topic.age,
            topic.history_id,
            topic.fork_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE topics
            SET title = $1, text = $2, updated_at = $3, topic_type = $4,
                res_count = $5, hash = $6, one = $7, profile_id = $8,
                age = $9, history_id = $10, fork_id = $11
            WHERE id = $12
            "#,
            topic.title,
            topic.text,
            topic.updated_at,
            topic.topic_type as TopicType,
            topic.res_count,
            topic.hash,
            topic.one,
            topic.profile_id,
            topic.age,
            topic.history_id,
            topic.fork_id,
            topic.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_res_count(&self, id: &str, count: i64) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE topics
            SET res_count = $1, updated_at = $2
            WHERE id = $3
            "#,
            count,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_age(&self, id: &str, age: bool) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE topics
            SET age = $1, updated_at = $2
            WHERE id = $3
            "#,
            age,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn count_by_type(&self, topic_type: TopicType) -> Result<i64, Box<dyn std::error::Error>> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM topics
            WHERE topic_type = $1
            "#,
            topic_type as TopicType
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0);

        Ok(count)
    }

    async fn count_by_user_id(&self, user_id: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM topics
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0);

        Ok(count)
    }
} 