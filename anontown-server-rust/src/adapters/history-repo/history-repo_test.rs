use chrono::Utc;
use tokio;

use crate::adapters::history_repo::history_repo::HistoryRepo;
use crate::adapters::history_repo::history_repo_mock::HistoryRepoMock;
use crate::entities::History;

#[tokio::test]
async fn test_history_repo_mock() {
    let repo = HistoryRepoMock::new();
    let history = History {
        id: "history1".to_string(),
        topic_id: "topic1".to_string(),
        user_id: "user1".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test create
    repo.create(&history).await.unwrap();

    // Test find_by_id
    let found = repo.find_by_id(&history.id).await.unwrap().unwrap();
    assert_eq!(found.id, history.id);

    // Test find_by_topic_id
    let histories = repo.find_by_topic_id(&history.topic_id, 10, 0).await.unwrap();
    assert_eq!(histories.len(), 1);
    assert_eq!(histories[0].id, history.id);

    // Test find_by_user_id
    let histories = repo.find_by_user_id(&history.user_id, 10, 0).await.unwrap();
    assert_eq!(histories.len(), 1);
    assert_eq!(histories[0].id, history.id);

    // Test update
    let mut updated_history = history.clone();
    updated_history.topic_id = "topic2".to_string();
    repo.update(&updated_history).await.unwrap();

    let found = repo.find_by_id(&history.id).await.unwrap().unwrap();
    assert_eq!(found.topic_id, updated_history.topic_id);
}

#[tokio::test]
async fn test_history_repo() {
    let repo = HistoryRepo::new().await.unwrap();
    let history = History {
        id: "history1".to_string(),
        topic_id: "topic1".to_string(),
        user_id: "user1".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test create
    repo.create(&history).await.unwrap();

    // Test find_by_id
    let found = repo.find_by_id(&history.id).await.unwrap().unwrap();
    assert_eq!(found.id, history.id);

    // Test find_by_topic_id
    let histories = repo.find_by_topic_id(&history.topic_id, 10, 0).await.unwrap();
    assert_eq!(histories.len(), 1);
    assert_eq!(histories[0].id, history.id);

    // Test find_by_user_id
    let histories = repo.find_by_user_id(&history.user_id, 10, 0).await.unwrap();
    assert_eq!(histories.len(), 1);
    assert_eq!(histories[0].id, history.id);

    // Test update
    let mut updated_history = history.clone();
    updated_history.topic_id = "topic2".to_string();
    repo.update(&updated_history).await.unwrap();

    let found = repo.find_by_id(&history.id).await.unwrap().unwrap();
    assert_eq!(found.topic_id, updated_history.topic_id);
} 