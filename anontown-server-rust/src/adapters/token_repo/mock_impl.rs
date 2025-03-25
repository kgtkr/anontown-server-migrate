use async_trait::async_trait;
use std::collections::HashMap;
use crate::{AuthTokenMaster, AuthUser, Token, AtError, AtErrorKind, AtResult};
use crate::ports::TokenRepo;
use super::model::TokenRepoModel;

pub struct TokenRepoMockImpl {
    tokens: HashMap<String, TokenRepoModel>,
}

impl TokenRepoMockImpl {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }
}

#[async_trait]
impl TokenRepo for TokenRepoMockImpl {
    async fn find_one(&self, id: &str) -> AtResult<Token> {
        let model = self
            .tokens
            .get(id)
            .cloned()
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
        let tokens = self
            .tokens
            .values()
            .filter(|token| token.user_id == auth_token.user)
            .cloned()
            .map(|model| Token {
                id: model.id,
                user_id: model.user_id,
                client_id: model.client_id,
                access_token: model.access_token,
                refresh_token: model.refresh_token,
                expires_at: model.expires_at,
                date: model.date,
            })
            .collect();

        Ok(tokens)
    }

    async fn insert(&self, token: &Token) -> AtResult<()> {
        let model = TokenRepoModel {
            id: token.id.clone(),
            user_id: token.user_id.clone(),
            client_id: token.client_id.clone(),
            access_token: token.access_token.clone(),
            refresh_token: token.refresh_token.clone(),
            expires_at: token.expires_at,
            date: token.date,
        };
        self.tokens.insert(token.id.clone(), model);
        Ok(())
    }

    async fn update(&self, token: &Token) -> AtResult<()> {
        if !self.tokens.contains_key(&token.id) {
            return Err(AtError::new(AtErrorKind::NotFound, "トークンが存在しません"));
        }

        let model = TokenRepoModel {
            id: token.id.clone(),
            user_id: token.user_id.clone(),
            client_id: token.client_id.clone(),
            access_token: token.access_token.clone(),
            refresh_token: token.refresh_token.clone(),
            expires_at: token.expires_at,
            date: token.date,
        };
        self.tokens.insert(token.id.clone(), model);
        Ok(())
    }

    async fn del_client_token(&self, token: &AuthTokenMaster, client_id: &str) -> AtResult<()> {
        self.tokens.retain(|_, t| {
            !(t.user_id == token.user && t.client_id == client_id)
        });
        Ok(())
    }

    async fn del_master_token(&self, user: &AuthUser) -> AtResult<()> {
        self.tokens.retain(|_, t| t.user_id != user.id);
        Ok(())
    }
} 