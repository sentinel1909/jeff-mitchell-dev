// src/lib/errors/error.rs

// dependencies
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("File read error: {0}")]
    FileRead(std::io::Error),
    #[error("Directory read error: {0}")]
    DirectoryRead(std::io::Error),
    #[error("Deserialization error: {0}")]
    DeserializeFrontMatter(serde_json::error::Error),
    #[error("Database error: {0}")]
    Database(sqlx::Error),
    #[error("Regex error: {0}")]
    Regex(regex::Error),
}

// implement the From trait for sqlx::Error, for use in the AppError enum
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}
