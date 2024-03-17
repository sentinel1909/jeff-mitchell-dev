// src/bin/main.rs

// dependencies
use jeff_mitchell_dev::startup::application;
use shuttle_axum::ShuttleAxum;

// the main function, calls the application function from startup.rs and returns a Result<AxumService, Error>
// (ShuttleAxum expands to: pub type = Result<AxumServcie, Error>)
#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    Ok(application().await)
}
