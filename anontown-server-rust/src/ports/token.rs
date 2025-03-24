use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::entities::token::Token;

#[async_trait]
pub trait TokenRepoPort: Send + Sync {
    async fn save(&mut self, token: &Token) -> Result<(), Box<dyn std::error::Error>>;
    async fn find_one(&mut self, id: &str) -> Result<Option<Token>, Box<dyn std::error::Error>>;
    async fn find_by_user_id(&mut self, user_id: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>>;
    async fn find_by_client_id(&mut self, client_id: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>>;
}

pub async fn run_token_repo_laws(repo: &mut impl TokenRepoPort) {
    let user_id = "test_user";
    let client_id = "test_client";
    let access_token = "test_access_token";
    let refresh_token = "test_refresh_token";
    let date = Utc::now();
    let expires_at = date + Duration::hours(1);

    // テストデータの作成
    let token = Token {
        id: Uuid::new_v4().to_string(),
        user_id: user_id.to_string(),
        client_id: client_id.to_string(),
        access_token: access_token.to_string(),
        refresh_token: refresh_token.to_string(),
        expires_at,
        date,
    };

    // 保存テスト
    repo.save(&token).await.unwrap();

    // find_oneテスト
    let found = repo.find_one(&token.id).await.unwrap();
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, token.id);
    assert_eq!(found.user_id, user_id);
    assert_eq!(found.client_id, client_id);
    assert_eq!(found.access_token, access_token);
    assert_eq!(found.refresh_token, refresh_token);
    assert_eq!(found.expires_at, expires_at);

    // find_by_user_idテスト
    let found = repo.find_by_user_id(user_id).await.unwrap();
    assert!(!found.is_empty());
    assert_eq!(found[0].user_id, user_id);

    // find_by_client_idテスト
    let found = repo.find_by_client_id(client_id).await.unwrap();
    assert!(!found.is_empty());
    assert_eq!(found[0].client_id, client_id);

    // 存在しないIDのテスト
    let not_found = repo.find_one("non_existent").await.unwrap();
    assert!(not_found.is_none());
}

#[async_trait]
pub trait TokenPort {
    async fn find_one(&mut self, id: &str) -> Result<Token, Box<dyn std::error::Error>>;
    async fn find_all(&mut self) -> Result<Vec<Token>, Box<dyn std::error::Error>>;
    async fn insert(&mut self, token: &Token) -> Result<(), Box<dyn std::error::Error>>;
    async fn update(&mut self, token: &Token) -> Result<(), Box<dyn std::error::Error>>;
    async fn del_client_token(&mut self, client_id: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn del_master_token(&mut self, user_id: &str) -> Result<(), Box<dyn std::error::Error>>;
} 