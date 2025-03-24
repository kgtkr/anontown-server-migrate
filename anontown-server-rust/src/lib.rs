pub mod adapters;
pub mod entities;
pub mod ports;
pub mod schema;

use actix_web::web;
use juniper::http::GraphQLResponse;

pub struct Context {
    pub ports: ports::Ports,
}

impl Context {
    pub fn new(ports: ports::Ports) -> Self {
        Self { ports }
    }
}

impl juniper::Context for Context {}

pub type Schema = juniper::RootNode<'static, schema::Query, schema::Mutation, schema::Subscription>; 