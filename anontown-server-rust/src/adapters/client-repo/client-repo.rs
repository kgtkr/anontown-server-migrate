use async_trait::async_trait;
use sqlx::PgPool;

use super::client_model::ClientModel;
use crate::ports::client::{Client, ClientRepoPort, run_client_repo_laws};

pub struct ClientRepo {
    pool: PgPool,
}

impl ClientRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ClientRepoPort for ClientRepo {
    async fn save(&mut self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientModel {
            id: client.id.clone(),
            name: client.name.clone(),
            description: client.description.clone(),
            date: client.date,
        };

        sqlx::query_as!(
            ClientModel,
            r#"
            INSERT INTO clients (id, name, description, date)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE
            SET name = $2, description = $3, date = $4
            "#,
            client.id,
            client.name,
            client.description,
            client.date
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_one(&mut self, id: &str) -> Result<Option<Client>, Box<dyn std::error::Error>> {
        let client = sqlx::query_as!(
            ClientModel,
            r#"
            SELECT id, name, description, date
            FROM clients
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(client.map(|c| Client {
            id: c.id,
            name: c.name,
            description: c.description,
            date: c.date,
        }))
    }

    async fn find(&mut self) -> Result<Vec<Client>, Box<dyn std::error::Error>> {
        let clients = sqlx::query_as!(
            ClientModel,
            r#"
            SELECT id, name, description, date
            FROM clients
            ORDER BY date DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(clients
            .into_iter()
            .map(|c| Client {
                id: c.id,
                name: c.name,
                description: c.description,
                date: c.date,
            })
            .collect())
    }
}

#[sqlx::test]
async fn test_client_repo(pool: PgPool) {
    let mut repo = ClientRepo::new(pool);
    run_client_repo_laws(&mut repo).await;
} 