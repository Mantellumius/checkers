use askama::Template;
use serde::{Deserialize, Serialize};

mod board_template;

pub use board_template::BoardTemplate;

use crate::Room;

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

impl From<Room> for RoomTemplate {
    fn from(value: Room) -> Self {
        RoomTemplate {
            id: value.id.clone(),
            title: value.id.clone(),
            board: BoardTemplate {
                board: value.board,
                id: value.id.clone(),
            },
        }
    }
}
