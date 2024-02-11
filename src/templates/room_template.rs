use askama::Template;
use serde::{Deserialize, Serialize};

use super::{BoardTemplate, Side};

#[derive(Deserialize, Template, Serialize)]
#[template(path = "pages/room.html")]
pub struct RoomTemplate {
    pub id: String,
    pub title: String,
    pub board: BoardTemplate,
    pub side: Side,
}
