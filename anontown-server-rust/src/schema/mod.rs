mod query;
mod mutation;
mod subscription;
mod types;
mod scalar;

pub use query::Query;
pub use mutation::Mutation;
pub use subscription::Subscription;
pub use types::*;
pub use scalar::*;

#[derive(Clone)]
pub struct Schema(juniper::RootNode<'static, Query, Mutation, Subscription>);

impl Schema {
    pub fn new(query: Query, mutation: Mutation, subscription: Subscription) -> Self {
        Self(juniper::RootNode::new(query, mutation, subscription))
    }
}

impl std::ops::Deref for Schema {
    type Target = juniper::RootNode<'static, Query, Mutation, Subscription>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
} 