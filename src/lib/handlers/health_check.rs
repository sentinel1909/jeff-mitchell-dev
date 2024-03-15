// src/lib/handlers/health_check.rs

// dependencies
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

// health_check handler
pub async fn health_check() -> impl IntoResponse {
    let body = r#"
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link rel="stylesheet" type="text/css" href="assets/screen.css" media="screen" />
            <link rel="icon" type="image/png" href="assets/favicon.png" />
            <title>jeff-mitchell-dev</title>
        </head>
        <body>
            <main>
                <section>
                    <p>200 OK</p>
                </section>
            </main>
        </body>
    "#;
    let resp = body.to_string();
    (StatusCode::OK, Html(resp))
}
