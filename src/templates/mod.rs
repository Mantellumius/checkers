use askama::Template;
use serde::{Deserialize, Serialize};

mod board_template;
mod room_template;

pub use board_template::BoardTemplate;
pub use room_template::RoomTemplate;

#[derive(Deserialize, Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub rooms: Vec<RoomHref>,
}

#[derive(Deserialize, Serialize, Clone, Copy, Default)]
pub enum Side {
    #[default]
    White,
    Black,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RoomHref {
    pub id: String,
    pub title: String,
}