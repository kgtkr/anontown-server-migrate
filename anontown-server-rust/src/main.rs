use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use std::env;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use juniper::http::playground::playground_source;

mod context;
mod error;
mod handlers;
mod routes;
mod schema;
mod ports;
mod entities;

use context::Context;
use schema::{Query, Mutation, Subscription, Schema};

async fn health_check() -> impl Responder {
    serde_json::json!({
        "status": "ok",
        "message": "Anontown Server is running"
    })
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql", None))
}

async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", None))
}

async fn graphql(
    schema: web::Data<Schema>,
    context: web::Data<Context>,
    req: web::Json<GraphQLRequest>,
) -> HttpResponse {
    let res = req.execute(&schema, &context).await;
    match res {
        Ok(graphql_response) => {
            if graphql_response.is_ok() {
                HttpResponse::Ok().json(graphql_response)
            } else {
                HttpResponse::BadRequest().json(graphql_response)
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize logger
    env_logger::init();

    // Get the port from environment variable or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    // Get the host from environment variable or use default
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    // Initialize database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Initialize Redis connection
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis = redis::Client::open(redis_url).expect("Failed to create Redis client");

    // Create context
    let context = Context { db: pool, redis };

    // Create schema
    let schema = Schema::new(Query, Mutation, Subscription);

    log::info!("Starting server at {}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(context.clone()))
            .route("/graphql", web::post().to(graphql))
            .route("/graphiql", web::get().to(graphiql))
            .route("/playground", web::get().to(graphql_playground))
    })
    .bind((host, port))?
    .run()
    .await
} 