use chrono::Utc;
use tokio;

use crate::adapters::storage_repo::storage_repo::StorageRepo;
use crate::adapters::storage_repo::storage_repo_mock::StorageRepoMock;
use crate::entities::Storage;

#[tokio::test]
async fn test_storage_repo_mock() {
    let repo = StorageRepoMock::new();
    let storage = Storage {
        id: "storage1".to_string(),
        user_id: "user1".to_string(),
        name: "Test Storage".to_string(),
        description: "Test Description".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test create
    repo.create(&storage).await.unwrap();

    // Test find_by_id
    let found = repo.find_by_id(&storage.id).await.unwrap().unwrap();
    assert_eq!(found.id, storage.id);

    // Test find_by_user_id
    let found = repo.find_by_user_id(&storage.user_id).await.unwrap().unwrap();
    assert_eq!(found.id, storage.id);

    // Test update
    let mut updated_storage = storage.clone();
    updated_storage.name = "Updated Storage".to_string();
    repo.update(&updated_storage).await.unwrap();

    let found = repo.find_by_id(&storage.id).await.unwrap().unwrap();
    assert_eq!(found.name, updated_storage.name);
}

#[tokio::test]
async fn test_storage_repo() {
    let repo = StorageRepo::new().await.unwrap();
    let storage = Storage {
        id: "storage1".to_string(),
        user_id: "user1".to_string(),
        name: "Test Storage".to_string(),
        description: "Test Description".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test create
    repo.create(&storage).await.unwrap();

    // Test find_by_id
    let found = repo.find_by_id(&storage.id).await.unwrap().unwrap();
    assert_eq!(found.id, storage.id);

    // Test find_by_user_id
    let found = repo.find_by_user_id(&storage.user_id).await.unwrap().unwrap();
    assert_eq!(found.id, storage.id);

    // Test update
    let mut updated_storage = storage.clone();
    updated_storage.name = "Updated Storage".to_string();
    repo.update(&updated_storage).await.unwrap();

    let found = repo.find_by_id(&storage.id).await.unwrap().unwrap();
    assert_eq!(found.name, updated_storage.name);
} 