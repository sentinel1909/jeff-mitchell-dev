// src/main.rs

// dependencies
use jeff_mitchell_dev_lib::{build, get_subscriber, init_subscriber};

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // initialize tracing
    let subscriber = get_subscriber("jeff-mitchell-dev".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // build the router
    let application = build();

    // start the service
    Ok(application.into())
}
