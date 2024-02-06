use askama_axum::IntoResponse;
use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};

use crate::{
    engine::Board,
    store::Store,
    templates::{BoardTemplate, RoomTemplate, Side},
    Room,
};

pub struct RoomsRouter {}

impl RoomsRouter {
    pub fn get() -> Router {
        Router::new().nest(
            "/:id",
            Router::new()
                .route("/", get(Self::get_room))
                .route("/reset", post(Self::reset_room)),
        )
    }

    pub async fn get_room(Path(id): Path<String>) -> impl IntoResponse {
        let room = Store::get_room(id).unwrap();
        RoomTemplate {
            title: format!("Room {}", room.id.clone()),
            board: BoardTemplate::from(&room),
            id: room.id.clone(),
            side: Side::White,
        }
    }

    pub async fn reset_room(Path(id): Path<String>) -> impl IntoResponse {
        let room = Store::get_room(id).unwrap();
        let new_room = Room {
            board: Board::new(),
            id: room.id.clone(),
        };
        Store::insert_room(room.id.clone(), new_room.clone());
        RoomTemplate {
            id: room.id.clone(),
            title: room.id.clone(),
            board: BoardTemplate::from(&room),
            side: Side::White,
        }
    }
}
