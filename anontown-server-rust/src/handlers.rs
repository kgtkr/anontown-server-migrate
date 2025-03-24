use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Anontown Server is running"
    }))
} 