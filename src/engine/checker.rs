use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Checker {
    Empty,
    White,
    Black,
    WhiteQueen,
    BlackQueen,
}

impl Display for Checker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Checker::Empty => write!(f, " "),
            Checker::White => write!(f, "w"),
            Checker::Black => write!(f, "b"),
            Checker::WhiteQueen => write!(f, "W"),
            Checker::BlackQueen => write!(f, "B"),
        }
    }
}
