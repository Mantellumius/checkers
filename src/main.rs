use axum::{response::IntoResponse, routing::get, Router};
use std::env;

mod templates;

use templates::IndexTemplate;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let public = ServeDir::new("public");
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/assets", public);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    println!("Listening on port {port}");
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    IndexTemplate {
        title: "Checkers".to_string(),
    }
}
