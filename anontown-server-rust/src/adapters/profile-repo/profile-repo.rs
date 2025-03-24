use async_trait::async_trait;
use sqlx::PgPool;

use crate::entities::Profile;
use crate::ports::profile::ProfilePort;

pub struct ProfileRepo {
    pool: PgPool,
}

impl ProfileRepo {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl ProfilePort for ProfileRepo {
    async fn find_by_id(&self, id: &str) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
        let profile = sqlx::query_as!(
            Profile,
            r#"
            SELECT id, user_id, name, description, created_at, updated_at
            FROM profiles
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(profile)
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
        let profile = sqlx::query_as!(
            Profile,
            r#"
            SELECT id, user_id, name, description, created_at, updated_at
            FROM profiles
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(profile)
    }

    async fn create(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO profiles (id, user_id, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            profile.id,
            profile.user_id,
            profile.name,
            profile.description,
            profile.created_at,
            profile.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE profiles
            SET name = $1, description = $2, updated_at = $3
            WHERE id = $4
            "#,
            profile.name,
            profile.description,
            profile.updated_at,
            profile.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
} 