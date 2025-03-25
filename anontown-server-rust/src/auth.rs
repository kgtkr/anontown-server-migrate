
#[derive(Debug, Clone)]
pub struct AuthTokenBase {
    pub id: String,
    pub key: String,
    pub user: String,
}

#[derive(Debug, Clone)]
pub struct AuthTokenMaster {
    pub base: AuthTokenBase,
}

#[derive(Debug, Clone)]
pub struct AuthTokenGeneral {
    pub base: AuthTokenBase,
    pub client: String,
}

#[derive(Debug, Clone)]
pub enum AuthToken {
    Master(AuthTokenMaster),
    General(AuthTokenGeneral),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub pass: String,
}
