// src/lib/handlers/blog.rs

// dependencies
use crate::templates::ProjectsTemplate;

// index handler function
pub async fn projects() -> ProjectsTemplate {
    ProjectsTemplate {
        header: "Projects".to_string(),
        content: "These are the current projects I'm working on...".to_string()
    }
}
