// src/main.rs

// dependencies
use jeff_mitchell_dev_lib::service::JeffMitchellDevService;
use jeff_mitchell_dev_lib::{build, get_subscriber, init_subscriber};
use shuttle_runtime::Error;

// main function
#[shuttle_runtime::main]
async fn main() -> Result<JeffMitchellDevService, Error> {
    // initialize tracing
    let subscriber = get_subscriber("jeff-mitchell-dev".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // build the router
    let app_router = build();

    // start the service
    Ok(JeffMitchellDevService { app_router })
}
