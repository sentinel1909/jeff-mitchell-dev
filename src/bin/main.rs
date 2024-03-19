// src/bin/main.rs

// dependencies
use jeff_mitchell_dev::startup::application;
use shuttle_axum::ShuttleAxum;
use sqlx::PgPool;

// the main function, calls the application function from startup.rs and returns a Result<AxumService, Error>
// (ShuttleAxum expands to: pub type = Result<AxumService, Error>)
#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> ShuttleAxum {
    Ok(application(pool).await?)
}
