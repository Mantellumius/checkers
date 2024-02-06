use askama::Template;
use serde::{Deserialize, Serialize};

use crate::{engine::Board, utility::Point, Room};

#[derive(Deserialize, Template, Serialize, Default)]
#[template(path = "components/board.html", print = "code")]
pub struct BoardTemplate {
    pub board: Board,
    pub id: String,
    pub selected_point: Option<Point>,
}

impl From<&Room> for BoardTemplate {
    fn from(value: &Room) -> Self {
        Self {
            board: value.board.clone(),
            id: value.id.clone(),
            selected_point: None,
        }
    }
}
