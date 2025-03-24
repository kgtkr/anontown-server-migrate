use crate::models::{client::ClientAPI, history::HistoryAPI, profile::ProfileAPI};
use crate::usecases::{get_client, get_history, get_profile};
use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

use super::Context;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn history(context: &Context, id: String) -> FieldResult<HistoryAPI> {
        Ok(get_history(
            &id,
            &mut *context.history_loader,
            &*context.auth_container,
        )
        .await?)
    }

    async fn profile(context: &Context, id: String) -> FieldResult<ProfileAPI> {
        Ok(get_profile(
            &id,
            &mut *context.profile_loader,
            &*context.auth_container,
        )
        .await?)
    }

    async fn client(context: &Context, id: String) -> FieldResult<ClientAPI> {
        Ok(get_client(
            &id,
            &mut *context.client_loader,
            &*context.auth_container,
        )
        .await?)
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
} 