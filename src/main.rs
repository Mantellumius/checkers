use axum::{response::IntoResponse, routing::get, Router};
use engine::Board;
use serde::{Deserialize, Serialize};
use std::env;
use store::Store;

mod engine;
mod routes;
mod store;
mod templates;
mod utility;

pub use engine::{Cell, Checker};
use routes::{GamesRouter, RoomsRouter, WSRouter};
use templates::{IndexTemplate, RoomHref};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let public = ServeDir::new("public");
    let app = Router::new()
        .route("/", get(index))
        .nest("/ws", WSRouter::get())
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
    let rooms = Store::get_rooms().unwrap();
    let mut room_hrefs = rooms
        .values()
        .map(|r| RoomHref {
            id: r.id.clone(),
            title: format!("Room {}", r.id.clone()),
        })
        .collect::<Vec<RoomHref>>();
    room_hrefs.sort_by_key(|key| key.id.clone());
    IndexTemplate {
        title: "Checkers".to_string(),
        rooms: room_hrefs,
    }
}

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct Room {
    id: String,
    board: Board,
}
