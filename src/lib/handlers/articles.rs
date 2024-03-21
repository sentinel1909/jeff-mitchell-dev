// src/lib/handlers/blog.rs

// dependencies
use crate::domain::AppState;
use crate::queries::get_articles;
use crate::templates::{ArticlesTemplate, NotFoundTemplate};
use askama_axum::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

// enum to represent the response from the blog handler function, wraps the template types
enum BlogHandlerResponse {
    Articles(ArticlesTemplate),
    NotFound(NotFoundTemplate),
}

// implement the IntoResponse template for the blog handler response type
impl IntoResponse for BlogHandlerResponse {
    fn into_response(self) -> Response {
        match self {
            BlogHandlerResponse::Articles(template) => match template.render() {
                Ok(content) => Html(content).into_response(),
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
                }
            },
            BlogHandlerResponse::NotFound(template) => match template.render() {
                Ok(content) => Html(content).into_response(),
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
                }
            },
        }
    }
}

// blog handler function
pub async fn articles(State(app_state): State<AppState>) -> impl IntoResponse {
    match get_articles(axum::extract::State(app_state)).await {
        Ok(articles) => BlogHandlerResponse::Articles(ArticlesTemplate { articles }),
        Err(e) => BlogHandlerResponse::NotFound(NotFoundTemplate {
            error: e.to_string(),
        }),
    }
}
