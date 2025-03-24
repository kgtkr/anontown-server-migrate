use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::entities::storage::Storage;

#[async_trait]
pub trait StorageRepoPort: Send + Sync {
    async fn save(&mut self, storage: &Storage) -> Result<(), Box<dyn std::error::Error>>;
    async fn find_one_key(
        &mut self,
        user_id: &str,
        client_id: Option<&str>,
        key: &str,
    ) -> Result<Option<Storage>, Box<dyn std::error::Error>>;
    async fn find(
        &mut self,
        user_id: &str,
        client_id: Option<&str>,
        keys: &[String],
    ) -> Result<Vec<Storage>, Box<dyn std::error::Error>>;
}

pub async fn run_storage_repo_laws(repo: &mut impl StorageRepoPort) {
    let user_id = "test_user";
    let client_id = Some("test_client");
    let key = "test_key";
    let value = "test_value";
    let date = Utc::now();

    // テストデータの作成
    let storage = Storage {
        id: Uuid::new_v4().to_string(),
        client_id: client_id.map(|s| s.to_string()),
        user_id: user_id.to_string(),
        key: key.to_string(),
        value: value.to_string(),
        date,
    };

    // 保存テスト
    repo.save(&storage).await.unwrap();

    // find_one_keyテスト
    let found = repo.find_one_key(user_id, client_id, key).await.unwrap();
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.user_id, user_id);
    assert_eq!(found.client_id, client_id);
    assert_eq!(found.key, key);
    assert_eq!(found.value, value);

    // findテスト
    let found = repo.find(user_id, client_id, &[]).await.unwrap();
    assert!(!found.is_empty());
    let found = repo.find(user_id, client_id, &[key.to_string()]).await.unwrap();
    assert!(!found.is_empty());
    assert_eq!(found[0].key, key);

    // 存在しないキーのテスト
    let not_found = repo.find_one_key(user_id, client_id, "non_existent").await.unwrap();
    assert!(not_found.is_none());

    // 異なるクライアントIDのテスト
    let different_client = repo.find_one_key(user_id, Some("different_client"), key).await.unwrap();
    assert!(different_client.is_none());
}

pub struct StorageQuery {
    pub key: Option<Vec<String>>,
    pub key_prefix: Option<String>,
} 