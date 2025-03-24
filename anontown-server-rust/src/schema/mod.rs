mod query;
mod mutation;
mod subscription;
mod types;

pub use query::Query;
pub use mutation::Mutation;
pub use subscription::Subscription;
pub use types::*;

pub type Schema = juniper::RootNode<'static, Query, Mutation, Subscription>; 