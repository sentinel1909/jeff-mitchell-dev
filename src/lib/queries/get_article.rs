// src/lib/queries/get_articles.rs

// dependencies
use crate::domain::{AppState, Article, Segments};
use crate::errors::AppError;
use axum::extract::State;

// function to get all articles from the database
pub async fn get_article(
    segments: Segments,
    State(app_state): State<AppState>,
) -> Result<Article, AppError> {
    let article = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE article_date = $1 AND article_slug = $2 LIMIT 1",
    )
    .bind(segments.date)
    .bind(segments.slug)
    .fetch_one(&app_state.pool)
    .await?;
    Ok(article)
}
