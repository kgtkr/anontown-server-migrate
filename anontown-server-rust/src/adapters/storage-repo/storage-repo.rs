use async_trait::async_trait;
use sqlx::PgPool;

use crate::entities::Storage;
use crate::ports::storage::StoragePort;

pub struct StorageRepo {
    pool: PgPool,
}

impl StorageRepo {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl StoragePort for StorageRepo {
    async fn find_by_id(&self, id: &str) -> Result<Option<Storage>, Box<dyn std::error::Error>> {
        let storage = sqlx::query_as!(
            Storage,
            r#"
            SELECT id, user_id, name, description, created_at, updated_at
            FROM storages
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(storage)
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Option<Storage>, Box<dyn std::error::Error>> {
        let storage = sqlx::query_as!(
            Storage,
            r#"
            SELECT id, user_id, name, description, created_at, updated_at
            FROM storages
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(storage)
    }

    async fn create(&self, storage: &Storage) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO storages (id, user_id, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            storage.id,
            storage.user_id,
            storage.name,
            storage.description,
            storage.created_at,
            storage.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, storage: &Storage) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE storages
            SET name = $1, description = $2, updated_at = $3
            WHERE id = $4
            "#,
            storage.name,
            storage.description,
            storage.updated_at,
            storage.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
} 