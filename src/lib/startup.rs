// src/lib/startup.rs

// dependencies
use crate::handlers::health_check;
use crate::telemetry::MakeRequestUuid;
use axum::{http::HeaderName, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

// function to build and return a configured application that serves out the Zola built public assets
pub fn build() -> Router {
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
        .service(ServeDir::new("public").append_index_html_on_directories(true));

    // build the router and wrap it with the telemetry layers
    let x_request_id = HeaderName::from_static("x-request-id");
    let api = Router::new()
        .route("/health_check", get(health_check))
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    x_request_id.clone(),
                    MakeRequestUuid,
                ))
                .layer(trace_layer)
                .layer(PropagateRequestIdLayer::new(x_request_id)),
        );

    // combine the api and public assets to make the app
    Router::new()
        .nest_service("/api", api)
        .nest_service("/", public_assets)
}
