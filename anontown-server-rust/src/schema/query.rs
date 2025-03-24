use juniper::{graphql_object, FieldResult, ID};

use crate::schema::types::{
    ClientType, HistoryType, ProfileType, ResType, StorageType, TopicType, UserType, ToSchemaType,
};
use crate::schema::input::{ResQuery, TopicQuery};
use crate::schema::context::Context;

pub struct Query;

#[graphql_object]
impl Query {
    async fn user(&self, context: &Context, id: Option<ID>) -> FieldResult<UserType> {
        let user_id = match id {
            Some(id) => id,
            None => context.ports.auth_container.get_token().user,
        };
        let user = context.ports.user_repo.find_one(user_id).await?;
        Ok(user.to_schema_type(&context.ports.auth_container))
    }

    async fn user_id(&self, sn: String, context: &Context) -> FieldResult<ID> {
        let id = context.ports.user_repo.find_id(&sn).await?;
        Ok(id)
    }

    async fn user_sn(&self, id: ID, context: &Context) -> FieldResult<String> {
        let user = context.ports.user_repo.find_one(&id).await?;
        Ok(user.sn)
    }

    async fn users(&self, context: &Context) -> FieldResult<Vec<UserType>> {
        let users = context.ports.user_repo.find_all().await?;
        Ok(users.into_iter().map(|u| u.to_schema_type(&context.ports.auth_container)).collect())
    }

    async fn client(&self, id: ID, context: &Context) -> FieldResult<ClientType> {
        let client = context.ports.client_repo.find_one(&id).await?;
        Ok(client.to_schema_type(&context.ports.auth_container))
    }

    async fn clients(&self, context: &Context) -> FieldResult<Vec<ClientType>> {
        let clients = context.ports.client_repo.find_all().await?;
        Ok(clients.into_iter().map(|c| c.to_schema_type(&context.ports.auth_container)).collect())
    }

    async fn token(&self, context: &Context) -> FieldResult<TokenType> {
        let token = context.ports.token_repo.find_one(
            context.ports.auth_container.get_token().id,
        ).await?;
        Ok(token.to_schema_type(&context.ports.auth_container))
    }

    async fn tokens(&self, context: &Context) -> FieldResult<Vec<TokenType>> {
        let tokens = context.ports.token_repo.find_all().await?;
        Ok(tokens.into_iter().map(|t| t.to_schema_type(&context.ports.auth_container)).collect())
    }

    async fn topic(&self, id: ID, context: &Context) -> FieldResult<TopicType> {
        let topic = context.ports.topic_repo.find_one(&id).await?;
        Ok(topic.to_schema_type(&context.ports.auth_container))
    }

    async fn topics(
        &self,
        query: TopicQuery,
        skip: i32,
        limit: i32,
        context: &Context,
    ) -> FieldResult<Vec<TopicType>> {
        let topics = context.ports.topic_repo.find(query, skip, limit).await?;
        Ok(topics.into_iter().map(|t| t.to_schema_type(&context.ports.auth_container)).collect())
    }

    async fn topic_tags(
        &self,
        limit: i32,
        context: &Context,
    ) -> FieldResult<Vec<TagType>> {
        let tags = context.ports.topic_repo.find_tags(limit).await?;
        Ok(tags.into_iter().map(|(name, count)| TagType { name, count }).collect())
    }

    async fn res(&self, id: ID, context: &Context) -> FieldResult<ResType> {
        let res = context.ports.res_repo.find_one(&id).await?;
        Ok(res.to_schema_type(&context.ports.auth_container))
    }

    async fn reses(
        &self,
        query: ResQuery,
        limit: i32,
        context: &Context,
    ) -> FieldResult<Vec<ResType>> {
        let reses = context.ports.res_repo.find(query, limit).await?;
        Ok(reses.into_iter().map(|r| r.to_schema_type(&context.ports.auth_container)).collect())
    }

    async fn history(&self, id: ID, context: &Context) -> FieldResult<HistoryType> {
        let history = context.ports.history_repo.find_one(&id).await?;
        Ok(history.to_schema_type(&context.ports.auth_container))
    }

    async fn histories(
        &self,
        query: HistoryQuery,
        limit: i32,
        context: &Context,
    ) -> FieldResult<Vec<HistoryType>> {
        let histories = context.ports.history_repo.find(query, limit).await?;
        Ok(histories.into_iter().map(|h| h.to_schema_type(&context.ports.auth_container)).collect())
    }

    async fn profile(&self, id: ID, context: &Context) -> FieldResult<ProfileType> {
        let profile = context.ports.profile_repo.find_one(&id).await?;
        Ok(profile.to_schema_type(&context.ports.auth_container))
    }

    async fn profiles(&self, context: &Context) -> FieldResult<Vec<ProfileType>> {
        let profiles = context.ports.profile_repo.find_all().await?;
        Ok(profiles.into_iter().map(|p| p.to_schema_type(&context.ports.auth_container)).collect())
    }

    async fn storage(&self, key: String, context: &Context) -> FieldResult<StorageType> {
        let storage = context.ports.storage_repo.find_one(&key).await?;
        Ok(storage.to_schema_type(&context.ports.auth_container))
    }

    async fn storages(&self, context: &Context) -> FieldResult<Vec<StorageType>> {
        let storages = context.ports.storage_repo.find_all().await?;
        Ok(storages.into_iter().map(|s| s.to_schema_type(&context.ports.auth_container)).collect())
    }
} 