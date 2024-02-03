use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::Checker;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Board {
    pub cells: [[Checker; 8]; 8],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[Checker::Empty; 8]; 8],
        }
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self::default();
        for i in 0..8 {
            for j in 0..8 {
                if (i + j) % 2 != 0 {
                    match i {
                        0..=2 => board.cells[i][j] = Checker::Black,
                        5..=7 => board.cells[i][j] = Checker::White,
                        _ => continue,
                    }
                }
            }
        }
        board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for checker in row {
                write!(f, "{}", checker)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
