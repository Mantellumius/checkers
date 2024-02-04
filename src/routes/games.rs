use askama_axum::IntoResponse;
use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Router,
};
use std::collections::HashMap;

use crate::{store::Store, templates::BoardTemplate, utility::Point};

pub struct GamesRouter {}

impl GamesRouter {
    pub fn get() -> Router {
        Router::new().nest(
            "/:id",
            Router::new()
                .route("/moves", get(Self::get_legal_moves))
                .route("/make_move", post(Self::make_move)),
        )
    }

    async fn get_legal_moves(
        Path(id): Path<String>,
        Query(mut query): Query<HashMap<String, i8>>,
    ) -> impl IntoResponse {
        let room = Store::get_room(id).unwrap();
        let x = query.remove("x").unwrap();
        let y = query.remove("y").unwrap();
        // let new_board = room.board.with_captures(Point { x, y });
        let new_board = room.board.with_legal_moves(Point { x, y });
        Store::update_board(room.id.clone(), new_board.clone());
        BoardTemplate {
            id: room.id,
            board: new_board,
        }
    }

    async fn make_move(
        Path(id): Path<String>,
        Query(mut query): Query<HashMap<String, i8>>,
    ) -> impl IntoResponse {
        let room = Store::get_room(id).unwrap();
        let x = query.remove("x").unwrap();
        let y = query.remove("y").unwrap();
        let new_board = room.board.make_move(Point { x, y });
        Store::update_board(room.id.clone(), new_board.clone());
        BoardTemplate {
            id: room.id,
            board: new_board,
        }
    }
}
