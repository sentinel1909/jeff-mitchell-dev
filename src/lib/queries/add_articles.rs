// src/lib/queries/add_articles.rs

// dependencies
use crate::domain::{AppState, BodyContent, FrontMatter};
use crate::errors::AppError;
use axum::extract::State;

// function to initialize the articles database by inserting the frontmatters and content
// pulled from the content markdown files
pub async fn add_articles(
    State(app_state): State<AppState>,
    front_matters: Vec<FrontMatter>,
    bodies: Vec<BodyContent>,
) -> Result<(), AppError> {
    for (front_matter, body) in front_matters.into_iter().zip(bodies.into_iter()) {
        let _insert_frontmatter = sqlx::query("INSERT INTO articles (article_title, article_date, article_slug, article_category, article_tag, article_summary, article_content) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (article_title) DO NOTHING")
            .bind(front_matter.title)
            .bind(front_matter.date)
            .bind(front_matter.slug)
            .bind(front_matter.category)
            .bind(front_matter.tag)
            .bind(front_matter.summary)
            .bind(body.content)
            .execute(&app_state.pool)
            .await?;
        }
    Ok(())
}
