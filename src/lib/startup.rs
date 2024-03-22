// src/lib/startup.rs

// dependencies
use crate::domain::AppState;
use crate::handlers::{
    about, admin, article, articles, handler_404, health_check, index, music, photography, projects,
};
use crate::queries::add_articles;
use crate::utilities::{get_bodies, get_frontmatters};
use axum::{routing::get, Router};
use shuttle_axum::AxumService;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use tower_http::services::ServeDir;

// configure function
pub async fn application(pool: PgPool) -> Result<AxumService, CustomError> {
    pool.execute(include_str!(
        "../../migrations/20240317203318_jdm-dev-db.up.sql"
    ))
    .await
    .map_err(CustomError::new)?;

    let app_state = AppState { pool };

    let front_matters = get_frontmatters()
        .expect("Unable to retrieve frontmatter content from markdown files, cannot continue...");

    let bodies = get_bodies()
        .expect("Unable to retrieve body content from markdown files, cannot continue...");

    add_articles(
        axum::extract::State(app_state.clone()),
        front_matters,
        bodies,
    )
    .await
    .unwrap();

    let router = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/admin", get(admin))
        .route("/articles", get(articles))
        .route("/article/:date/:slug", get(article))
        .route("/music", get(music))
        .route("/projects", get(projects))
        .route("/photography", get(photography))
        .route("/health_check", get(health_check))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(handler_404)
        .with_state(app_state);

    Ok(router.into())
}
