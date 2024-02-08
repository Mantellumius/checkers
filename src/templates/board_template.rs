use askama::Template;
use serde::{Deserialize, Serialize};

use crate::{engine::Board, utility::Point, Room};

#[derive(Deserialize, Template, Serialize, Default)]
#[template(path = "components/board.html")]
pub struct BoardTemplate {
    pub id: String,
    pub board: Board,
    pub selected_point: Option<Point>,
}

impl BoardTemplate {
    pub fn is_selected(&self, x: &usize, y: &usize) -> bool {
        self.selected_point
            .map_or(false, |p| p.x == *x as i8 && p.y == *y as i8)
    }
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
