use crate::{AuthToken, AuthTokenMaster, AtError, AtResult};
use crate::ports::AuthContainerPort;
use std::option::Option;

pub struct AuthContainer {
    token: Option<AuthToken>,
}

impl AuthContainer {
    pub fn new() -> Self {
        Self { token: None }
    }
}

impl AuthContainerPort for AuthContainer {
    fn get_token(&self) -> AtResult<&AuthToken> {
        self.token.as_ref().ok_or_else(|| AtError::new(AtErrorKind::Auth, "認証が必要です"))
    }

    fn get_token_master(&self) -> AtResult<&AuthTokenMaster> {
        match self.token.as_ref().ok_or_else(|| AtError::new(AtErrorKind::Auth, "認証が必要です"))? {
            AuthToken::Master(token) => Ok(token),
            AuthToken::General(_) => Err(AtError::new(AtErrorKind::Auth, "マスタートークンでの認証が必要です")),
        }
    }

    fn get_token_or_null(&self) -> Option<&AuthToken> {
        self.token.as_ref()
    }

    fn get_token_master_or_null(&self) -> Option<&AuthTokenMaster> {
        match self.token.as_ref() {
            Some(AuthToken::Master(token)) => Some(token),
            Some(AuthToken::General(_)) => None,
            None => None,
        }
    }
} 