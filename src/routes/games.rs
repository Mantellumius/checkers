use std::collections::HashMap;

use askama_axum::IntoResponse;
use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};

use crate::{store::Store, templates::BoardTemplate};

pub struct GamesRouter {}

impl GamesRouter {
    pub fn get() -> Router {
        Router::new().nest(
            "/:id/moves",
            Router::new().route("/", get(Self::get_legal_moves)),
        )
    }

    pub async fn get_legal_moves(
        Path(id): Path<String>,
        Query(mut query): Query<HashMap<String, usize>>,
    ) -> impl IntoResponse {
        let room = Store::get_room(id).unwrap();
        let x = query.remove("x").unwrap();
        let y = query.remove("y").unwrap();
        let new_board = room.board.with_legal_moves(x, y);
        BoardTemplate {
            id: room.id,
            board: new_board,
        }
    }
}
