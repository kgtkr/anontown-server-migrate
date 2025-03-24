use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;

use crate::entities::User;
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
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, sn, created_at, updated_at, lv, point, one, age, history_id
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_sn(&self, sn: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, sn, created_at, updated_at, lv, point, one, age, history_id
            FROM users
            WHERE sn = $1
            "#,
            sn
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, name, sn, created_at, updated_at, lv, point, one, age, history_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            user.id,
            user.name,
            user.sn,
            user.created_at,
            user.updated_at,
            user.lv,
            user.point,
            user.one,
            user.age,
            user.history_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE users
            SET name = $1, sn = $2, updated_at = $3, lv = $4,
                point = $5, one = $6, age = $7, history_id = $8
            WHERE id = $9
            "#,
            user.name,
            user.sn,
            user.updated_at,
            user.lv,
            user.point,
            user.one,
            user.age,
            user.history_id,
            user.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_lv(&self, id: &str, lv: i64) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE users
            SET lv = $1, updated_at = $2
            WHERE id = $3
            "#,
            lv,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_point(&self, id: &str, point: i64) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE users
            SET point = $1, updated_at = $2
            WHERE id = $3
            "#,
            point,
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
            UPDATE users
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
} 