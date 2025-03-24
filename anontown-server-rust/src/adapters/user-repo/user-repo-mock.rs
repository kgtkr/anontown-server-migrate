use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::entities::{User, UserType};
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
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        Ok(self.users.get(id).cloned())
    }

    async fn find_by_sn(&self, sn: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        Ok(self.users
            .values()
            .find(|user| user.sn == sn)
            .cloned())
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        Ok(self.users
            .values()
            .find(|user| user.name == name)
            .cloned())
    }

    async fn create(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        self.users.insert(user.id.clone(), user.clone());
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        if self.users.contains_key(&user.id) {
            self.users.insert(user.id.clone(), user.clone());
            Ok(())
        } else {
            Err("User not found".into())
        }
    }

    async fn count_by_type(&self, user_type: UserType) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(self.users
            .values()
            .filter(|user| user.user_type == user_type)
            .count() as i64)
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

    async fn update_point(&self, id: &str, point: i64) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = self.users.get_mut(id) {
            user.point = point;
            user.updated_at = Utc::now();
            Ok(())
        } else {
            Err("User not found".into())
        }
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

    async fn update_lv(&self, id: &str, lv: i64) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = self.users.get_mut(id) {
            user.lv = lv;
            user.updated_at = Utc::now();
            Ok(())
        } else {
            Err("User not found".into())
        }
    }

    async fn update_age(&self, id: &str, age: bool) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = self.users.get_mut(id) {
            user.age = age;
            user.updated_at = Utc::now();
            Ok(())
        } else {
            Err("User not found".into())
        }
    }
} 