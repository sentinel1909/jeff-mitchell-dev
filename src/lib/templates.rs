// src/lib/templates.rs

// dependencies
use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
  pub header: String,
  pub content: String
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
  pub header: String,
  pub content: String
}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {
  pub header: String,
  pub content: String
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {
  pub header: String,
  pub content: String
}
