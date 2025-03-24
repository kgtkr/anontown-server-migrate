use async_trait::async_trait;
use crate::ports::ip::IpPort;

pub struct IpContainer {
    ip: Option<String>,
}

impl IpContainer {
    pub fn new(ip: Option<String>) -> Self {
        Self { ip }
    }
}

#[async_trait]
impl IpPort for IpContainer {
    async fn get_ip(&self) -> Option<String> {
        self.ip.clone()
    }
} 