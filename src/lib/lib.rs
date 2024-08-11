// src/lib/lib.rs

// module declarations
pub mod handlers;
pub mod startup;
pub mod telemetry;

// re-exports
pub use startup::*;
pub use telemetry::*;
