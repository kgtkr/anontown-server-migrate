pub mod mutation;
pub mod query;
pub mod scalar;
pub mod subscription;
pub mod types;

pub use mutation::Mutation;
pub use query::Query;
pub use scalar::DateTimeScalar;
pub use subscription::Subscription;
pub use types::*;

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