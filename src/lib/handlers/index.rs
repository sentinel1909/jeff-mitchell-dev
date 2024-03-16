// src/lib/handlers/index.rs

// dependencies
use crate::templates::IndexTemplate;

// index handler function
pub async fn index() -> IndexTemplate {
    IndexTemplate {
        header: "Welcome!".to_string(),
        content: "This is my site.".to_string()
    }
}
