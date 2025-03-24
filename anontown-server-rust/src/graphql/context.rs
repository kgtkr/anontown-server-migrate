use crate::ports::{
    auth::AuthPort,
    client_loader::ClientLoaderPort,
    history_loader::HistoryLoaderPort,
    profile_loader::ProfileLoaderPort,
};

pub struct Context {
    pub auth_container: Box<dyn AuthPort>,
    pub history_loader: Box<dyn HistoryLoaderPort>,
    pub profile_loader: Box<dyn ProfileLoaderPort>,
    pub client_loader: Box<dyn ClientLoaderPort>,
}

impl juniper::Context for Context {} 