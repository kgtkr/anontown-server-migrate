use async_trait::async_trait;
use sqlx::PgPool;

use super::token_model::TokenModel;
use crate::ports::token::{Token, TokenRepoPort, run_token_repo_laws};

pub struct TokenRepo {
    pool: PgPool,
}

impl TokenRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TokenRepoPort for TokenRepo {
    async fn save(&mut self, token: &Token) -> Result<(), Box<dyn std::error::Error>> {
        let token = TokenModel {
            id: token.id.clone(),
            user_id: token.user_id.clone(),
            client_id: token.client_id.clone(),
            access_token: token.access_token.clone(),
            refresh_token: token.refresh_token.clone(),
            expires_at: token.expires_at,
            date: token.date,
        };

        sqlx::query_as!(
            TokenModel,
            r#"
            INSERT INTO tokens (id, user_id, client_id, access_token, refresh_token, expires_at, date)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE
            SET user_id = $2, client_id = $3, access_token = $4, refresh_token = $5, expires_at = $6, date = $7
            "#,
            token.id,
            token.user_id,
            token.client_id,
            token.access_token,
            token.refresh_token,
            token.expires_at,
            token.date
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_one(&mut self, id: &str) -> Result<Option<Token>, Box<dyn std::error::Error>> {
        let token = sqlx::query_as!(
            TokenModel,
            r#"
            SELECT id, user_id, client_id, access_token, refresh_token, expires_at, date
            FROM tokens
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(token.map(|t| Token {
            id: t.id,
            user_id: t.user_id,
            client_id: t.client_id,
            access_token: t.access_token,
            refresh_token: t.refresh_token,
            expires_at: t.expires_at,
            date: t.date,
        }))
    }

    async fn find(&mut self, user_id: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        let tokens = sqlx::query_as!(
            TokenModel,
            r#"
            SELECT id, user_id, client_id, access_token, refresh_token, expires_at, date
            FROM tokens
            WHERE user_id = $1
            ORDER BY date DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tokens
            .into_iter()
            .map(|t| Token {
                id: t.id,
                user_id: t.user_id,
                client_id: t.client_id,
                access_token: t.access_token,
                refresh_token: t.refresh_token,
                expires_at: t.expires_at,
                date: t.date,
            })
            .collect())
    }
}

#[sqlx::test]
async fn test_token_repo(pool: PgPool) {
    let mut repo = TokenRepo::new(pool);
    run_token_repo_laws(&mut repo).await;
} 