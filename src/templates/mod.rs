use askama::Template;
use serde::{Deserialize, Serialize};

mod board_template;

pub use board_template::BoardTemplate;

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
