use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use crate::entities::History;
use crate::ports::history::{HistoryPort, HistoryQuery, DateQuery};

pub struct HistoryRepo {
    pool: PgPool,
}

impl HistoryRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HistoryPort for HistoryRepo {
    async fn insert(&mut self, history: &History) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO histories (id, topic_id, title, description, created_at, hash, user_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            history.id,
            history.topic_id,
            history.title,
            history.description,
            history.created_at,
            history.hash,
            history.user_id
        )
        .execute(&self.pool)
        .await?;

        // タグの挿入
        for (i, tag) in history.tags.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO history_tags (history_id, "order", tag)
                VALUES ($1, $2, $3)
                "#,
                history.id,
                i as i32,
                tag
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    async fn update(&mut self, history: &History) -> Result<(), Box<dyn std::error::Error>> {
        // 既存のタグを削除
        sqlx::query!(
            r#"
            DELETE FROM history_tags
            WHERE history_id = $1
            "#,
            history.id
        )
        .execute(&self.pool)
        .await?;

        // 履歴を更新
        sqlx::query!(
            r#"
            UPDATE histories
            SET topic_id = $1, title = $2, description = $3, created_at = $4, hash = $5, user_id = $6
            WHERE id = $7
            "#,
            history.topic_id,
            history.title,
            history.description,
            history.created_at,
            history.hash,
            history.user_id,
            history.id
        )
        .execute(&self.pool)
        .await?;

        // 新しいタグを挿入
        for (i, tag) in history.tags.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO history_tags (history_id, "order", tag)
                VALUES ($1, $2, $3)
                "#,
                history.id,
                i as i32,
                tag
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    async fn find_one(&mut self, id: &str) -> Result<History, Box<dyn std::error::Error>> {
        let history = sqlx::query_as!(
            History,
            r#"
            SELECT h.id, h.topic_id, h.title, h.description, h.created_at, h.hash, h.user_id,
                   array_agg(ht.tag ORDER BY ht.order) as tags
            FROM histories h
            LEFT JOIN history_tags ht ON h.id = ht.history_id
            WHERE h.id = $1
            GROUP BY h.id, h.topic_id, h.title, h.description, h.created_at, h.hash, h.user_id
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(history)
    }

    async fn find(&mut self, query: &HistoryQuery, limit: i32) -> Result<Vec<History>, Box<dyn std::error::Error>> {
        let mut conditions = Vec::new();
        let mut params = Vec::new();

        if let Some(ids) = &query.id {
            conditions.push("h.id = ANY($1)");
            params.push(ids);
        }

        if let Some(topic_ids) = &query.topic {
            conditions.push("h.topic_id = ANY($2)");
            params.push(topic_ids);
        }

        if let Some(date_query) = &query.date {
            match date_query {
                DateQuery::Gt(date) => {
                    conditions.push("h.created_at > $3");
                    params.push(date);
                }
                DateQuery::Gte(date) => {
                    conditions.push("h.created_at >= $3");
                    params.push(date);
                }
                DateQuery::Lt(date) => {
                    conditions.push("h.created_at < $3");
                    params.push(date);
                }
                DateQuery::Lte(date) => {
                    conditions.push("h.created_at <= $3");
                    params.push(date);
                }
            }
        }

        let where_clause = if conditions.is_empty() {
            "".to_string()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let order_by = if let Some(date_query) = &query.date {
            match date_query {
                DateQuery::Gt(_) | DateQuery::Gte(_) => "ORDER BY h.created_at ASC",
                DateQuery::Lt(_) | DateQuery::Lte(_) => "ORDER BY h.created_at DESC",
            }
        } else {
            "ORDER BY h.created_at DESC"
        };

        let query = format!(
            r#"
            SELECT h.id, h.topic_id, h.title, h.description, h.created_at, h.hash, h.user_id,
                   array_agg(ht.tag ORDER BY ht.order) as tags
            FROM histories h
            LEFT JOIN history_tags ht ON h.id = ht.history_id
            {}
            GROUP BY h.id, h.topic_id, h.title, h.description, h.created_at, h.hash, h.user_id
            {}
            LIMIT $4
            "#,
            where_clause, order_by
        );

        let histories = sqlx::query_as!(
            History,
            &query,
            params[0] as Vec<String>,
            params[1] as Vec<String>,
            params[2] as DateTime<Utc>,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(histories)
    }
} 