use super::*;
use crate::entities::{Res, ResType, ResDeleteFlag, ResNormal, ResHistory, ResTopic, ResFork};
use chrono::Utc;
use redis::Client;

#[tokio::test]
async fn test_res_repo_mock() {
    let repo = ResRepoMock::new();
    let now = Utc::now();

    // Test create and find_by_id
    let res = Res {
        id: "test1".to_string(),
        text: "Test Response".to_string(),
        created_at: now,
        updated_at: now,
        user_id: "user1".to_string(),
        topic_id: "topic1".to_string(),
        history_id: None,
    };

    assert!(repo.create(&res).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.id, "test1");

    // Test find_by_topic_id
    let reses = repo.find_by_topic_id("topic1", 10, 0).await.unwrap();
    assert_eq!(reses.len(), 1);
    assert_eq!(reses[0].id, "test1");

    // Test find_by_user_id
    let reses = repo.find_by_user_id("user1", 10, 0).await.unwrap();
    assert_eq!(reses.len(), 1);
    assert_eq!(reses[0].id, "test1");

    // Test update
    let mut updated = res.clone();
    updated.text = "Updated Text".to_string();
    assert!(repo.update(&updated).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.text, "Updated Text");

    // Test subscribe_insert_event
    let mut stream = repo.subscribe_insert_event("topic1").await.unwrap();
    assert!(stream.next().await.is_none());
}

#[tokio::test]
async fn test_res_repo() {
    let pool = PgPool::connect("postgres://postgres:postgres@localhost:5432/anontown_test").await.unwrap();
    let redis = Client::open("redis://127.0.0.1/").unwrap();
    let redis = std::sync::Arc::new(redis);
    let repo = ResRepo::new(pool, redis);
    let now = Utc::now();

    // Test create and find_by_id
    let res = Res {
        id: "test1".to_string(),
        text: "Test Response".to_string(),
        created_at: now,
        updated_at: now,
        user_id: "user1".to_string(),
        topic_id: "topic1".to_string(),
        history_id: None,
    };

    assert!(repo.create(&res).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.id, "test1");

    // Test find_by_topic_id
    let reses = repo.find_by_topic_id("topic1", 10, 0).await.unwrap();
    assert_eq!(reses.len(), 1);
    assert_eq!(reses[0].id, "test1");

    // Test find_by_user_id
    let reses = repo.find_by_user_id("user1", 10, 0).await.unwrap();
    assert_eq!(reses.len(), 1);
    assert_eq!(reses[0].id, "test1");

    // Test update
    let mut updated = res.clone();
    updated.text = "Updated Text".to_string();
    assert!(repo.update(&updated).await.is_ok());
    let found = repo.find_by_id("test1").await.unwrap().unwrap();
    assert_eq!(found.text, "Updated Text");

    // Test subscribe_insert_event
    let mut stream = repo.subscribe_insert_event("topic1").await.unwrap();
    // Note: We can't easily test the stream in a unit test as it requires actual Redis events
    // In a real integration test, we would publish events to Redis and verify they are received
} 