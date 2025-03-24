use async_trait::async_trait;
use sqlx::PgPool;

use super::profile_model::ProfileModel;
use crate::ports::profile::{Profile, ProfileRepoPort, run_profile_repo_laws};

pub struct ProfileRepo {
    pool: PgPool,
}

impl ProfileRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfileRepoPort for ProfileRepo {
    async fn save(&mut self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        let profile = ProfileModel {
            id: profile.id.clone(),
            user_id: profile.user_id.clone(),
            name: profile.name.clone(),
            description: profile.description.clone(),
            date: profile.date,
        };

        sqlx::query_as!(
            ProfileModel,
            r#"
            INSERT INTO profiles (id, user_id, name, description, date)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE
            SET user_id = $2, name = $3, description = $4, date = $5
            "#,
            profile.id,
            profile.user_id,
            profile.name,
            profile.description,
            profile.date
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_one(&mut self, id: &str) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
        let profile = sqlx::query_as!(
            ProfileModel,
            r#"
            SELECT id, user_id, name, description, date
            FROM profiles
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(profile.map(|p| Profile {
            id: p.id,
            user_id: p.user_id,
            name: p.name,
            description: p.description,
            date: p.date,
        }))
    }

    async fn find(&mut self, user_id: &str) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
        let profiles = sqlx::query_as!(
            ProfileModel,
            r#"
            SELECT id, user_id, name, description, date
            FROM profiles
            WHERE user_id = $1
            ORDER BY date DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(profiles
            .into_iter()
            .map(|p| Profile {
                id: p.id,
                user_id: p.user_id,
                name: p.name,
                description: p.description,
                date: p.date,
            })
            .collect())
    }
}

#[sqlx::test]
async fn test_profile_repo(pool: PgPool) {
    let mut repo = ProfileRepo::new(pool);
    run_profile_repo_laws(&mut repo).await;
} 