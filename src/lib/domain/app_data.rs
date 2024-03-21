// src/lib/domain/app_data.rs

// dependencies
use serde::Deserialize;
use sqlx::FromRow;

// struct to represent the front matter from a particular post
#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub category: String,
    pub tag: String,
    pub summary: String,
}

// implement the Default trait for the Frontmatter type
impl Default for FrontMatter {
    fn default() -> Self {
        FrontMatter {
            title: "".to_string(),
            date: "".to_string(),
            slug: "".to_string(),
            category: "".to_string(),
            tag: "".to_string(),
            summary: "".to_string(),
        }
    }
}

// struct to represent the body content from a particular post
#[derive(Debug, Deserialize)]
pub struct BodyContent {
    pub content: String,
}

// implement the default trait for the BodyContent type
impl Default for BodyContent {
    fn default() -> Self {
        BodyContent {
            content: "".to_string(),
        }
    }
}

// struct to represent an article
#[derive(Debug, Deserialize, FromRow)]
pub struct Article {
    #[sqlx(rename = "article_id")]
    pub id: i32,
    #[sqlx(rename = "article_title")]
    pub title: String,
    #[sqlx(rename = "article_date")]
    pub date: String,
    #[sqlx(rename = "article_slug")]
    pub slug: String,
    #[sqlx(rename = "article_category")]
    pub category: String,
    #[sqlx(rename = "article_tag")]
    pub tag: String,
    #[sqlx(rename = "article_summary")]
    pub summary: String,
    #[sqlx(rename = "article_content")]
    pub content: String,
}

// struct to represent path segments for the article handler
#[derive(Debug, Deserialize)]
pub struct Segments {
    pub date: String,
    pub slug: String,
}
