// src/lib/handlers/mod.rs

pub mod handler_404;
pub mod health_check;

pub use handler_404::*;
pub use health_check::*;
