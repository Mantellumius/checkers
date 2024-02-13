use std::sync::Arc;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    routing::post,
    Form, Router,
};
use serde::Deserialize;

use crate::{engine::Engine, store::Store, templates::BoardTemplate, utility::Point, AppState};

pub struct GamesRouter {}

impl GamesRouter {
    pub fn get() -> Router<Arc<AppState>> {
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
        let room = Store::get_room(&id).unwrap();
        let from = Point::new(body.clicked_x, body.clicked_y);
        let board = Engine::with_legal_moves(room.board, from);
        BoardTemplate::new(&board, room.id, Some(from))
    }

    async fn make_move(
        Path(id): Path<String>,
        State(state): State<Arc<AppState>>,
        Form(body): Form<MakeMoveBody>,
    ) -> impl IntoResponse {
        let room = Store::get_room(&id).unwrap();
        let from = Point::new(body.selected_x, body.selected_y);
        let to: Point = Point::new(body.clicked_x, body.clicked_y);
        let board = Engine::make_move(room.board, from, to);
        Store::update_board(room.id.clone(), board.clone()).unwrap();
        let board = BoardTemplate::new(&board, room.id, None);
        let senders = state.rooms.lock().await;
        let sender = senders.get(&id).unwrap();
        if let Err(e) = sender.send(board.render().unwrap()) {
            println!("Error sending message: {}", e);
        }
        board
    }
}

#[derive(Deserialize)]
struct GetLegalMovesBody {
    clicked_x: i8,
    clicked_y: i8,
}

#[derive(Deserialize)]
struct MakeMoveBody {
    selected_x: i8,
    selected_y: i8,
    clicked_x: i8,
    clicked_y: i8,
}
