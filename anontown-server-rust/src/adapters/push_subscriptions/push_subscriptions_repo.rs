use async_trait::async_trait;
use sqlx::PgPool;
use crate::models::push_subscription::PushSubscription;
use crate::ports::push_subscriptions::PushSubscriptionsPort;

pub struct PushSubscriptionsRepo {
    pool: PgPool,
}

impl PushSubscriptionsRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PushSubscriptionsPort for PushSubscriptionsRepo {
    async fn find_all(&self) -> Result<Vec<PushSubscription>, Box<dyn std::error::Error>> {
        let subscriptions = sqlx::query_as!(
            PushSubscription,
            r#"
            SELECT id, endpoint, p256dh, auth
            FROM push_subscriptions
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(subscriptions)
    }

    async fn insert(&self, subscription: PushSubscription) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO push_subscriptions (id, endpoint, p256dh, auth)
            VALUES ($1, $2, $3, $4)
            "#,
            subscription.id,
            subscription.endpoint,
            subscription.p256dh,
            subscription.auth
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            DELETE FROM push_subscriptions
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
} 