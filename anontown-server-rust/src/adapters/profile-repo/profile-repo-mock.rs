use async_trait::async_trait;
use std::collections::HashMap;
use crate::models::Profile;
use crate::ports::profile::{ProfilePort, ProfileQuery};

pub struct ProfileRepoMock {
    profiles: HashMap<String, Profile>,
}

impl ProfileRepoMock {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }
}

#[async_trait]
impl ProfilePort for ProfileRepoMock {
    async fn find_one(&mut self, id: &str) -> Result<Profile, Box<dyn std::error::Error>> {
        self.profiles
            .get(id)
            .cloned()
            .ok_or_else(|| "Profile not found".into())
    }

    async fn find(&mut self, query: &ProfileQuery) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
        let mut profiles = self.profiles.values().cloned().collect::<Vec<_>>();

        // IDでフィルタリング
        if let Some(ids) = &query.id {
            profiles.retain(|p| ids.contains(&p.id));
        }

        // ユーザーIDでフィルタリング
        if let Some(self_) = query.self_ {
            profiles.retain(|p| p.user_id == self_);
        }

        // 作成日時でソート
        profiles.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(profiles)
    }

    async fn insert(&mut self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        self.profiles.insert(profile.id.clone(), profile.clone());
        Ok(())
    }

    async fn update(&mut self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        if self.profiles.contains_key(&profile.id) {
            self.profiles.insert(profile.id.clone(), profile.clone());
            Ok(())
        } else {
            Err("Profile not found".into())
        }
    }
} 