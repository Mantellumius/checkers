use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::Checker;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Cell {
    Empty,
    Move,
    Checker(Checker),
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    pub fn is_checker(&self) -> bool {
        matches!(self, Cell::Checker(_))
    }

    pub fn is_move(&self) -> bool {
        matches!(self, Cell::Move)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Move => write!(f, "M"),
            Cell::Checker(checker) => write!(f, "{:?}", checker),
        }
    }
}
