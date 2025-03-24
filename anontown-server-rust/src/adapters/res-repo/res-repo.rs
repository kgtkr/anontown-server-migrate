use async_trait::async_trait;
use chrono::Utc;
use futures::Stream;
use redis::AsyncCommands;
use sqlx::PgPool;
use std::sync::Arc;

use crate::entities::Res;
use crate::ports::res::ResPort;

const RES_PUBSUB_CHANNEL: &str = "res/add";

#[derive(serde::Serialize)]
struct ResPubSub {
    id: String,
    topic: String,
    count: i64,
}

pub struct ResRepo {
    pool: PgPool,
    redis: Arc<redis::Client>,
}

impl ResRepo {
    pub fn new(pool: PgPool, redis: Arc<redis::Client>) -> Self {
        Self { pool, redis }
    }
}

#[async_trait]
impl ResPort for ResRepo {
    async fn find_by_id(&self, id: &str) -> Result<Option<Res>, Box<dyn std::error::Error>> {
        let res = sqlx::query_as!(
            Res,
            r#"
            SELECT id, text, created_at, updated_at, user_id, topic_id, history_id
            FROM reses
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(res)
    }

    async fn find_by_topic_id(
        &self,
        topic_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Res>, Box<dyn std::error::Error>> {
        let reses = sqlx::query_as!(
            Res,
            r#"
            SELECT id, text, created_at, updated_at, user_id, topic_id, history_id
            FROM reses
            WHERE topic_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            topic_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reses)
    }

    async fn find_by_user_id(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Res>, Box<dyn std::error::Error>> {
        let reses = sqlx::query_as!(
            Res,
            r#"
            SELECT id, text, created_at, updated_at, user_id, topic_id, history_id
            FROM reses
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

        Ok(reses)
    }

    async fn create(&self, res: &Res) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO reses (id, text, created_at, updated_at, user_id, topic_id, history_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            res.id,
            res.text,
            res.created_at,
            res.updated_at,
            res.user_id,
            res.topic_id,
            res.history_id
        )
        .execute(&self.pool)
        .await?;

        // Get count of responses for the topic
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM reses
            WHERE topic_id = $1
            "#,
            res.topic_id
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0);

        // Publish event to Redis
        let pubsub = ResPubSub {
            id: res.id.clone(),
            topic: res.topic_id.clone(),
            count,
        };

        let mut redis = self.redis.get_async_connection().await?;
        redis
            .publish(
                RES_PUBSUB_CHANNEL,
                serde_json::to_string(&pubsub)?,
            )
            .await?;

        Ok(())
    }

    async fn update(&self, res: &Res) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE reses
            SET text = $1, updated_at = $2, history_id = $3
            WHERE id = $4
            "#,
            res.text,
            res.updated_at,
            res.history_id,
            res.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn subscribe_insert_event(
        &self,
        topic_id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<(Res, i64), Box<dyn std::error::Error>>> + Send + Unpin>, Box<dyn std::error::Error>> {
        let mut redis = self.redis.get_async_connection().await?;
        let mut pubsub = redis.as_pubsub();
        pubsub.subscribe(RES_PUBSUB_CHANNEL)?;

        let topic_id = topic_id.to_string();
        let pool = self.pool.clone();

        let stream = async_stream::stream! {
            while let Ok(msg) = pubsub.get_message() {
                if let Ok(payload) = msg.get_payload::<String>() {
                    if let Ok(pubsub) = serde_json::from_str::<ResPubSub>(&payload) {
                        if pubsub.topic == topic_id {
                            if let Ok(Some(res)) = sqlx::query_as!(
                                Res,
                                r#"
                                SELECT id, text, created_at, updated_at, user_id, topic_id, history_id
                                FROM reses
                                WHERE id = $1
                                "#,
                                pubsub.id
                            )
                            .fetch_optional(&pool)
                            .await
                            {
                                yield Ok((res, pubsub.count));
                            }
                        }
                    }
                }
            }
        };

        Ok(Box::new(stream))
    }
} 