use askama::Template;
use serde::{Deserialize, Serialize};

use crate::Cell;

#[derive(Deserialize, Template, Serialize, Default)]
#[template(path = "components/cells.html")]
pub struct CellsTemplate {
    pub cells: Vec<Vec<Cell>>,
}
