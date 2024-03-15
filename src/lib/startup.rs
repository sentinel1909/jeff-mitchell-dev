// src/lib/startup.rs

// dependencies
use crate::handlers::handler_404;
use crate::handlers::health_check;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

// configure function
pub async fn configure() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/health_check", get(health_check))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(handler_404);

    Ok(router.into())
}
