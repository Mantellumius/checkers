use askama::Template;
use serde::{Deserialize, Serialize};

mod board_template;
mod cell_template;
mod room_template;

pub use board_template::BoardTemplate;
pub use cell_template::CellTemplate;
pub use room_template::RoomTemplate;

#[derive(Deserialize, Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub rooms: Vec<RoomHrefTemplate>,
}

#[derive(Deserialize, Serialize, Clone, Copy, Default)]
pub enum Side {
    #[default]
    White,
    Black,
}

#[derive(Deserialize, Serialize, Clone, Template)]
#[template(path = "components/room_href.html")]
pub struct RoomHrefTemplate {
    pub id: String,
    pub title: String,
}
