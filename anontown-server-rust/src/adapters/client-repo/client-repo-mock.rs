use async_trait::async_trait;
use std::collections::HashMap;
use crate::models::Client;
use crate::ports::client::{ClientPort, ClientQuery};

pub struct ClientRepoMock {
    clients: HashMap<String, Client>,
}

impl ClientRepoMock {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
}

#[async_trait]
impl ClientPort for ClientRepoMock {
    async fn find_one(&mut self, id: &str) -> Result<Client, Box<dyn std::error::Error>> {
        self.clients
            .get(id)
            .cloned()
            .ok_or_else(|| "Client not found".into())
    }

    async fn find(&mut self, query: &ClientQuery) -> Result<Vec<Client>, Box<dyn std::error::Error>> {
        let mut clients = self.clients.values().cloned().collect::<Vec<_>>();

        // IDでフィルタリング
        if let Some(ids) = &query.id {
            clients.retain(|c| ids.contains(&c.id));
        }

        // ユーザーIDでフィルタリング
        if let Some(self_) = query.self_ {
            clients.retain(|c| c.user_id == self_);
        }

        // 作成日時でソート
        clients.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(clients)
    }

    async fn insert(&mut self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        self.clients.insert(client.id.clone(), client.clone());
        Ok(())
    }

    async fn update(&mut self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        if self.clients.contains_key(&client.id) {
            self.clients.insert(client.id.clone(), client.clone());
            Ok(())
        } else {
            Err("Client not found".into())
        }
    }
} 