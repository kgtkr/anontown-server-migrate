use chrono::{DateTime, Utc};
use crate::schema::types::{ToSchemaType, ClientType, UserType, TokenType, TopicType, ResType, HistoryType, ProfileType, StorageType};
use crate::ports::AuthContainer;
use crate::entities::{client::Client, user::User, token::Token, topic::Topic, res::Res, history::History, profile::Profile, storage::Storage};

#[test]
fn test_client_to_schema_type() {
    let now = Utc::now();
    let client = Client {
        id: "test_client".to_string(),
        name: "Test Client".to_string(),
        url: "http://test.com".to_string(),
        date: now,
        update: now,
    };

    // Test with master token
    let mut auth_container = AuthContainer::new();
    auth_container.set_token_master("test_client".to_string());
    let schema_type = client.to_schema_type(&auth_container);
    assert_eq!(schema_type.id, "test_client");
    assert_eq!(schema_type.name, "Test Client");
    assert_eq!(schema_type.url, "http://test.com");
    assert_eq!(schema_type.self_, Some(true));
    assert_eq!(schema_type.created_at, now);
    assert_eq!(schema_type.updated_at, now);

    // Test without master token
    auth_container.set_token_master("other_client".to_string());
    let schema_type = client.to_schema_type(&auth_container);
    assert_eq!(schema_type.self_, Some(false));

    // Test without token
    auth_container.set_token_master("".to_string());
    let schema_type = client.to_schema_type(&auth_container);
    assert_eq!(schema_type.self_, None);
}

#[test]
fn test_user_to_schema_type() {
    let now = Utc::now();
    let user = User {
        id: "test_user".to_string(),
        sn: "test_sn".to_string(),
        created_at: now,
        updated_at: now,
    };

    let auth_container = AuthContainer::new();
    let schema_type = user.to_schema_type(&auth_container);
    assert_eq!(schema_type.id, "test_user");
    assert_eq!(schema_type.sn, "test_sn");
    assert_eq!(schema_type.created_at, now);
    assert_eq!(schema_type.updated_at, now);
}

#[test]
fn test_token_to_schema_type() {
    let now = Utc::now();
    let token = Token {
        id: "test_token".to_string(),
        key: "test_key".to_string(),
        date: now,
    };

    let auth_container = AuthContainer::new();
    let schema_type = token.to_schema_type(&auth_container);
    assert_eq!(schema_type.id, "test_token");
    assert_eq!(schema_type.key, "test_key");
    assert_eq!(schema_type.date, now);
}

#[test]
fn test_topic_to_schema_type() {
    let now = Utc::now();
    let topic = Topic::Normal(TopicNormal {
        base: TopicBase {
            id: "test_topic".to_string(),
            title: "Test Topic".to_string(),
            description: "Test Description".to_string(),
            user_id: "test_user".to_string(),
            created_at: now,
            updated_at: now,
            res_count: 0,
            last_res_at: now,
            is_closed: false,
            tags: vec!["test".to_string()],
        },
    });

    // Test with token
    let mut auth_container = AuthContainer::new();
    auth_container.set_token("test_user".to_string());
    let schema_type = topic.to_schema_type(&auth_container);
    assert_eq!(schema_type.id, "test_topic");
    assert_eq!(schema_type.title, "Test Topic");
    assert_eq!(schema_type.description, "Test Description");
    assert_eq!(schema_type.user_id, "test_user");
    assert_eq!(schema_type.created_at, now);
    assert_eq!(schema_type.updated_at, now);
    assert_eq!(schema_type.res_count, 0);
    assert_eq!(schema_type.last_res_at, now);
    assert_eq!(schema_type.is_closed, false);
    assert_eq!(schema_type.tags, vec!["test".to_string()]);

    // Test without token
    auth_container.set_token("".to_string());
    let schema_type = topic.to_schema_type(&auth_container);
    assert_eq!(schema_type.subscribe, None);
}

