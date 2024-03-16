// src/lib/templates.rs

// dependencies
use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {}
