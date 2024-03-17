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
#[template(path = "music.html")]
pub struct MusicTemplate {}

#[derive(Template)]
#[template(path = "photography.html")]
pub struct PhotographyTemplate {}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate {}
