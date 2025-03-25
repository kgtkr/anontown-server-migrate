use crate::{AuthToken, AuthTokenMaster, AtResult};
use std::option::Option;

pub trait AuthContainer {
    fn get_token(&self) -> AtResult<&AuthToken>;
    fn get_token_master(&self) -> AtResult<&AuthTokenMaster>;
    fn get_token_or_null(&self) -> Option<&AuthToken>;
    fn get_token_master_or_null(&self) -> Option<&AuthTokenMaster>;
} 