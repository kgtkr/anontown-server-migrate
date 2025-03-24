use super::*;
use crate::entities::User;
use chrono::Utc;

#[tokio::test]
async fn test_user_repo_mock() {
    let repo = UserRepoMock::new();
    let now = Utc::now();

    // Test create and find_by_id
    let user = User {
        id: "test1".to_string(),
        name: "Test User".to_string(),
        sn: "test1".to_string(),
        created_at: now,
        updated_at: now,
        lv: 1,
        point: 0,
        one: false,
        age: false,
        history_id: None,
    };

    assert!(repo.create(&user).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.id, "test1");

    // Test find_by_sn
    let found = repo.find_by_sn("test1").await.unwrap().unwrap();
    assert_eq!(found.id, "test1");

    // Test update
    let mut updated = user.clone();
    updated.name = "Updated Name".to_string();
    assert!(repo.update(&updated).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.name, "Updated Name");

    // Test update_lv
    assert!(repo.update_lv("test1", 2).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.lv, 2);

    // Test update_point
    assert!(repo.update_point("test1", 100).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.point, 100);

    // Test update_age
    assert!(repo.update_age("test1", true).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert!(found.age);
}

#[tokio::test]
async fn test_user_repo() {
    let pool = PgPool::connect("postgres://postgres:postgres@localhost:5432/anontown_test").await.unwrap();
    let repo = UserRepo::new(pool);
    let now = Utc::now();

    // Test create and find_by_id
    let user = User {
        id: "test1".to_string(),
        name: "Test User".to_string(),
        sn: "test1".to_string(),
        created_at: now,
        updated_at: now,
        lv: 1,
        point: 0,
        one: false,
        age: false,
        history_id: None,
    };

    assert!(repo.create(&user).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.id, "test1");

    // Test find_by_sn
    let found = repo.find_by_sn("test1").await.unwrap().unwrap();
    assert_eq!(found.id, "test1");

    // Test update
    let mut updated = user.clone();
    updated.name = "Updated Name".to_string();
    assert!(repo.update(&updated).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.name, "Updated Name");

    // Test update_lv
    assert!(repo.update_lv("test1", 2).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.lv, 2);

    // Test update_point
    assert!(repo.update_point("test1", 100).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.point, 100);

    // Test update_age
    assert!(repo.update_age("test1", true).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert!(found.age);
} 