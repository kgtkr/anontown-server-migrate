use sqlx::PgPool;
use redis::Client as RedisClient;

pub struct Context {
    pub db: PgPool,
    pub redis: RedisClient,
}

impl juniper::Context for Context {} 