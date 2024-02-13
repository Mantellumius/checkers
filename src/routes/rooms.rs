use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use std::{io, sync::Arc};
use tokio::sync::broadcast;

use crate::{
    engine::Board,
    store::Store,
    templates::{BoardTemplate, RoomTemplate, Side},
    AppState, Room,
};

pub struct RoomsRouter {}

impl RoomsRouter {
    pub fn get() -> Router<Arc<AppState>> {
        Router::new().nest(
            "/:id",
            Router::new()
                .route("/", get(Self::get_room))
                .route("/reset", post(Self::reset_room)),
        )
    }

    pub async fn get_room(
        Path(id): Path<String>,
        State(state): State<Arc<AppState>>,
    ) -> impl IntoResponse {
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
        let mut senders = state.rooms.lock().await;
        let (tx, _rx) = broadcast::channel(100);
        senders.insert(id.clone(), tx);
        RoomTemplate {
            title: format!("Room {}", id.clone()),
            board: BoardTemplate::new(&room.board, id.clone(), None),
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
            board: BoardTemplate::new(&new_room.board, id, None),
            side: Side::White,
        }
    }
}
