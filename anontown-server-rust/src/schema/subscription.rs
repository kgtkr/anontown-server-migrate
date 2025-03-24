use juniper::{FieldResult, GraphQLObject};
use crate::schema::types::{ResType, ResSubscript};
use crate::ports::ResPort;

pub struct Subscription;

#[juniper::graphql_subscription]
impl Subscription {
    pub async fn res_added(
        &self,
        topic_id: String,
        res_port: &dyn ResPort,
    ) -> FieldResult<ResSubscript> {
        let (res, count) = res_port.subscribe(topic_id).await?;
        Ok(ResSubscript {
            res: ResType::from(res),
            count,
        })
    }
} 