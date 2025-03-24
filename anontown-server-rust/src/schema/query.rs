use juniper::{graphql_object, FieldResult, ID};

use crate::schema::types::{
    ClientType, HistoryType, ProfileType, ResType, StorageType, TopicType, UserType,
};

pub struct Query;

#[graphql_object]
impl Query {
    async fn user(&self, context: &Context, id: Option<ID>) -> FieldResult<UserType> {
        let user_id = match id {
            Some(id) => id,
            None => context.ports.auth_container.get_token().user,
        };
        let user = context.ports.user_repo.find_one(user_id).await?;
        Ok(UserType::from(user))
    }

    async fn user_id(&self, sn: String) -> FieldResult<ID> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn user_sn(&self, id: ID) -> FieldResult<String> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn users(&self) -> FieldResult<Vec<UserType>> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn client(&self, id: ID) -> FieldResult<ClientType> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn clients(&self) -> FieldResult<Vec<ClientType>> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn token(&self) -> FieldResult<TokenType> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn tokens(&self) -> FieldResult<Vec<TokenType>> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn topic(&self, id: ID) -> FieldResult<TopicType> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn topics(&self, limit: Option<i32>, offset: Option<i32>) -> FieldResult<Vec<TopicType>> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn topic_tags(&self, limit: Option<i32>) -> FieldResult<Vec<TagType>> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn res(&self, id: ID) -> FieldResult<ResType> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn reses(&self, topic_id: ID, limit: Option<i32>, offset: Option<i32>) -> FieldResult<Vec<ResType>> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn history(&self, id: ID) -> FieldResult<HistoryType> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn histories(&self, limit: Option<i32>) -> FieldResult<Vec<HistoryType>> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn profile(&self, id: ID) -> FieldResult<ProfileType> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn profiles(&self) -> FieldResult<Vec<ProfileType>> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn storage(&self, key: String) -> FieldResult<StorageType> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }

    async fn storages(&self) -> FieldResult<Vec<StorageType>> {
        // TODO: 実装
        Err(juniper::FieldError::new("Not implemented", juniper::graphql_value!(null)))
    }
} 