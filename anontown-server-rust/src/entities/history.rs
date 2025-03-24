use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::ports::object_id::ObjectIdGenerator;
use crate::entities::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub id: String,
    pub topic_id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub text: String,
    pub date: DateTime<Utc>,
    pub hash: String,
    pub user_id: String,
}

impl History {
    pub fn create(
        id_gen: &dyn ObjectIdGenerator,
        topic_id: String,
        title: String,
        tags: Vec<String>,
        text: String,
        user: &User,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: id_gen.generate(),
            topic_id,
            title,
            tags,
            text,
            date: now,
            hash: Self::calculate_hash(&topic_id, &title, &tags, &text, now, user),
            user_id: user.id.clone(),
        }
    }

    fn calculate_hash(
        topic_id: &str,
        title: &str,
        tags: &[String],
        text: &str,
        date: DateTime<Utc>,
        user: &User,
    ) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(topic_id.as_bytes());
        hasher.update(title.as_bytes());
        for tag in tags {
            hasher.update(tag.as_bytes());
        }
        hasher.update(text.as_bytes());
        hasher.update(date.to_rfc3339().as_bytes());
        hasher.update(user.id.as_bytes());
        format!("{:x}", hasher.finalize())
    }
} 