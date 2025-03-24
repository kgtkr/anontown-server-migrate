use chrono::Utc;
use tokio;

use crate::adapters::profile_repo::profile_repo::ProfileRepo;
use crate::adapters::profile_repo::profile_repo_mock::ProfileRepoMock;
use crate::entities::Profile;

#[tokio::test]
async fn test_profile_repo_mock() {
    let repo = ProfileRepoMock::new();
    let profile = Profile {
        id: "profile1".to_string(),
        user_id: "user1".to_string(),
        name: "Test Profile".to_string(),
        description: "Test Description".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test create
    repo.create(&profile).await.unwrap();

    // Test find_by_id
    let found = repo.find_by_id(&profile.id).await.unwrap().unwrap();
    assert_eq!(found.id, profile.id);

    // Test find_by_user_id
    let found = repo.find_by_user_id(&profile.user_id).await.unwrap().unwrap();
    assert_eq!(found.id, profile.id);

    // Test update
    let mut updated_profile = profile.clone();
    updated_profile.name = "Updated Profile".to_string();
    repo.update(&updated_profile).await.unwrap();

    let found = repo.find_by_id(&profile.id).await.unwrap().unwrap();
    assert_eq!(found.name, updated_profile.name);
}

#[tokio::test]
async fn test_profile_repo() {
    let repo = ProfileRepo::new().await.unwrap();
    let profile = Profile {
        id: "profile1".to_string(),
        user_id: "user1".to_string(),
        name: "Test Profile".to_string(),
        description: "Test Description".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test create
    repo.create(&profile).await.unwrap();

    // Test find_by_id
    let found = repo.find_by_id(&profile.id).await.unwrap().unwrap();
    assert_eq!(found.id, profile.id);

    // Test find_by_user_id
    let found = repo.find_by_user_id(&profile.user_id).await.unwrap().unwrap();
    assert_eq!(found.id, profile.id);

    // Test update
    let mut updated_profile = profile.clone();
    updated_profile.name = "Updated Profile".to_string();
    repo.update(&updated_profile).await.unwrap();

    let found = repo.find_by_id(&profile.id).await.unwrap().unwrap();
    assert_eq!(found.name, updated_profile.name);
} 