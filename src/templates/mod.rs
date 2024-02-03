use askama::Template;
use serde::{Deserialize, Serialize};

use crate::engine::Board;

#[derive(Deserialize, Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate {
    pub title: String,
}

#[derive(Deserialize, Template, Serialize)]
#[template(path = "pages/room.html")]
pub struct RoomTemplate {
    pub id: String,
    pub title: String,
    pub board: BoardTemplate,
}

#[derive(Deserialize, Template, Serialize, Default)]
#[template(path = "components/board.html")]
pub struct BoardTemplate {
    pub board: Board,
}