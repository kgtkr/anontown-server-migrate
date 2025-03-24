use async_trait::async_trait;
use std::collections::HashMap;
use tokio::test;

use crate::models::storage::Storage;
use crate::ports::storage::StorageRepoPort;
use super::laws::run_storage_repo_laws;

pub struct MockStorageRepo {
    storages: HashMap<String, Storage>,
}

impl MockStorageRepo {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }
}

#[async_trait]
impl StorageRepoPort for MockStorageRepo {
    async fn save(&mut self, storage: &Storage) -> Result<(), Box<dyn std::error::Error>> {
        self.storages.insert(storage.id.clone(), storage.clone());
        Ok(())
    }

    async fn find_one_key(
        &mut self,
        user_id: &str,
        client_id: Option<&str>,
        key: &str,
    ) -> Result<Option<Storage>, Box<dyn std::error::Error>> {
        for storage in self.storages.values() {
            if storage.user_id == user_id
                && storage.client_id.as_deref() == client_id
                && storage.key == key
            {
                return Ok(Some(storage.clone()));
            }
        }
        Ok(None)
    }

    async fn find(
        &mut self,
        user_id: &str,
        client_id: Option<&str>,
        keys: &[String],
    ) -> Result<Vec<Storage>, Box<dyn std::error::Error>> {
        let mut result = Vec::new();
        for storage in self.storages.values() {
            if storage.user_id == user_id
                && storage.client_id.as_deref() == client_id
                && (keys.is_empty() || keys.contains(&storage.key))
            {
                result.push(storage.clone());
            }
        }
        Ok(result)
    }
}

#[test]
async fn test_mock_storage_repo() {
    let mut repo = MockStorageRepo::new();
    run_storage_repo_laws(&mut repo).await;
} 