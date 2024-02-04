use axum::{response::IntoResponse, routing::get, Router};
use engine::Board;
use serde::{Deserialize, Serialize};
use std::env;

mod engine;
mod routes;
mod store;
mod templates;
mod utility;

pub use engine::{Cell, Checker};
use routes::{GamesRouter, RoomsRouter};
use templates::IndexTemplate;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let public = ServeDir::new("public");
    let app = Router::new()
        .route("/", get(index))
        .nest("/rooms", RoomsRouter::get())
        .nest("/games", GamesRouter::get())
        .nest_service("/assets", public)
        .layer(TraceLayer::new_for_http());
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
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

#[derive(Clone, Deserialize, Serialize)]
pub struct Room {
    id: String,
    board: Board,
}
