// src/lib/startup.rs

// dependencies
use crate::handlers::about;
use crate::handlers::blog;
use crate::handlers::handler_404;
use crate::handlers::health_check;
use crate::handlers::index;
use crate::handlers::music;
use crate::handlers::photography;
use crate::handlers::projects;
use axum::{routing::get, Router};
use shuttle_axum::AxumService;
use tower_http::services::ServeDir;

// configure function
pub async fn application() -> AxumService {
    let router = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/blog", get(blog))
        .route("/music", get(music))
        .route("/projects", get(projects))
        .route("/photography", get(photography))
        .route("/health_check", get(health_check))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(handler_404);

    router.into()
}
