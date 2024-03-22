// src/lib/queries/get_articles.rs

// dependencies
use crate::domain::{AppState, Article};
use crate::errors::AppError;
use axum::extract::State;

// function to get all articles from the database
pub async fn get_articles(State(app_state): State<AppState>) -> Result<Vec<Article>, AppError> {
    let articles = sqlx::query_as::<_, Article>("SELECT * FROM articles ORDER BY article_date DESC")
        .fetch_all(&app_state.pool)
        .await?;
    Ok(articles)
}
