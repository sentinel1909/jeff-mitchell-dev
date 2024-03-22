// src/lib/handlers/admin.rs

// dependencies
use crate::templates::AdminTemplate;
use axum::response::IntoResponse;

// fallback handler for unknown routes
pub async fn admin() -> impl IntoResponse {
    AdminTemplate {}
}
