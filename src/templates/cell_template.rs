use askama::Template;
use serde::{Deserialize, Serialize};

use crate::{engine::Turn, utility::Point, Cell};

#[derive(Deserialize, Template, Serialize, Default, Clone)]
#[template(path = "components/cell.html")]
pub struct CellTemplate {
    pub cell: Cell,
    pub x: usize,
    pub y: usize,
    pub turn: Turn,
    pub id: String,
    pub is_selected: bool,
    pub selected_point: Option<Point>,
}
