// src/lib/handlers/handler_404.rs

// dependencies
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

// fallback handler for unknown routes
pub async fn handler_404() -> impl IntoResponse {
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
                    <p>404 Not Found</p>
                </section>
            </main>
        </body>
    "#;
    let resp = body.to_string();
    (StatusCode::NOT_FOUND, Html(resp))
}
