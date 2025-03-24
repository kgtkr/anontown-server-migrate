use super::*;
use crate::entities::{Topic, TopicType};
use chrono::Utc;

#[tokio::test]
async fn test_topic_repo_mock() {
    let repo = TopicRepoMock::new();
    let now = Utc::now();

    // Test create and find_by_id
    let topic = Topic {
        id: "test1".to_string(),
        title: "Test Topic".to_string(),
        text: "Test Text".to_string(),
        created_at: now,
        updated_at: now,
        user_id: "user1".to_string(),
        topic_type: TopicType::Normal,
        res_count: 0,
        hash: "hash1".to_string(),
        one: false,
        profile_id: None,
        age: false,
        history_id: None,
        fork_id: None,
    };

    assert!(repo.create(&topic).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.id, "test1");

    // Test find_by_user_id
    let topics = repo.find_by_user_id("user1", 10, 0).await.unwrap();
    assert_eq!(topics.len(), 1);
    assert_eq!(topics[0].id, "test1");

    // Test find_by_hash
    let found = repo.find_by_hash("hash1").await.unwrap().unwrap();
    assert_eq!(found.id, "test1");

    // Test update
    let mut updated = topic.clone();
    updated.title = "Updated Title".to_string();
    assert!(repo.update(&updated).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.title, "Updated Title");

    // Test update_res_count
    assert!(repo.update_res_count("test1", 1).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.res_count, 1);

    // Test update_age
    assert!(repo.update_age("test1", true).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert!(found.age);

    // Test count_by_type
    let count = repo.count_by_type(TopicType::Normal).await.unwrap();
    assert_eq!(count, 1);

    // Test count_by_user_id
    let count = repo.count_by_user_id("user1").await.unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
async fn test_topic_repo() {
    let pool = PgPool::connect("postgres://postgres:postgres@localhost:5432/anontown_test").await.unwrap();
    let repo = TopicRepo::new(pool);
    let now = Utc::now();

    // Test create and find_by_id
    let topic = Topic {
        id: "test1".to_string(),
        title: "Test Topic".to_string(),
        text: "Test Text".to_string(),
        created_at: now,
        updated_at: now,
        user_id: "user1".to_string(),
        topic_type: TopicType::Normal,
        res_count: 0,
        hash: "hash1".to_string(),
        one: false,
        profile_id: None,
        age: false,
        history_id: None,
        fork_id: None,
    };

    assert!(repo.create(&topic).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.id, "test1");

    // Test find_by_user_id
    let topics = repo.find_by_user_id("user1", 10, 0).await.unwrap();
    assert_eq!(topics.len(), 1);
    assert_eq!(topics[0].id, "test1");

    // Test find_by_hash
    let found = repo.find_by_hash("hash1").await.unwrap().unwrap();
    assert_eq!(found.id, "test1");

    // Test update
    let mut updated = topic.clone();
    updated.title = "Updated Title".to_string();
    assert!(repo.update(&updated).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.title, "Updated Title");

    // Test update_res_count
    assert!(repo.update_res_count("test1", 1).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.res_count, 1);

    // Test update_age
    assert!(repo.update_age("test1", true).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert!(found.age);

    // Test count_by_type
    let count = repo.count_by_type(TopicType::Normal).await.unwrap();
    assert_eq!(count, 1);

    // Test count_by_user_id
    let count = repo.count_by_user_id("user1").await.unwrap();
    assert_eq!(count, 1);
} 