// src/main.rs

// dependencies
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use jeff_mitchell_dev_lib::{get_subscriber, init_subscriber, MakeRequestUuid};
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::Level;

// health_check handler; returns a 200 OK with empty body
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // initialize tracing
    let subscriber = get_subscriber("jeff-mitchell-dev".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // build the router
    let api = Router::new()
        .route("/api/health_check", get(health_check))
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            DefaultMakeSpan::new()
                                .include_headers(true)
                                .level(Level::INFO),
                        )
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .propagate_x_request_id(),
        )
        .nest_service("/", ServeDir::new("public"));

    // start the service
    Ok(api.into())
}
