use askama::Template;
use serde::{Deserialize, Serialize};

use crate::{engine::Turn, Cell};

#[derive(Deserialize, Template, Serialize, Default, Clone)]
#[template(path = "components/cell.html")]
pub struct CellTemplate {
    pub cell: Cell,
    pub x: usize,
    pub y: usize,
    pub turn: Turn,
    pub id: String,
    pub is_selected: bool,
}
