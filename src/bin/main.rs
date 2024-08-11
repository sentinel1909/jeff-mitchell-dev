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

    // define the tracing layer
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .include_headers(true)
                .level(Level::INFO),
        )
        .on_response(DefaultOnResponse::new().include_headers(true));

    // create public assets, wrap them in a trace layer
    let public_assets = ServiceBuilder::new()
        .layer(&trace_layer)
        .service(ServeDir::new("public"));

    // build the router
    let api = Router::new()
        .route("/api/health_check", get(health_check))
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(trace_layer)
                .propagate_x_request_id(),
        );

    // combine the api and public assets to make the app
    let app = Router::new()
        .nest_service("/api", api)
        .nest_service("/", public_assets);
        

    // start the service
    Ok(app.into())
}
