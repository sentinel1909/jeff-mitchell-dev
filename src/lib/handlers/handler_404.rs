// src/lib/handlers/handler_404.rs

// dependencies
use crate::templates::NotFoundTemplate;
use axum::{http::StatusCode, response::IntoResponse};

// fallback handler for unknown routes
pub async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        NotFoundTemplate {
            error: "Content not found".to_string(),
        },
    )
}
