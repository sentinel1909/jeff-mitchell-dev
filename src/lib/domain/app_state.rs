// src/lib/domain/app_state.rs

// dependencies
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
}