#[test]
fn test_res_to_schema_type() {
    let now = Utc::now();
    let topic = Topic::Normal(TopicNormal {
        base: TopicBase {
            id: "test_topic".to_string(),
            title: "Test Topic".to_string(),
            description: "Test Description".to_string(),
            user_id: "test_user".to_string(),
            created_at: now,
            updated_at: now,
            res_count: 0,
            last_res_at: now,
            is_closed: false,
            tags: vec!["test".to_string()],
        },
    });

    let res = Res::Normal(ResNormal {
        base: ResBase {
            id: "test_res".to_string(),
            topic: topic.clone(),
            user_id: "test_user".to_string(),
            created_at: now,
            updated_at: now,
            uv: 1,
            dv: 0,
            hash: "test_hash".to_string(),
            reply_count: 0,
            vote_flag: None,
        },
        name: Some("Test Name".to_string()),
        text: "Test Text".to_string(),
        reply: None,
    });

    // Test with token
    let mut auth_container = AuthContainer::new();
    auth_container.set_token("test_user".to_string());
    let schema_type = res.to_schema_type(&auth_container);
    assert_eq!(schema_type.id, "test_res");
    assert_eq!(schema_type.text, "Test Text");
    assert_eq!(schema_type.topic_id, "test_topic");
    assert_eq!(schema_type.user_id, "test_user");
    assert_eq!(schema_type.created_at, now);
    assert_eq!(schema_type.updated_at, now);
    assert_eq!(schema_type.self_, Some(true));

    // Test without token
    auth_container.set_token("".to_string());
    let schema_type = res.to_schema_type(&auth_container);
    assert_eq!(schema_type.self_, None);
}

#[test]
fn test_history_to_schema_type() {
    let now = Utc::now();
    let topic = Topic::Normal(TopicNormal {
        base: TopicBase {
            id: "test_topic".to_string(),
            title: "Test Topic".to_string(),
            description: "Test Description".to_string(),
            user_id: "test_user".to_string(),
            created_at: now,
            updated_at: now,
            res_count: 0,
            last_res_at: now,
            is_closed: false,
            tags: vec!["test".to_string()],
        },
    });

    let history = History {
        id: "test_history".to_string(),
        topic: topic.clone(),
        title: "Test History".to_string(),
        text: "Test Text".to_string(),
        tags: vec!["test".to_string()],
        created_at: now,
    };

    let auth_container = AuthContainer::new();
    let schema_type = history.to_schema_type(&auth_container);
    assert_eq!(schema_type.id, "test_history");
    assert_eq!(schema_type.title, "Test History");
    assert_eq!(schema_type.text, "Test Text");
    assert_eq!(schema_type.tags, vec!["test".to_string()]);
    assert_eq!(schema_type.created_at, now);
}

#[test]
fn test_profile_to_schema_type() {
    let now = Utc::now();
    let profile = Profile {
        id: "test_profile".to_string(),
        name: "Test Name".to_string(),
        text: "Test Text".to_string(),
        sn: "test_sn".to_string(),
        created_at: now,
        updated_at: now,
    };

    let auth_container = AuthContainer::new();
    let schema_type = profile.to_schema_type(&auth_container);
    assert_eq!(schema_type.id, "test_profile");
    assert_eq!(schema_type.name, "Test Name");
    assert_eq!(schema_type.text, "Test Text");
    assert_eq!(schema_type.sn, "test_sn");
    assert_eq!(schema_type.created_at, now);
    assert_eq!(schema_type.updated_at, now);
}

#[test]
fn test_storage_to_schema_type() {
    let storage = Storage {
        key: "test_key".to_string(),
        value: "test_value".to_string(),
    };

    let auth_container = AuthContainer::new();
    let schema_type = storage.to_schema_type(&auth_container);
    assert_eq!(schema_type.key, "test_key");
    assert_eq!(schema_type.value, "test_value");
} 