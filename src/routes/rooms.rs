use askama_axum::IntoResponse;
use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};

use crate::{
    engine::Board,
    store::Store,
    templates::{BoardTemplate, RoomTemplate},
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
            title: room.id.clone(),
            board: BoardTemplate {
                board: room.board,
                id: room.id.clone(),
            },
            id: room.id.clone(),
        }
    }

    pub async fn reset_room(Path(id): Path<String>) -> impl IntoResponse {
        let room = Store::get_room(id).unwrap();
        let new_room = Room {
            board: Board::new(),
            id: room.id.clone(),
        };
        Store::insert_room(room.id.clone(), new_room.clone());
        RoomTemplate::from(new_room)
    }
}
