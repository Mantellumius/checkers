use askama::Template;
use serde::{Deserialize, Serialize};

use crate::Room;

use super::{BoardTemplate, Side};

#[derive(Deserialize, Template, Serialize)]
#[template(path = "pages/room.html")]
pub struct RoomTemplate {
    pub id: String,
    pub title: String,
    pub board: BoardTemplate,
    pub side: Side,
}

impl From<Room> for RoomTemplate {
    fn from(value: Room) -> Self {
        RoomTemplate {
            id: value.id.clone(),
            title: value.id.clone(),
            board: BoardTemplate::from(&value),
            side: Side::White,
        }
    }
}
