use askama::Template;
use serde::{Deserialize, Serialize};

use crate::engine::Board;

#[derive(Deserialize, Template, Serialize, Default)]
#[template(path = "components/board.html", print="code")]
pub struct BoardTemplate {
    pub board: Board,
    pub id: String,
}
