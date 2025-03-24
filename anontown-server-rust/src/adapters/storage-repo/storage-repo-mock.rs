use async_trait::async_trait;
use std::collections::HashMap;
use tokio::test;

use crate::entities::Storage;
use crate::ports::storage::StoragePort;
use super::laws::run_storage_repo_laws;

pub struct StorageRepoMock {
    storages: HashMap<String, Storage>,
}

impl StorageRepoMock {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }
}

#[async_trait]
impl StoragePort for StorageRepoMock {
    async fn find_by_id(&self, id: &str) -> Result<Option<Storage>, Box<dyn std::error::Error>> {
        Ok(self.storages.get(id).cloned())
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Option<Storage>, Box<dyn std::error::Error>> {
        Ok(self.storages.values().find(|s| s.user_id == user_id).cloned())
    }

    async fn create(&self, storage: &Storage) -> Result<(), Box<dyn std::error::Error>> {
        self.storages.insert(storage.id.clone(), storage.clone());
        Ok(())
    }

    async fn update(&self, storage: &Storage) -> Result<(), Box<dyn std::error::Error>> {
        if self.storages.contains_key(&storage.id) {
            self.storages.insert(storage.id.clone(), storage.clone());
            Ok(())
        } else {
            Err("Storage not found".into())
        }
    }
}

#[test]
async fn test_mock_storage_repo() {
    let mut repo = StorageRepoMock::new();
    run_storage_repo_laws(&mut repo).await;
} 