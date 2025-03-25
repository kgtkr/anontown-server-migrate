use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtErrorPublic {
    pub code: &'static str,
    pub message: String,
    pub data: serde_json::Value,
}

#[derive(Debug)]
pub enum AtError {
    Captcha,
    Params(Vec<ParamErrorData>),
    Right(String),
    Conflict(String),
    Prerequisite(String),
    TokenAuth,
    Auth(String),
    UserAuth,
    NotFound(String),
    Internal(anyhow::Error),
}

impl AtError {
    fn to_message(&self) -> String {
        match self {
            AtError::Captcha => "キャプチャ認証に失敗".to_string(),
            AtError::Params(msg) => msg.clone(),
            AtError::Right(msg) => msg.clone(),
            AtError::Conflict(msg) => msg.clone(),
            AtError::Prerequisite(msg) => msg.clone(),
            AtError::TokenAuth => "認証に失敗しました".to_string(),
            AtError::Auth(msg) => msg.clone(),
            AtError::UserAuth => "認証に失敗しました".to_string(),
            AtError::NotFound(msg) => msg.clone(),
            AtError::Internal(_) => "内部エラーが発生しました".to_string(),
        }
    }

    fn to_code(&self) -> &'static str {
        match self {
            AtError::Captcha => "captcha",
            AtError::Params(_) => "params",
            AtError::Right(_) => "right",
            AtError::Conflict(_) => "conflict",
            AtError::Prerequisite(_) => "prerequisite",
            AtError::TokenAuth => "token_auth",
            AtError::Auth(_) => "auth",
            AtError::UserAuth => "user_auth",
            AtError::NotFound(_) => "not_found",
            AtError::Internal(_) => "internal",
        }
    }

    fn to_data(&self) -> serde_json::Value {
        match self {
            AtError::Params(errors) => serde_json::to_value(errors).unwrap(),
            _ => serde_json::Value::Null,
        }
    }

    pub fn to_public(&self) -> AtErrorPublic {
        AtErrorPublic {
            code: self.to_code(),
            message: self.to_message(),
            data: self.to_data(),
        }
    }
}

impl fmt::Display for AtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtError::Internal(e) => write!(f, "{}", e),
            _ => write!(f, "{}", self.to_message()),
        }
    }
}

impl std::error::Error for AtError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamErrorData {
    pub field: String,
    pub message: String,
}
