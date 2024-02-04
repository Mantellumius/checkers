use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Checker {
    White,
    Black,
    WhiteQueen,
    BlackQueen,
}

impl Checker {
    pub fn is_black(&self) -> bool {
        matches!(self, Checker::Black | Checker::BlackQueen)
    }

    pub fn is_white(&self) -> bool {
        matches!(self, Checker::White | Checker::WhiteQueen)
    }

    pub fn is_enemy(&self, other_checker: Checker) -> bool {
        (self.is_black() && other_checker.is_white())
            || (self.is_white() && other_checker.is_black())
    }

    pub fn promote(&self) -> Self {
        match self {
            Checker::White => Checker::WhiteQueen,
            Checker::Black => Checker::BlackQueen,
            _ => *self,
        }
    }
}

impl Display for Checker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Checker::White => write!(f, "w"),
            Checker::Black => write!(f, "b"),
            Checker::WhiteQueen => write!(f, "W"),
            Checker::BlackQueen => write!(f, "B"),
        }
    }
}
