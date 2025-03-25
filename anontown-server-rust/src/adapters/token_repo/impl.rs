use async_trait::async_trait;
use sqlx::PgPool;
use crate::{AuthTokenMaster, AuthUser, Token, AtError, AtErrorKind, AtResult};
use crate::ports::TokenRepo;
use super::model::TokenRepoModel;

pub struct TokenRepoImpl {
    pool: PgPool,
}

impl TokenRepoImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TokenRepo for TokenRepoImpl {
    async fn find_one(&self, id: &str) -> AtResult<Token> {
        let model = sqlx::query_as!(
            TokenRepoModel,
            r#"
            SELECT id, user_id, client_id, access_token, refresh_token, expires_at, date
            FROM tokens
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AtError::new(AtErrorKind::NotFound, "トークンが存在しません"))?;

        Ok(Token {
            id: model.id,
            user_id: model.user_id,
            client_id: model.client_id,
            access_token: model.access_token,
            refresh_token: model.refresh_token,
            expires_at: model.expires_at,
            date: model.date,
        })
    }

    async fn find_all(&self, auth_token: &AuthTokenMaster) -> AtResult<Vec<Token>> {
        let models = sqlx::query_as!(
            TokenRepoModel,
            r#"
            SELECT id, user_id, client_id, access_token, refresh_token, expires_at, date
            FROM tokens
            WHERE user_id = $1
            "#,
            auth_token.user
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(models
            .into_iter()
            .map(|model| Token {
                id: model.id,
                user_id: model.user_id,
                client_id: model.client_id,
                access_token: model.access_token,
                refresh_token: model.refresh_token,
                expires_at: model.expires_at,
                date: model.date,
            })
            .collect())
    }

    async fn insert(&self, token: &Token) -> AtResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO tokens (id, user_id, client_id, access_token, refresh_token, expires_at, date)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
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

    async fn update(&self, token: &Token) -> AtResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE tokens
            SET access_token = $1,
                refresh_token = $2,
                expires_at = $3
            WHERE id = $4
            "#,
            token.access_token,
            token.refresh_token,
            token.expires_at,
            token.id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AtError::new(AtErrorKind::NotFound, "トークンが存在しません"));
        }

        Ok(())
    }

    async fn del_client_token(&self, token: &AuthTokenMaster, client_id: &str) -> AtResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM tokens
            WHERE user_id = $1 AND type = 'general' AND client_id = $2
            "#,
            token.user,
            client_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn del_master_token(&self, user: &AuthUser) -> AtResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM tokens
            WHERE user_id = $1 AND type = 'master'
            "#,
            user.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
} 