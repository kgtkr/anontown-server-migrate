use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::entities::client::Client;

#[async_trait]
pub trait ClientRepoPort: Send + Sync {
    async fn save(&mut self, client: &Client) -> Result<(), Box<dyn std::error::Error>>;
    async fn find_one(&mut self, id: &str) -> Result<Option<Client>, Box<dyn std::error::Error>>;
    async fn find(&mut self, ids: &[String]) -> Result<Vec<Client>, Box<dyn std::error::Error>>;
}

pub async fn run_client_repo_laws(repo: &mut impl ClientRepoPort) {
    let name = "Test Client";
    let description = "Test Description";
    let date = Utc::now();

    // テストデータの作成
    let client = Client {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        description: description.to_string(),
        date,
    };

    // 保存テスト
    repo.save(&client).await.unwrap();

    // find_oneテスト
    let found = repo.find_one(&client.id).await.unwrap();
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, client.id);
    assert_eq!(found.name, name);
    assert_eq!(found.description, description);

    // findテスト
    let found = repo.find(&[]).await.unwrap();
    assert!(!found.is_empty());
    let found = repo.find(&[client.id.clone()]).await.unwrap();
    assert!(!found.is_empty());
    assert_eq!(found[0].id, client.id);

    // 存在しないIDのテスト
    let not_found = repo.find_one("non_existent").await.unwrap();
    assert!(not_found.is_none());
}

pub struct ClientQuery {
    pub id: Option<Vec<String>>,
    pub is_self: Option<bool>,
} 