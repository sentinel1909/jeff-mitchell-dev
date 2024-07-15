// src/lib/startup.rs
// build the server, including configuration and routes

// dependencies
use crate::config::ServerConfig;
use crate::handlers::health_check;
use anyhow::Result;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

// struct type to represent the server application
pub struct Application(pub Router);

// methods for the Application type
impl Application {
    pub fn build(ServerConfig {}: ServerConfig) -> Result<Application> {
        let app = Router::new()
            .route("/health_check", get(health_check))
            .nest_service("/", ServeDir::new("public"));

        Ok(Self(app))
    }
}
