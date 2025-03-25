use async_trait::async_trait;
use crate::auth::{AuthTokenMaster, AuthUser};
use crate::at_error::AtResult;
use crate::entities::Token;

#[async_trait]
pub trait TokenRepo {
    async fn find_one(&self, id: &str) -> AtResult<Token>;
    async fn find_all(&self, auth_token: &AuthTokenMaster) -> AtResult<Vec<Token>>;
    async fn insert(&self, token: &Token) -> AtResult<()>;
    async fn update(&self, token: &Token) -> AtResult<()>;
    async fn del_client_token(&self, token: &AuthTokenMaster, client_id: &str) -> AtResult<()>;
    async fn del_master_token(&self, user: &AuthUser) -> AtResult<()>;
}
