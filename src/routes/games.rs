use askama_axum::IntoResponse;
use axum::{extract::Path, routing::post, Form, Router};
use serde::Deserialize;

use crate::{engine::Engine, store::Store, templates::BoardTemplate, utility::Point};

pub struct GamesRouter {}

impl GamesRouter {
    pub fn get() -> Router {
        Router::new().nest(
            "/:id",
            Router::new()
                .route("/moves", post(Self::get_legal_moves))
                .route("/make_move", post(Self::make_move)),
        )
    }

    async fn get_legal_moves(
        Path(id): Path<String>,
        Form(body): Form<GetLegalMovesBody>,
    ) -> impl IntoResponse {
        let room = Store::get_room(id).unwrap();
        let from = Point::new(body.from_x, body.from_y);
        let board = Engine::with_legal_moves(room.board, from);
        BoardTemplate {
            id: room.id,
            board,
            selected_point: Some(from),
        }
    }

    async fn make_move(
        Path(id): Path<String>,
        Form(body): Form<MakeMoveBody>,
    ) -> impl IntoResponse {
        let room = Store::get_room(id).unwrap();
        let from = Point::new(body.from_x, body.from_y);
        let to: Point = Point::new(body.to_x, body.to_y);
        let board = Engine::make_move(room.board, from, to);
        Store::update_board(room.id.clone(), board.clone());
        BoardTemplate {
            id: room.id,
            board,
            selected_point: None,
        }
    }
}

#[derive(Deserialize)]
struct GetLegalMovesBody {
    from_x: i8,
    from_y: i8,
}

#[derive(Deserialize)]
struct MakeMoveBody {
    from_x: i8,
    from_y: i8,
    to_x: i8,
    to_y: i8,
}
