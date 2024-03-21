// src/lib/handlers/article.rs

// dependencies
use crate::domain::{AppState, Segments};
use crate::queries::get_article;
use crate::templates::{ArticleTemplate, NotFoundTemplate};
use crate::utilities::convert_markdown;
use askama_axum::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use axum_macros::debug_handler;

// enum to represent the response from the article handler function, wraps the template types
enum ArticleHandlerResponse {
    Article(ArticleTemplate),
    NotFound(NotFoundTemplate),
}

// implement the IntoResponse template for the blog handler response type
impl IntoResponse for ArticleHandlerResponse {
    fn into_response(self) -> Response {
        match self {
            ArticleHandlerResponse::Article(template) => match template.render() {
                Ok(content) => Html(content).into_response(),
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
                }
            },
            ArticleHandlerResponse::NotFound(template) => match template.render() {
                Ok(content) => Html(content).into_response(),
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
                }
            },
        }
    }
}

// articles handler function
#[debug_handler]
pub async fn article(
    State(app_state): State<AppState>,
    Path(segments): Path<Segments>,
) -> impl IntoResponse {
    match get_article(segments, axum::extract::State(app_state)).await {
        Ok(article) => ArticleHandlerResponse::Article(ArticleTemplate {
            article: convert_markdown(article.content),
        }),
        Err(e) => ArticleHandlerResponse::NotFound(NotFoundTemplate {
            error: e.to_string(),
        }),
    }
}
