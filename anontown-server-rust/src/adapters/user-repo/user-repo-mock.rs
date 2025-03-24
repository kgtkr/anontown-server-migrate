use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::models::User;
use crate::ports::user::UserPort;

pub struct UserRepoMock {
    users: HashMap<String, User>,
}

impl UserRepoMock {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

#[async_trait]
impl UserPort for UserRepoMock {
    async fn find_by_id(&mut self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        Ok(self.users.get(id).cloned())
    }

    async fn find_by_screen_name(&mut self, screen_name: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        Ok(self.users.values().find(|u| u.screen_name == screen_name).cloned())
    }

    async fn create(&mut self, user: &User) -> Result<User, Box<dyn std::error::Error>> {
        self.users.insert(user.id.clone(), user.clone());
        Ok(user.clone())
    }

    async fn update(&mut self, user: &User) -> Result<User, Box<dyn std::error::Error>> {
        self.users.insert(user.id.clone(), user.clone());
        Ok(user.clone())
    }

    async fn update_res_count(&mut self, id: &str, count: i32) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = self.users.get_mut(id) {
            user.count_created_res_m10 += count;
            user.count_created_res_m30 += count;
            user.count_created_res_h1 += count;
            user.count_created_res_h6 += count;
            user.count_created_res_h12 += count;
            user.count_created_res_d1 += count;
        }
        Ok(())
    }

    async fn update_topic_count(&mut self, id: &str, _count: i32) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = self.users.get_mut(id) {
            user.topic_last_created_at = Utc::now();
        }
        Ok(())
    }

    async fn update_point(&mut self, id: &str, point: i32) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = self.users.get_mut(id) {
            user.point += point;
        }
        Ok(())
    }

    async fn update_res_last_created_at(&mut self, id: &str, created_at: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = self.users.get_mut(id) {
            user.res_last_created_at = created_at;
        }
        Ok(())
    }

    async fn update_topic_last_created_at(&mut self, id: &str, created_at: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = self.users.get_mut(id) {
            user.topic_last_created_at = created_at;
        }
        Ok(())
    }

    async fn update_one_topic_last_created_at(&mut self, id: &str, created_at: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = self.users.get_mut(id) {
            user.one_topic_last_created_at = created_at;
        }
        Ok(())
    }
} 