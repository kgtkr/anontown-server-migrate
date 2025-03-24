use actix_web::web;
use actix_web::Scope;

use crate::handlers;

pub fn create_scope() -> Scope {
    web::scope("")
        .route("/", web::get().to(handlers::health_check))
        // Add more routes here as needed
} 