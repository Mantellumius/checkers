use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::Checker;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Cell {
    Empty,
    Move,
    Capture,
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
        matches!(self, Cell::Move | Cell::Capture)
    }

    pub fn is_enemy(&self, other_cell: Cell) -> bool {
        match (self, other_cell) {
            (Cell::Checker(checker), Cell::Checker(other_checker)) => checker.is_enemy(other_checker),
            _ => false,
        }
    }
    
    pub fn promote(&self) -> Self {
        if let Cell::Checker(checker) = self {
            return Cell::Checker(checker.promote());
        }
        *self
    }
    
    pub fn is_queen(&self) -> bool {
        match self {
            Cell::Checker(checker) => checker.is_queen(),
            _ => false,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Move => write!(f, "M"),
            Cell::Capture => write!(f, "C"),
            Cell::Checker(checker) => write!(f, "{:?}", checker),
        }
    }
}
