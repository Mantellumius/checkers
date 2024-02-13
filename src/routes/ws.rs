use std::sync::Arc;

use askama_axum::IntoResponse;
use axum::{
    extract::{
        ws::{self},
        Path, State, WebSocketUpgrade,
    },
    routing::get,
    Router,
};
use tracing::event;

use crate::AppState;

pub struct WSRouter {}

impl WSRouter {
    pub fn get() -> Router<Arc<AppState>> {
        Router::new().nest("/", Router::new().route("/rooms/:id", get(Self::handle_ws)))
    }

    async fn handle_ws(
        ws: WebSocketUpgrade,
        Path(id): Path<String>,
        State(state): State<Arc<AppState>>,
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| Self::handle_socket(socket, state, id))
    }

    async fn handle_socket(mut socket: ws::WebSocket, state: Arc<AppState>, id: String) {
        let senders = state.rooms.lock().await;
        let sender = senders.get(&id).unwrap();
        let mut rx: tokio::sync::broadcast::Receiver<String> = sender.subscribe();
        drop(senders);
        while let Ok(message) = rx.recv().await {
            if let Err(e) = socket
                .send(ws::Message::Text(message))
                .await
            {
                event!(tracing::Level::ERROR, "Error sending message by websocket: {e}");
            }
        }
    }
}
