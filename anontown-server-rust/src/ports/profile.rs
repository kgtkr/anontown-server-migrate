use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::entities::profile::Profile;

#[async_trait]
pub trait ProfileRepoPort: Send + Sync {
    async fn save(&mut self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>>;
    async fn find_one(&mut self, user_id: &str) -> Result<Option<Profile>, Box<dyn std::error::Error>>;
    async fn find(&mut self, user_ids: &[String]) -> Result<Vec<Profile>, Box<dyn std::error::Error>>;
}

pub async fn run_profile_repo_laws(repo: &mut impl ProfileRepoPort) {
    let user_id = "test_user";
    let name = "Test User";
    let description = "Test Description";
    let date = Utc::now();

    // テストデータの作成
    let profile = Profile {
        id: Uuid::new_v4().to_string(),
        user_id: user_id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        date,
    };

    // 保存テスト
    repo.save(&profile).await.unwrap();

    // find_oneテスト
    let found = repo.find_one(user_id).await.unwrap();
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.user_id, user_id);
    assert_eq!(found.name, name);
    assert_eq!(found.description, description);

    // findテスト
    let found = repo.find(&[]).await.unwrap();
    assert!(!found.is_empty());
    let found = repo.find(&[user_id.to_string()]).await.unwrap();
    assert!(!found.is_empty());
    assert_eq!(found[0].user_id, user_id);

    // 存在しないユーザーのテスト
    let not_found = repo.find_one("non_existent").await.unwrap();
    assert!(not_found.is_none());
}

pub struct ProfileQuery {
    pub id: Option<Vec<String>>,
    pub is_self: Option<bool>,
} 