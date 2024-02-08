use std::io;
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
        let room = Store::get_room(&id).unwrap_or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                let new_room = Room {
                    id: id.clone(),
                    board: Board::new(),
                };
                Store::insert_room(id.clone(), new_room.clone()).unwrap();
                new_room
            } else {
                panic!("{:?}", e);
            }
        });
        RoomTemplate {
            title: format!("Room {}", id.clone()),
            board: BoardTemplate::from(&room),
            id: id.clone(),
            side: Side::White,
        }
    }

    pub async fn reset_room(Path(id): Path<String>) -> impl IntoResponse {
        let new_room = Room {
            board: Board::new(),
            id: id.clone(),
        };
        Store::insert_room(id.clone(), new_room.clone()).unwrap();
        RoomTemplate {
            id: id.clone(),
            title: id.clone(),
            board: BoardTemplate::from(&new_room),
            side: Side::White,
        }
    }
}
