// src/lib/handlers/health_check

// dependencies
use axum::{http::StatusCode, response::IntoResponse};

// health_check handler; returns a 200 OK with empty body
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
