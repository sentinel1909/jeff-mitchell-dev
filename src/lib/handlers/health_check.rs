// src/lib/handlers/health_check.rs

use axum::{http::StatusCode, response::IntoResponse};

// health_check endpoint handler
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
