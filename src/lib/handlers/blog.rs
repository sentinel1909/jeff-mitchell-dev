// src/lib/handlers/blog.rs

// dependencies
use crate::templates::BlogTemplate;

// index handler function
pub async fn blog() -> BlogTemplate {
    BlogTemplate {
        header: "Latest Articles".to_string(),
        content: "Here are all the things I've written...".to_string()
    }
}
