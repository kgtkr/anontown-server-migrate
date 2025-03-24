pub mod context;
pub mod input;
pub mod mutation;
pub mod query;
pub mod scalar;
pub mod subscription;
pub mod types;

#[cfg(test)]
mod types_test;

pub use query::Query;
pub use mutation::Mutation;
pub use subscription::Subscription;

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