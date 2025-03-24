use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use crate::models::User;
use crate::ports::user::UserPort;

pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserPort for UserRepo {
    async fn find_by_id(&mut self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, screen_name, encrypted_password, lv, res_last_created_at,
                   count_created_res_m10, count_created_res_m30, count_created_res_h1,
                   count_created_res_h6, count_created_res_h12, count_created_res_d1,
                   topic_last_created_at, created_at, point, one_topic_last_created_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_screen_name(&mut self, screen_name: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, screen_name, encrypted_password, lv, res_last_created_at,
                   count_created_res_m10, count_created_res_m30, count_created_res_h1,
                   count_created_res_h6, count_created_res_h12, count_created_res_d1,
                   topic_last_created_at, created_at, point, one_topic_last_created_at
            FROM users
            WHERE screen_name = $1
            "#,
            screen_name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create(&mut self, user: &User) -> Result<User, Box<dyn std::error::Error>> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                id, screen_name, encrypted_password, lv, res_last_created_at,
                count_created_res_m10, count_created_res_m30, count_created_res_h1,
                count_created_res_h6, count_created_res_h12, count_created_res_d1,
                topic_last_created_at, created_at, point, one_topic_last_created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#,
            user.id,
            user.screen_name,
            user.encrypted_password,
            user.lv,
            user.res_last_created_at,
            user.count_created_res_m10,
            user.count_created_res_m30,
            user.count_created_res_h1,
            user.count_created_res_h6,
            user.count_created_res_h12,
            user.count_created_res_d1,
            user.topic_last_created_at,
            user.created_at,
            user.point,
            user.one_topic_last_created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update(&mut self, user: &User) -> Result<User, Box<dyn std::error::Error>> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET screen_name = $1, encrypted_password = $2, lv = $3,
                res_last_created_at = $4, count_created_res_m10 = $5,
                count_created_res_m30 = $6, count_created_res_h1 = $7,
                count_created_res_h6 = $8, count_created_res_h12 = $9,
                count_created_res_d1 = $10, topic_last_created_at = $11,
                created_at = $12, point = $13, one_topic_last_created_at = $14
            WHERE id = $15
            RETURNING *
            "#,
            user.screen_name,
            user.encrypted_password,
            user.lv,
            user.res_last_created_at,
            user.count_created_res_m10,
            user.count_created_res_m30,
            user.count_created_res_h1,
            user.count_created_res_h6,
            user.count_created_res_h12,
            user.count_created_res_d1,
            user.topic_last_created_at,
            user.created_at,
            user.point,
            user.one_topic_last_created_at,
            user.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update_res_count(&mut self, id: &str, count: i32) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE users
            SET count_created_res_m10 = count_created_res_m10 + $1,
                count_created_res_m30 = count_created_res_m30 + $1,
                count_created_res_h1 = count_created_res_h1 + $1,
                count_created_res_h6 = count_created_res_h6 + $1,
                count_created_res_h12 = count_created_res_h12 + $1,
                count_created_res_d1 = count_created_res_d1 + $1
            WHERE id = $2
            "#,
            count,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_topic_count(&mut self, id: &str, count: i32) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE users
            SET topic_last_created_at = NOW()
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_point(&mut self, id: &str, point: i32) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE users
            SET point = point + $1
            WHERE id = $2
            "#,
            point,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_res_last_created_at(&mut self, id: &str, created_at: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE users
            SET res_last_created_at = $1
            WHERE id = $2
            "#,
            created_at,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_topic_last_created_at(&mut self, id: &str, created_at: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE users
            SET topic_last_created_at = $1
            WHERE id = $2
            "#,
            created_at,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_one_topic_last_created_at(&mut self, id: &str, created_at: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE users
            SET one_topic_last_created_at = $1
            WHERE id = $2
            "#,
            created_at,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
} 