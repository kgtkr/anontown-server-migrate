use chrono::{DateTime, Duration, Utc};

use crate::ports::{clock::ClockPort, object_id_generator::ObjectIdGeneratorPort};

#[derive(Debug)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    pub client_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Token {
    pub fn new(
        user_id: String,
        client_id: String,
        access_token: String,
        refresh_token: String,
        expires_at: DateTime<Utc>,
        clock: &impl ClockPort,
        id_generator: &impl ObjectIdGeneratorPort,
    ) -> Self {
        let now = clock.now();
        Self {
            id: id_generator.generate(),
            user_id,
            client_id,
            access_token,
            refresh_token,
            expires_at,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_expired(&self, clock: &impl ClockPort) -> bool {
        self.expires_at <= clock.now()
    }

    pub fn update(&mut self, clock: &impl ClockPort) {
        self.updated_at = clock.now();
    }

    pub fn is_self(&self, user_id: &str) -> bool {
        self.user_id == user_id
    }

    pub fn is_client(&self, client_id: &str) -> bool {
        self.client_id == client_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::{
        clock::clock::Clock,
        object_id_generator::object_id_generator::ObjectIdGenerator,
    };

    #[tokio::test]
    async fn test_new() {
        let clock = Clock::new();
        let id_generator = ObjectIdGenerator::new();
        let now = clock.now();
        let expires_at = now + Duration::hours(1);

        let token = Token::new(
            "user1".to_string(),
            "client1".to_string(),
            "access_token1".to_string(),
            "refresh_token1".to_string(),
            expires_at,
            &clock,
            &id_generator,
        );

        assert_eq!(token.user_id, "user1");
        assert_eq!(token.client_id, "client1");
        assert_eq!(token.access_token, "access_token1");
        assert_eq!(token.refresh_token, "refresh_token1");
        assert_eq!(token.expires_at, expires_at);
        assert_eq!(token.created_at, now);
        assert_eq!(token.updated_at, now);
    }

    #[tokio::test]
    async fn test_is_expired() {
        let clock = Clock::new();
        let id_generator = ObjectIdGenerator::new();
        let now = clock.now();
        let expires_at = now + Duration::hours(1);

        let token = Token::new(
            "user1".to_string(),
            "client1".to_string(),
            "access_token1".to_string(),
            "refresh_token1".to_string(),
            expires_at,
            &clock,
            &id_generator,
        );

        assert!(!token.is_expired(&clock));

        let expired_token = Token::new(
            "user1".to_string(),
            "client1".to_string(),
            "access_token1".to_string(),
            "refresh_token1".to_string(),
            now - Duration::hours(1),
            &clock,
            &id_generator,
        );

        assert!(expired_token.is_expired(&clock));
    }

    #[tokio::test]
    async fn test_update() {
        let clock = Clock::new();
        let id_generator = ObjectIdGenerator::new();
        let now = clock.now();
        let expires_at = now + Duration::hours(1);

        let mut token = Token::new(
            "user1".to_string(),
            "client1".to_string(),
            "access_token1".to_string(),
            "refresh_token1".to_string(),
            expires_at,
            &clock,
            &id_generator,
        );

        let original_updated_at = token.updated_at;
        token.update(&clock);
        assert!(token.updated_at > original_updated_at);
    }

    #[tokio::test]
    async fn test_is_self() {
        let clock = Clock::new();
        let id_generator = ObjectIdGenerator::new();
        let now = clock.now();
        let expires_at = now + Duration::hours(1);

        let token = Token::new(
            "user1".to_string(),
            "client1".to_string(),
            "access_token1".to_string(),
            "refresh_token1".to_string(),
            expires_at,
            &clock,
            &id_generator,
        );

        assert!(token.is_self("user1"));
        assert!(!token.is_self("user2"));
    }

    #[tokio::test]
    async fn test_is_client() {
        let clock = Clock::new();
        let id_generator = ObjectIdGenerator::new();
        let now = clock.now();
        let expires_at = now + Duration::hours(1);

        let token = Token::new(
            "user1".to_string(),
            "client1".to_string(),
            "access_token1".to_string(),
            "refresh_token1".to_string(),
            expires_at,
            &clock,
            &id_generator,
        );

        assert!(token.is_client("client1"));
        assert!(!token.is_client("client2"));
    }
} 