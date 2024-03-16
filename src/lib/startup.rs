// src/lib/startup.rs

// dependencies
use crate::handlers::about;
use crate::handlers::blog;
use crate::handlers::handler_404;
use crate::handlers::health_check;
use crate::handlers::index;
use crate::handlers::projects;
use crate::utilities::convert_to_html;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

// configure function
pub async fn configure() -> shuttle_axum::ShuttleAxum {
    convert_to_html().unwrap();
    let router = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/blog", get(blog))
        .route("/projects", get(projects))
        .route("/health_check", get(health_check))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(handler_404);

    Ok(router.into())
}
