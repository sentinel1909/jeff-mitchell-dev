// src/lib/handlers/about.rs

// dependencies
use crate::templates::AboutTemplate;

// index handler function
pub async fn about() -> AboutTemplate {
    AboutTemplate {
        
        header: "About Me".to_string(),
        content: "All about me...".to_string()
    }
}
