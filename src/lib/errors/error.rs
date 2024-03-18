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
    #[error("Regex error: {0}")]
    Regex(regex::Error),
}
