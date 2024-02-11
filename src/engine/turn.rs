use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default, Copy)]
pub enum Turn {
    #[default]
    White,
    Black,
}

impl Turn {
    pub fn next(&self) -> Self {
        match self {
            Turn::Black => Turn::White,
            Turn::White => Turn::Black,
        }
    }

    pub fn is_black(&self) -> bool {
        matches!(self, Turn::Black)
    }

    pub fn is_white(&self) -> bool {
        matches!(self, Turn::White)
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Turn::Black => "Turn::Black",
                Turn::White => "Turn::White",
            }
        )
    }
}
