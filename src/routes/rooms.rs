use askama_axum::IntoResponse;
use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};

use crate::{
    engine::Board,
    store::Store,
    templates::{BoardTemplate, RoomTemplate},
};

pub struct RoomsRouter {}

impl RoomsRouter {
    pub fn get() -> Router {
        Router::new().route("/:id", get(Self::get_room))
    }

    pub async fn get_room(Path(id): Path<String>) -> impl IntoResponse {
        let room = Store::get_room(id).unwrap();
        RoomTemplate {
            title: room.name,
            board: BoardTemplate { board: room.board, id: room.id.clone() },
            id: room.id.clone(),
        }
    }
}
