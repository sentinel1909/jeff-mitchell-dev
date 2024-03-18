// src/lib/utilities.rs

// dependencies
use crate::domain::app_data::{BodyContent, FrontMatter};
use crate::errors::error::AppError;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use regex::Regex;
use std::fs;
use std::path::Path;

// utility function to grab the front matter from each markdown article
pub fn get_frontmatters() -> Result<Vec<FrontMatter>, AppError> {
    let folder_path = Path::new("./content");
    let mut front_matters = Vec::new();
    for entry in fs::read_dir(folder_path).map_err(AppError::FileRead)? {
        let entry = entry.map_err(AppError::DirectoryRead)?;
        let path = entry.path();
        if path.is_file() {
            let content = fs::read_to_string(&path).map_err(AppError::FileRead)?;
            let matter = Matter::<YAML>::new().parse(&content);
            let front_matter: FrontMatter = matter
                .data
                .as_ref()
                .map(|data| data.deserialize())
                .transpose()
                .map_err(AppError::DeserializeFrontMatter)?
                .unwrap_or_default();
            front_matters.push(front_matter);
        }
    }
    Ok(front_matters)
}

// utility function to grab the body content from each markdown article
pub fn get_bodies() -> Result<Vec<BodyContent>, AppError> {
    let folder_path = Path::new("./content");
    let mut bodies = Vec::new();
    for entry in fs::read_dir(folder_path).map_err(AppError::FileRead)? {
        let entry = entry.map_err(AppError::DirectoryRead)?;
        let path = entry.path();
        if path.is_file() {
            let content = fs::read_to_string(&path).map_err(AppError::FileRead)?;
            let frontmatter_regex =
                Regex::new(r"---\s*\n(?s:.+?)\n---\s*\n").map_err(AppError::Regex)?;
            let body: BodyContent = BodyContent {
                content: frontmatter_regex.replace(&content, "").to_string(),
            };
            bodies.push(body);
        }
    }
    Ok(bodies)
}
