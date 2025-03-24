use async_trait::async_trait;
use log::{error, warn, info, debug, trace};
use crate::ports::logger::LoggerPort;

pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl LoggerPort for Logger {
    async fn error(&self, msg: &str) {
        error!("{}", msg);
    }

    async fn warn(&self, msg: &str) {
        warn!("{}", msg);
    }

    async fn info(&self, msg: &str) {
        info!("{}", msg);
    }

    async fn verbose(&self, msg: &str) {
        debug!("{}", msg);
    }

    async fn debug(&self, msg: &str) {
        debug!("{}", msg);
    }

    async fn silly(&self, msg: &str) {
        trace!("{}", msg);
    }
} 