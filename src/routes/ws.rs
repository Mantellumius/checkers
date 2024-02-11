use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{
        ws::{self, WebSocket},
        Path, WebSocketUpgrade,
    },
    routing::get,
    Router,
};

use crate::{store::Store, templates::BoardTemplate};

pub struct WSRouter {}

impl WSRouter {
    pub fn get() -> Router {
        Router::new().nest("/", Router::new().route("/rooms/:id", get(Self::handle_ws)))
    }

    async fn handle_ws(ws: WebSocketUpgrade, Path(id): Path<String>) -> impl IntoResponse {
        ws.on_upgrade(move |socket| Self::handle_socket(socket, id))
    }

    async fn handle_socket(mut socket: WebSocket, id: String) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            let room = Store::get_room(&id).unwrap();
            let board_template = BoardTemplate::new(&room.board, room.id.clone(), None);
            if let Err(e) = socket
                .send(ws::Message::Text(board_template.render().unwrap()))
                .await
            {
                eprintln!("Error sending message: {e}");
            }

            interval.tick().await;
        }
    }
}
