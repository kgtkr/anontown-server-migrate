use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use crate::models::Topic;
use crate::ports::topic::{TopicPort, TopicQuery};

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
    async fn find_one(&mut self, id: &str) -> Result<Topic, Box<dyn std::error::Error>> {
        let topic = sqlx::query_as!(
            Topic,
            r#"
            SELECT id, title, description, type, parent_id, updated_at,
                   created_at, age_updated_at, active, tags
            FROM topics
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(topic)
    }

    async fn find_tags(&mut self, limit: i32) -> Result<Vec<(String, i32)>, Box<dyn std::error::Error>> {
        let tags = sqlx::query!(
            r#"
            SELECT tag, COUNT(*) as count
            FROM topic_tags
            GROUP BY tag
            ORDER BY count DESC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags.into_iter().map(|t| (t.tag, t.count.unwrap_or(0) as i32)).collect())
    }

    async fn insert(&mut self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO topics (
                id, title, description, type, parent_id, updated_at,
                created_at, age_updated_at, active, tags
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            topic.id,
            topic.title,
            topic.description,
            topic.topic_type,
            topic.parent_id,
            topic.updated_at,
            topic.created_at,
            topic.age_updated_at,
            topic.active,
            &topic.tags
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&mut self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE topics
            SET title = $1, description = $2, type = $3,
                parent_id = $4, updated_at = $5,
                created_at = $6, age_updated_at = $7,
                active = $8, tags = $9
            WHERE id = $10
            "#,
            topic.title,
            topic.description,
            topic.topic_type,
            topic.parent_id,
            topic.updated_at,
            topic.created_at,
            topic.age_updated_at,
            topic.active,
            &topic.tags,
            topic.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn cron_topic_check(&mut self, now: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE topics
            SET active = false
            WHERE age_updated_at < $1
            "#,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find(&mut self, query: &TopicQuery, skip: i32, limit: i32) -> Result<Vec<Topic>, Box<dyn std::error::Error>> {
        let mut conditions = Vec::new();
        let mut params = Vec::new();

        if let Some(active_only) = query.active_only {
            conditions.push("active = $1");
            params.push(active_only);
        }

        if let Some(ids) = &query.id {
            if !ids.is_empty() {
                conditions.push(format!("id = ANY(${})", params.len() + 1));
                params.push(ids);
            }
        }

        if let Some(parent) = &query.parent {
            conditions.push(format!("parent_id = ${}", params.len() + 1));
            params.push(parent);
        }

        if let Some(tags) = &query.tags {
            if !tags.is_empty() {
                conditions.push(format!("tags && ${}", params.len() + 1));
                params.push(tags);
            }
        }

        if let Some(title) = &query.title {
            conditions.push(format!("title ILIKE ${}", params.len() + 1));
            params.push(format!("%{}%", title));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let query = format!(
            r#"
            SELECT id, title, description, type, parent_id, updated_at,
                   created_at, age_updated_at, active, tags
            FROM topics
            {}
            ORDER BY created_at DESC
            LIMIT ${{}} OFFSET ${{}}
            "#,
            where_clause
        );

        let mut query = sqlx::query_as::<_, Topic>(&query);
        for param in params {
            query = query.bind(param);
        }
        query = query.bind(limit).bind(skip);

        let topics = query.fetch_all(&self.pool).await?;
        Ok(topics)
    }

    async fn subscription_user_ids(&mut self, topic_id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let user_ids = sqlx::query!(
            r#"
            SELECT user_id
            FROM topic_subscriptions
            WHERE topic_id = $1
            "#,
            topic_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(user_ids.into_iter().map(|r| r.user_id).collect())
    }

    async fn enable_subscription(&mut self, topic_id: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO topic_subscriptions (topic_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            topic_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn disable_subscription(&mut self, topic_id: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            DELETE FROM topic_subscriptions
            WHERE topic_id = $1 AND user_id = $2
            "#,
            topic_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_subscription(&mut self, topic_id: &str, user_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let subscription = sqlx::query!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM topic_subscriptions
                WHERE topic_id = $1 AND user_id = $2
            ) as exists
            "#,
            topic_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(subscription.exists.unwrap_or(false))
    }
} 