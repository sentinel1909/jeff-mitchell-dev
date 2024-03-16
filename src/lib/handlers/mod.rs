// src/lib/handlers/mod.rs

pub mod about;
pub mod blog;
pub mod handler_404;
pub mod health_check;
pub mod index;
pub mod projects;

pub use about::*;
pub use blog::*;
pub use handler_404::*;
pub use health_check::*;
pub use index::*;
pub use projects::*;
