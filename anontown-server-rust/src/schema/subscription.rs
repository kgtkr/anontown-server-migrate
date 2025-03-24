use juniper::{graphql_subscription, FieldResult, ID};
use futures::Stream;

use crate::schema::types::{ResType, ResSubscript};

pub struct Subscription;

#[graphql_subscription]
impl Subscription {
    async fn res_added(&self, topic: ID) -> impl Stream<Item = FieldResult<ResSubscript>> {
        // TODO: 実装
        futures::stream::empty()
    }
} 