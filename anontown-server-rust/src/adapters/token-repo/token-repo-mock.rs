use async_trait::async_trait;
use std::collections::HashMap;
use crate::models::Token;
use crate::ports::token::TokenPort;

pub struct TokenRepoMock {
    tokens: HashMap<String, Token>,
}

impl TokenRepoMock {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }
}

#[async_trait]
impl TokenPort for TokenRepoMock {
    async fn find_one(&mut self, id: &str) -> Result<Token, Box<dyn std::error::Error>> {
        self.tokens
            .get(id)
            .cloned()
            .ok_or_else(|| "Token not found".into())
    }

    async fn find_all(&mut self) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        let mut tokens = self.tokens.values().cloned().collect::<Vec<_>>();
        tokens.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(tokens)
    }

    async fn insert(&mut self, token: &Token) -> Result<(), Box<dyn std::error::Error>> {
        self.tokens.insert(token.id.clone(), token.clone());
        Ok(())
    }

    async fn update(&mut self, token: &Token) -> Result<(), Box<dyn std::error::Error>> {
        if self.tokens.contains_key(&token.id) {
            self.tokens.insert(token.id.clone(), token.clone());
            Ok(())
        } else {
            Err("Token not found".into())
        }
    }

    async fn del_client_token(&mut self, client_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.tokens.retain(|_, t| t.client_id.as_ref() != Some(client_id));
        Ok(())
    }

    async fn del_master_token(&mut self, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.tokens.retain(|_, t| t.user_id != user_id || t.client_id.is_some());
        Ok(())
    }
} 