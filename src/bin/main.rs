// src/bin/main.rs

// dependencies
use jeff_mitchell_dev::startup::configure;

// the main function, spawns and runs the app
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    configure().await
}
