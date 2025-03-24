use sqlx::PgPool;
use redis::Client;

#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
    pub redis: Client,
}

impl juniper::Context for Context {} 