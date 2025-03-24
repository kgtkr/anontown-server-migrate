use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::ports::object_id::ObjectIdGenerator;
use crate::ports::clock::ClockPort;
use crate::entities::user::User;
use crate::entities::res::Res;
use crate::adapters::clock::fix_clock::FixClock;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TopicType {
    Normal,
    One,
    Fork,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicBase {
    pub id: String,
    pub title: String,
    pub description: String,
    pub topic_type: TopicType,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub res_count: i32,
    pub last_res_at: DateTime<Utc>,
    pub is_closed: bool,
    pub tags: Vec<String>,
}

impl TopicBase {
    pub fn hash(&self, date: DateTime<Utc>, user: &User) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(self.id.as_bytes());
        hasher.update(date.to_rfc3339().as_bytes());
        hasher.update(user.id.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn check_data(title: &str, tags: &[String], text: &str) -> Result<(), String> {
        // タイトルのバリデーション
        if title.is_empty() {
            return Err("タイトルが空です".to_string());
        }
        if title.len() > 100 {
            return Err("タイトルが長すぎます".to_string());
        }

        // タグのバリデーション
        if tags.len() > 15 {
            return Err("タグが多すぎます".to_string());
        }
        let mut unique_tags = std::collections::HashSet::new();
        for tag in tags {
            if tag.is_empty() {
                return Err("タグが空です".to_string());
            }
            if tag.len() > 20 {
                return Err("タグが長すぎます".to_string());
            }
            if !unique_tags.insert(tag) {
                return Err("タグに重複があります".to_string());
            }
        }

        // 本文のバリデーション
        if text.is_empty() {
            return Err("本文が空です".to_string());
        }
        if text.len() > 10000 {
            return Err("本文が長すぎます".to_string());
        }

        Ok(())
    }

    pub fn can_create_res(&self) -> bool {
        !self.is_closed
    }

    pub fn res_update(&mut self, res: &Res, clock: &dyn ClockPort) -> &mut Self {
        self.res_count += 1;
        self.last_res_at = clock.now();
        self.updated_at = clock.now();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicNormal {
    #[serde(flatten)]
    base: TopicBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicOne {
    #[serde(flatten)]
    base: TopicBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicFork {
    #[serde(flatten)]
    base: TopicBase,
    pub parent_id: String,
}

impl TopicNormal {
    pub fn create(
        id_gen: &dyn ObjectIdGenerator,
        clock: &dyn ClockPort,
        title: String,
        description: String,
        user_id: String,
        tags: Vec<String>,
    ) -> Self {
        let now = clock.now();
        Self {
            base: TopicBase {
                id: id_gen.generate(),
                title,
                description,
                topic_type: TopicType::Normal,
                user_id,
                created_at: now,
                updated_at: now,
                res_count: 1,
                last_res_at: now,
                is_closed: false,
                tags,
            },
        }
    }

    pub fn change_data(
        &mut self,
        clock: &dyn ClockPort,
        title: String,
        description: String,
        tags: Vec<String>,
        user: &mut User,
    ) -> Result<(), String> {
        TopicBase::check_data(&title, &tags, &description)?;
        self.base.title = title;
        self.base.description = description;
        self.base.tags = tags;
        self.base.updated_at = clock.now();
        user.point += 1;
        Ok(())
    }

    pub fn base(&self) -> &TopicBase {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut TopicBase {
        &mut self.base
    }
}

impl TopicOne {
    pub fn create(
        id_gen: &dyn ObjectIdGenerator,
        clock: &dyn ClockPort,
        title: String,
        description: String,
        user_id: String,
        tags: Vec<String>,
    ) -> Self {
        let now = clock.now();
        Self {
            base: TopicBase {
                id: id_gen.generate(),
                title,
                description,
                topic_type: TopicType::One,
                user_id,
                created_at: now,
                updated_at: now,
                res_count: 1,
                last_res_at: now,
                is_closed: false,
                tags,
            },
        }
    }

    pub fn change_data(
        &mut self,
        clock: &dyn ClockPort,
        title: String,
        description: String,
        tags: Vec<String>,
        user: &mut User,
    ) -> Result<(), String> {
        TopicBase::check_data(&title, &tags, &description)?;
        self.base.title = title;
        self.base.description = description;
        self.base.tags = tags;
        self.base.updated_at = clock.now();
        user.point += 1;
        Ok(())
    }

    pub fn base(&self) -> &TopicBase {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut TopicBase {
        &mut self.base
    }
}

impl TopicFork {
    pub fn create(
        id_gen: &dyn ObjectIdGenerator,
        clock: &dyn ClockPort,
        title: String,
        description: String,
        user_id: String,
        tags: Vec<String>,
        parent_id: String,
    ) -> Self {
        let now = clock.now();
        Self {
            base: TopicBase {
                id: id_gen.generate(),
                title,
                description,
                topic_type: TopicType::Fork,
                user_id,
                created_at: now,
                updated_at: now,
                res_count: 1,
                last_res_at: now,
                is_closed: false,
                tags,
            },
            parent_id,
        }
    }

    pub fn change_data(
        &mut self,
        clock: &dyn ClockPort,
        title: String,
        description: String,
        tags: Vec<String>,
        user: &mut User,
    ) -> Result<(), String> {
        TopicBase::check_data(&title, &tags, &description)?;
        self.base.title = title;
        self.base.description = description;
        self.base.tags = tags;
        self.base.updated_at = clock.now();
        user.point += 1;
        Ok(())
    }

    pub fn base(&self) -> &TopicBase {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut TopicBase {
        &mut self.base
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopicSearch {
    Normal(TopicNormal),
    One(TopicOne),
}

impl TopicSearch {
    pub fn base(&self) -> &TopicBase {
        match self {
            TopicSearch::Normal(t) => t.base(),
            TopicSearch::One(t) => t.base(),
        }
    }

    pub fn base_mut(&mut self) -> &mut TopicBase {
        match self {
            TopicSearch::Normal(t) => t.base_mut(),
            TopicSearch::One(t) => t.base_mut(),
        }
    }
}

impl From<TopicSearch> for Topic {
    fn from(search: TopicSearch) -> Self {
        match search {
            TopicSearch::Normal(t) => Topic::Normal(t),
            TopicSearch::One(t) => Topic::One(t),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Topic {
    Normal(TopicNormal),
    One(TopicOne),
    Fork(TopicFork),
}

impl Topic {
    pub fn base(&self) -> &TopicBase {
        match self {
            Topic::Normal(t) => t.base(),
            Topic::One(t) => t.base(),
            Topic::Fork(t) => t.base(),
        }
    }

    pub fn base_mut(&mut self) -> &mut TopicBase {
        match self {
            Topic::Normal(t) => t.base_mut(),
            Topic::One(t) => t.base_mut(),
            Topic::Fork(t) => t.base_mut(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    struct DummyObjectIdGenerator {
        id: String,
    }

    impl ObjectIdGenerator for DummyObjectIdGenerator {
        fn generate(&self) -> String {
            self.id.clone()
        }
    }

    #[test]
    fn test_topic_base_check_data() {
        // 正常なケース
        assert!(TopicBase::check_data("title", &["a", "b"], "text").is_ok());
        assert!(TopicBase::check_data("title", &[], "text").is_ok());

        // タイトルが空
        assert!(TopicBase::check_data("", &["a"], "text").is_err());

        // タイトルが長すぎる
        assert!(TopicBase::check_data(&"a".repeat(101), &["a"], "text").is_err());

        // タグが空
        assert!(TopicBase::check_data("title", &[""], "text").is_err());

        // タグが長すぎる
        assert!(TopicBase::check_data("title", &[&"a".repeat(21)], "text").is_err());

        // タグが多すぎる
        assert!(TopicBase::check_data("title", &vec!["a".to_string(); 16], "text").is_err());

        // タグに重複がある
        assert!(TopicBase::check_data("title", &["a", "a"], "text").is_err());

        // 本文が空
        assert!(TopicBase::check_data("title", &["a"], "").is_err());

        // 本文が長すぎる
        assert!(TopicBase::check_data("title", &["a"], &"a".repeat(10001)).is_err());
    }

    #[test]
    fn test_topic_base_hash() {
        let user = User::create(
            &DummyObjectIdGenerator { id: "user".to_string() },
            "sn".to_string(),
            "pass".to_string(),
            10,
            Utc.timestamp_opt(0, 0).unwrap(),
        );

        let now = Utc.timestamp_opt(1000, 0).unwrap();
        let clock = FixClock::new(now);
        let topic = TopicNormal::create(
            &DummyObjectIdGenerator { id: "topic".to_string() },
            &clock,
            "title".to_string(),
            "description".to_string(),
            "user".to_string(),
            vec!["tag".to_string()],
        );

        let date1 = Utc.timestamp_opt(1000, 0).unwrap();
        let date2 = Utc.timestamp_opt(1000, 1000).unwrap();
        let date3 = Utc.timestamp_opt(1001, 0).unwrap();

        // 同じ日付の場合は同じハッシュ
        assert_eq!(
            topic.base().hash(date1, &user),
            topic.base().hash(date2, &user)
        );

        // 異なる日付の場合は異なるハッシュ
        assert_ne!(
            topic.base().hash(date1, &user),
            topic.base().hash(date3, &user)
        );

        // 異なるユーザーの場合は異なるハッシュ
        let user2 = User::create(
            &DummyObjectIdGenerator { id: "user2".to_string() },
            "sn2".to_string(),
            "pass2".to_string(),
            10,
            Utc.timestamp_opt(0, 0).unwrap(),
        );
        assert_ne!(
            topic.base().hash(date1, &user),
            topic.base().hash(date1, &user2)
        );
    }

    #[test]
    fn test_topic_normal_create() {
        let id_gen = DummyObjectIdGenerator { id: "topic".to_string() };
        let now = Utc.timestamp_opt(86400, 0).unwrap();
        let clock = FixClock::new(now);
        let topic = TopicNormal::create(
            &id_gen,
            &clock,
            "title".to_string(),
            "description".to_string(),
            "user".to_string(),
            vec!["tag".to_string()],
        );

        assert_eq!(topic.base().id, "topic");
        assert_eq!(topic.base().title, "title");
        assert_eq!(topic.base().description, "description");
        assert_eq!(topic.base().user_id, "user");
        assert_eq!(topic.base().res_count, 1);
        assert_eq!(topic.base().is_closed, false);
        assert_eq!(topic.base().tags, vec!["tag"]);
    }

    #[test]
    fn test_topic_one_create() {
        let id_gen = DummyObjectIdGenerator { id: "topic".to_string() };
        let now = Utc.timestamp_opt(86400, 0).unwrap();
        let clock = FixClock::new(now);
        let topic = TopicOne::create(
            &id_gen,
            &clock,
            "title".to_string(),
            "description".to_string(),
            "user".to_string(),
            vec!["tag".to_string()],
        );

        assert_eq!(topic.base().id, "topic");
        assert_eq!(topic.base().title, "title");
        assert_eq!(topic.base().description, "description");
        assert_eq!(topic.base().user_id, "user");
        assert_eq!(topic.base().res_count, 1);
        assert_eq!(topic.base().is_closed, false);
        assert_eq!(topic.base().tags, vec!["tag"]);
    }

    #[test]
    fn test_topic_fork_create() {
        let id_gen = DummyObjectIdGenerator { id: "topic".to_string() };
        let now = Utc.timestamp_opt(86400, 0).unwrap();
        let clock = FixClock::new(now);
        let topic = TopicFork::create(
            &id_gen,
            &clock,
            "title".to_string(),
            "description".to_string(),
            "user".to_string(),
            vec!["tag".to_string()],
            "parent".to_string(),
        );

        assert_eq!(topic.base().id, "topic");
        assert_eq!(topic.base().title, "title");
        assert_eq!(topic.base().description, "description");
        assert_eq!(topic.base().user_id, "user");
        assert_eq!(topic.base().res_count, 1);
        assert_eq!(topic.base().is_closed, false);
        assert_eq!(topic.base().tags, vec!["tag"]);
        assert_eq!(topic.parent_id, "parent");
    }

    #[test]
    fn test_topic_search_conversion() {
        let now = Utc.timestamp_opt(1000, 0).unwrap();
        let clock = FixClock::new(now);
        let topic_normal = TopicNormal::create(
            &DummyObjectIdGenerator { id: "topic".to_string() },
            &clock,
            "title".to_string(),
            "description".to_string(),
            "user".to_string(),
            vec!["tag".to_string()],
        );

        let topic_one = TopicOne::create(
            &DummyObjectIdGenerator { id: "topic2".to_string() },
            &clock,
            "title2".to_string(),
            "description2".to_string(),
            "user".to_string(),
            vec!["tag2".to_string()],
        );

        let search_normal = TopicSearch::Normal(topic_normal.clone());
        let search_one = TopicSearch::One(topic_one.clone());

        let topic_from_normal: Topic = search_normal.into();
        let topic_from_one: Topic = search_one.into();

        match topic_from_normal {
            Topic::Normal(t) => assert_eq!(t.base().id, "topic"),
            _ => panic!("Expected Topic::Normal"),
        }

        match topic_from_one {
            Topic::One(t) => assert_eq!(t.base().id, "topic2"),
            _ => panic!("Expected Topic::One"),
        }
    }

    #[test]
    fn test_topic_normal_change_data() {
        let id_gen = DummyObjectIdGenerator { id: "topic".to_string() };
        let now = Utc.timestamp_opt(86400, 0).unwrap();
        let clock = FixClock::new(now);
        let mut topic = TopicNormal::create(
            &id_gen,
            &clock,
            "title".to_string(),
            "description".to_string(),
            "user".to_string(),
            vec!["tag".to_string()],
        );

        let mut user = User::create(
            &DummyObjectIdGenerator { id: "user".to_string() },
            "sn".to_string(),
            "pass".to_string(),
            10,
            Utc.timestamp_opt(0, 0).unwrap(),
        );

        // 正常なケース
        assert!(topic.change_data(
            &clock,
            "new_title".to_string(),
            "new_description".to_string(),
            vec!["new_tag".to_string()],
            &mut user,
        ).is_ok());

        assert_eq!(topic.base().title, "new_title");
        assert_eq!(topic.base().description, "new_description");
        assert_eq!(topic.base().tags, vec!["new_tag"]);
        assert_eq!(user.point, 11);

        // バリデーションエラー
        assert!(topic.change_data(
            &clock,
            "".to_string(),
            "new_description".to_string(),
            vec!["new_tag".to_string()],
            &mut user,
        ).is_err());
    }

    #[test]
    fn test_topic_one_change_data() {
        let id_gen = DummyObjectIdGenerator { id: "topic".to_string() };
        let now = Utc.timestamp_opt(86400, 0).unwrap();
        let clock = FixClock::new(now);
        let mut topic = TopicOne::create(
            &id_gen,
            &clock,
            "title".to_string(),
            "description".to_string(),
            "user".to_string(),
            vec!["tag".to_string()],
        );

        let mut user = User::create(
            &DummyObjectIdGenerator { id: "user".to_string() },
            "sn".to_string(),
            "pass".to_string(),
            10,
            Utc.timestamp_opt(0, 0).unwrap(),
        );

        // 正常なケース
        assert!(topic.change_data(
            &clock,
            "new_title".to_string(),
            "new_description".to_string(),
            vec!["new_tag".to_string()],
            &mut user,
        ).is_ok());

        assert_eq!(topic.base().title, "new_title");
        assert_eq!(topic.base().description, "new_description");
        assert_eq!(topic.base().tags, vec!["new_tag"]);
        assert_eq!(user.point, 11);

        // バリデーションエラー
        assert!(topic.change_data(
            &clock,
            "".to_string(),
            "new_description".to_string(),
            vec!["new_tag".to_string()],
            &mut user,
        ).is_err());
    }

    #[test]
    fn test_topic_fork_change_data() {
        let id_gen = DummyObjectIdGenerator { id: "topic".to_string() };
        let now = Utc.timestamp_opt(86400, 0).unwrap();
        let clock = FixClock::new(now);
        let mut topic = TopicFork::create(
            &id_gen,
            &clock,
            "title".to_string(),
            "description".to_string(),
            "user".to_string(),
            vec!["tag".to_string()],
            "parent".to_string(),
        );

        let mut user = User::create(
            &DummyObjectIdGenerator { id: "user".to_string() },
            "sn".to_string(),
            "pass".to_string(),
            10,
            Utc.timestamp_opt(0, 0).unwrap(),
        );

        // 正常なケース
        assert!(topic.change_data(
            &clock,
            "new_title".to_string(),
            "new_description".to_string(),
            vec!["new_tag".to_string()],
            &mut user,
        ).is_ok());

        assert_eq!(topic.base().title, "new_title");
        assert_eq!(topic.base().description, "new_description");
        assert_eq!(topic.base().tags, vec!["new_tag"]);
        assert_eq!(user.point, 11);

        // バリデーションエラー
        assert!(topic.change_data(
            &clock,
            "".to_string(),
            "new_description".to_string(),
            vec!["new_tag".to_string()],
            &mut user,
        ).is_err());
    }
} 