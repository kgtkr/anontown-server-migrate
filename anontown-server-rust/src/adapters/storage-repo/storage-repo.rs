use async_trait::async_trait;
use sqlx::PgPool;

use super::storage_model::StorageModel;
use crate::ports::storage::{Storage, StorageRepoPort, run_storage_repo_laws};

pub struct StorageRepo {
    pool: PgPool,
}

impl StorageRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StorageRepoPort for StorageRepo {
    async fn save(&mut self, storage: &Storage) -> Result<(), Box<dyn std::error::Error>> {
        let storage = StorageModel {
            id: storage.id.clone(),
            client_id: storage.client_id.clone(),
            user_id: storage.user_id.clone(),
            key: storage.key.clone(),
            value: storage.value.clone(),
            date: storage.date,
        };

        sqlx::query_as!(
            StorageModel,
            r#"
            INSERT INTO storages (id, client_id, user_id, key, value, date)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE
            SET client_id = $2, user_id = $3, key = $4, value = $5, date = $6
            "#,
            storage.id,
            storage.client_id,
            storage.user_id,
            storage.key,
            storage.value,
            storage.date
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_one_key(
        &mut self,
        user_id: &str,
        client_id: Option<&str>,
        key: &str,
    ) -> Result<Option<Storage>, Box<dyn std::error::Error>> {
        let storage = sqlx::query_as!(
            StorageModel,
            r#"
            SELECT id, client_id, user_id, key, value, date
            FROM storages
            WHERE user_id = $1 AND key = $2
            "#,
            user_id,
            key
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(storage.map(|s| Storage {
            id: s.id,
            client_id: s.client_id,
            user_id: s.user_id,
            key: s.key,
            value: s.value,
            date: s.date,
        }))
    }

    async fn find(
        &mut self,
        user_id: &str,
        client_id: Option<&str>,
        keys: &[String],
    ) -> Result<Vec<Storage>, Box<dyn std::error::Error>> {
        let mut query = sqlx::query_as!(
            StorageModel,
            r#"
            SELECT id, client_id, user_id, key, value, date
            FROM storages
            WHERE user_id = $1
            ORDER BY date DESC
            "#,
            user_id
        );

        if let Some(client_id) = client_id {
            query = sqlx::query_as!(
                StorageModel,
                r#"
                SELECT id, client_id, user_id, key, value, date
                FROM storages
                WHERE user_id = $1 AND client_id = $2
                ORDER BY date DESC
                "#,
                user_id,
                client_id
            );
        } else {
            query = sqlx::query_as!(
                StorageModel,
                r#"
                SELECT id, client_id, user_id, key, value, date
                FROM storages
                WHERE user_id = $1 AND client_id IS NULL
                ORDER BY date DESC
                "#,
                user_id
            );
        }

        let mut storages = query.fetch_all(&self.pool).await?;

        if !keys.is_empty() {
            storages.retain(|s| keys.contains(&s.key));
        }

        Ok(storages
            .into_iter()
            .map(|s| Storage {
                id: s.id,
                client_id: s.client_id,
                user_id: s.user_id,
                key: s.key,
                value: s.value,
                date: s.date,
            })
            .collect())
    }
}

#[sqlx::test]
async fn test_storage_repo(pool: PgPool) {
    let mut repo = StorageRepo::new(pool);
    run_storage_repo_laws(&mut repo).await;
} 